<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `connectors1` command-line interface *(CLI)* allows to use most features of the *Google Connectors* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Connectors* API can be found at the
[official documentation site](https://cloud.google.com/apigee/docs/api-platform/connectors/about-connectors).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-connectors1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/connectors1-cli).

# Usage

This documentation was generated from the *Connectors* API at revision *20240619*. The CLI is at version *6.0.0*.

```bash
connectors1 [options]
        projects
                locations-connections-connection-schema-metadata-get-action <name> [-p <v>]... [-o <out>]
                locations-connections-connection-schema-metadata-get-entity-type <name> [-p <v>]... [-o <out>]
                locations-connections-connection-schema-metadata-list-actions <name> [-p <v>]... [-o <out>]
                locations-connections-connection-schema-metadata-list-entity-types <name> [-p <v>]... [-o <out>]
                locations-connections-connection-schema-metadata-refresh <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-connections-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-connections-delete <name> [-p <v>]... [-o <out>]
                locations-connections-event-subscriptions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-connections-event-subscriptions-delete <name> [-p <v>]... [-o <out>]
                locations-connections-event-subscriptions-get <name> [-p <v>]... [-o <out>]
                locations-connections-event-subscriptions-list <parent> [-p <v>]... [-o <out>]
                locations-connections-event-subscriptions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-connections-event-subscriptions-retry <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-connections-get <name> [-p <v>]... [-o <out>]
                locations-connections-get-connection-schema-metadata <name> [-p <v>]... [-o <out>]
                locations-connections-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-connections-list <parent> [-p <v>]... [-o <out>]
                locations-connections-listen-event <resource-path> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-connections-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-connections-repair-eventing <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-connections-runtime-action-schemas-list <parent> [-p <v>]... [-o <out>]
                locations-connections-runtime-entity-schemas-list <parent> [-p <v>]... [-o <out>]
                locations-connections-search <name> [-p <v>]... [-o <out>]
                locations-connections-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-connections-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-custom-connectors-custom-connector-versions-delete <name> [-p <v>]... [-o <out>]
                locations-custom-connectors-custom-connector-versions-deprecate <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-custom-connectors-validate-custom-connector-spec <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-endpoint-attachments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-endpoint-attachments-delete <name> [-p <v>]... [-o <out>]
                locations-endpoint-attachments-get <name> [-p <v>]... [-o <out>]
                locations-endpoint-attachments-list <parent> [-p <v>]... [-o <out>]
                locations-endpoint-attachments-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-get <name> [-p <v>]... [-o <out>]
                locations-get-regional-settings <name> [-p <v>]... [-o <out>]
                locations-get-runtime-config <name> [-p <v>]... [-o <out>]
                locations-global-custom-connectors-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-custom-connectors-custom-connector-versions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-custom-connectors-custom-connector-versions-get <name> [-p <v>]... [-o <out>]
                locations-global-custom-connectors-custom-connector-versions-list <parent> [-p <v>]... [-o <out>]
                locations-global-custom-connectors-delete <name> [-p <v>]... [-o <out>]
                locations-global-custom-connectors-get <name> [-p <v>]... [-o <out>]
                locations-global-custom-connectors-list <parent> [-p <v>]... [-o <out>]
                locations-global-custom-connectors-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-get-settings <name> [-p <v>]... [-o <out>]
                locations-global-managed-zones-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-managed-zones-delete <name> [-p <v>]... [-o <out>]
                locations-global-managed-zones-get <name> [-p <v>]... [-o <out>]
                locations-global-managed-zones-list <parent> [-p <v>]... [-o <out>]
                locations-global-managed-zones-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-global-update-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-operations-delete <name> [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <name> [-p <v>]... [-o <out>]
                locations-providers-connectors-get <name> [-p <v>]... [-o <out>]
                locations-providers-connectors-list <parent> [-p <v>]... [-o <out>]
                locations-providers-connectors-versions-eventtypes-get <name> [-p <v>]... [-o <out>]
                locations-providers-connectors-versions-eventtypes-list <parent> [-p <v>]... [-o <out>]
                locations-providers-connectors-versions-get <name> [-p <v>]... [-o <out>]
                locations-providers-connectors-versions-list <parent> [-p <v>]... [-o <out>]
                locations-providers-get <name> [-p <v>]... [-o <out>]
                locations-providers-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-providers-list <parent> [-p <v>]... [-o <out>]
                locations-providers-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-providers-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-update-regional-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
  connectors1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `connectors1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/connectors1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/connectors1-secret.json`, assuming that the required *connectors* API
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `connectors1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
