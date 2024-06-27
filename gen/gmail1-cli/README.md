<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `gmail1` command-line interface *(CLI)* allows to use most features of the *Google Gmail* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Gmail* API can be found at the
[official documentation site](https://developers.google.com/gmail/api/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-gmail1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/gmail1-cli).

# Usage

This documentation was generated from the *Gmail* API at revision *20240624*. The CLI is at version *5.0.5*.

```bash
gmail1 [options]
        users
                drafts-create <user-id> (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                drafts-delete <user-id> <id> [-p <v>]...
                drafts-get <user-id> <id> [-p <v>]... [-o <out>]
                drafts-list <user-id> [-p <v>]... [-o <out>]
                drafts-send <user-id> (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                drafts-update <user-id> <id> (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                get-profile <user-id> [-p <v>]... [-o <out>]
                history-list <user-id> [-p <v>]... [-o <out>]
                labels-create <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                labels-delete <user-id> <id> [-p <v>]...
                labels-get <user-id> <id> [-p <v>]... [-o <out>]
                labels-list <user-id> [-p <v>]... [-o <out>]
                labels-patch <user-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                labels-update <user-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                messages-attachments-get <user-id> <message-id> <id> [-p <v>]... [-o <out>]
                messages-batch-delete <user-id> (-r <kv>)... [-p <v>]...
                messages-batch-modify <user-id> (-r <kv>)... [-p <v>]...
                messages-delete <user-id> <id> [-p <v>]...
                messages-get <user-id> <id> [-p <v>]... [-o <out>]
                messages-import <user-id> (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                messages-insert <user-id> (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                messages-list <user-id> [-p <v>]... [-o <out>]
                messages-modify <user-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                messages-send <user-id> (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                messages-trash <user-id> <id> [-p <v>]... [-o <out>]
                messages-untrash <user-id> <id> [-p <v>]... [-o <out>]
                settings-cse-identities-create <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-cse-identities-delete <user-id> <cse-email-address> [-p <v>]...
                settings-cse-identities-get <user-id> <cse-email-address> [-p <v>]... [-o <out>]
                settings-cse-identities-list <user-id> [-p <v>]... [-o <out>]
                settings-cse-identities-patch <user-id> <email-address> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-cse-keypairs-create <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-cse-keypairs-disable <user-id> <key-pair-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-cse-keypairs-enable <user-id> <key-pair-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-cse-keypairs-get <user-id> <key-pair-id> [-p <v>]... [-o <out>]
                settings-cse-keypairs-list <user-id> [-p <v>]... [-o <out>]
                settings-cse-keypairs-obliterate <user-id> <key-pair-id> (-r <kv>)... [-p <v>]...
                settings-delegates-create <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-delegates-delete <user-id> <delegate-email> [-p <v>]...
                settings-delegates-get <user-id> <delegate-email> [-p <v>]... [-o <out>]
                settings-delegates-list <user-id> [-p <v>]... [-o <out>]
                settings-filters-create <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-filters-delete <user-id> <id> [-p <v>]...
                settings-filters-get <user-id> <id> [-p <v>]... [-o <out>]
                settings-filters-list <user-id> [-p <v>]... [-o <out>]
                settings-forwarding-addresses-create <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-forwarding-addresses-delete <user-id> <forwarding-email> [-p <v>]...
                settings-forwarding-addresses-get <user-id> <forwarding-email> [-p <v>]... [-o <out>]
                settings-forwarding-addresses-list <user-id> [-p <v>]... [-o <out>]
                settings-get-auto-forwarding <user-id> [-p <v>]... [-o <out>]
                settings-get-imap <user-id> [-p <v>]... [-o <out>]
                settings-get-language <user-id> [-p <v>]... [-o <out>]
                settings-get-pop <user-id> [-p <v>]... [-o <out>]
                settings-get-vacation <user-id> [-p <v>]... [-o <out>]
                settings-send-as-create <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-send-as-delete <user-id> <send-as-email> [-p <v>]...
                settings-send-as-get <user-id> <send-as-email> [-p <v>]... [-o <out>]
                settings-send-as-list <user-id> [-p <v>]... [-o <out>]
                settings-send-as-patch <user-id> <send-as-email> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-send-as-smime-info-delete <user-id> <send-as-email> <id> [-p <v>]...
                settings-send-as-smime-info-get <user-id> <send-as-email> <id> [-p <v>]... [-o <out>]
                settings-send-as-smime-info-insert <user-id> <send-as-email> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-send-as-smime-info-list <user-id> <send-as-email> [-p <v>]... [-o <out>]
                settings-send-as-smime-info-set-default <user-id> <send-as-email> <id> [-p <v>]...
                settings-send-as-update <user-id> <send-as-email> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-send-as-verify <user-id> <send-as-email> [-p <v>]...
                settings-update-auto-forwarding <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-update-imap <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-update-language <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-update-pop <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                settings-update-vacation <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                stop <user-id> [-p <v>]...
                threads-delete <user-id> <id> [-p <v>]...
                threads-get <user-id> <id> [-p <v>]... [-o <out>]
                threads-list <user-id> [-p <v>]... [-o <out>]
                threads-modify <user-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                threads-trash <user-id> <id> [-p <v>]... [-o <out>]
                threads-untrash <user-id> <id> [-p <v>]... [-o <out>]
                watch <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
  gmail1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `gmail1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/gmail1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/gmail1-secret.json`, assuming that the required *gmail* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `gmail1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
