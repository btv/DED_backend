# Daily Exercise Diary - Backend
[![Build Status](https://travis-ci.com/btv/DED_backend.svg?token=cBa9dzkvRhGroiTNpmQn&branch=master)](https://travis-ci.com/btv/DED_backend)

This repo is for the DED backend repository.

The main language of the backend is [Rust](https://www.rust-lang.org/). The choosen web framework for this language is [actix-web](https://github.com/actix/actix-web) framework. The library for database interaction is performed by [diesel](https://diesel.rs/).

## Requirements
* Rust - stable
    * the [cargo](https://doc.rust-lang.org/cargo/) executable. 
* this repo!

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