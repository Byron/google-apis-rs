<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `baremetalsolution2` command-line interface *(CLI)* allows to use most features of the *Google baremetalsolution* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *baremetalsolution* API can be found at the
[official documentation site](https://cloud.google.com/bare-metal).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-baremetalsolution2-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/baremetalsolution2-cli).

# Usage

This documentation was generated from the *baremetalsolution* API at revision *20240617*. The CLI is at version *5.0.5*.

```bash
baremetalsolution2 [options]
        projects
                locations-get <name> [-p <v>]... [-o <out>]
                locations-instances-detach-lun <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-instances-disable-hyperthreading <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-instances-disable-interactive-serial-console <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-instances-enable-hyperthreading <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-instances-enable-interactive-serial-console <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-instances-get <name> [-p <v>]... [-o <out>]
                locations-instances-list <parent> [-p <v>]... [-o <out>]
                locations-instances-load-auth-info <name> [-p <v>]... [-o <out>]
                locations-instances-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-instances-reimage <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-instances-rename <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-instances-reset <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-instances-start <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-instances-stop <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
                locations-networks-get <name> [-p <v>]... [-o <out>]
                locations-networks-list <parent> [-p <v>]... [-o <out>]
                locations-networks-list-network-usage <location> [-p <v>]... [-o <out>]
                locations-networks-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-networks-rename <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-nfs-shares-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-nfs-shares-delete <name> [-p <v>]... [-o <out>]
                locations-nfs-shares-get <name> [-p <v>]... [-o <out>]
                locations-nfs-shares-list <parent> [-p <v>]... [-o <out>]
                locations-nfs-shares-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-nfs-shares-rename <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-os-images-get <name> [-p <v>]... [-o <out>]
                locations-os-images-list <parent> [-p <v>]... [-o <out>]
                locations-provisioning-configs-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-provisioning-configs-get <name> [-p <v>]... [-o <out>]
                locations-provisioning-configs-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-provisioning-configs-submit <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-provisioning-quotas-list <parent> [-p <v>]... [-o <out>]
                locations-ssh-keys-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-ssh-keys-delete <name> [-p <v>]... [-o <out>]
                locations-ssh-keys-list <parent> [-p <v>]... [-o <out>]
                locations-volumes-evict <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-volumes-get <name> [-p <v>]... [-o <out>]
                locations-volumes-list <parent> [-p <v>]... [-o <out>]
                locations-volumes-luns-evict <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-volumes-luns-get <name> [-p <v>]... [-o <out>]
                locations-volumes-luns-list <parent> [-p <v>]... [-o <out>]
                locations-volumes-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-volumes-rename <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-volumes-resize <volume> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-volumes-snapshots-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-volumes-snapshots-delete <name> [-p <v>]... [-o <out>]
                locations-volumes-snapshots-get <name> [-p <v>]... [-o <out>]
                locations-volumes-snapshots-list <parent> [-p <v>]... [-o <out>]
                locations-volumes-snapshots-restore-volume-snapshot <volume-snapshot> (-r <kv>)... [-p <v>]... [-o <out>]
  baremetalsolution2 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `baremetalsolution2-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/baremetalsolution2-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/baremetalsolution2-secret.json`, assuming that the required *baremetalsolution* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `baremetalsolution2 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
