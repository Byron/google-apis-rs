<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `logging2` command-line interface *(CLI)* allows to use most features of the *Google Logging* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Logging* API can be found at the
[official documentation site](https://cloud.google.com/logging/docs/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-logging2-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/logging2-cli).

# Usage

This documentation was generated from the *Logging* API at revision *20240531*. The CLI is at version *5.0.5*.

```bash
logging2 [options]
        billing-accounts
                exclusions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                exclusions-delete <name> [-p <v>]... [-o <out>]
                exclusions-get <name> [-p <v>]... [-o <out>]
                exclusions-list <parent> [-p <v>]... [-o <out>]
                exclusions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                get-cmek-settings <name> [-p <v>]... [-o <out>]
                get-settings <name> [-p <v>]... [-o <out>]
                locations-buckets-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-create-async <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-get <name> [-p <v>]... [-o <out>]
                locations-buckets-links-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-links-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-links-get <name> [-p <v>]... [-o <out>]
                locations-buckets-links-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-undelete <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-update-async <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-views-get <name> [-p <v>]... [-o <out>]
                locations-buckets-views-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-views-logs-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-views-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-get <name> [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <name> [-p <v>]... [-o <out>]
                locations-recent-queries-list <parent> [-p <v>]... [-o <out>]
                locations-saved-queries-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-saved-queries-delete <name> [-p <v>]... [-o <out>]
                locations-saved-queries-list <parent> [-p <v>]... [-o <out>]
                logs-delete <log-name> [-p <v>]... [-o <out>]
                logs-list <parent> [-p <v>]... [-o <out>]
                sinks-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                sinks-delete <sink-name> [-p <v>]... [-o <out>]
                sinks-get <sink-name> [-p <v>]... [-o <out>]
                sinks-list <parent> [-p <v>]... [-o <out>]
                sinks-patch <sink-name> (-r <kv>)... [-p <v>]... [-o <out>]
                sinks-update <sink-name> (-r <kv>)... [-p <v>]... [-o <out>]
        entries
                copy (-r <kv>)... [-p <v>]... [-o <out>]
                list (-r <kv>)... [-p <v>]... [-o <out>]
                tail (-r <kv>)... [-p <v>]... [-o <out>]
                write (-r <kv>)... [-p <v>]... [-o <out>]
        exclusions
                create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <name> [-p <v>]... [-o <out>]
                get <name> [-p <v>]... [-o <out>]
                list <parent> [-p <v>]... [-o <out>]
                patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
        folders
                exclusions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                exclusions-delete <name> [-p <v>]... [-o <out>]
                exclusions-get <name> [-p <v>]... [-o <out>]
                exclusions-list <parent> [-p <v>]... [-o <out>]
                exclusions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                get-cmek-settings <name> [-p <v>]... [-o <out>]
                get-settings <name> [-p <v>]... [-o <out>]
                locations-buckets-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-create-async <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-get <name> [-p <v>]... [-o <out>]
                locations-buckets-links-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-links-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-links-get <name> [-p <v>]... [-o <out>]
                locations-buckets-links-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-undelete <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-update-async <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-views-get <name> [-p <v>]... [-o <out>]
                locations-buckets-views-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-views-logs-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-views-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-get <name> [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <name> [-p <v>]... [-o <out>]
                locations-recent-queries-list <parent> [-p <v>]... [-o <out>]
                locations-saved-queries-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-saved-queries-delete <name> [-p <v>]... [-o <out>]
                locations-saved-queries-list <parent> [-p <v>]... [-o <out>]
                logs-delete <log-name> [-p <v>]... [-o <out>]
                logs-list <parent> [-p <v>]... [-o <out>]
                sinks-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                sinks-delete <sink-name> [-p <v>]... [-o <out>]
                sinks-get <sink-name> [-p <v>]... [-o <out>]
                sinks-list <parent> [-p <v>]... [-o <out>]
                sinks-patch <sink-name> (-r <kv>)... [-p <v>]... [-o <out>]
                sinks-update <sink-name> (-r <kv>)... [-p <v>]... [-o <out>]
                update-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
        locations
                buckets-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                buckets-create-async <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                buckets-delete <name> [-p <v>]... [-o <out>]
                buckets-get <name> [-p <v>]... [-o <out>]
                buckets-links-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                buckets-links-delete <name> [-p <v>]... [-o <out>]
                buckets-links-get <name> [-p <v>]... [-o <out>]
                buckets-links-list <parent> [-p <v>]... [-o <out>]
                buckets-list <parent> [-p <v>]... [-o <out>]
                buckets-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                buckets-undelete <name> (-r <kv>)... [-p <v>]... [-o <out>]
                buckets-update-async <name> (-r <kv>)... [-p <v>]... [-o <out>]
                buckets-views-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                buckets-views-delete <name> [-p <v>]... [-o <out>]
                buckets-views-get <name> [-p <v>]... [-o <out>]
                buckets-views-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                buckets-views-list <parent> [-p <v>]... [-o <out>]
                buckets-views-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                buckets-views-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                buckets-views-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                get <name> [-p <v>]... [-o <out>]
                list <name> [-p <v>]... [-o <out>]
                operations-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                operations-get <name> [-p <v>]... [-o <out>]
                operations-list <name> [-p <v>]... [-o <out>]
        logs
                delete <log-name> [-p <v>]... [-o <out>]
                list <parent> [-p <v>]... [-o <out>]
        methods
                get-cmek-settings <name> [-p <v>]... [-o <out>]
                get-settings <name> [-p <v>]... [-o <out>]
                update-cmek-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
                update-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
        monitored-resource-descriptors
                list [-p <v>]... [-o <out>]
        organizations
                exclusions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                exclusions-delete <name> [-p <v>]... [-o <out>]
                exclusions-get <name> [-p <v>]... [-o <out>]
                exclusions-list <parent> [-p <v>]... [-o <out>]
                exclusions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                get-cmek-settings <name> [-p <v>]... [-o <out>]
                get-settings <name> [-p <v>]... [-o <out>]
                locations-buckets-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-create-async <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-get <name> [-p <v>]... [-o <out>]
                locations-buckets-links-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-links-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-links-get <name> [-p <v>]... [-o <out>]
                locations-buckets-links-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-undelete <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-update-async <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-views-get <name> [-p <v>]... [-o <out>]
                locations-buckets-views-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-views-logs-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-views-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-get <name> [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <name> [-p <v>]... [-o <out>]
                locations-recent-queries-list <parent> [-p <v>]... [-o <out>]
                locations-saved-queries-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-saved-queries-delete <name> [-p <v>]... [-o <out>]
                locations-saved-queries-list <parent> [-p <v>]... [-o <out>]
                logs-delete <log-name> [-p <v>]... [-o <out>]
                logs-list <parent> [-p <v>]... [-o <out>]
                sinks-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                sinks-delete <sink-name> [-p <v>]... [-o <out>]
                sinks-get <sink-name> [-p <v>]... [-o <out>]
                sinks-list <parent> [-p <v>]... [-o <out>]
                sinks-patch <sink-name> (-r <kv>)... [-p <v>]... [-o <out>]
                sinks-update <sink-name> (-r <kv>)... [-p <v>]... [-o <out>]
                update-cmek-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
                update-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
        projects
                exclusions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                exclusions-delete <name> [-p <v>]... [-o <out>]
                exclusions-get <name> [-p <v>]... [-o <out>]
                exclusions-list <parent> [-p <v>]... [-o <out>]
                exclusions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                get-cmek-settings <name> [-p <v>]... [-o <out>]
                get-settings <name> [-p <v>]... [-o <out>]
                locations-buckets-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-create-async <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-get <name> [-p <v>]... [-o <out>]
                locations-buckets-links-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-links-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-links-get <name> [-p <v>]... [-o <out>]
                locations-buckets-links-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-undelete <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-update-async <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-delete <name> [-p <v>]... [-o <out>]
                locations-buckets-views-get <name> [-p <v>]... [-o <out>]
                locations-buckets-views-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-views-logs-list <parent> [-p <v>]... [-o <out>]
                locations-buckets-views-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-buckets-views-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-get <name> [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <name> [-p <v>]... [-o <out>]
                locations-recent-queries-list <parent> [-p <v>]... [-o <out>]
                locations-saved-queries-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-saved-queries-delete <name> [-p <v>]... [-o <out>]
                locations-saved-queries-list <parent> [-p <v>]... [-o <out>]
                logs-delete <log-name> [-p <v>]... [-o <out>]
                logs-list <parent> [-p <v>]... [-o <out>]
                metrics-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                metrics-delete <metric-name> [-p <v>]... [-o <out>]
                metrics-get <metric-name> [-p <v>]... [-o <out>]
                metrics-list <parent> [-p <v>]... [-o <out>]
                metrics-update <metric-name> (-r <kv>)... [-p <v>]... [-o <out>]
                sinks-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                sinks-delete <sink-name> [-p <v>]... [-o <out>]
                sinks-get <sink-name> [-p <v>]... [-o <out>]
                sinks-list <parent> [-p <v>]... [-o <out>]
                sinks-patch <sink-name> (-r <kv>)... [-p <v>]... [-o <out>]
                sinks-update <sink-name> (-r <kv>)... [-p <v>]... [-o <out>]
        sinks
                create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <sink-name> [-p <v>]... [-o <out>]
                get <sink-name> [-p <v>]... [-o <out>]
                list <parent> [-p <v>]... [-o <out>]
                update <sink-name> (-r <kv>)... [-p <v>]... [-o <out>]
  logging2 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `logging2-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/logging2-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/logging2-secret.json`, assuming that the required *logging* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `logging2 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
