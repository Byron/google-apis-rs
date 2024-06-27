<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `analytics3` command-line interface *(CLI)* allows to use most features of the *Google analytics* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *analytics* API can be found at the
[official documentation site](https://developers.google.com/analytics/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-analytics3-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/analytics3-cli).

# Usage

This documentation was generated from the *analytics* API at revision *20190807*. The CLI is at version *5.0.5*.

```bash
analytics3 [options]
        data
                ga-get <ids> <start-date> <end-date> <metrics> [-p <v>]... [-o <out>]
                mcf-get <ids> <start-date> <end-date> <metrics> [-p <v>]... [-o <out>]
                realtime-get <ids> <metrics> [-p <v>]... [-o <out>]
        management
                account-summaries-list [-p <v>]... [-o <out>]
                account-user-links-delete <account-id> <link-id> [-p <v>]...
                account-user-links-insert <account-id> (-r <kv>)... [-p <v>]... [-o <out>]
                account-user-links-list <account-id> [-p <v>]... [-o <out>]
                account-user-links-update <account-id> <link-id> (-r <kv>)... [-p <v>]... [-o <out>]
                accounts-list [-p <v>]... [-o <out>]
                client-id-hash-client-id (-r <kv>)... [-p <v>]... [-o <out>]
                custom-data-sources-list <account-id> <web-property-id> [-p <v>]... [-o <out>]
                custom-dimensions-get <account-id> <web-property-id> <custom-dimension-id> [-p <v>]... [-o <out>]
                custom-dimensions-insert <account-id> <web-property-id> (-r <kv>)... [-p <v>]... [-o <out>]
                custom-dimensions-list <account-id> <web-property-id> [-p <v>]... [-o <out>]
                custom-dimensions-patch <account-id> <web-property-id> <custom-dimension-id> (-r <kv>)... [-p <v>]... [-o <out>]
                custom-dimensions-update <account-id> <web-property-id> <custom-dimension-id> (-r <kv>)... [-p <v>]... [-o <out>]
                custom-metrics-get <account-id> <web-property-id> <custom-metric-id> [-p <v>]... [-o <out>]
                custom-metrics-insert <account-id> <web-property-id> (-r <kv>)... [-p <v>]... [-o <out>]
                custom-metrics-list <account-id> <web-property-id> [-p <v>]... [-o <out>]
                custom-metrics-patch <account-id> <web-property-id> <custom-metric-id> (-r <kv>)... [-p <v>]... [-o <out>]
                custom-metrics-update <account-id> <web-property-id> <custom-metric-id> (-r <kv>)... [-p <v>]... [-o <out>]
                experiments-delete <account-id> <web-property-id> <profile-id> <experiment-id> [-p <v>]...
                experiments-get <account-id> <web-property-id> <profile-id> <experiment-id> [-p <v>]... [-o <out>]
                experiments-insert <account-id> <web-property-id> <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                experiments-list <account-id> <web-property-id> <profile-id> [-p <v>]... [-o <out>]
                experiments-patch <account-id> <web-property-id> <profile-id> <experiment-id> (-r <kv>)... [-p <v>]... [-o <out>]
                experiments-update <account-id> <web-property-id> <profile-id> <experiment-id> (-r <kv>)... [-p <v>]... [-o <out>]
                filters-delete <account-id> <filter-id> [-p <v>]... [-o <out>]
                filters-get <account-id> <filter-id> [-p <v>]... [-o <out>]
                filters-insert <account-id> (-r <kv>)... [-p <v>]... [-o <out>]
                filters-list <account-id> [-p <v>]... [-o <out>]
                filters-patch <account-id> <filter-id> (-r <kv>)... [-p <v>]... [-o <out>]
                filters-update <account-id> <filter-id> (-r <kv>)... [-p <v>]... [-o <out>]
                goals-get <account-id> <web-property-id> <profile-id> <goal-id> [-p <v>]... [-o <out>]
                goals-insert <account-id> <web-property-id> <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                goals-list <account-id> <web-property-id> <profile-id> [-p <v>]... [-o <out>]
                goals-patch <account-id> <web-property-id> <profile-id> <goal-id> (-r <kv>)... [-p <v>]... [-o <out>]
                goals-update <account-id> <web-property-id> <profile-id> <goal-id> (-r <kv>)... [-p <v>]... [-o <out>]
                profile-filter-links-delete <account-id> <web-property-id> <profile-id> <link-id> [-p <v>]...
                profile-filter-links-get <account-id> <web-property-id> <profile-id> <link-id> [-p <v>]... [-o <out>]
                profile-filter-links-insert <account-id> <web-property-id> <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                profile-filter-links-list <account-id> <web-property-id> <profile-id> [-p <v>]... [-o <out>]
                profile-filter-links-patch <account-id> <web-property-id> <profile-id> <link-id> (-r <kv>)... [-p <v>]... [-o <out>]
                profile-filter-links-update <account-id> <web-property-id> <profile-id> <link-id> (-r <kv>)... [-p <v>]... [-o <out>]
                profile-user-links-delete <account-id> <web-property-id> <profile-id> <link-id> [-p <v>]...
                profile-user-links-insert <account-id> <web-property-id> <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                profile-user-links-list <account-id> <web-property-id> <profile-id> [-p <v>]... [-o <out>]
                profile-user-links-update <account-id> <web-property-id> <profile-id> <link-id> (-r <kv>)... [-p <v>]... [-o <out>]
                profiles-delete <account-id> <web-property-id> <profile-id> [-p <v>]...
                profiles-get <account-id> <web-property-id> <profile-id> [-p <v>]... [-o <out>]
                profiles-insert <account-id> <web-property-id> (-r <kv>)... [-p <v>]... [-o <out>]
                profiles-list <account-id> <web-property-id> [-p <v>]... [-o <out>]
                profiles-patch <account-id> <web-property-id> <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                profiles-update <account-id> <web-property-id> <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                remarketing-audience-delete <account-id> <web-property-id> <remarketing-audience-id> [-p <v>]...
                remarketing-audience-get <account-id> <web-property-id> <remarketing-audience-id> [-p <v>]... [-o <out>]
                remarketing-audience-insert <account-id> <web-property-id> (-r <kv>)... [-p <v>]... [-o <out>]
                remarketing-audience-list <account-id> <web-property-id> [-p <v>]... [-o <out>]
                remarketing-audience-patch <account-id> <web-property-id> <remarketing-audience-id> (-r <kv>)... [-p <v>]... [-o <out>]
                remarketing-audience-update <account-id> <web-property-id> <remarketing-audience-id> (-r <kv>)... [-p <v>]... [-o <out>]
                segments-list [-p <v>]... [-o <out>]
                unsampled-reports-delete <account-id> <web-property-id> <profile-id> <unsampled-report-id> [-p <v>]...
                unsampled-reports-get <account-id> <web-property-id> <profile-id> <unsampled-report-id> [-p <v>]... [-o <out>]
                unsampled-reports-insert <account-id> <web-property-id> <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                unsampled-reports-list <account-id> <web-property-id> <profile-id> [-p <v>]... [-o <out>]
                uploads-delete-upload-data <account-id> <web-property-id> <custom-data-source-id> (-r <kv>)... [-p <v>]...
                uploads-get <account-id> <web-property-id> <custom-data-source-id> <upload-id> [-p <v>]... [-o <out>]
                uploads-list <account-id> <web-property-id> <custom-data-source-id> [-p <v>]... [-o <out>]
                uploads-upload-data <account-id> <web-property-id> <custom-data-source-id> (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                web-property-ad-words-links-delete <account-id> <web-property-id> <web-property-ad-words-link-id> [-p <v>]...
                web-property-ad-words-links-get <account-id> <web-property-id> <web-property-ad-words-link-id> [-p <v>]... [-o <out>]
                web-property-ad-words-links-insert <account-id> <web-property-id> (-r <kv>)... [-p <v>]... [-o <out>]
                web-property-ad-words-links-list <account-id> <web-property-id> [-p <v>]... [-o <out>]
                web-property-ad-words-links-patch <account-id> <web-property-id> <web-property-ad-words-link-id> (-r <kv>)... [-p <v>]... [-o <out>]
                web-property-ad-words-links-update <account-id> <web-property-id> <web-property-ad-words-link-id> (-r <kv>)... [-p <v>]... [-o <out>]
                webproperties-get <account-id> <web-property-id> [-p <v>]... [-o <out>]
                webproperties-insert <account-id> (-r <kv>)... [-p <v>]... [-o <out>]
                webproperties-list <account-id> [-p <v>]... [-o <out>]
                webproperties-patch <account-id> <web-property-id> (-r <kv>)... [-p <v>]... [-o <out>]
                webproperties-update <account-id> <web-property-id> (-r <kv>)... [-p <v>]... [-o <out>]
                webproperty-user-links-delete <account-id> <web-property-id> <link-id> [-p <v>]...
                webproperty-user-links-insert <account-id> <web-property-id> (-r <kv>)... [-p <v>]... [-o <out>]
                webproperty-user-links-list <account-id> <web-property-id> [-p <v>]... [-o <out>]
                webproperty-user-links-update <account-id> <web-property-id> <link-id> (-r <kv>)... [-p <v>]... [-o <out>]
        metadata
                columns-list <report-type> [-p <v>]... [-o <out>]
        provisioning
                create-account-ticket (-r <kv>)... [-p <v>]... [-o <out>]
                create-account-tree (-r <kv>)... [-p <v>]... [-o <out>]
        user-deletion
                user-deletion-request-upsert (-r <kv>)... [-p <v>]... [-o <out>]
  analytics3 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `analytics3-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/analytics3-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/analytics3-secret.json`, assuming that the required *analytics* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `analytics3 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
