This repository holds [mako][mako] scripts to generate all google APIs as described by the [google discovery service][api-discovery].

The generate source code of each google API can be found in the `gen` subdirectory. Each google API resides in it's own crate which can be used as any other crate.

To find a library of your interest, you might want to proceed looking at the [API documentation index][api-index].

# Project Features

* provide an idiomatic rust implementation for google APIs
* first-class documentation with cross-links and complete code-examples
* support all features, including downloads and resumable uploads
* safety and resilience are built-in, allowing you to create highly available tools on top of it. For example, you can trigger retries for all operations that may temporarily fail, e.g. due to network outage.

# Live-Development

[![Build Status](https://travis-ci.org/Byron/google-apis-rs.svg?branch=master)](https://travis-ci.org/Byron/google-apis-rs)
[![Stackshare.io](https://img.shields.io/badge/stackshare.io-GO-blue.svg)](http://stackshare.io/Byron/google-apis-for-rust)

All work done on the project is based on [github issues][issues], not only to track progress and show what's going on, but
to have a place to link screen-recordings to. [Milestones][milestones] are used to provide a bigger picture.

Additionally, there is a development diary which serves as summary of major steps taken so far. As opposed to issue-screencasts,
it is not made live, but is authored and narrated, which should make it more accessible.

Click the image below to see the playlist with all project related content:

[![thumb][playlist-thumb]][playlist]

# Developer Diary

Each episode sums up one major step in project development:

* [Episode 1](http://youtu.be/2U3SpepKaBE): How to write 78 APIs in 5 seconds
* [Episode 2](https://youtu.be/wHlE1pNThjE): Making CLIs
* [Episode 3](https://youtu.be/zrw2Qy-Ho5A): To make it work right

# Build Instructions

## Prerequisites

To generate the APIs yourself, you will need to meet the following prerequisites:

* **make**
 * Make is used to automate and efficiently call all involved programs
* **python**
 * As [*mako*][mako] is a python program, you will need python installed on your system to run it. Some other programs we call depend on python being present as well.
* **an internet connection and wget**
 * Make will download all other prerequisites automatically into hidden directories within this repository, which requires it to make some downloads via wget.
* **Rust Stable**
 * This project compiles on *stable* Rust only. You might consider using [Multirust][multirust] to control
   the toolchain on a per-project basis

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

## Cross Platform Compilation

This is still a difficult topic in Rust, and even though in theory it's possible to do that on a single system without virutalization, it's difficult to achieve when you are not a pure Rust program. Therefore using VMs is the only option, and we are trying to make this as easy as possible.

### Linux AMD-64 from OSX

This setup is based on Docker, which comes with a *virtual-box*-based linux VM. To use it, just install [boot2docker](http://boot2docker.io/).

The following commands should do the job:

```bash
boot2docker up
# export listed variables to prepare your shell
make wheezy-build
```

You will find your *release* build in the *build/* subdirectory of the project's root.

# Deployment

The deployment process is not very automated, but this paragraph shall document the steps required
to obtain a release build on various platforms and deploy them.

## Setup API and CLI version numbers

The version numbers for the respective program types are setup in `etc/api/type-*.yaml` where `*` resolves
to the supported program types, being *cli* and *api* at the time of writing. You can change the
version for all expected artifacts by editing the respective key inside of the yaml (*cargo.build_version*
at the time of writing).

The following script would regenerate all higher-level programs (*CLI*), add the result to git and push it.

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

## Publish API to Cargo

Before anything else happens, be sure we publish the latest APIs to cargo.

```bash
# We want as many publishes to work as possible ... -k keeps going on error.
# sometimes uploads fail spuriously, and just need to be retried.
$ make publish-api -k
# another attempt to do this should not do actual work
$ make publish-api
# all clear ? Please proceed ... .
```

The previous call will have created plenty of marker files, which need to be committed to the repository to prevent to
attempt multiple publishes of the same version.

```bash
$ git add .
$ git commit -m "chore(cargo): publish latest version to crates.io"
$ git push origin master
```

## Release-Build on all Platforms

Please apply the following script to your build-systems. Currently it differentiates between Linux and OSX.

```bash
# We use the `cli` program type, but it could be any type to builds a binary
# Parallelize with -j4, but prepare for occasionally failed builds due to out-of-memory situations
# or cargo interfering with itself. You might have to run this multiple times, thus -k
# Also adjust -j to suit the capabilities of your machine.
$ make cargo-cli ARGS="build --release" -j4 -k

# Finally, gather the build result by executing the respective utility script,
# depending on the platform you build on
$ src/bash/linux-deploy.bash ... # OR

$ src/bash/osx-deploy.bash ...
```

At the end of this step, you will have `tar` files which contain all build-artifacts for deployment
on a CDN.

## Deploy Build-Artifacts on CDN

Here we copy the tar files onto the CDN or VPS you are using, and apply a script to place it
at the right spot for the download.

```bash
$ scp *.tar.gz user@your-cdn-or-vps.org
```

On the CDN, you want to execute the deployment script - the following example assumes a valid login
with sufficient rights to write the files.

```bash
# This script requires you to type in information already contained in the tar file - it's easy to
# do though ;).
$ src/bash/linux-deployment-to-downloads.bash ...
```

Now the programs have been placed in the right spot to be downloadable from the documentation index
which is built next.

## Build Documentation and post it onto GitHub

The last step will update the documentation index to point to the latest program versions.
For now we assume hosting on GitHub-Pages, but the compiled documentation directory can be
hosted statically anywhere if required.

Please note that the generated download URLs are based on the `url_info.download_base_url` key in the
`etc/api/shared.yaml` file, in case you want to host the downloads anywhere else. In the latter case, you may
want to adjust other base-urls as well.

```bash
# This will run rust-doc on all APIs, generate the index html file and run gh-import at the end.
# The latter might require user-input in case no credentials are setup.
$ make github-pages
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

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
[multirust]: https://github.com/Diggsey/multirust-rs
