<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `displayvideo1` command-line interface *(CLI)* allows to use most features of the *Google Display Video* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Display Video* API can be found at the
[official documentation site](https://developers.google.com/display-video/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-displayvideo1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/displayvideo1-cli).

# Usage

This documentation was generated from the *Display Video* API at revision *20240620*. The CLI is at version *5.0.5*.

```bash
displayvideo1 [options]
        advertisers
                assets-upload <advertiser-id> (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                audit <advertiser-id> [-p <v>]... [-o <out>]
                bulk-edit-advertiser-assigned-targeting-options <advertiser-id> (-r <kv>)... [-p <v>]... [-o <out>]
                bulk-list-advertiser-assigned-targeting-options <advertiser-id> [-p <v>]... [-o <out>]
                campaigns-bulk-list-campaign-assigned-targeting-options <advertiser-id> <campaign-id> [-p <v>]... [-o <out>]
                campaigns-create <advertiser-id> (-r <kv>)... [-p <v>]... [-o <out>]
                campaigns-delete <advertiser-id> <campaign-id> [-p <v>]... [-o <out>]
                campaigns-get <advertiser-id> <campaign-id> [-p <v>]... [-o <out>]
                campaigns-list <advertiser-id> [-p <v>]... [-o <out>]
                campaigns-patch <advertiser-id> <campaign-id> (-r <kv>)... [-p <v>]... [-o <out>]
                campaigns-targeting-types-assigned-targeting-options-get <advertiser-id> <campaign-id> <targeting-type> <assigned-targeting-option-id> [-p <v>]... [-o <out>]
                campaigns-targeting-types-assigned-targeting-options-list <advertiser-id> <campaign-id> <targeting-type> [-p <v>]... [-o <out>]
                channels-create <advertiser-id> (-r <kv>)... [-p <v>]... [-o <out>]
                channels-get <advertiser-id> <channel-id> [-p <v>]... [-o <out>]
                channels-list <advertiser-id> [-p <v>]... [-o <out>]
                channels-patch <advertiser-id> <channel-id> (-r <kv>)... [-p <v>]... [-o <out>]
                channels-sites-bulk-edit <advertiser-id> <channel-id> (-r <kv>)... [-p <v>]... [-o <out>]
                channels-sites-create <advertiser-id> <channel-id> (-r <kv>)... [-p <v>]... [-o <out>]
                channels-sites-delete <advertiser-id> <channel-id> <url-or-app-id> [-p <v>]... [-o <out>]
                channels-sites-list <advertiser-id> <channel-id> [-p <v>]... [-o <out>]
                channels-sites-replace <advertiser-id> <channel-id> (-r <kv>)... [-p <v>]... [-o <out>]
                create (-r <kv>)... [-p <v>]... [-o <out>]
                creatives-create <advertiser-id> (-r <kv>)... [-p <v>]... [-o <out>]
                creatives-delete <advertiser-id> <creative-id> [-p <v>]... [-o <out>]
                creatives-get <advertiser-id> <creative-id> [-p <v>]... [-o <out>]
                creatives-list <advertiser-id> [-p <v>]... [-o <out>]
                creatives-patch <advertiser-id> <creative-id> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <advertiser-id> [-p <v>]... [-o <out>]
                get <advertiser-id> [-p <v>]... [-o <out>]
                insertion-orders-bulk-list-insertion-order-assigned-targeting-options <advertiser-id> <insertion-order-id> [-p <v>]... [-o <out>]
                insertion-orders-create <advertiser-id> (-r <kv>)... [-p <v>]... [-o <out>]
                insertion-orders-delete <advertiser-id> <insertion-order-id> [-p <v>]... [-o <out>]
                insertion-orders-get <advertiser-id> <insertion-order-id> [-p <v>]... [-o <out>]
                insertion-orders-list <advertiser-id> [-p <v>]... [-o <out>]
                insertion-orders-patch <advertiser-id> <insertion-order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                insertion-orders-targeting-types-assigned-targeting-options-get <advertiser-id> <insertion-order-id> <targeting-type> <assigned-targeting-option-id> [-p <v>]... [-o <out>]
                insertion-orders-targeting-types-assigned-targeting-options-list <advertiser-id> <insertion-order-id> <targeting-type> [-p <v>]... [-o <out>]
                invoices-list <advertiser-id> [-p <v>]... [-o <out>]
                invoices-lookup-invoice-currency <advertiser-id> [-p <v>]... [-o <out>]
                line-items-bulk-edit-line-item-assigned-targeting-options <advertiser-id> <line-item-id> (-r <kv>)... [-p <v>]... [-o <out>]
                line-items-bulk-list-line-item-assigned-targeting-options <advertiser-id> <line-item-id> [-p <v>]... [-o <out>]
                line-items-create <advertiser-id> (-r <kv>)... [-p <v>]... [-o <out>]
                line-items-delete <advertiser-id> <line-item-id> [-p <v>]... [-o <out>]
                line-items-generate-default <advertiser-id> (-r <kv>)... [-p <v>]... [-o <out>]
                line-items-get <advertiser-id> <line-item-id> [-p <v>]... [-o <out>]
                line-items-list <advertiser-id> [-p <v>]... [-o <out>]
                line-items-patch <advertiser-id> <line-item-id> (-r <kv>)... [-p <v>]... [-o <out>]
                line-items-targeting-types-assigned-targeting-options-create <advertiser-id> <line-item-id> <targeting-type> (-r <kv>)... [-p <v>]... [-o <out>]
                line-items-targeting-types-assigned-targeting-options-delete <advertiser-id> <line-item-id> <targeting-type> <assigned-targeting-option-id> [-p <v>]... [-o <out>]
                line-items-targeting-types-assigned-targeting-options-get <advertiser-id> <line-item-id> <targeting-type> <assigned-targeting-option-id> [-p <v>]... [-o <out>]
                line-items-targeting-types-assigned-targeting-options-list <advertiser-id> <line-item-id> <targeting-type> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                location-lists-assigned-locations-bulk-edit <advertiser-id> <location-list-id> (-r <kv>)... [-p <v>]... [-o <out>]
                location-lists-assigned-locations-create <advertiser-id> <location-list-id> (-r <kv>)... [-p <v>]... [-o <out>]
                location-lists-assigned-locations-delete <advertiser-id> <location-list-id> <assigned-location-id> [-p <v>]... [-o <out>]
                location-lists-assigned-locations-list <advertiser-id> <location-list-id> [-p <v>]... [-o <out>]
                location-lists-create <advertiser-id> (-r <kv>)... [-p <v>]... [-o <out>]
                location-lists-get <advertiser-id> <location-list-id> [-p <v>]... [-o <out>]
                location-lists-list <advertiser-id> [-p <v>]... [-o <out>]
                location-lists-patch <advertiser-id> <location-list-id> (-r <kv>)... [-p <v>]... [-o <out>]
                manual-triggers-activate <advertiser-id> <trigger-id> (-r <kv>)... [-p <v>]... [-o <out>]
                manual-triggers-create <advertiser-id> (-r <kv>)... [-p <v>]... [-o <out>]
                manual-triggers-deactivate <advertiser-id> <trigger-id> (-r <kv>)... [-p <v>]... [-o <out>]
                manual-triggers-get <advertiser-id> <trigger-id> [-p <v>]... [-o <out>]
                manual-triggers-list <advertiser-id> [-p <v>]... [-o <out>]
                manual-triggers-patch <advertiser-id> <trigger-id> (-r <kv>)... [-p <v>]... [-o <out>]
                negative-keyword-lists-create <advertiser-id> (-r <kv>)... [-p <v>]... [-o <out>]
                negative-keyword-lists-delete <advertiser-id> <negative-keyword-list-id> [-p <v>]... [-o <out>]
                negative-keyword-lists-get <advertiser-id> <negative-keyword-list-id> [-p <v>]... [-o <out>]
                negative-keyword-lists-list <advertiser-id> [-p <v>]... [-o <out>]
                negative-keyword-lists-negative-keywords-bulk-edit <advertiser-id> <negative-keyword-list-id> (-r <kv>)... [-p <v>]... [-o <out>]
                negative-keyword-lists-negative-keywords-create <advertiser-id> <negative-keyword-list-id> (-r <kv>)... [-p <v>]... [-o <out>]
                negative-keyword-lists-negative-keywords-delete <advertiser-id> <negative-keyword-list-id> <keyword-value> [-p <v>]... [-o <out>]
                negative-keyword-lists-negative-keywords-list <advertiser-id> <negative-keyword-list-id> [-p <v>]... [-o <out>]
                negative-keyword-lists-negative-keywords-replace <advertiser-id> <negative-keyword-list-id> (-r <kv>)... [-p <v>]... [-o <out>]
                negative-keyword-lists-patch <advertiser-id> <negative-keyword-list-id> (-r <kv>)... [-p <v>]... [-o <out>]
                patch <advertiser-id> (-r <kv>)... [-p <v>]... [-o <out>]
                targeting-types-assigned-targeting-options-create <advertiser-id> <targeting-type> (-r <kv>)... [-p <v>]... [-o <out>]
                targeting-types-assigned-targeting-options-delete <advertiser-id> <targeting-type> <assigned-targeting-option-id> [-p <v>]... [-o <out>]
                targeting-types-assigned-targeting-options-get <advertiser-id> <targeting-type> <assigned-targeting-option-id> [-p <v>]... [-o <out>]
                targeting-types-assigned-targeting-options-list <advertiser-id> <targeting-type> [-p <v>]... [-o <out>]
        combined-audiences
                get <combined-audience-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
        custom-bidding-algorithms
                create (-r <kv>)... [-p <v>]... [-o <out>]
                get <custom-bidding-algorithm-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                patch <custom-bidding-algorithm-id> (-r <kv>)... [-p <v>]... [-o <out>]
                scripts-create <custom-bidding-algorithm-id> (-r <kv>)... [-p <v>]... [-o <out>]
                scripts-get <custom-bidding-algorithm-id> <custom-bidding-script-id> [-p <v>]... [-o <out>]
                scripts-list <custom-bidding-algorithm-id> [-p <v>]... [-o <out>]
                upload-script <custom-bidding-algorithm-id> [-p <v>]... [-o <out>]
        custom-lists
                get <custom-list-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
        first-and-third-party-audiences
                create (-r <kv>)... [-p <v>]... [-o <out>]
                edit-customer-match-members <first-and-third-party-audience-id> (-r <kv>)... [-p <v>]... [-o <out>]
                get <first-and-third-party-audience-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                patch <first-and-third-party-audience-id> (-r <kv>)... [-p <v>]... [-o <out>]
        floodlight-groups
                get <floodlight-group-id> [-p <v>]... [-o <out>]
                patch <floodlight-group-id> (-r <kv>)... [-p <v>]... [-o <out>]
        google-audiences
                get <google-audience-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
        guaranteed-orders
                create (-r <kv>)... [-p <v>]... [-o <out>]
                edit-guaranteed-order-read-accessors <guaranteed-order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                get <guaranteed-order-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                patch <guaranteed-order-id> (-r <kv>)... [-p <v>]... [-o <out>]
        inventory-source-groups
                assigned-inventory-sources-bulk-edit <inventory-source-group-id> (-r <kv>)... [-p <v>]... [-o <out>]
                assigned-inventory-sources-create <inventory-source-group-id> (-r <kv>)... [-p <v>]... [-o <out>]
                assigned-inventory-sources-delete <inventory-source-group-id> <assigned-inventory-source-id> [-p <v>]... [-o <out>]
                assigned-inventory-sources-list <inventory-source-group-id> [-p <v>]... [-o <out>]
                create (-r <kv>)... [-p <v>]... [-o <out>]
                delete <inventory-source-group-id> [-p <v>]... [-o <out>]
                get <inventory-source-group-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                patch <inventory-source-group-id> (-r <kv>)... [-p <v>]... [-o <out>]
        inventory-sources
                create (-r <kv>)... [-p <v>]... [-o <out>]
                edit-inventory-source-read-write-accessors <inventory-source-id> (-r <kv>)... [-p <v>]... [-o <out>]
                get <inventory-source-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                patch <inventory-source-id> (-r <kv>)... [-p <v>]... [-o <out>]
        media
                download <resource-name> [-p <v>]... [-o <out>]
                upload <resource-name> (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
        partners
                bulk-edit-partner-assigned-targeting-options <partner-id> (-r <kv>)... [-p <v>]... [-o <out>]
                channels-create <partner-id> (-r <kv>)... [-p <v>]... [-o <out>]
                channels-get <partner-id> <channel-id> [-p <v>]... [-o <out>]
                channels-list <partner-id> [-p <v>]... [-o <out>]
                channels-patch <partner-id> <channel-id> (-r <kv>)... [-p <v>]... [-o <out>]
                channels-sites-bulk-edit <partner-id> <channel-id> (-r <kv>)... [-p <v>]... [-o <out>]
                channels-sites-create <partner-id> <channel-id> (-r <kv>)... [-p <v>]... [-o <out>]
                channels-sites-delete <partner-id> <channel-id> <url-or-app-id> [-p <v>]... [-o <out>]
                channels-sites-list <partner-id> <channel-id> [-p <v>]... [-o <out>]
                channels-sites-replace <partner-id> <channel-id> (-r <kv>)... [-p <v>]... [-o <out>]
                get <partner-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                targeting-types-assigned-targeting-options-create <partner-id> <targeting-type> (-r <kv>)... [-p <v>]... [-o <out>]
                targeting-types-assigned-targeting-options-delete <partner-id> <targeting-type> <assigned-targeting-option-id> [-p <v>]... [-o <out>]
                targeting-types-assigned-targeting-options-get <partner-id> <targeting-type> <assigned-targeting-option-id> [-p <v>]... [-o <out>]
                targeting-types-assigned-targeting-options-list <partner-id> <targeting-type> [-p <v>]... [-o <out>]
        sdfdownloadtasks
                create (-r <kv>)... [-p <v>]... [-o <out>]
                operations-get <name> [-p <v>]... [-o <out>]
        targeting-types
                targeting-options-get <targeting-type> <targeting-option-id> [-p <v>]... [-o <out>]
                targeting-options-list <targeting-type> [-p <v>]... [-o <out>]
                targeting-options-search <targeting-type> (-r <kv>)... [-p <v>]... [-o <out>]
        users
                bulk-edit-assigned-user-roles <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                create (-r <kv>)... [-p <v>]... [-o <out>]
                delete <user-id> [-p <v>]... [-o <out>]
                get <user-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                patch <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
  displayvideo1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `displayvideo1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/displayvideo1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/displayvideo1-secret.json`, assuming that the required *displayvideo* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `displayvideo1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
