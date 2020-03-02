# Daily Exercise Diary - Backend
[![Build Status](https://travis-ci.com/coloradocollective/DED_Backend.svg?branch=master)](https://travis-ci.com/coloradocollective/DED_Backend)

This repo is for the DED backend repository.

The main language of the backend is [Rust](https://www.rust-lang.org/). The choosen web framework for this language is [actix-web](https://github.com/actix/actix-web) framework. The library for database interaction is performed by [diesel](https://diesel.rs/).

## Requirements
* Rust - stable
    * Current version of stable (at time of writing) is `1.41.01`
    * the [cargo](https://doc.rust-lang.org/cargo/) executable. 
* this repo!

## How to install Rust
### OSX
* Option 1: [Homebrew](https://brew.sh/)

If you have have homebrew installed, you can install Rust with the following command:

```
brew install rust
``` 
This will give you the current stable version
* Option 2: [Rustup](https://rustup.rs/)

If you don't have homebrew installed, another option is to use the rustup command to help keep your installation of rust organzied. To install rustup use this command:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After rustup is installed, you need to configure it to use the stable version, run the following commands to do so:

```
rustup toolchain install stable
rustup default stable
```
### Linux
* Option 1:  Local package manager

Please search for and install the stable version of rust used by your package manager. If for some reason the version provided by your distro is less than the stable version above, please use the second option.

* Option 2: [Rustup](https://rustup.rs/)

Please follow the OSX: Option 2 instructions above.

## How to build
After making sure that rust and cargo are installed, using the following command from the top of the repo:

```
cargo build
```

## How to run
After making sure that rust and cargo are installed, using the following command from the top of the repo:

```
cargo run
```

after cargo rebuilds the code, there should now be a service running on [your localhost](http://127.0.0.1:8080)

## How to test
After making your changes, you can run the test by using this command:

```
cargo test
```

## How to view the documentation
Cargo can automatically build the documentation for you to view and use. The best command to do this is:

```
cargo doc --open
```

After the documentation for this project and all of its dependencies has been created, the machine's default web browser will open up to the local documentation for this project.
