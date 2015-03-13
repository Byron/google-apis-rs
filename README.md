This repository holds [mako][mako] scripts to generate all google APIs as described by the [google discovery service][api-discovery].

The generate source code of each google API can be found in the `gen` subdirectory. Each google API resides in it's own crate which can be used as any other crate.

To find a library of your interest, you might want to proceed looking at the [API documentation index][api-index].

# Project Features

* provide an idiomatic rust implementation for google APIs
* first-class documentation with cross-links and complete code-examples to increase ease-of-use
* support all features, including uploads and resumable uploads
* safety and resilience are built-in, allowing you to create highly available tools on top of it. For example, you can trigger retries for all operations that may temporarily fail, e.g. due to network outage.
* *(soon)* Feature-complete command line tool to interact with each API.

# Live-Development

[![Build Status](https://travis-ci.org/Byron/google-apis-rs.svg?branch=master)](https://travis-ci.org/Byron/google-apis-rs)

All work done on the project is based on [github issues][issues], not only to track progress and show what's going on, but
to have a place to link screen-recordings to. [Milestones][milestones] are used to provide a bigger picture.

Additionally, there is a development diary which serves as summary of major steps taken so far. As opposed to issue-screencasts,
it is not made live, but is authored and narrated, which should make it more accessible.

Click the image below to see the playlist with all project related content:

[![thumb][playlist-thumb]][playlist]

# License

The license of everything not explicitly under a different license are licensed as specified in `LICENSE.md`.


[oauth]: https://crates.io/crates/yup-oauth2
[google-lic]: https://github.com/google/google-api-go-client/blob/master/LICENSE
[api-discovery-video]: https://www.youtube.com/watch?v=lQbT1NrxpUo
[api-discovery]: https://developers.google.com/discovery
[mako]: http://www.makotemplates.org/
[api-index]: http://byron.github.io/google-apis-rs
[issues]: https://github.com/Byron/google-apis-rs/issues
[playlist]: https://www.youtube.com/playlist?list=PLMHbQxe1e9Mnnqj3Hs1hRDUXFEK-TgCnz
[playlist-thumb]: http://img.youtube.com/vi/aGXuGEl90Mo/0.jpg
[milestones]: https://github.com/Byron/google-apis-rs/milestones