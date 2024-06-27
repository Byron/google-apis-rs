<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `spanner1` command-line interface *(CLI)* allows to use most features of the *Google Spanner* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Spanner* API can be found at the
[official documentation site](https://cloud.google.com/spanner/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-spanner1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/spanner1-cli).

# Usage

This documentation was generated from the *Spanner* API at revision *20240618*. The CLI is at version *5.0.5*.

```bash
spanner1 [options]
        projects
                instance-config-operations-list <parent> [-p <v>]... [-o <out>]
                instance-configs-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                instance-configs-delete <name> [-p <v>]... [-o <out>]
                instance-configs-get <name> [-p <v>]... [-o <out>]
                instance-configs-list <parent> [-p <v>]... [-o <out>]
                instance-configs-operations-cancel <name> [-p <v>]... [-o <out>]
                instance-configs-operations-delete <name> [-p <v>]... [-o <out>]
                instance-configs-operations-get <name> [-p <v>]... [-o <out>]
                instance-configs-operations-list <name> [-p <v>]... [-o <out>]
                instance-configs-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                instance-configs-ssd-caches-operations-cancel <name> [-p <v>]... [-o <out>]
                instance-configs-ssd-caches-operations-delete <name> [-p <v>]... [-o <out>]
                instance-configs-ssd-caches-operations-get <name> [-p <v>]... [-o <out>]
                instance-configs-ssd-caches-operations-list <name> [-p <v>]... [-o <out>]
                instances-backup-operations-list <parent> [-p <v>]... [-o <out>]
                instances-backups-copy <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-backups-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-backups-delete <name> [-p <v>]... [-o <out>]
                instances-backups-get <name> [-p <v>]... [-o <out>]
                instances-backups-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-backups-list <parent> [-p <v>]... [-o <out>]
                instances-backups-operations-cancel <name> [-p <v>]... [-o <out>]
                instances-backups-operations-delete <name> [-p <v>]... [-o <out>]
                instances-backups-operations-get <name> [-p <v>]... [-o <out>]
                instances-backups-operations-list <name> [-p <v>]... [-o <out>]
                instances-backups-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-backups-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-backups-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-database-operations-list <parent> [-p <v>]... [-o <out>]
                instances-databases-changequorum <name> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-database-roles-list <parent> [-p <v>]... [-o <out>]
                instances-databases-database-roles-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-drop-database <database> [-p <v>]... [-o <out>]
                instances-databases-get <name> [-p <v>]... [-o <out>]
                instances-databases-get-ddl <database> [-p <v>]... [-o <out>]
                instances-databases-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-get-scans <name> [-p <v>]... [-o <out>]
                instances-databases-list <parent> [-p <v>]... [-o <out>]
                instances-databases-operations-cancel <name> [-p <v>]... [-o <out>]
                instances-databases-operations-delete <name> [-p <v>]... [-o <out>]
                instances-databases-operations-get <name> [-p <v>]... [-o <out>]
                instances-databases-operations-list <name> [-p <v>]... [-o <out>]
                instances-databases-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-restore <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-batch-create <database> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-batch-write <session> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-begin-transaction <session> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-commit <session> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-create <database> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-delete <name> [-p <v>]... [-o <out>]
                instances-databases-sessions-execute-batch-dml <session> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-execute-sql <session> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-execute-streaming-sql <session> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-get <name> [-p <v>]... [-o <out>]
                instances-databases-sessions-list <database> [-p <v>]... [-o <out>]
                instances-databases-sessions-partition-query <session> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-partition-read <session> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-read <session> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-rollback <session> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-sessions-streaming-read <session> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-databases-update-ddl <database> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-delete <name> [-p <v>]... [-o <out>]
                instances-get <name> [-p <v>]... [-o <out>]
                instances-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-instance-partition-operations-list <parent> [-p <v>]... [-o <out>]
                instances-instance-partitions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-instance-partitions-delete <name> [-p <v>]... [-o <out>]
                instances-instance-partitions-get <name> [-p <v>]... [-o <out>]
                instances-instance-partitions-list <parent> [-p <v>]... [-o <out>]
                instances-instance-partitions-operations-cancel <name> [-p <v>]... [-o <out>]
                instances-instance-partitions-operations-delete <name> [-p <v>]... [-o <out>]
                instances-instance-partitions-operations-get <name> [-p <v>]... [-o <out>]
                instances-instance-partitions-operations-list <name> [-p <v>]... [-o <out>]
                instances-instance-partitions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-list <parent> [-p <v>]... [-o <out>]
                instances-move <name> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-operations-cancel <name> [-p <v>]... [-o <out>]
                instances-operations-delete <name> [-p <v>]... [-o <out>]
                instances-operations-get <name> [-p <v>]... [-o <out>]
                instances-operations-list <name> [-p <v>]... [-o <out>]
                instances-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
        scans
                list <parent> [-p <v>]... [-o <out>]
  spanner1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `spanner1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/spanner1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/spanner1-secret.json`, assuming that the required *spanner* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `spanner1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
