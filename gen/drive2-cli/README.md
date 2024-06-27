<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `drive2` command-line interface *(CLI)* allows to use most features of the *Google drive* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *drive* API can be found at the
[official documentation site](https://developers.google.com/drive/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-drive2-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/drive2-cli).

# Usage

This documentation was generated from the *drive* API at revision *20240618*. The CLI is at version *5.0.5*.

```bash
drive2 [options]
        about
                get [-p <v>]... [-o <out>]
        apps
                get <app-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
        changes
                get <change-id> [-p <v>]... [-o <out>]
                get-start-page-token [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                watch (-r <kv>)... [-p <v>]... [-o <out>]
        channels
                stop (-r <kv>)... [-p <v>]...
        children
                delete <folder-id> <child-id> [-p <v>]...
                get <folder-id> <child-id> [-p <v>]... [-o <out>]
                insert <folder-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <folder-id> [-p <v>]... [-o <out>]
        comments
                delete <file-id> <comment-id> [-p <v>]...
                get <file-id> <comment-id> [-p <v>]... [-o <out>]
                insert <file-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <file-id> [-p <v>]... [-o <out>]
                patch <file-id> <comment-id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <file-id> <comment-id> (-r <kv>)... [-p <v>]... [-o <out>]
        drives
                delete <drive-id> [-p <v>]...
                get <drive-id> [-p <v>]... [-o <out>]
                hide <drive-id> [-p <v>]... [-o <out>]
                insert <request-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                unhide <drive-id> [-p <v>]... [-o <out>]
                update <drive-id> (-r <kv>)... [-p <v>]... [-o <out>]
        files
                copy <file-id> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <file-id> [-p <v>]...
                empty-trash [-p <v>]...
                export <file-id> <mime-type> [-p <v>]... [-o <out>]
                generate-ids [-p <v>]... [-o <out>]
                get <file-id> [-p <v>]... [-o <out>]
                insert (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                list-labels <file-id> [-p <v>]... [-o <out>]
                modify-labels <file-id> (-r <kv>)... [-p <v>]... [-o <out>]
                patch <file-id> (-r <kv>)... [-p <v>]... [-o <out>]
                touch <file-id> [-p <v>]... [-o <out>]
                trash <file-id> [-p <v>]... [-o <out>]
                untrash <file-id> [-p <v>]... [-o <out>]
                update <file-id> (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                watch <file-id> (-r <kv>)... [-p <v>]... [-o <out>]
        parents
                delete <file-id> <parent-id> [-p <v>]...
                get <file-id> <parent-id> [-p <v>]... [-o <out>]
                insert <file-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <file-id> [-p <v>]... [-o <out>]
        permissions
                delete <file-id> <permission-id> [-p <v>]...
                get <file-id> <permission-id> [-p <v>]... [-o <out>]
                get-id-for-email <email> [-p <v>]... [-o <out>]
                insert <file-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <file-id> [-p <v>]... [-o <out>]
                patch <file-id> <permission-id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <file-id> <permission-id> (-r <kv>)... [-p <v>]... [-o <out>]
        properties
                delete <file-id> <property-key> [-p <v>]...
                get <file-id> <property-key> [-p <v>]... [-o <out>]
                insert <file-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <file-id> [-p <v>]... [-o <out>]
                patch <file-id> <property-key> (-r <kv>)... [-p <v>]... [-o <out>]
                update <file-id> <property-key> (-r <kv>)... [-p <v>]... [-o <out>]
        replies
                delete <file-id> <comment-id> <reply-id> [-p <v>]...
                get <file-id> <comment-id> <reply-id> [-p <v>]... [-o <out>]
                insert <file-id> <comment-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <file-id> <comment-id> [-p <v>]... [-o <out>]
                patch <file-id> <comment-id> <reply-id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <file-id> <comment-id> <reply-id> (-r <kv>)... [-p <v>]... [-o <out>]
        revisions
                delete <file-id> <revision-id> [-p <v>]...
                get <file-id> <revision-id> [-p <v>]... [-o <out>]
                list <file-id> [-p <v>]... [-o <out>]
                patch <file-id> <revision-id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <file-id> <revision-id> (-r <kv>)... [-p <v>]... [-o <out>]
        teamdrives
                delete <team-drive-id> [-p <v>]...
                get <team-drive-id> [-p <v>]... [-o <out>]
                insert <request-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                update <team-drive-id> (-r <kv>)... [-p <v>]... [-o <out>]
  drive2 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `drive2-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/drive2-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/drive2-secret.json`, assuming that the required *drive* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `drive2 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
