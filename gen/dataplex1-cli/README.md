<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/cli/README.md.mako'
DO NOT EDIT !
-->
The `dataplex1` command-line interface *(CLI)* allows to use most features of the *Google Cloud Dataplex* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Cloud Dataplex* API can be found at the
[official documentation site](https://cloud.google.com/dataplex/docs).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-dataplex1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dataplex1-cli).

# Usage

This documentation was generated from the *Cloud Dataplex* API at revision *20220223*. The CLI is at version *3.1.0*.

```bash
dataplex1 [options]
        projects
                locations-get <name> [-p <v>]... [-o <out>]
                locations-lakes-actions-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-content-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-lakes-content-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-content-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-contentitems-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-contentitems-delete <name> [-p <v>]... [-o <out>]
                locations-lakes-contentitems-get <name> [-p <v>]... [-o <out>]
                locations-lakes-contentitems-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-contentitems-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-delete <name> [-p <v>]... [-o <out>]
                locations-lakes-environments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-environments-delete <name> [-p <v>]... [-o <out>]
                locations-lakes-environments-get <name> [-p <v>]... [-o <out>]
                locations-lakes-environments-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-lakes-environments-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-environments-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-environments-sessions-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-environments-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-environments-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-get <name> [-p <v>]... [-o <out>]
                locations-lakes-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-lakes-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-tasks-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-tasks-delete <name> [-p <v>]... [-o <out>]
                locations-lakes-tasks-get <name> [-p <v>]... [-o <out>]
                locations-lakes-tasks-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-lakes-tasks-jobs-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-tasks-jobs-get <name> [-p <v>]... [-o <out>]
                locations-lakes-tasks-jobs-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-tasks-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-tasks-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-tasks-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-tasks-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-zones-actions-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-zones-assets-actions-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-zones-assets-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-zones-assets-delete <name> [-p <v>]... [-o <out>]
                locations-lakes-zones-assets-get <name> [-p <v>]... [-o <out>]
                locations-lakes-zones-assets-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-lakes-zones-assets-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-zones-assets-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-zones-assets-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-zones-assets-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-zones-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-zones-delete <name> [-p <v>]... [-o <out>]
                locations-lakes-zones-entities-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-zones-entities-delete <name> [-p <v>]... [-o <out>]
                locations-lakes-zones-entities-get <name> [-p <v>]... [-o <out>]
                locations-lakes-zones-entities-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-zones-entities-partitions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-zones-entities-partitions-delete <name> [-p <v>]... [-o <out>]
                locations-lakes-zones-entities-partitions-get <name> [-p <v>]... [-o <out>]
                locations-lakes-zones-entities-partitions-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-zones-entities-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-zones-get <name> [-p <v>]... [-o <out>]
                locations-lakes-zones-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-lakes-zones-list <parent> [-p <v>]... [-o <out>]
                locations-lakes-zones-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-zones-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-lakes-zones-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-operations-delete <name> [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <name> [-p <v>]... [-o <out>]
  dataplex1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `dataplex1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/dataplex1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/dataplex1-secret.json`, assuming that the required *dataplex* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `dataplex1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/