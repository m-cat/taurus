# Taurus

[![Build Status](https://travis-ci.com/m-cat/taurus.svg?branch=master)](https://travis-ci.com/m-cat/taurus)
[![crates.io](https://img.shields.io/crates/v/taurus.svg)](https://crates.io/crates/taurus)
[![Downloads](https://img.shields.io/crates/d/taurus.svg)](https://crates.io/crates/taurus)
[![Issues](https://img.shields.io/github/issues-raw/m-cat/taurus.svg)](https://github.com/m-cat/taurus/issues)
[![LoC](https://tokei.rs/b1/github/m-cat/taurus)](https://github.com/m-cat/taurus)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A roguelike game made in Rust!

![Taurus](https://github.com/m-cat/taurus/blob/master/screenshot.png)

## Instructions

Make sure you have the build dependencies:

* Rust: [Install](https://www.rust-lang.org/en-US/install.html)
* Libtcod's dependencies: [Install](https://github.com/tomassedovic/tcod-rs#how-to-use-this)

Download...

```
cargo install taurus
```

... and play:

```
taurus
```

### Running from source

Download the source:

```
git clone https://github.com/m-cat/taurus.git
cd taurus
```

Run the game:

```
cargo run --release
```

## About

A WIP roguelike game being developed in Rust. If you see anything that can be improved, please submit an issue.

### Current State

Basic demo is working. You can move around with the arrow keys.

I learned Rust by doing this project but I don't have enough interest in roguelikes to continue development :)

### Todo

* Implement the UI.
* Messages system.
* Examining tiles with mouse.
* Enemies/AI.

## Disclaimer

I'm not responsible for anything.

(c) 2017 Marcin Swieczkowski
