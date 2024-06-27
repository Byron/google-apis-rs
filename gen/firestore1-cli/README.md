<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `firestore1` command-line interface *(CLI)* allows to use most features of the *Google Firestore* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Firestore* API can be found at the
[official documentation site](https://cloud.google.com/firestore).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-firestore1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/firestore1-cli).

# Usage

This documentation was generated from the *Firestore* API at revision *20240617*. The CLI is at version *5.0.5*.

```bash
firestore1 [options]
        projects
                databases-backup-schedules-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-backup-schedules-delete <name> [-p <v>]... [-o <out>]
                databases-backup-schedules-get <name> [-p <v>]... [-o <out>]
                databases-backup-schedules-list <parent> [-p <v>]... [-o <out>]
                databases-backup-schedules-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-bulk-delete-documents <name> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-collection-groups-fields-get <name> [-p <v>]... [-o <out>]
                databases-collection-groups-fields-list <parent> [-p <v>]... [-o <out>]
                databases-collection-groups-fields-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-collection-groups-indexes-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-collection-groups-indexes-delete <name> [-p <v>]... [-o <out>]
                databases-collection-groups-indexes-get <name> [-p <v>]... [-o <out>]
                databases-collection-groups-indexes-list <parent> [-p <v>]... [-o <out>]
                databases-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-delete <name> [-p <v>]... [-o <out>]
                databases-documents-batch-get <database> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-batch-write <database> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-begin-transaction <database> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-commit <database> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-create-document <parent> <collection-id> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-delete <name> [-p <v>]... [-o <out>]
                databases-documents-get <name> [-p <v>]... [-o <out>]
                databases-documents-list <parent> <collection-id> [-p <v>]... [-o <out>]
                databases-documents-list-collection-ids <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-list-documents <parent> <collection-id> [-p <v>]... [-o <out>]
                databases-documents-listen <database> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-partition-query <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-rollback <database> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-run-aggregation-query <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-run-query <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-documents-write <database> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-export-documents <name> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-get <name> [-p <v>]... [-o <out>]
                databases-import-documents <name> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-list <parent> [-p <v>]... [-o <out>]
                databases-operations-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-operations-delete <name> [-p <v>]... [-o <out>]
                databases-operations-get <name> [-p <v>]... [-o <out>]
                databases-operations-list <name> [-p <v>]... [-o <out>]
                databases-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                databases-restore <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-backups-delete <name> [-p <v>]... [-o <out>]
                locations-backups-get <name> [-p <v>]... [-o <out>]
                locations-backups-list <parent> [-p <v>]... [-o <out>]
                locations-get <name> [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
  firestore1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `firestore1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/firestore1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/firestore1-secret.json`, assuming that the required *firestore* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `firestore1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
