use chrono::{DateTime, Duration, Local};
use failure::{format_err, Error, Fail};
use futures::future::{select_all, BoxFuture, FutureExt};
use lazy_static::lazy_static;
use log::{debug, info, warn};
use pulldown_cmark::{Event, Parser, Tag};
use regex::Regex;
use reqwest::{header, redirect::Policy, Client, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::Write;
use std::time;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;

#[derive(Debug, Fail, Serialize, Deserialize)]
enum CheckerError {
    #[fail(display = "failed to try url")]
    NotTried, // Generally shouldn't happen, but useful to have

    #[fail(display = "http error: {}", status)]
    HttpError {
        status: u16,
        location: Option<String>,
    },

    #[fail(display = "reqwest error: {}", error)]
    ReqwestError { error: String },

    #[fail(display = "travis build is unknown")]
    TravisBuildUnknown,

    #[fail(display = "travis build image with no branch")]
    TravisBuildNoBranch,

    #[fail(display = "github actions image with no branch")]
    GithubActionNoBranch,
}

fn formatter(err: &CheckerError, url: &String) -> String {
    match err {
        CheckerError::HttpError { status, location } => match location {
            Some(loc) => {
                format!("[{}] {} -> {}", status, url, loc)
            }
            None => {
                format!("[{}] {}", status, url)
            }
        },
        CheckerError::TravisBuildUnknown => {
            format!("[Unknown travis build] {}", url)
        }
        CheckerError::TravisBuildNoBranch => {
            format!("[Travis build image with no branch specified] {}", url)
        }
        CheckerError::GithubActionNoBranch => {
            format!("[Github action image with no branch specified] {}", url)
        }
        _ => {
            format!("{:?}", err)
        }
    }
}

struct MaxHandles {
    remaining: Semaphore,
}

struct Handle<'a> {
    _permit: SemaphorePermit<'a>,
}

impl MaxHandles {
    fn new(max: usize) -> MaxHandles {
        MaxHandles {
            remaining: Semaphore::new(max),
        }
    }

    async fn get<'a>(&'a self) -> Handle<'a> {
        let permit = self.remaining.acquire().await;
        return Handle { _permit: permit };
    }
}

impl<'a> Drop for Handle<'a> {
    fn drop(&mut self) {
        debug!("Dropping");
    }
}

lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .danger_accept_invalid_certs(true) // because some certs are out of date
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:68.0) Gecko/20100101 Firefox/68.0") // so some sites (e.g. sciter.com) don't reject us
        .redirect(Policy::none())
        .max_idle_per_host(0)
        .timeout(time::Duration::from_secs(20))
        .build().unwrap();

    // This is to avoid errors with running out of file handles, so we only do 20 requests at a time
    static ref HANDLES: MaxHandles = MaxHandles::new(20);
}

fn get_url(url: String) -> BoxFuture<'static, (String, Result<(), CheckerError>)> {
    debug!("Need handle for {}", url);
    async move {
        let _handle = HANDLES.get().await;
        return get_url_core(url).await;
    }
    .boxed()
}

