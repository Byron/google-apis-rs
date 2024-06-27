<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `books1` command-line interface *(CLI)* allows to use most features of the *Google books* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *books* API can be found at the
[official documentation site](https://code.google.com/apis/books/docs/v1/getting_started.html).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-books1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/books1-cli).

# Usage

This documentation was generated from the *books* API at revision *20240621*. The CLI is at version *5.0.5*.

```bash
books1 [options]
        bookshelves
                get <user-id> <shelf> [-p <v>]... [-o <out>]
                list <user-id> [-p <v>]... [-o <out>]
                volumes-list <user-id> <shelf> [-p <v>]... [-o <out>]
        cloudloading
                add-book [-p <v>]... [-o <out>]
                delete-book <volume-id> [-p <v>]... [-o <out>]
                update-book (-r <kv>)... [-p <v>]... [-o <out>]
        dictionary
                list-offline-metadata <cpksver> [-p <v>]... [-o <out>]
        familysharing
                get-family-info [-p <v>]... [-o <out>]
                share [-p <v>]... [-o <out>]
                unshare [-p <v>]... [-o <out>]
        layers
                annotation-data-get <volume-id> <layer-id> <annotation-data-id> <content-version> [-p <v>]... [-o <out>]
                annotation-data-list <volume-id> <layer-id> <content-version> [-p <v>]... [-o <out>]
                get <volume-id> <summary-id> [-p <v>]... [-o <out>]
                list <volume-id> [-p <v>]... [-o <out>]
                volume-annotations-get <volume-id> <layer-id> <annotation-id> [-p <v>]... [-o <out>]
                volume-annotations-list <volume-id> <layer-id> <content-version> [-p <v>]... [-o <out>]
        myconfig
                get-user-settings [-p <v>]... [-o <out>]
                release-download-access <cpksver> <volume-ids>... [-p <v>]... [-o <out>]
                request-access <cpksver> <nonce> <source> <volume-id> [-p <v>]... [-o <out>]
                sync-volume-licenses <cpksver> <nonce> <source> [-p <v>]... [-o <out>]
                update-user-settings (-r <kv>)... [-p <v>]... [-o <out>]
        mylibrary
                annotations-delete <annotation-id> [-p <v>]... [-o <out>]
                annotations-insert (-r <kv>)... [-p <v>]... [-o <out>]
                annotations-list [-p <v>]... [-o <out>]
                annotations-summary <layer-ids>... <volume-id> [-p <v>]... [-o <out>]
                annotations-update <annotation-id> (-r <kv>)... [-p <v>]... [-o <out>]
                bookshelves-add-volume <shelf> <volume-id> [-p <v>]... [-o <out>]
                bookshelves-clear-volumes <shelf> [-p <v>]... [-o <out>]
                bookshelves-get <shelf> [-p <v>]... [-o <out>]
                bookshelves-list [-p <v>]... [-o <out>]
                bookshelves-move-volume <shelf> <volume-id> <volume-position> [-p <v>]... [-o <out>]
                bookshelves-remove-volume <shelf> <volume-id> [-p <v>]... [-o <out>]
                bookshelves-volumes-list <shelf> [-p <v>]... [-o <out>]
                readingpositions-get <volume-id> [-p <v>]... [-o <out>]
                readingpositions-set-position <volume-id> <position> <timestamp> [-p <v>]... [-o <out>]
        notification
                get <notification-id> [-p <v>]... [-o <out>]
        onboarding
                list-categories [-p <v>]... [-o <out>]
                list-category-volumes [-p <v>]... [-o <out>]
        personalizedstream
                get [-p <v>]... [-o <out>]
        promooffer
                accept [-p <v>]... [-o <out>]
                dismiss [-p <v>]... [-o <out>]
                get [-p <v>]... [-o <out>]
        series
                get <series-id>... [-p <v>]... [-o <out>]
                membership-get <series-id> [-p <v>]... [-o <out>]
        volumes
                associated-list <volume-id> [-p <v>]... [-o <out>]
                get <volume-id> [-p <v>]... [-o <out>]
                list <q> [-p <v>]... [-o <out>]
                mybooks-list [-p <v>]... [-o <out>]
                recommended-list [-p <v>]... [-o <out>]
                recommended-rate <rating> <volume-id> [-p <v>]... [-o <out>]
                useruploaded-list [-p <v>]... [-o <out>]
  books1 --help

Configuration:
  [--scope <url>]...
            Specify the authentication a method should be executed in. Each scope
            requires the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to
            a user-writable directory that we will create during the first invocation.
            [default: ~/.google-service-cli]

```

# Configuration

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `books1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

More information about the various kinds of persistent data are given in the following paragraphs.

# Authentication

Most APIs require a user to authenticate any request. If this is the case, the [scope][scopes] determines the 
set of permissions granted. The granularity of these is usually no more than *read-only* or *full-access*.

If not set, the system will automatically select the smallest feasible scope, e.g. when invoking a
method that is read-only, it will ask only for a read-only scope. 
You may use the `--scope` flag to specify a scope directly. 
All applicable scopes are documented in the respective method's CLI documentation.

The first time a scope is used, the user is asked for permission. Follow the instructions given 
by the CLI to grant permissions, or to decline.

If a scope was authenticated by the user, the respective information will be stored as *JSON* in the configuration
directory, e.g. `~/.google-service-cli/books1-token-<scope-hash>.json`. No manual management of these tokens
is necessary.

To revoke granted authentication, please refer to the [official documentation][revoke-access].

# Application Secrets

In order to allow any application to use Google services, it will need to be registered using the 
[Google Developer Console][google-dev-console]. APIs the application may use are then enabled for it
one by one. Most APIs can be used for free and have a daily quota.

To allow more comfortable usage of the CLI without forcing anyone to register an own application, the CLI
comes with a default application secret that is configured accordingly. This also means that heavy usage
all around the world may deplete the daily quota.

You can workaround this limitation by putting your own secrets file at this location: 
`~/.google-service-cli/books1-secret.json`, assuming that the required *books* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `books1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
