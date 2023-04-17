This repository holds [mako][mako] scripts to generate all Google APIs as described by the [google discovery service][api-discovery].

The generate source code of each google API can be found in the `gen` subdirectory. Each google API resides in it's own crate which can be used as any other crate.

To find a library of your interest, you might want to proceed looking at the [API documentation index][api-index].

# Maintenance Mode

These crates are considered done and only minimal time will be invested to keep them relevant. This includes the following tasks:

- reply to issues, triage them
- provide support for PRs
- occasional updates of all crates to update them with the latest API definitions, probably no more than twice a year
- dependency updates to avoid security issues and keep the crates usable in modern projects

New features will not be implemented but PRs are welcome. Please feel free to [start a discussion][discussions] to talk about desired features.

**Please be aware of the [alternative implementation][all-rust-org]** of these crates, which may be better suited for you.

[discussions]: https://github.com/Byron/google-apis-rs/discussions
[all-rust-org]: http://github.com/google-apis-rs

# Project Features

- provide an idiomatic rust implementation for google APIs
- first-class documentation with cross-links and complete code-examples
- support all features, including downloads and resumable uploads
- safety and resilience are built-in, allowing you to create highly available tools on top of it. For example, you can trigger retries for all operations that may temporarily fail, e.g. due to network outage.

# Build Instructions

## Prerequisites

To generate the APIs yourself, you will need to meet the following prerequisites:

- **make**
- Make is used to automate and efficiently call all involved programs
- **python**
- As [_mako_][mako] is a python program, you will need python installed on your system to run it. Some other programs we call depend on python being present as well.
  Note that you need python 3.8, as 3.9+ introduced some breaking changes that breaks the dependencies.
- **an internet connection and wget**
- Make will download all other prerequisites automatically into hidden directories within this repository, which requires it to make some downloads via wget.
- **Rust Stable**
- This project compiles on _stable_ Rust _1.6 or greater_ only. You might consider using [Rustup][rustup] to control
  the toolchain on a per-project basis.

## Using Make

The makefile is written to be self-documenting. Just calling `make` will yield a list of all valid targets.

```bash
➜  google-apis-rs git:(main) make
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
rm -f .api.deps .cli.deps && FETCH_APIS=1 make update-json -j8
```

Now run `make cargo-api ARGS=check` and each time that fails, add it to [the list of forbidden APIs](https://github.com/Byron/google-apis-rs/blob/main/etc/api/shared.yaml#L3) along with a note as to why it fails,
regenerate all APIs with the make invocation above.
When done and all APIs pass `cargo check`, commit changes in the `shared.yml` file.

# Setup API and CLI version numbers

The version numbers for the respective program types are setup in `etc/api/type-*.yaml` where `*` resolves
to the supported program types, being _cli_ and _api_ at the time of writing. You can change the
version for all expected artifacts by editing the respective key inside of the yaml (_cargo.build_version_
at the time of writing).

The following script would regenerate all higher-level programs (_CLI_), add the result to git and push it.

```bash
$ make gen-all-cli
# Use the version you are comfortable with in the changelog - sometimes you only want to
# update one program type. Here we go with a multi-version, containing all version strings
# in one heading. It's just for the visuals, after all.
$ clog --setversion=api-v<api-version>
$ git add .
$ git commit -m "chore(versionup): added code for latest version"
$ git tag api-v<api-version> cli-v<cli-version> && git push --tags origin master
```

# Publish API to Cargo

Before anything else happens, be sure we publish the latest APIs to cargo.

```bash
# We want as many publishes to work as possible ... -k keeps going on error.
# sometimes uploads fail spuriously, and just need to be retried.
$ make publish-api publish-cli -k
# another attempt to do this should not do actual work
$ make publish-api publish-cli
# all clear ? Please proceed ... .
```

The previous call will have created plenty of marker files, which need to be committed to the repository to prevent to
attempt multiple publishes of the same version.

```bash
git add .
git commit -m "chore(cargo): publish latest version to crates.io"
git push origin main
```

# Build Documentation and post it onto GitHub

The last step will update the documentation index to point to the latest program versions.
For now we assume hosting on GitHub-Pages, but the compiled documentation directory can be
hosted statically anywhere if required.

```bash
# This will run rust-doc on all APIs, generate the index html file and run gh-import at the end.
# The latter might require user-input in case no credentials are setup.
$ make github-pages
```

# License

The license of everything not explicitly under a different license are licensed as specified in `LICENSE.md`.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

# Archive of Development Videos

![Rust](https://github.com/Byron/google-apis-rs/workflows/Rust/badge.svg)
[![Stackshare.io](https://img.shields.io/badge/stackshare.io-GO-blue.svg)](http://stackshare.io/Byron/google-apis-for-rust)

All work done on the project is based on [github issues][issues], not only to track progress and show what's going on, but
to have a place to link screen-recordings to. [Milestones][milestones] are used to provide a bigger picture.

Additionally, there is a development diary which serves as summary of major steps taken so far. As opposed to issue-screencasts,
it is not made live, but is authored and narrated, which should make it more accessible.

Click the image below to see the playlist with all project related content:

[![thumb][playlist-thumb]][playlist]

# Super-Entertaining Videos

Each episode sums up one major step in project development:

- [Episode 1](http://youtu.be/2U3SpepKaBE): How to write 78 APIs in 5 seconds
- [Episode 2](https://youtu.be/wHlE1pNThjE): Making CLIs
- [Episode 3](https://youtu.be/zrw2Qy-Ho5A): To make it work right

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
[rustup]: https://github.com/rust-lang-nursery/rustup.rs
