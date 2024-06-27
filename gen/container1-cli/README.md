<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `container1` command-line interface *(CLI)* allows to use most features of the *Google Container* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Container* API can be found at the
[official documentation site](https://cloud.google.com/container-engine/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-container1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/container1-cli).

# Usage

This documentation was generated from the *Container* API at revision *20240608*. The CLI is at version *5.0.5*.

```bash
container1 [options]
        projects
                aggregated-usable-subnetworks-list <parent> [-p <v>]... [-o <out>]
                locations-clusters-check-autopilot-compatibility <name> [-p <v>]... [-o <out>]
                locations-clusters-complete-ip-rotation <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-delete <name> [-p <v>]... [-o <out>]
                locations-clusters-get <name> [-p <v>]... [-o <out>]
                locations-clusters-get-jwks <parent> [-p <v>]... [-o <out>]
                locations-clusters-list <parent> [-p <v>]... [-o <out>]
                locations-clusters-node-pools-complete-upgrade <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-node-pools-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-node-pools-delete <name> [-p <v>]... [-o <out>]
                locations-clusters-node-pools-get <name> [-p <v>]... [-o <out>]
                locations-clusters-node-pools-list <parent> [-p <v>]... [-o <out>]
                locations-clusters-node-pools-rollback <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-node-pools-set-autoscaling <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-node-pools-set-management <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-node-pools-set-size <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-node-pools-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-set-addons <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-set-legacy-abac <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-set-locations <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-set-logging <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-set-maintenance-policy <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-set-master-auth <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-set-monitoring <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-set-network-policy <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-set-resource-labels <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-start-ip-rotation <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-update-master <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-clusters-well-known-get-openid-configuration <parent> [-p <v>]... [-o <out>]
                locations-get-server-config <name> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <parent> [-p <v>]... [-o <out>]
                zones-clusters-addons <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-complete-ip-rotation <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-create <project-id> <zone> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-delete <project-id> <zone> <cluster-id> [-p <v>]... [-o <out>]
                zones-clusters-get <project-id> <zone> <cluster-id> [-p <v>]... [-o <out>]
                zones-clusters-legacy-abac <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-list <project-id> <zone> [-p <v>]... [-o <out>]
                zones-clusters-locations <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-logging <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-master <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-monitoring <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-node-pools-autoscaling <project-id> <zone> <cluster-id> <node-pool-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-node-pools-create <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-node-pools-delete <project-id> <zone> <cluster-id> <node-pool-id> [-p <v>]... [-o <out>]
                zones-clusters-node-pools-get <project-id> <zone> <cluster-id> <node-pool-id> [-p <v>]... [-o <out>]
                zones-clusters-node-pools-list <project-id> <zone> <cluster-id> [-p <v>]... [-o <out>]
                zones-clusters-node-pools-rollback <project-id> <zone> <cluster-id> <node-pool-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-node-pools-set-management <project-id> <zone> <cluster-id> <node-pool-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-node-pools-set-size <project-id> <zone> <cluster-id> <node-pool-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-node-pools-update <project-id> <zone> <cluster-id> <node-pool-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-resource-labels <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-set-maintenance-policy <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-set-master-auth <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-set-network-policy <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-start-ip-rotation <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-clusters-update <project-id> <zone> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-get-serverconfig <project-id> <zone> [-p <v>]... [-o <out>]
                zones-operations-cancel <project-id> <zone> <operation-id> (-r <kv>)... [-p <v>]... [-o <out>]
                zones-operations-get <project-id> <zone> <operation-id> [-p <v>]... [-o <out>]
                zones-operations-list <project-id> <zone> [-p <v>]... [-o <out>]
  container1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `container1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/container1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/container1-secret.json`, assuming that the required *container* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `container1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
