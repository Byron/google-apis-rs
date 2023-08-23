<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `analyticsadmin1-alpha` command-line interface *(CLI)* allows to use most features of the *Google Google Analytics Admin* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Google Analytics Admin* API can be found at the
[official documentation site](http://code.google.com/apis/analytics/docs/mgmt/home.html).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-analyticsadmin1_alpha-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/analyticsadmin1_alpha-cli).

# Usage

This documentation was generated from the *Google Analytics Admin* API at revision *20220307*. The CLI is at version *5.0.3*.

```bash
analyticsadmin1-alpha [options]
        account-summaries
                list [-p <v>]... [-o <out>]
        accounts
                delete <name> [-p <v>]... [-o <out>]
                get <name> [-p <v>]... [-o <out>]
                get-data-sharing-settings <name> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                provision-account-ticket (-r <kv>)... [-p <v>]... [-o <out>]
                search-change-history-events <account> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-audit <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-batch-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-batch-delete <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-batch-get <parent> [-p <v>]... [-o <out>]
                user-links-batch-update <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-delete <name> [-p <v>]... [-o <out>]
                user-links-get <name> [-p <v>]... [-o <out>]
                user-links-list <parent> [-p <v>]... [-o <out>]
                user-links-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
        properties
                acknowledge-user-data-collection <property> (-r <kv>)... [-p <v>]... [-o <out>]
                conversion-events-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                conversion-events-delete <name> [-p <v>]... [-o <out>]
                conversion-events-get <name> [-p <v>]... [-o <out>]
                conversion-events-list <parent> [-p <v>]... [-o <out>]
                create (-r <kv>)... [-p <v>]... [-o <out>]
                custom-dimensions-archive <name> (-r <kv>)... [-p <v>]... [-o <out>]
                custom-dimensions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                custom-dimensions-get <name> [-p <v>]... [-o <out>]
                custom-dimensions-list <parent> [-p <v>]... [-o <out>]
                custom-dimensions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                custom-metrics-archive <name> (-r <kv>)... [-p <v>]... [-o <out>]
                custom-metrics-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                custom-metrics-get <name> [-p <v>]... [-o <out>]
                custom-metrics-list <parent> [-p <v>]... [-o <out>]
                custom-metrics-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                data-streams-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                data-streams-delete <name> [-p <v>]... [-o <out>]
                data-streams-get <name> [-p <v>]... [-o <out>]
                data-streams-get-global-site-tag <name> [-p <v>]... [-o <out>]
                data-streams-list <parent> [-p <v>]... [-o <out>]
                data-streams-measurement-protocol-secrets-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                data-streams-measurement-protocol-secrets-delete <name> [-p <v>]... [-o <out>]
                data-streams-measurement-protocol-secrets-get <name> [-p <v>]... [-o <out>]
                data-streams-measurement-protocol-secrets-list <parent> [-p <v>]... [-o <out>]
                data-streams-measurement-protocol-secrets-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                data-streams-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <name> [-p <v>]... [-o <out>]
                display-video360-advertiser-link-proposals-approve <name> (-r <kv>)... [-p <v>]... [-o <out>]
                display-video360-advertiser-link-proposals-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                display-video360-advertiser-link-proposals-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                display-video360-advertiser-link-proposals-delete <name> [-p <v>]... [-o <out>]
                display-video360-advertiser-link-proposals-get <name> [-p <v>]... [-o <out>]
                display-video360-advertiser-link-proposals-list <parent> [-p <v>]... [-o <out>]
                display-video360-advertiser-links-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                display-video360-advertiser-links-delete <name> [-p <v>]... [-o <out>]
                display-video360-advertiser-links-get <name> [-p <v>]... [-o <out>]
                display-video360-advertiser-links-list <parent> [-p <v>]... [-o <out>]
                display-video360-advertiser-links-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                firebase-links-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                firebase-links-delete <name> [-p <v>]... [-o <out>]
                firebase-links-list <parent> [-p <v>]... [-o <out>]
                get <name> [-p <v>]... [-o <out>]
                get-data-retention-settings <name> [-p <v>]... [-o <out>]
                get-google-signals-settings <name> [-p <v>]... [-o <out>]
                google-ads-links-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                google-ads-links-delete <name> [-p <v>]... [-o <out>]
                google-ads-links-list <parent> [-p <v>]... [-o <out>]
                google-ads-links-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                update-data-retention-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
                update-google-signals-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-audit <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-batch-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-batch-delete <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-batch-get <parent> [-p <v>]... [-o <out>]
                user-links-batch-update <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                user-links-delete <name> [-p <v>]... [-o <out>]
                user-links-get <name> [-p <v>]... [-o <out>]
                user-links-list <parent> [-p <v>]... [-o <out>]
                user-links-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
  analyticsadmin1-alpha --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `analyticsadmin1-alpha-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/analyticsadmin1-alpha-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/analyticsadmin1-alpha-secret.json`, assuming that the required *analyticsadmin* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `analyticsadmin1-alpha --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
