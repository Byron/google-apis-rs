<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `sqladmin1` command-line interface *(CLI)* allows to use most features of the *Google SQL Admin* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *SQL Admin* API can be found at the
[official documentation site](https://developers.google.com/cloud-sql/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-sqladmin1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/sqladmin1-cli).

# Usage

This documentation was generated from the *SQL Admin* API at revision *20240618*. The CLI is at version *5.0.5*.

```bash
sqladmin1 [options]
        backup-runs
                delete <project> <instance> <id> [-p <v>]... [-o <out>]
                get <project> <instance> <id> [-p <v>]... [-o <out>]
                insert <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <instance> [-p <v>]... [-o <out>]
        connect
                generate-ephemeral <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                get <project> <instance> [-p <v>]... [-o <out>]
        databases
                delete <project> <instance> <database> [-p <v>]... [-o <out>]
                get <project> <instance> <database> [-p <v>]... [-o <out>]
                insert <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <instance> [-p <v>]... [-o <out>]
                patch <project> <instance> <database> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <instance> <database> (-r <kv>)... [-p <v>]... [-o <out>]
        flags
                list [-p <v>]... [-o <out>]
        instances
                acquire-ssrs-lease <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                add-server-ca <project> <instance> [-p <v>]... [-o <out>]
                clone <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <project> <instance> [-p <v>]... [-o <out>]
                demote <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                demote-master <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                export <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                failover <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                get <project> <instance> [-p <v>]... [-o <out>]
                import <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                list-server-cas <project> <instance> [-p <v>]... [-o <out>]
                patch <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                promote-replica <project> <instance> [-p <v>]... [-o <out>]
                reencrypt <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                release-ssrs-lease <project> <instance> [-p <v>]... [-o <out>]
                reset-ssl-config <project> <instance> [-p <v>]... [-o <out>]
                restart <project> <instance> [-p <v>]... [-o <out>]
                restore-backup <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                rotate-server-ca <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                start-replica <project> <instance> [-p <v>]... [-o <out>]
                stop-replica <project> <instance> [-p <v>]... [-o <out>]
                switchover <project> <instance> [-p <v>]... [-o <out>]
                truncate-log <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
        operations
                cancel <project> <operation> [-p <v>]... [-o <out>]
                get <project> <operation> [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
        projects
                instances-get-disk-shrink-config <project> <instance> [-p <v>]... [-o <out>]
                instances-get-latest-recovery-time <project> <instance> [-p <v>]... [-o <out>]
                instances-perform-disk-shrink <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-reschedule-maintenance <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-reset-replica-size <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-start-external-sync <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-verify-external-sync-settings <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
        ssl-certs
                create-ephemeral <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <project> <instance> <sha1-fingerprint> [-p <v>]... [-o <out>]
                get <project> <instance> <sha1-fingerprint> [-p <v>]... [-o <out>]
                insert <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <instance> [-p <v>]... [-o <out>]
        tiers
                list <project> [-p <v>]... [-o <out>]
        users
                delete <project> <instance> [-p <v>]... [-o <out>]
                get <project> <instance> <name> [-p <v>]... [-o <out>]
                insert <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <instance> [-p <v>]... [-o <out>]
                update <project> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
  sqladmin1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `sqladmin1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/sqladmin1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/sqladmin1-secret.json`, assuming that the required *sqladmin* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `sqladmin1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