fn get_url_core(url: String) -> BoxFuture<'static, (String, Result<(), CheckerError>)> {
    async move {
        let mut res = Err(CheckerError::NotTried);
        for _ in 0..5u8 {
            debug!("Running {}", url);
            lazy_static! {
                static ref GITHUB_REPO_REGEX: Regex = Regex::new(r"^https://github.com/(?P<org>[^/]+)/(?P<repo>[^/]+)$").unwrap();
                static ref GITHUB_API_REGEX: Regex = Regex::new(r"https://api.github.com/").unwrap();
            }
            if env::var("GITHUB_USERNAME").is_ok() && env::var("GITHUB_TOKEN").is_ok() && GITHUB_REPO_REGEX.is_match(&url) {
                let rewritten = GITHUB_REPO_REGEX.replace_all(&url, "https://api.github.com/repos/$org/$repo");
                info!("Replacing {} with {} to workaround rate limits on Github", url, rewritten);
                let (_new_url, res) = get_url_core(rewritten.to_string()).await;
                return (url, res);
            }
            let mut req = CLIENT
                .get(&url)
                .header(header::ACCEPT, "image/svg+xml, text/html, */*;q=0.8");

            if GITHUB_API_REGEX.is_match(&url) {
                if let Ok(username) = env::var("GITHUB_USERNAME") {
                    if let Ok(password) = env::var("GITHUB_TOKEN") {
                        // needs a token with at least public_repo scope
                        info!("Using basic auth for {}", url);
                        req = req.basic_auth(username, Some(password));
                    }
                }
            }

            let resp = req.send().await;
            match resp {
                Err(err) => {
                    warn!("Error while getting {}, retrying: {}", url, err);
                    res = Err(CheckerError::ReqwestError{error: err.to_string()});
                    continue;
                }
                Ok(ok) => {
                    let status = ok.status();
                    if status != StatusCode::OK {
                        lazy_static! {
                            static ref ACTIONS_REGEX: Regex = Regex::new(r"https://github.com/(?P<org>[^/]+)/(?P<repo>[^/]+)/actions(?:\?workflow=.+)?").unwrap();
                            static ref YOUTUBE_REGEX: Regex = Regex::new(r"https://www.youtube.com/watch\?v=(?P<video_id>.+)").unwrap();
                            static ref AZURE_BUILD_REGEX: Regex = Regex::new(r"https://dev.azure.com/[^/]+/[^/]+/_build").unwrap();
                        }
                        if status == StatusCode::NOT_FOUND && ACTIONS_REGEX.is_match(&url) {
                            let rewritten = ACTIONS_REGEX.replace_all(&url, "https://github.com/$org/$repo");
                            warn!("Got 404 with Github actions, so replacing {} with {}", url, rewritten);
                            let (_new_url, res) = get_url_core(rewritten.to_string()).await;
                            return (url, res);
                        }
                        if status == StatusCode::FOUND && YOUTUBE_REGEX.is_match(&url) {
                            // Based off of https://gist.github.com/tonY1883/a3b85925081688de569b779b4657439b
                            // Guesswork is that the img feed will cause less 302's than the main url
                            // See https://github.com/rust-unofficial/awesome-rust/issues/814 for original issue
                            let rewritten = YOUTUBE_REGEX.replace_all(&url, "http://img.youtube.com/vi/$video_id/mqdefault.jpg");
                            warn!("Got 302 with Youtube, so replacing {} with {}", url, rewritten);
                            let (_new_url, res) = get_url_core(rewritten.to_string()).await;
                            return (url, res);
                        };
                        if status == StatusCode::FOUND && AZURE_BUILD_REGEX.is_match(&url) {
                            // Azure build urls always redirect to a particular build id, so no stable url guarantees
                            let redirect = ok.headers().get(header::LOCATION).unwrap().to_str().unwrap();
                            let merged_url = Url::parse(&url).unwrap().join(redirect).unwrap();
                            info!("Got 302 from Azure devops, so replacing {} with {}", url, merged_url);
                            let (_new_url, res) = get_url_core(merged_url.into()).await;
                            return (url, res);
                        }

                        warn!("Error while getting {}, retrying: {}", url, status);
                        if status.is_redirection() {
                            res = Err(CheckerError::HttpError {status: status.as_u16(), location: ok.headers().get(header::LOCATION).and_then(|h| h.to_str().ok()).map(|x| x.to_string())});
                            break;
                        } else {
                            res = Err(CheckerError::HttpError {status: status.as_u16(), location: None});
                            continue;
                        }
                    }
                    lazy_static! {
                        static ref TRAVIS_IMG_REGEX: Regex = Regex::new(r"https://api.travis-ci.(?:com|org)/[^/]+/.+\.svg(\?.+)?").unwrap();
                        static ref GITHUB_ACTIONS_REGEX: Regex = Regex::new(r"https://github.com/[^/]+/[^/]+/workflows/[^/]+/badge.svg(\?.+)?").unwrap();
                    }
                    if let Some(matches) = TRAVIS_IMG_REGEX.captures(&url) {
                        // Previously we checked the Content-Disposition headers, but sometimes that is incorrect
                        // We're now looking for the explicit text "unknown" in the middle of the SVG
                        let content = ok.text().await.unwrap();
                        if content.contains("unknown") {
                            res = Err(CheckerError::TravisBuildUnknown);
                            break;
                        }
                        let query = matches.get(1).map(|x| x.as_str()).unwrap_or("");
                        if !query.starts_with("?") || query.find("branch=").is_none() {
                            res = Err(CheckerError::TravisBuildNoBranch);
                            break;
                        }
                    }
                    if let Some(matches) = GITHUB_ACTIONS_REGEX.captures(&url) {
                        debug!("Github actions match {:?}", matches);
                        let query = matches.get(1).map(|x| x.as_str()).unwrap_or("");
                        if !query.starts_with("?") || query.find("branch=").is_none() {
                            res = Err(CheckerError::GithubActionNoBranch);
                            break;
                        }
                    }
                    debug!("Finished {}", url);
                    res = Ok(());
                    break;
                }
            }
        }
        (url, res)
    }.boxed()
}

