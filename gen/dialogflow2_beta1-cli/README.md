<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `dialogflow2-beta1` command-line interface *(CLI)* allows to use most features of the *Google Dialogflow* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Dialogflow* API can be found at the
[official documentation site](https://cloud.google.com/dialogflow/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-dialogflow2_beta1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dialogflow2_beta1-cli).

# Usage

This documentation was generated from the *Dialogflow* API at revision *20240614*. The CLI is at version *5.0.5*.

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
                agent-environments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-environments-delete <name> [-p <v>]... [-o <out>]
                agent-environments-get <name> [-p <v>]... [-o <out>]
                agent-environments-get-history <parent> [-p <v>]... [-o <out>]
                agent-environments-intents-list <parent> [-p <v>]... [-o <out>]
                agent-environments-list <parent> [-p <v>]... [-o <out>]
                agent-environments-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
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
                agent-get-fulfillment <name> [-p <v>]... [-o <out>]
                agent-get-validation-result <parent> [-p <v>]... [-o <out>]
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
                agent-update-fulfillment <name> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-versions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                agent-versions-delete <name> [-p <v>]... [-o <out>]
                agent-versions-get <name> [-p <v>]... [-o <out>]
                agent-versions-list <parent> [-p <v>]... [-o <out>]
                agent-versions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                answer-records-get <name> [-p <v>]... [-o <out>]
                answer-records-list <parent> [-p <v>]... [-o <out>]
                answer-records-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                conversation-profiles-clear-suggestion-feature-config <conversation-profile> (-r <kv>)... [-p <v>]... [-o <out>]
                conversation-profiles-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                conversation-profiles-delete <name> [-p <v>]... [-o <out>]
                conversation-profiles-get <name> [-p <v>]... [-o <out>]
                conversation-profiles-list <parent> [-p <v>]... [-o <out>]
                conversation-profiles-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                conversation-profiles-set-suggestion-feature-config <conversation-profile> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-complete <name> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-get <name> [-p <v>]... [-o <out>]
                conversations-list <parent> [-p <v>]... [-o <out>]
                conversations-messages-batch-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-messages-list <parent> [-p <v>]... [-o <out>]
                conversations-participants-analyze-content <participant> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-participants-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-participants-get <name> [-p <v>]... [-o <out>]
                conversations-participants-list <parent> [-p <v>]... [-o <out>]
                conversations-participants-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-participants-suggestions-compile <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-participants-suggestions-list <parent> [-p <v>]... [-o <out>]
                conversations-participants-suggestions-suggest-articles <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-participants-suggestions-suggest-faq-answers <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-participants-suggestions-suggest-knowledge-assist <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-participants-suggestions-suggest-smart-replies <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-suggestions-search-knowledge <conversation> (-r <kv>)... [-p <v>]... [-o <out>]
                conversations-suggestions-suggest-conversation-summary <conversation> (-r <kv>)... [-p <v>]... [-o <out>]
                delete-agent <parent> [-p <v>]... [-o <out>]
                generators-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                generators-list <parent> [-p <v>]... [-o <out>]
                get-agent <parent> [-p <v>]... [-o <out>]
                knowledge-bases-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                knowledge-bases-delete <name> [-p <v>]... [-o <out>]
                knowledge-bases-documents-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                knowledge-bases-documents-delete <name> [-p <v>]... [-o <out>]
                knowledge-bases-documents-get <name> [-p <v>]... [-o <out>]
                knowledge-bases-documents-import <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                knowledge-bases-documents-list <parent> [-p <v>]... [-o <out>]
                knowledge-bases-documents-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                knowledge-bases-documents-reload <name> (-r <kv>)... [-p <v>]... [-o <out>]
                knowledge-bases-get <name> [-p <v>]... [-o <out>]
                knowledge-bases-list <parent> [-p <v>]... [-o <out>]
                knowledge-bases-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-entity-types-batch-delete <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-entity-types-batch-update <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-entity-types-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-entity-types-delete <name> [-p <v>]... [-o <out>]
                locations-agent-entity-types-entities-batch-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-entity-types-entities-batch-delete <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-entity-types-entities-batch-update <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-entity-types-get <name> [-p <v>]... [-o <out>]
                locations-agent-entity-types-list <parent> [-p <v>]... [-o <out>]
                locations-agent-entity-types-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-environments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-environments-delete <name> [-p <v>]... [-o <out>]
                locations-agent-environments-get <name> [-p <v>]... [-o <out>]
                locations-agent-environments-get-history <parent> [-p <v>]... [-o <out>]
                locations-agent-environments-intents-list <parent> [-p <v>]... [-o <out>]
                locations-agent-environments-list <parent> [-p <v>]... [-o <out>]
                locations-agent-environments-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-contexts-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-contexts-delete <name> [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-contexts-get <name> [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-contexts-list <parent> [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-contexts-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-delete-contexts <parent> [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-detect-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-entity-types-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-entity-types-delete <name> [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-entity-types-get <name> [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-entity-types-list <parent> [-p <v>]... [-o <out>]
                locations-agent-environments-users-sessions-entity-types-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-export <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-get-fulfillment <name> [-p <v>]... [-o <out>]
                locations-agent-get-validation-result <parent> [-p <v>]... [-o <out>]
                locations-agent-import <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-intents-batch-delete <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-intents-batch-update <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-intents-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-intents-delete <name> [-p <v>]... [-o <out>]
                locations-agent-intents-get <name> [-p <v>]... [-o <out>]
                locations-agent-intents-list <parent> [-p <v>]... [-o <out>]
                locations-agent-intents-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-restore <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-search <parent> [-p <v>]... [-o <out>]
                locations-agent-sessions-contexts-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-sessions-contexts-delete <name> [-p <v>]... [-o <out>]
                locations-agent-sessions-contexts-get <name> [-p <v>]... [-o <out>]
                locations-agent-sessions-contexts-list <parent> [-p <v>]... [-o <out>]
                locations-agent-sessions-contexts-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-sessions-delete-contexts <parent> [-p <v>]... [-o <out>]
                locations-agent-sessions-detect-intent <session> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-sessions-entity-types-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-sessions-entity-types-delete <name> [-p <v>]... [-o <out>]
                locations-agent-sessions-entity-types-get <name> [-p <v>]... [-o <out>]
                locations-agent-sessions-entity-types-list <parent> [-p <v>]... [-o <out>]
                locations-agent-sessions-entity-types-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-train <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-update-fulfillment <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-versions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-agent-versions-delete <name> [-p <v>]... [-o <out>]
                locations-agent-versions-get <name> [-p <v>]... [-o <out>]
                locations-agent-versions-list <parent> [-p <v>]... [-o <out>]
                locations-agent-versions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-answer-records-get <name> [-p <v>]... [-o <out>]
                locations-answer-records-list <parent> [-p <v>]... [-o <out>]
                locations-answer-records-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversation-profiles-clear-suggestion-feature-config <conversation-profile> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversation-profiles-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversation-profiles-delete <name> [-p <v>]... [-o <out>]
                locations-conversation-profiles-get <name> [-p <v>]... [-o <out>]
                locations-conversation-profiles-list <parent> [-p <v>]... [-o <out>]
                locations-conversation-profiles-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversation-profiles-set-suggestion-feature-config <conversation-profile> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-complete <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-get <name> [-p <v>]... [-o <out>]
                locations-conversations-list <parent> [-p <v>]... [-o <out>]
                locations-conversations-messages-batch-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-messages-list <parent> [-p <v>]... [-o <out>]
                locations-conversations-participants-analyze-content <participant> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-participants-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-participants-get <name> [-p <v>]... [-o <out>]
                locations-conversations-participants-list <parent> [-p <v>]... [-o <out>]
                locations-conversations-participants-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-participants-suggestions-suggest-articles <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-participants-suggestions-suggest-faq-answers <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-participants-suggestions-suggest-knowledge-assist <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-participants-suggestions-suggest-smart-replies <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-suggestions-search-knowledge <conversation> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-suggestions-suggest-conversation-summary <conversation> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-delete-agent <parent> [-p <v>]... [-o <out>]
                locations-generators-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-generators-delete <name> [-p <v>]... [-o <out>]
                locations-generators-get <name> [-p <v>]... [-o <out>]
                locations-generators-list <parent> [-p <v>]... [-o <out>]
                locations-generators-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-get <name> [-p <v>]... [-o <out>]
                locations-get-agent <parent> [-p <v>]... [-o <out>]
                locations-knowledge-bases-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-knowledge-bases-delete <name> [-p <v>]... [-o <out>]
                locations-knowledge-bases-documents-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-knowledge-bases-documents-delete <name> [-p <v>]... [-o <out>]
                locations-knowledge-bases-documents-get <name> [-p <v>]... [-o <out>]
                locations-knowledge-bases-documents-import <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-knowledge-bases-documents-list <parent> [-p <v>]... [-o <out>]
                locations-knowledge-bases-documents-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-knowledge-bases-documents-reload <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-knowledge-bases-get <name> [-p <v>]... [-o <out>]
                locations-knowledge-bases-list <parent> [-p <v>]... [-o <out>]
                locations-knowledge-bases-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <name> [-p <v>]... [-o <out>]
                locations-set-agent <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-stateless-suggestion-generate <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-suggestions-generate-stateless-summary <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-suggestions-search-knowledge <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                operations-cancel <name> [-p <v>]... [-o <out>]
                operations-get <name> [-p <v>]... [-o <out>]
                operations-list <name> [-p <v>]... [-o <out>]
                set-agent <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                suggestions-generate-stateless-summary <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                suggestions-search-knowledge <parent> (-r <kv>)... [-p <v>]... [-o <out>]
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

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `dialogflow2-beta1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
