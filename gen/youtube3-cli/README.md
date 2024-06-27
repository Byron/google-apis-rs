<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `youtube3` command-line interface *(CLI)* allows to use most features of the *Google YouTube* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *YouTube* API can be found at the
[official documentation site](https://developers.google.com/youtube/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-youtube3-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/youtube3-cli).

# Usage

This documentation was generated from the *YouTube* API at revision *20240626*. The CLI is at version *5.0.5*.

```bash
youtube3 [options]
        abuse-reports
                insert (-r <kv>)... [-p <v>]... [-o <out>]
        activities
                list <part>... [-p <v>]... [-o <out>]
        captions
                delete <id> [-p <v>]...
                download <id> [-p <v>]... [-o <out>]
                insert (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                list <part>... <video-id> [-p <v>]... [-o <out>]
                update (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
        channel-banners
                insert (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
        channel-sections
                delete <id> [-p <v>]...
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list <part>... [-p <v>]... [-o <out>]
                update (-r <kv>)... [-p <v>]... [-o <out>]
        channels
                list <part>... [-p <v>]... [-o <out>]
                update (-r <kv>)... [-p <v>]... [-o <out>]
        comment-threads
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list <part>... [-p <v>]... [-o <out>]
        comments
                delete <id> [-p <v>]...
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list <part>... [-p <v>]... [-o <out>]
                mark-as-spam <id>... [-p <v>]...
                set-moderation-status <id>... <moderation-status> [-p <v>]...
                update (-r <kv>)... [-p <v>]... [-o <out>]
        i18n-languages
                list <part>... [-p <v>]... [-o <out>]
        i18n-regions
                list <part>... [-p <v>]... [-o <out>]
        live-broadcasts
                bind <id> <part>... [-p <v>]... [-o <out>]
                delete <id> [-p <v>]...
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                insert-cuepoint (-r <kv>)... [-p <v>]... [-o <out>]
                list <part>... [-p <v>]... [-o <out>]
                transition <broadcast-status> <id> <part>... [-p <v>]... [-o <out>]
                update (-r <kv>)... [-p <v>]... [-o <out>]
        live-chat-bans
                delete <id> [-p <v>]...
                insert (-r <kv>)... [-p <v>]... [-o <out>]
        live-chat-messages
                delete <id> [-p <v>]...
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list <live-chat-id> <part>... [-p <v>]... [-o <out>]
                transition [-p <v>]... [-o <out>]
        live-chat-moderators
                delete <id> [-p <v>]...
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list <live-chat-id> <part>... [-p <v>]... [-o <out>]
        live-streams
                delete <id> [-p <v>]...
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list <part>... [-p <v>]... [-o <out>]
                update (-r <kv>)... [-p <v>]... [-o <out>]
        members
                list <part>... [-p <v>]... [-o <out>]
        memberships-levels
                list <part>... [-p <v>]... [-o <out>]
        playlist-images
                delete [-p <v>]...
                insert (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                update (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
        playlist-items
                delete <id> [-p <v>]...
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list <part>... [-p <v>]... [-o <out>]
                update (-r <kv>)... [-p <v>]... [-o <out>]
        playlists
                delete <id> [-p <v>]...
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list <part>... [-p <v>]... [-o <out>]
                update (-r <kv>)... [-p <v>]... [-o <out>]
        search
                list <part>... [-p <v>]... [-o <out>]
        subscriptions
                delete <id> [-p <v>]...
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list <part>... [-p <v>]... [-o <out>]
        super-chat-events
                list <part>... [-p <v>]... [-o <out>]
        tests
                insert (-r <kv>)... [-p <v>]... [-o <out>]
        third-party-links
                delete <linking-token> <type> [-p <v>]...
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list <part>... [-p <v>]... [-o <out>]
                update (-r <kv>)... [-p <v>]... [-o <out>]
        thumbnails
                set <video-id> (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
        video-abuse-report-reasons
                list <part>... [-p <v>]... [-o <out>]
        video-categories
                list <part>... [-p <v>]... [-o <out>]
        videos
                delete <id> [-p <v>]...
                get-rating <id>... [-p <v>]... [-o <out>]
                insert (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                list <part>... [-p <v>]... [-o <out>]
                rate <id> <rating> [-p <v>]...
                report-abuse (-r <kv>)... [-p <v>]...
                update (-r <kv>)... [-p <v>]... [-o <out>]
        watermarks
                set <channel-id> (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]...
                unset <channel-id> [-p <v>]...
        youtube
                v3-update-comment-threads (-r <kv>)... [-p <v>]... [-o <out>]
  youtube3 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `youtube3-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/youtube3-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/youtube3-secret.json`, assuming that the required *youtube* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `youtube3 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
