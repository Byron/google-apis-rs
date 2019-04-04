<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/cli/README.md.mako'
DO NOT EDIT !
-->
The `dialogflow2-beta1` command-line interface *(CLI)* allows to use most features of the *Google Dialogflow* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Dialogflow* API can be found at the
[official documentation site](https://cloud.google.com/dialogflow-enterprise/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-dialogflow2_beta1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/dialogflow2_beta1-cli).

# Usage

This documentation was generated from the *Dialogflow* API at revision *20190402*. The CLI is at version *1.0.8*.

```bash
dialogflow2-beta1 [options]
        projects
                agent-entity-types-batch-delete <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-entity-types-batch-update <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-entity-types-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-entity-types-delete <name> [-p <v>]... [-o <out>]
                agent-entity-types-entities-batch-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-entity-types-entities-batch-delete <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-entity-types-entities-batch-update <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-entity-types-get <name> [-p <v>]... [-o <out>]
                agent-entity-types-list <parent> [-p <v>]... [-o <out>]
                agent-entity-types-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-environments-users-sessions-contexts-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-environments-users-sessions-contexts-delete <name> [-p <v>]... [-o <out>]
                agent-environments-users-sessions-contexts-get <name> [-p <v>]... [-o <out>]
                agent-environments-users-sessions-contexts-list <parent> [-p <v>]... [-o <out>]
                agent-environments-users-sessions-contexts-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-environments-users-sessions-delete-contexts <parent> [-p <v>]... [-o <out>]
                agent-environments-users-sessions-detect-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-environments-users-sessions-entity-types-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-environments-users-sessions-entity-types-delete <name> [-p <v>]... [-o <out>]
                agent-environments-users-sessions-entity-types-get <name> [-p <v>]... [-o <out>]
                agent-environments-users-sessions-entity-types-list <parent> [-p <v>]... [-o <out>]
                agent-environments-users-sessions-entity-types-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-export <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-import <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-intents-batch-delete <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-intents-batch-update <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-intents-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-intents-delete <name> [-p <v>]... [-o <out>]
                agent-intents-get <name> [-p <v>]... [-o <out>]
                agent-intents-list <parent> [-p <v>]... [-o <out>]
                agent-intents-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-knowledge-bases-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-knowledge-bases-delete <name> [-p <v>]... [-o <out>]
                agent-knowledge-bases-documents-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-knowledge-bases-documents-delete <name> [-p <v>]... [-o <out>]
                agent-knowledge-bases-documents-get <name> [-p <v>]... [-o <out>]
                agent-knowledge-bases-documents-list <parent> [-p <v>]... [-o <out>]
                agent-knowledge-bases-documents-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-knowledge-bases-documents-reload <name> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-knowledge-bases-get <name> [-p <v>]... [-o <out>]
                agent-knowledge-bases-list <parent> [-p <v>]... [-o <out>]
                agent-knowledge-bases-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-restore <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-search <parent> [-p <v>]... [-o <out>]
                agent-sessions-contexts-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-sessions-contexts-delete <name> [-p <v>]... [-o <out>]
                agent-sessions-contexts-get <name> [-p <v>]... [-o <out>]
                agent-sessions-contexts-list <parent> [-p <v>]... [-o <out>]
                agent-sessions-contexts-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-sessions-delete-contexts <parent> [-p <v>]... [-o <out>]
                agent-sessions-detect-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-sessions-entity-types-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-sessions-entity-types-delete <name> [-p <v>]... [-o <out>]
                agent-sessions-entity-types-get <name> [-p <v>]... [-o <out>]
                agent-sessions-entity-types-list <parent> [-p <v>]... [-o <out>]
                agent-sessions-entity-types-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-train <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                get-agent <parent> [-p <v>]... [-o <out>]
                knowledge-bases-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                knowledge-bases-delete <name> [-p <v>]... [-o <out>]
                knowledge-bases-documents-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                knowledge-bases-documents-delete <name> [-p <v>]... [-o <out>]
                knowledge-bases-documents-get <name> [-p <v>]... [-o <out>]
                knowledge-bases-documents-list <parent> [-p <v>]... [-o <out>]
                knowledge-bases-documents-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                knowledge-bases-documents-reload <name> (-r <kv>)... [-p <v>]... [-o <out>]
                knowledge-bases-get <name> [-p <v>]... [-o <out>]
                knowledge-bases-list <parent> [-p <v>]... [-o <out>]
                knowledge-bases-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                operations-get <name> [-p <v>]... [-o <out>]
  dialogflow2-beta1 --help

Configuration:
  [--scope <url>]...
            Specify the authentication a method should be executed in. Each scope
            requires the user to grant this application permission to use it.
            If unset, it defaults to the shortest scope url for a particular method.
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to
            a user-writable directory that we will create during the first invocation.
            [default: ~/.google-service-cli]
  --debug
            Output all server communication to standard error. `tx` and `rx` are placed
            into the same stream.
  --debug-auth
            Output all communication related to authentication to standard error. `tx`
            and `rx` are placed into the same stream.

```

# Configuration

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `dialogflow2-beta1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/dialogflow2-beta1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/dialogflow2-beta1-secret.json`, assuming that the required *dialogflow* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print all client-server communication to standard error, whereas the `--debug-auth` flag
will cause all communication related to authentication to standard error.
If the `--debug` flag is set, error-results will be debug-printed, possibly yielding more information about the 
issue at hand.

You may consider redirecting standard error into a file for ease of use, e.g. `dialogflow2-beta1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/