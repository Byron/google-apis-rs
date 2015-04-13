This repository holds [mako][mako] scripts to generate all google APIs as described by the [google discovery service][api-discovery].

The generate source code of each google API can be found in the `gen` subdirectory. Each google API resides in it's own crate which can be used as any other crate.

To find a library of your interest, you might want to proceed looking at the [API documentation index][api-index].

# Project Features

* provide an idiomatic rust implementation for google APIs
* first-class documentation with cross-links and complete code-examples
* support all features, including downloads and resumable uploads
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

# Developer Diary

Each episode sums up one major step in project development:

* [Episode 1](http://youtu.be/2U3SpepKaBE): How to write 78 APIs in 5 seconds

# Build Instructions

## Prerequisites

To generate the APIs yourself, you will need to meet the following prerequisites:

* **make**
 * Make is used to automate and efficiently call all involved programs
* **python**
 * As [*mako*][mako] is a python program, you will need python installed on your system to run it. Some other programs we call depend on python being present as well.
* **an internet connection and wget**
 * Make will download all other prerequisites automatically into hidden directories within this repository, which requires it to make some downloads via wget.

## Using Make

The makefile is written to be self-documenting. Just calling `make` will yield a list of all valid targets.

```bash
➜  google-apis-rs git:(master) make
using template engine: '.pyenv/bin/python etc/bin/mako-render'

Targets
help-api       -   show all api targets to build individually
help-cli       -   show all cli targets to build individually
docs-all       -   cargo-doc on all APIs and associates, assemble them together and generate index
docs-all-clean -   remove the entire set of generated documentation
github-pages   -   invoke ghp-import on all documentation
regen-apis     -   clear out all generated apis, and regenerate them
license        -   regenerate the main license file
update-json    -   rediscover API schema json files and update api-list.yaml with latest versions
deps           -   generate a file to tell how to build libraries and programs
help           -   print this help
make: Nothing to be done for `help'.
```

You can easily build the documentation index using `make docs-all` and individual API documentation using `make <api-name>-doc`. Run doctests on all apis with `make cargo-api ARGS=test` or on individual ones using `make <api-name>-cargo ARGS=test`. To see which API targets exist, run `make help-api`.

The same goes for commandline programs, just ust `-cli` instead of `-api`, and have a look at `help-cli` for individual targets.

## Make and parallel job execution

In theory, you can run make with `-j4` to process 4 jobs in parallel. However, please note that `cargo` may be run for some jobs and do a full build, which could cause it to fail as it will be updating it's index. The latter isn't working for multiple cargo processes at once as the index is a shared resource.

Nonetheless, once you have built all dependencies for all APIs once, you can safely run cargo in parallel, as it will not update it's index again.

In other words: The first time, you run `make docs` and `make cargo ARGS=test`, you shouldn't run things in parallel. The second time, you are free to parallelize at will.

## Adding new APIs and updating API schemas

The list of available APIs to generate is based on a query of the [Google discovery API][api-discovery], and then baked into a make-compatible dependency file. That will represents a cache, and the only way to enforce a full update is to delete it and run make again.

For example, to update all json files and possibly retrieve new API schemas, do as follows:

```bash
# -j8 will allow 8 parallel schema downloads
rm -f .api.deps .cli.deps && make update-json -j8
```

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