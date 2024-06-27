<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `prod-tt-sasportal1-alpha1` command-line interface *(CLI)* allows to use most features of the *Google SAS Portal Testing* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *SAS Portal Testing* API can be found at the
[official documentation site](https://developers.google.com/spectrum-access-system/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-prod_tt_sasportal1_alpha1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/prod_tt_sasportal1_alpha1-cli).

# Usage

This documentation was generated from the *SAS Portal Testing* API at revision *20240626*. The CLI is at version *5.0.5*.

```bash
prod-tt-sasportal1-alpha1 [options]
        customers
                deployments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                deployments-delete <name> [-p <v>]... [-o <out>]
                deployments-devices-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                deployments-devices-create-signed <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                deployments-devices-list <parent> [-p <v>]... [-o <out>]
                deployments-get <name> [-p <v>]... [-o <out>]
                deployments-list <parent> [-p <v>]... [-o <out>]
                deployments-move <name> (-r <kv>)... [-p <v>]... [-o <out>]
                deployments-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-create-signed <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-delete <name> [-p <v>]... [-o <out>]
                devices-get <name> [-p <v>]... [-o <out>]
                devices-list <parent> [-p <v>]... [-o <out>]
                devices-move <name> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-sign-device <name> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-update-signed <name> (-r <kv>)... [-p <v>]... [-o <out>]
                get <name> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                list-gcp-project-deployments [-p <v>]... [-o <out>]
                list-legacy-organizations [-p <v>]... [-o <out>]
                migrate-organization (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-delete <name> [-p <v>]... [-o <out>]
                nodes-deployments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-deployments-list <parent> [-p <v>]... [-o <out>]
                nodes-devices-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-devices-create-signed <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-devices-list <parent> [-p <v>]... [-o <out>]
                nodes-get <name> [-p <v>]... [-o <out>]
                nodes-list <parent> [-p <v>]... [-o <out>]
                nodes-move <name> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-nodes-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-nodes-list <parent> [-p <v>]... [-o <out>]
                nodes-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                provision-deployment (-r <kv>)... [-p <v>]... [-o <out>]
                setup-sas-analytics (-r <kv>)... [-p <v>]... [-o <out>]
        deployments
                devices-delete <name> [-p <v>]... [-o <out>]
                devices-get <name> [-p <v>]... [-o <out>]
                devices-move <name> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-sign-device <name> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-update-signed <name> (-r <kv>)... [-p <v>]... [-o <out>]
                get <name> [-p <v>]... [-o <out>]
        installer
                generate-secret (-r <kv>)... [-p <v>]... [-o <out>]
                validate (-r <kv>)... [-p <v>]... [-o <out>]
        nodes
                deployments-delete <name> [-p <v>]... [-o <out>]
                deployments-devices-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                deployments-devices-create-signed <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                deployments-devices-list <parent> [-p <v>]... [-o <out>]
                deployments-get <name> [-p <v>]... [-o <out>]
                deployments-list <parent> [-p <v>]... [-o <out>]
                deployments-move <name> (-r <kv>)... [-p <v>]... [-o <out>]
                deployments-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-create-signed <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-delete <name> [-p <v>]... [-o <out>]
                devices-get <name> [-p <v>]... [-o <out>]
                devices-list <parent> [-p <v>]... [-o <out>]
                devices-move <name> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-sign-device <name> (-r <kv>)... [-p <v>]... [-o <out>]
                devices-update-signed <name> (-r <kv>)... [-p <v>]... [-o <out>]
                get <name> [-p <v>]... [-o <out>]
                nodes-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-delete <name> [-p <v>]... [-o <out>]
                nodes-deployments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-deployments-list <parent> [-p <v>]... [-o <out>]
                nodes-devices-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-devices-create-signed <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-devices-list <parent> [-p <v>]... [-o <out>]
                nodes-get <name> [-p <v>]... [-o <out>]
                nodes-list <parent> [-p <v>]... [-o <out>]
                nodes-move <name> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-nodes-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                nodes-nodes-list <parent> [-p <v>]... [-o <out>]
                nodes-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
        policies
                get (-r <kv>)... [-p <v>]... [-o <out>]
                set (-r <kv>)... [-p <v>]... [-o <out>]
                test (-r <kv>)... [-p <v>]... [-o <out>]
  prod-tt-sasportal1-alpha1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `prod-tt-sasportal1-alpha1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/prod-tt-sasportal1-alpha1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/prod-tt-sasportal1-alpha1-secret.json`, assuming that the required *prod_tt_sasportal* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `prod-tt-sasportal1-alpha1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
