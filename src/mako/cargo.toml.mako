[package]

name = "youtube3-dev"
version = "0.0.1"
authors = ["Sebastian Thiel <byronimo@gmail.com>"]
description = "A library to facilitate interacting with your youtube account"
repository = "https://github.com/Byron/youtube-rs"
license = "MIT"
keywords = ["youtube", "google", "protocol"]

[dependencies]
# Just to get hyper to work !
openssl = "= 0.4.3"
# Just to get hyper to work !
cookie = "= 0.1.13"
hyper = "*"
rustc-serialize = "*"
yup-oauth2 = "*"

[dev-dependencies]
yup-hyper-mock = "*"
