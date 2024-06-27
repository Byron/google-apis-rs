<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `tagmanager2` command-line interface *(CLI)* allows to use most features of the *Google Tag Manager* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Tag Manager* API can be found at the
[official documentation site](https://developers.google.com/tag-manager).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-tagmanager2-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/tagmanager2-cli).

# Usage

This documentation was generated from the *Tag Manager* API at revision *20240619*. The CLI is at version *5.0.5*.

```bash
tagmanager2 [options]
        accounts
                containers-combine <path> [-p <v>]... [-o <out>]
                containers-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-delete <path> [-p <v>]...
                containers-destinations-get <path> [-p <v>]... [-o <out>]
                containers-destinations-link <parent> [-p <v>]... [-o <out>]
                containers-destinations-list <parent> [-p <v>]... [-o <out>]
                containers-environments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-environments-delete <path> [-p <v>]...
                containers-environments-get <path> [-p <v>]... [-o <out>]
                containers-environments-list <parent> [-p <v>]... [-o <out>]
                containers-environments-reauthorize <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-environments-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-get <path> [-p <v>]... [-o <out>]
                containers-list <parent> [-p <v>]... [-o <out>]
                containers-lookup [-p <v>]... [-o <out>]
                containers-move-tag-id <path> [-p <v>]... [-o <out>]
                containers-snippet <path> [-p <v>]... [-o <out>]
                containers-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-version-headers-latest <parent> [-p <v>]... [-o <out>]
                containers-version-headers-list <parent> [-p <v>]... [-o <out>]
                containers-versions-delete <path> [-p <v>]...
                containers-versions-get <path> [-p <v>]... [-o <out>]
                containers-versions-live <parent> [-p <v>]... [-o <out>]
                containers-versions-publish <path> [-p <v>]... [-o <out>]
                containers-versions-set-latest <path> [-p <v>]... [-o <out>]
                containers-versions-undelete <path> [-p <v>]... [-o <out>]
                containers-versions-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-built-in-variables-create <parent> [-p <v>]... [-o <out>]
                containers-workspaces-built-in-variables-delete <path> [-p <v>]...
                containers-workspaces-built-in-variables-list <parent> [-p <v>]... [-o <out>]
                containers-workspaces-built-in-variables-revert <path> [-p <v>]... [-o <out>]
                containers-workspaces-clients-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-clients-delete <path> [-p <v>]...
                containers-workspaces-clients-get <path> [-p <v>]... [-o <out>]
                containers-workspaces-clients-list <parent> [-p <v>]... [-o <out>]
                containers-workspaces-clients-revert <path> [-p <v>]... [-o <out>]
                containers-workspaces-clients-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-create-version <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-delete <path> [-p <v>]...
                containers-workspaces-folders-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-folders-delete <path> [-p <v>]...
                containers-workspaces-folders-entities <path> [-p <v>]... [-o <out>]
                containers-workspaces-folders-get <path> [-p <v>]... [-o <out>]
                containers-workspaces-folders-list <parent> [-p <v>]... [-o <out>]
                containers-workspaces-folders-move-entities-to-folder <path> (-r <kv>)... [-p <v>]...
                containers-workspaces-folders-revert <path> [-p <v>]... [-o <out>]
                containers-workspaces-folders-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-get <path> [-p <v>]... [-o <out>]
                containers-workspaces-get-status <path> [-p <v>]... [-o <out>]
                containers-workspaces-gtag-config-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-gtag-config-delete <path> [-p <v>]...
                containers-workspaces-gtag-config-get <path> [-p <v>]... [-o <out>]
                containers-workspaces-gtag-config-list <parent> [-p <v>]... [-o <out>]
                containers-workspaces-gtag-config-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-list <parent> [-p <v>]... [-o <out>]
                containers-workspaces-quick-preview <path> [-p <v>]... [-o <out>]
                containers-workspaces-resolve-conflict <path> (-r <kv>)... [-p <v>]...
                containers-workspaces-sync <path> [-p <v>]... [-o <out>]
                containers-workspaces-tags-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-tags-delete <path> [-p <v>]...
                containers-workspaces-tags-get <path> [-p <v>]... [-o <out>]
                containers-workspaces-tags-list <parent> [-p <v>]... [-o <out>]
                containers-workspaces-tags-revert <path> [-p <v>]... [-o <out>]
                containers-workspaces-tags-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-templates-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-templates-delete <path> [-p <v>]...
                containers-workspaces-templates-get <path> [-p <v>]... [-o <out>]
                containers-workspaces-templates-list <parent> [-p <v>]... [-o <out>]
                containers-workspaces-templates-revert <path> [-p <v>]... [-o <out>]
                containers-workspaces-templates-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-transformations-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-transformations-delete <path> [-p <v>]...
                containers-workspaces-transformations-get <path> [-p <v>]... [-o <out>]
                containers-workspaces-transformations-list <parent> [-p <v>]... [-o <out>]
                containers-workspaces-transformations-revert <path> [-p <v>]... [-o <out>]
                containers-workspaces-transformations-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-triggers-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-triggers-delete <path> [-p <v>]...
                containers-workspaces-triggers-get <path> [-p <v>]... [-o <out>]
                containers-workspaces-triggers-list <parent> [-p <v>]... [-o <out>]
                containers-workspaces-triggers-revert <path> [-p <v>]... [-o <out>]
                containers-workspaces-triggers-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-variables-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-variables-delete <path> [-p <v>]...
                containers-workspaces-variables-get <path> [-p <v>]... [-o <out>]
                containers-workspaces-variables-list <parent> [-p <v>]... [-o <out>]
                containers-workspaces-variables-revert <path> [-p <v>]... [-o <out>]
                containers-workspaces-variables-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-zones-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                containers-workspaces-zones-delete <path> [-p <v>]...
                containers-workspaces-zones-get <path> [-p <v>]... [-o <out>]
                containers-workspaces-zones-list <parent> [-p <v>]... [-o <out>]
                containers-workspaces-zones-revert <path> [-p <v>]... [-o <out>]
                containers-workspaces-zones-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                get <path> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                update <path> (-r <kv>)... [-p <v>]... [-o <out>]
                user-permissions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                user-permissions-delete <path> [-p <v>]...
                user-permissions-get <path> [-p <v>]... [-o <out>]
                user-permissions-list <parent> [-p <v>]... [-o <out>]
                user-permissions-update <path> (-r <kv>)... [-p <v>]... [-o <out>]
  tagmanager2 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `tagmanager2-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/tagmanager2-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/tagmanager2-secret.json`, assuming that the required *tagmanager* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `tagmanager2 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