#[derive(Debug, Serialize, Deserialize)]
enum Working {
    Yes,
    No(CheckerError),
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
    last_working: Option<DateTime<Local>>,
    updated_at: DateTime<Local>,
    working: Working,
}

type Results = BTreeMap<String, Link>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let markdown_input = fs::read_to_string("README.md").expect("Can't read README.md");
    let parser = Parser::new(&markdown_input);

    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut results: Results = fs::read_to_string("results/results.yml")
        .map_err(|e| format_err!("{}", e))
        .and_then(|x| serde_yaml::from_str(&x).map_err(|e| format_err!("{}", e)))
        .unwrap_or(Results::new());

    let mut url_checks = vec![];

    let min_between_checks: Duration = Duration::days(3);
    let max_allowed_failed: Duration = Duration::days(7);
    let mut do_check = |url: String| {
        if !url.starts_with("http") {
            return;
        }
        used.insert(url.clone());
        if let Some(link) = results.get(&url) {
            if let Working::Yes = link.working {
                let since = Local::now() - link.updated_at;
                if since < min_between_checks {
                    return;
                }
            }
        }
        let check = get_url(url).boxed();
        url_checks.push(check);
    };

    for (event, _) in parser.into_offset_iter() {
        match event {
            Event::Start(tag) => match tag {
                Tag::Link(_link_type, url, _title) | Tag::Image(_link_type, url, _title) => {
                    do_check(url.to_string());
                }
                _ => {}
            },
            Event::Html(content) => {
                return Err(format_err!(
                    "Contains HTML content, not markdown: {}",
                    content
                ));
            }
            _ => {}
        }
    }

    let results_keys = results.keys().cloned().collect::<BTreeSet<String>>();
    let old_links = results_keys.difference(&used);
    for link in old_links {
        results.remove(link).unwrap();
    }
    fs::write("results/results.yml", serde_yaml::to_string(&results)?)?;

    let mut not_written = 0;
    let mut last_written = Local::now();
    while url_checks.len() > 0 {
        debug!("Waiting for {}", url_checks.len());
        let ((url, res), _index, remaining) = select_all(url_checks).await;
        url_checks = remaining;
        match res {
            Ok(_) => {
                print!("\u{2714} ");
                if let Some(link) = results.get_mut(&url) {
                    link.updated_at = Local::now();
                    link.last_working = Some(Local::now());
                    link.working = Working::Yes;
                } else {
                    results.insert(
                        url.clone(),
                        Link {
                            updated_at: Local::now(),
                            last_working: Some(Local::now()),
                            working: Working::Yes,
                        },
                    );
                }
            }
            Err(err) => {
                print!("\u{2718} ");
                if let Some(link) = results.get_mut(&url) {
                    link.updated_at = Local::now();
                    link.working = Working::No(err);
                } else {
                    results.insert(
                        url.clone(),
                        Link {
                            updated_at: Local::now(),
                            working: Working::No(err),
                            last_working: None,
                        },
                    );
                }
            }
        }
        std::io::stdout().flush().unwrap();

        not_written += 1;
        let duration = Local::now() - last_written;
        if duration > Duration::seconds(5) || not_written > 20 {
            fs::write("results/results.yml", serde_yaml::to_string(&results)?)?;
            not_written = 0;
            last_written = Local::now();
        }
    }
    fs::write("results/results.yml", serde_yaml::to_string(&results)?)?;
    println!("");
    let mut failed: u32 = 0;

    for (url, link) in results.iter() {
        if let Working::No(ref err) = link.working {
            match err {
                CheckerError::HttpError { status, .. }
                    if *status == 301 || *status == 302 || *status == 404 =>
                {
                    println!("{} {:?}", url, link);
                    failed += 1;
                    continue;
                }
                _ => {}
            };
            if let Some(last_working) = link.last_working {
                let since = Local::now() - last_working;
                if since > max_allowed_failed {
                    println!("{} {:?}", url, link);
                    failed += 1;
                } else {
                    println!(
                        "Failure occurred but only {}, so we're not worrying yet: {}",
                        chrono_humanize::HumanTime::from(-since),
                        formatter(err, url)
                    );
                }
            } else {
                println!("{} {:?}", url, link);
                failed += 1;
                continue;
            }
        }
    }
    if failed == 0 {
        println!("No errors!");
        Ok(())
    } else {
        Err(format_err!("{} urls with errors", failed))
    }
}
