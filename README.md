# Awesome Alternatives in Rust
[![github workflow status](https://img.shields.io/github/workflow/status/TaKO8Ki/awesome-alternatives-in-rust/CI/main)](https://github.com/TaKO8Ki/awesome-alternatives-in-rust/actions) [![stakes.social](https://badge.devprotocol.xyz/0xC65ca20B670aa65fDd05DEBAFBF02D3387F316Cd/descriptive)](https://stakes.social/0xC65ca20B670aa65fDd05DEBAFBF02D3387F316Cd)

A curated list of replacements for existing software written in Rust.

If you want to contribute, please read [CONTRIBUTING.md](CONTRIBUTING.md).

I renamed the repository to "Awesome Alternatives in Rust". The original name was "Awesome Rewrite It In Rust". For more details, please refer to [this issue](https://github.com/TaKO8Ki/awesome-alternatives-in-rust/issues/29).

## Table of contents

- [Applications](#applications)
  - [Container](#container)
  - [Games](#games)
  - [Performance](#performance)
  - [System tools](#system-tools)
  - [Terminal](#terminal)
  - [Text editors](#text-editors)
  - [Text processing](#text-processing)
  - [Utilities](#utilities)
  - [Web](#web)
- [Development tools](#development-tools)
  - [Command runners](#command-runners)
  - [Linters](#linters)
- [Libraries](#libraries)
  - [Email](#email)

## Applications

### Container

#### [runc](https://github.com/opencontainers/runc)

* [youki](https://github.com/containers/youki) - An experimental container runtime written in Rust

### Games

#### [Stockfish](https://github.com/official-stockfish/Stockfish/)

* [Pleco](https://github.com/sfleischman105/Pleco) - A Rust-based re-write of the Stockfish Chess Engine 

### Performance

#### [jMeter](https://github.com/apache/jmeter)

* [drill](https://github.com/fcsonline/drill) - A HTTP load testing application written in Rust

### System tools

#### autojump / z

* [zoxide](https://github.com/ajeetdsouza/zoxide) - A smarter cd command for your terminal.

#### awk

* [frawk](https://github.com/ezrosent/frawk) - an efficient awk-like language

#### bash/PowerShell/fish

* [nushell](https://github.com/nushell/nushell/) - An attractive structured shell

#### bc

* [cpc](https://github.com/probablykasper/cpc) - Text calculator with support for units and conversion
* [eva](https://github.com/NerdyPepper/eva) - a calculator REPL, similar to bc(1) 

#### cat

* [bat](https://github.com/sharkdp/bat) - A cat(1) clone with wings.

#### [cloc](https://github.com/AlDanial/cloc)

* [loc](https://github.com/cgag/loc) - Count lines of code quickly.
* [tokei](https://github.com/XAMPPRocky/tokei) - Count your code, quickly.

#### [coreboot](https://github.com/coreboot/coreboot)

* [oreboot](https://github.com/oreboot/oreboot) - oreboot is a fork of coreboot, with C removed, written in Rust.

#### cp

* [xcp](https://github.com/tarka/xcp) - An extended `cp`

#### cut

* [choose](https://github.com/theryangeary/choose) - A human-friendly and fast alternative to cut and (sometimes) awk

#### diff

* [delta](https://github.com/dandavison/delta) - A viewer for git and diff output

#### dig

* [dog](https://github.com/ogham/dog) - A command-line DNS client.

#### du

* [dua](https://github.com/Byron/dua-cli) - View disk space usage and delete unwanted data, fast.
* [dust](https://github.com/bootandy/dust) - A more intuitive version of du in rust

#### find

* [fd](https://github.com/sharkdp/fd) - A simple, fast and user-friendly alternative to 'find'

#### [fzf](https://github.com/junegunn/fzf)

* [skim](https://github.com/lotabout/skim) - Fuzzy Finder in rust!

#### [GNU coreutils](https://github.com/coreutils/coreutils)

* [coreutils](https://github.com/uutils/coreutils) - Cross-platform Rust rewrite of the GNU coreutils

#### hexdump

* [hexyl](https://github.com/sharkdp/hexyl) - A command-line hex viewer

#### [httpie](https://github.com/httpie/httpie)

* [xh](https://github.com/ducaale/xh) - Friendly and fast tool for sending HTTP requests

#### ls

* [exa](https://github.com/ogham/exa) - A replacement for 'ls'
* [lsd](https://github.com/Peltoche/lsd) - An ls with a lot of pretty colors and awesome icons
* [nat](https://github.com/willdoescode/nat) - `ls` alternative with useful info and a splash of color 🎨

#### [nvm](https://github.com/nvm-sh/nvm)

* [fnm](https://github.com/Schniz/fnm) - 🚀 Fast and simple Node.js version manager, built in Rust
* [volta](https://github.com/volta-cli/volta) - Volta: JS Toolchains as Code. ⚡

#### ps

* [procs](https://github.com/dalance/procs) - A modern replacement for ps written in Rust

#### [rbenv](https://github.com/rbenv/rbenv)

* [frum](https://github.com/TaKO8Ki/frum) - A little bit fast and modern Ruby version manager written in Rust

#### rename

* [rnr](https://github.com/ChuckDaniels87/rnr) - A command-line tool to batch rename files and directories

#### rm

* [rip](https://github.com/nivekuil/rip) - A safe and ergonomic alternative to rm

#### sed

* [sad](https://github.com/ms-jpq/sad) - CLI search and replace | Space Age seD
* [sd](https://github.com/chmln/sd) - Intuitive find & replace CLI (sed alternative)

#### strings

* [stringsext](https://github.com/getreu/stringsext) - Find multi-byte-encoded strings in binary data

#### sudo

* [please](https://gitlab.com/edneville/please) - `sudo` like program with regex support written in rust

#### time

* [hyperfine](https://github.com/sharkdp/hyperfine) - A command-line benchmarking tool

#### [tldr](https://github.com/tldr-pages/tldr)

* [navi](https://github.com/denisidoro/navi) - An interactive cheatsheet tool for the command-line
* [tealdeer](https://github.com/dbrgn/tealdeer) - A very fast implementation of tldr in Rust.

#### top

* [bottom](https://github.com/ClementTsang/bottom) - Yet another cross-platform graphical process/system monitor.
* [ytop](https://github.com/cjbassi/ytop) (no longer maintained) - A TUI system monitor written in Rust

#### uniq

* [huniq](https://github.com/koraa/huniq) - Filter out duplicates on the command line.

#### [yay](https://github.com/Jguer/yay)

* [paru](https://github.com/Morganamilo/paru) - Feature packed AUR helper

### Terminal

#### [Spaceship](https://github.com/spaceship-prompt/spaceship-prompt)

* [starship](https://github.com/starship/starship) - ☄️🌌 The minimal, blazing-fast, and infinitely customizable prompt for any shell!

#### [termite](https://github.com/thestinger/termite)

* [Alacritty](https://github.com/alacritty/alacritty) - A cross-platform, OpenGL terminal emulator.

#### [tmux](https://github.com/tmux/tmux)

* [Zellij](https://github.com/zellij-org/zellij) - A terminal workspace with batteries included

### Text editors

#### Vim

* [Amp](https://github.com/jmacdonald/amp) - A complete text editor for your terminal.

### Text processing

#### grep

* [ripgrep](https://github.com/BurntSushi/ripgrep) - ripgrep recursively searches directories for a regex pattern while respecting your gitignore

### Utilities

#### [codemod](https://github.com/facebookarchive/codemod)

* [fastmod](https://github.com/facebookincubator/fastmod) - A fast partial replacement for the codemod tool

#### [jq](https://github.com/stedolan/jq)

* [jql](https://github.com/yamafaktory/jql) - A JSON Query Language CLI tool built with Rust 🦀

#### [lazygit](https://github.com/jesseduffield/lazygit)

* [gitui](https://github.com/extrawurst/gitui) - Blazing fast terminal-ui for git written in Rust 🦀

### Web

#### [Reddit](https://www.reddit.com/)

* [Lemmy](https://github.com/LemmyNet/lemmy) - 🐀 Building a federated alternative to reddit in rust

#### [teddit](https://codeberg.org/teddit/teddit)

* [libreddit](https://github.com/spikecodes/libreddit) - Private front-end for Reddit written in Rust

## Development tools

### Command runners

#### make

* [just](https://github.com/casey/just) - A command runner and partial replacement for `make`

### Linters

#### [ESLint](https://github.com/eslint/eslint)

* [deno_lint](https://github.com/denoland/deno_lint) - Blazing fast linter for JavaScript and TypeScript written in Rust
* [RSLint](https://github.com/rslint/rslint) - A (WIP) Extremely fast JavaScript and TypeScript linter and Rust crate

#### [ShellCheck](https://github.com/koalaman/shellcheck)

* [Shellharden](https://github.com/anordal/shellharden) - The corrective bash syntax highlighter

## Libraries

### Email

#### [mjml](https://github.com/mjmlio/mjml)

* [mrml](https://github.com/jdrouet/mrml) - Blazing fast reimplementation of mjml in Rust (~200x faster)
