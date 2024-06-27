<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `dialogflow3` command-line interface *(CLI)* allows to use most features of the *Google Dialogflow* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Dialogflow* API can be found at the
[official documentation site](https://cloud.google.com/dialogflow/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-dialogflow3-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dialogflow3-cli).

# Usage

This documentation was generated from the *Dialogflow* API at revision *20240614*. The CLI is at version *5.0.5*.

```bash
dialogflow3 [options]
        projects
                locations-agents-changelogs-get <name> [-p <v>]... [-o <out>]
                locations-agents-changelogs-list <parent> [-p <v>]... [-o <out>]
                locations-agents-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-delete <name> [-p <v>]... [-o <out>]
                locations-agents-entity-types-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-entity-types-delete <name> [-p <v>]... [-o <out>]
                locations-agents-entity-types-export <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-entity-types-get <name> [-p <v>]... [-o <out>]
                locations-agents-entity-types-import <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-entity-types-list <parent> [-p <v>]... [-o <out>]
                locations-agents-entity-types-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-continuous-test-results-list <parent> [-p <v>]... [-o <out>]
                locations-agents-environments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-delete <name> [-p <v>]... [-o <out>]
                locations-agents-environments-deploy-flow <environment> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-deployments-get <name> [-p <v>]... [-o <out>]
                locations-agents-environments-deployments-list <parent> [-p <v>]... [-o <out>]
                locations-agents-environments-experiments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-experiments-delete <name> [-p <v>]... [-o <out>]
                locations-agents-environments-experiments-get <name> [-p <v>]... [-o <out>]
                locations-agents-environments-experiments-list <parent> [-p <v>]... [-o <out>]
                locations-agents-environments-experiments-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-experiments-start <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-experiments-stop <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-get <name> [-p <v>]... [-o <out>]
                locations-agents-environments-list <parent> [-p <v>]... [-o <out>]
                locations-agents-environments-lookup-environment-history <name> [-p <v>]... [-o <out>]
                locations-agents-environments-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-run-continuous-test <environment> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-sessions-detect-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-sessions-entity-types-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-sessions-entity-types-delete <name> [-p <v>]... [-o <out>]
                locations-agents-environments-sessions-entity-types-get <name> [-p <v>]... [-o <out>]
                locations-agents-environments-sessions-entity-types-list <parent> [-p <v>]... [-o <out>]
                locations-agents-environments-sessions-entity-types-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-sessions-fulfill-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-sessions-match-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-environments-sessions-server-streaming-detect-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-export <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-delete <name> [-p <v>]... [-o <out>]
                locations-agents-flows-export <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-get <name> [-p <v>]... [-o <out>]
                locations-agents-flows-get-validation-result <name> [-p <v>]... [-o <out>]
                locations-agents-flows-import <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-list <parent> [-p <v>]... [-o <out>]
                locations-agents-flows-pages-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-pages-delete <name> [-p <v>]... [-o <out>]
                locations-agents-flows-pages-get <name> [-p <v>]... [-o <out>]
                locations-agents-flows-pages-list <parent> [-p <v>]... [-o <out>]
                locations-agents-flows-pages-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-train <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-transition-route-groups-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-transition-route-groups-delete <name> [-p <v>]... [-o <out>]
                locations-agents-flows-transition-route-groups-get <name> [-p <v>]... [-o <out>]
                locations-agents-flows-transition-route-groups-list <parent> [-p <v>]... [-o <out>]
                locations-agents-flows-transition-route-groups-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-validate <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-versions-compare-versions <base-version> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-versions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-versions-delete <name> [-p <v>]... [-o <out>]
                locations-agents-flows-versions-get <name> [-p <v>]... [-o <out>]
                locations-agents-flows-versions-list <parent> [-p <v>]... [-o <out>]
                locations-agents-flows-versions-load <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-flows-versions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-generators-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-generators-delete <name> [-p <v>]... [-o <out>]
                locations-agents-generators-get <name> [-p <v>]... [-o <out>]
                locations-agents-generators-list <parent> [-p <v>]... [-o <out>]
                locations-agents-generators-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-get <name> [-p <v>]... [-o <out>]
                locations-agents-get-generative-settings <name> [-p <v>]... [-o <out>]
                locations-agents-get-validation-result <name> [-p <v>]... [-o <out>]
                locations-agents-intents-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-intents-delete <name> [-p <v>]... [-o <out>]
                locations-agents-intents-export <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-intents-get <name> [-p <v>]... [-o <out>]
                locations-agents-intents-import <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-intents-list <parent> [-p <v>]... [-o <out>]
                locations-agents-intents-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-list <parent> [-p <v>]... [-o <out>]
                locations-agents-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-restore <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-sessions-detect-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-sessions-entity-types-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-sessions-entity-types-delete <name> [-p <v>]... [-o <out>]
                locations-agents-sessions-entity-types-get <name> [-p <v>]... [-o <out>]
                locations-agents-sessions-entity-types-list <parent> [-p <v>]... [-o <out>]
                locations-agents-sessions-entity-types-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-sessions-fulfill-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-sessions-match-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-sessions-server-streaming-detect-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-sessions-submit-answer-feedback <session> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-test-cases-batch-delete <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-test-cases-batch-run <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-test-cases-calculate-coverage <agent> [-p <v>]... [-o <out>]
                locations-agents-test-cases-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-test-cases-export <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-test-cases-get <name> [-p <v>]... [-o <out>]
                locations-agents-test-cases-import <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-test-cases-list <parent> [-p <v>]... [-o <out>]
                locations-agents-test-cases-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-test-cases-results-get <name> [-p <v>]... [-o <out>]
                locations-agents-test-cases-results-list <parent> [-p <v>]... [-o <out>]
                locations-agents-test-cases-run <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-transition-route-groups-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-transition-route-groups-delete <name> [-p <v>]... [-o <out>]
                locations-agents-transition-route-groups-get <name> [-p <v>]... [-o <out>]
                locations-agents-transition-route-groups-list <parent> [-p <v>]... [-o <out>]
                locations-agents-transition-route-groups-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-update-generative-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-validate <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-webhooks-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agents-webhooks-delete <name> [-p <v>]... [-o <out>]
                locations-agents-webhooks-get <name> [-p <v>]... [-o <out>]
                locations-agents-webhooks-list <parent> [-p <v>]... [-o <out>]
                locations-agents-webhooks-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-get <name> [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <name> [-p <v>]... [-o <out>]
                locations-security-settings-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-security-settings-delete <name> [-p <v>]... [-o <out>]
                locations-security-settings-get <name> [-p <v>]... [-o <out>]
                locations-security-settings-list <parent> [-p <v>]... [-o <out>]
                locations-security-settings-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                operations-cancel <name> [-p <v>]... [-o <out>]
                operations-get <name> [-p <v>]... [-o <out>]
                operations-list <name> [-p <v>]... [-o <out>]
  dialogflow3 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `dialogflow3-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/dialogflow3-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/dialogflow3-secret.json`, assuming that the required *dialogflow* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `dialogflow3 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
