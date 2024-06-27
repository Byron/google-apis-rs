<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `networkconnectivity1` command-line interface *(CLI)* allows to use most features of the *Google networkconnectivity* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *networkconnectivity* API can be found at the
[official documentation site](https://cloud.google.com/network-connectivity/docs/reference/networkconnectivity/rest).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-networkconnectivity1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/networkconnectivity1-cli).

# Usage

This documentation was generated from the *networkconnectivity* API at revision *20240618*. The CLI is at version *5.0.5*.

```bash
networkconnectivity1 [options]
        projects
                locations-get <name> [-p <v>]... [-o <out>]
                locations-global-hubs-accept-spoke <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-hubs-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-hubs-delete <name> [-p <v>]... [-o <out>]
                locations-global-hubs-get <name> [-p <v>]... [-o <out>]
                locations-global-hubs-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-global-hubs-groups-get <name> [-p <v>]... [-o <out>]
                locations-global-hubs-groups-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-global-hubs-groups-list <parent> [-p <v>]... [-o <out>]
                locations-global-hubs-groups-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-hubs-groups-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-hubs-groups-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-hubs-list <parent> [-p <v>]... [-o <out>]
                locations-global-hubs-list-spokes <name> [-p <v>]... [-o <out>]
                locations-global-hubs-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-hubs-reject-spoke <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-hubs-route-tables-get <name> [-p <v>]... [-o <out>]
                locations-global-hubs-route-tables-list <parent> [-p <v>]... [-o <out>]
                locations-global-hubs-route-tables-routes-get <name> [-p <v>]... [-o <out>]
                locations-global-hubs-route-tables-routes-list <parent> [-p <v>]... [-o <out>]
                locations-global-hubs-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-hubs-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-policy-based-routes-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-policy-based-routes-delete <name> [-p <v>]... [-o <out>]
                locations-global-policy-based-routes-get <name> [-p <v>]... [-o <out>]
                locations-global-policy-based-routes-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-global-policy-based-routes-list <parent> [-p <v>]... [-o <out>]
                locations-global-policy-based-routes-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-policy-based-routes-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-internal-ranges-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-internal-ranges-delete <name> [-p <v>]... [-o <out>]
                locations-internal-ranges-get <name> [-p <v>]... [-o <out>]
                locations-internal-ranges-list <parent> [-p <v>]... [-o <out>]
                locations-internal-ranges-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-operations-delete <name> [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <name> [-p <v>]... [-o <out>]
                locations-regional-endpoints-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-regional-endpoints-delete <name> [-p <v>]... [-o <out>]
                locations-regional-endpoints-get <name> [-p <v>]... [-o <out>]
                locations-regional-endpoints-list <parent> [-p <v>]... [-o <out>]
                locations-service-classes-delete <name> [-p <v>]... [-o <out>]
                locations-service-classes-get <name> [-p <v>]... [-o <out>]
                locations-service-classes-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-service-classes-list <parent> [-p <v>]... [-o <out>]
                locations-service-classes-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-classes-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-classes-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-connection-maps-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-connection-maps-delete <name> [-p <v>]... [-o <out>]
                locations-service-connection-maps-get <name> [-p <v>]... [-o <out>]
                locations-service-connection-maps-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-service-connection-maps-list <parent> [-p <v>]... [-o <out>]
                locations-service-connection-maps-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-connection-maps-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-connection-maps-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-connection-policies-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-connection-policies-delete <name> [-p <v>]... [-o <out>]
                locations-service-connection-policies-get <name> [-p <v>]... [-o <out>]
                locations-service-connection-policies-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-service-connection-policies-list <parent> [-p <v>]... [-o <out>]
                locations-service-connection-policies-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-connection-policies-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-connection-policies-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-connection-tokens-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-service-connection-tokens-delete <name> [-p <v>]... [-o <out>]
                locations-service-connection-tokens-get <name> [-p <v>]... [-o <out>]
                locations-service-connection-tokens-list <parent> [-p <v>]... [-o <out>]
                locations-spokes-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-spokes-delete <name> [-p <v>]... [-o <out>]
                locations-spokes-get <name> [-p <v>]... [-o <out>]
                locations-spokes-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-spokes-list <parent> [-p <v>]... [-o <out>]
                locations-spokes-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-spokes-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-spokes-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
  networkconnectivity1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `networkconnectivity1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/networkconnectivity1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/networkconnectivity1-secret.json`, assuming that the required *networkconnectivity* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `networkconnectivity1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
