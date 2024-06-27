<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `androidpublisher3` command-line interface *(CLI)* allows to use most features of the *Google Android Publisher* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Android Publisher* API can be found at the
[official documentation site](https://developers.google.com/android-publisher).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-androidpublisher3-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/androidpublisher3-cli).

# Usage

This documentation was generated from the *Android Publisher* API at revision *20240626*. The CLI is at version *5.0.5*.

```bash
androidpublisher3 [options]
        applications
                data-safety <package-name> (-r <kv>)... [-p <v>]... [-o <out>]
                device-tier-configs-create <package-name> (-r <kv>)... [-p <v>]... [-o <out>]
                device-tier-configs-get <package-name> <device-tier-config-id> [-p <v>]... [-o <out>]
                device-tier-configs-list <package-name> [-p <v>]... [-o <out>]
        apprecovery
                add-targeting <package-name> <app-recovery-id> (-r <kv>)... [-p <v>]... [-o <out>]
                app-recoveries <package-name> [-p <v>]... [-o <out>]
                cancel <package-name> <app-recovery-id> (-r <kv>)... [-p <v>]... [-o <out>]
                create <package-name> (-r <kv>)... [-p <v>]... [-o <out>]
                deploy <package-name> <app-recovery-id> (-r <kv>)... [-p <v>]... [-o <out>]
        edits
                apks-addexternallyhosted <package-name> <edit-id> (-r <kv>)... [-p <v>]... [-o <out>]
                apks-list <package-name> <edit-id> [-p <v>]... [-o <out>]
                apks-upload <package-name> <edit-id> (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                bundles-list <package-name> <edit-id> [-p <v>]... [-o <out>]
                bundles-upload <package-name> <edit-id> (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                commit <package-name> <edit-id> [-p <v>]... [-o <out>]
                countryavailability-get <package-name> <edit-id> <track> [-p <v>]... [-o <out>]
                delete <package-name> <edit-id> [-p <v>]...
                deobfuscationfiles-upload <package-name> <edit-id> <apk-version-code> <deobfuscation-file-type> (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                details-get <package-name> <edit-id> [-p <v>]... [-o <out>]
                details-patch <package-name> <edit-id> (-r <kv>)... [-p <v>]... [-o <out>]
                details-update <package-name> <edit-id> (-r <kv>)... [-p <v>]... [-o <out>]
                expansionfiles-get <package-name> <edit-id> <apk-version-code> <expansion-file-type> [-p <v>]... [-o <out>]
                expansionfiles-patch <package-name> <edit-id> <apk-version-code> <expansion-file-type> (-r <kv>)... [-p <v>]... [-o <out>]
                expansionfiles-update <package-name> <edit-id> <apk-version-code> <expansion-file-type> (-r <kv>)... [-p <v>]... [-o <out>]
                expansionfiles-upload <package-name> <edit-id> <apk-version-code> <expansion-file-type> (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                get <package-name> <edit-id> [-p <v>]... [-o <out>]
                images-delete <package-name> <edit-id> <language> <image-type> <image-id> [-p <v>]...
                images-deleteall <package-name> <edit-id> <language> <image-type> [-p <v>]... [-o <out>]
                images-list <package-name> <edit-id> <language> <image-type> [-p <v>]... [-o <out>]
                images-upload <package-name> <edit-id> <language> <image-type> (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                insert <package-name> (-r <kv>)... [-p <v>]... [-o <out>]
                listings-delete <package-name> <edit-id> <language> [-p <v>]...
                listings-deleteall <package-name> <edit-id> [-p <v>]...
                listings-get <package-name> <edit-id> <language> [-p <v>]... [-o <out>]
                listings-list <package-name> <edit-id> [-p <v>]... [-o <out>]
                listings-patch <package-name> <edit-id> <language> (-r <kv>)... [-p <v>]... [-o <out>]
                listings-update <package-name> <edit-id> <language> (-r <kv>)... [-p <v>]... [-o <out>]
                testers-get <package-name> <edit-id> <track> [-p <v>]... [-o <out>]
                testers-patch <package-name> <edit-id> <track> (-r <kv>)... [-p <v>]... [-o <out>]
                testers-update <package-name> <edit-id> <track> (-r <kv>)... [-p <v>]... [-o <out>]
                tracks-create <package-name> <edit-id> (-r <kv>)... [-p <v>]... [-o <out>]
                tracks-get <package-name> <edit-id> <track> [-p <v>]... [-o <out>]
                tracks-list <package-name> <edit-id> [-p <v>]... [-o <out>]
                tracks-patch <package-name> <edit-id> <track> (-r <kv>)... [-p <v>]... [-o <out>]
                tracks-update <package-name> <edit-id> <track> (-r <kv>)... [-p <v>]... [-o <out>]
                validate <package-name> <edit-id> [-p <v>]... [-o <out>]
        externaltransactions
                createexternaltransaction <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                getexternaltransaction <name> [-p <v>]... [-o <out>]
                refundexternaltransaction <name> (-r <kv>)... [-p <v>]... [-o <out>]
        generatedapks
                download <package-name> <version-code> <download-id> [-p <v>]... [-o <out>]
                list <package-name> <version-code> [-p <v>]... [-o <out>]
        grants
                create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <name> [-p <v>]...
                patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
        inappproducts
                batch-delete <package-name> (-r <kv>)... [-p <v>]...
                batch-get <package-name> [-p <v>]... [-o <out>]
                batch-update <package-name> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <package-name> <sku> [-p <v>]...
                get <package-name> <sku> [-p <v>]... [-o <out>]
                insert <package-name> (-r <kv>)... [-p <v>]... [-o <out>]
                list <package-name> [-p <v>]... [-o <out>]
                patch <package-name> <sku> (-r <kv>)... [-p <v>]... [-o <out>]
                update <package-name> <sku> (-r <kv>)... [-p <v>]... [-o <out>]
        internalappsharingartifacts
                uploadapk <package-name> (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
                uploadbundle <package-name> (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
        monetization
                convert-region-prices <package-name> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-archive <package-name> <product-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-activate <package-name> <product-id> <base-plan-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-batch-migrate-prices <package-name> <product-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-batch-update-states <package-name> <product-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-deactivate <package-name> <product-id> <base-plan-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-delete <package-name> <product-id> <base-plan-id> [-p <v>]...
                subscriptions-base-plans-migrate-prices <package-name> <product-id> <base-plan-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-offers-activate <package-name> <product-id> <base-plan-id> <offer-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-offers-batch-get <package-name> <product-id> <base-plan-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-offers-batch-update <package-name> <product-id> <base-plan-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-offers-batch-update-states <package-name> <product-id> <base-plan-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-offers-create <package-name> <product-id> <base-plan-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-offers-deactivate <package-name> <product-id> <base-plan-id> <offer-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-base-plans-offers-delete <package-name> <product-id> <base-plan-id> <offer-id> [-p <v>]...
                subscriptions-base-plans-offers-get <package-name> <product-id> <base-plan-id> <offer-id> [-p <v>]... [-o <out>]
                subscriptions-base-plans-offers-list <package-name> <product-id> <base-plan-id> [-p <v>]... [-o <out>]
                subscriptions-base-plans-offers-patch <package-name> <product-id> <base-plan-id> <offer-id> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-batch-get <package-name> [-p <v>]... [-o <out>]
                subscriptions-batch-update <package-name> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-create <package-name> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-delete <package-name> <product-id> [-p <v>]...
                subscriptions-get <package-name> <product-id> [-p <v>]... [-o <out>]
                subscriptions-list <package-name> [-p <v>]... [-o <out>]
                subscriptions-patch <package-name> <product-id> (-r <kv>)... [-p <v>]... [-o <out>]
        orders
                refund <package-name> <order-id> [-p <v>]...
        purchases
                products-acknowledge <package-name> <product-id> <token> (-r <kv>)... [-p <v>]...
                products-consume <package-name> <product-id> <token> [-p <v>]...
                products-get <package-name> <product-id> <token> [-p <v>]... [-o <out>]
                subscriptions-acknowledge <package-name> <subscription-id> <token> (-r <kv>)... [-p <v>]...
                subscriptions-cancel <package-name> <subscription-id> <token> [-p <v>]...
                subscriptions-defer <package-name> <subscription-id> <token> (-r <kv>)... [-p <v>]... [-o <out>]
                subscriptions-get <package-name> <subscription-id> <token> [-p <v>]... [-o <out>]
                subscriptions-refund <package-name> <subscription-id> <token> [-p <v>]...
                subscriptions-revoke <package-name> <subscription-id> <token> [-p <v>]...
                subscriptionsv2-get <package-name> <token> [-p <v>]... [-o <out>]
                subscriptionsv2-revoke <package-name> <token> (-r <kv>)... [-p <v>]... [-o <out>]
                voidedpurchases-list <package-name> [-p <v>]... [-o <out>]
        reviews
                get <package-name> <review-id> [-p <v>]... [-o <out>]
                list <package-name> [-p <v>]... [-o <out>]
                reply <package-name> <review-id> (-r <kv>)... [-p <v>]... [-o <out>]
        systemapks
                variants-create <package-name> <version-code> (-r <kv>)... [-p <v>]... [-o <out>]
                variants-download <package-name> <version-code> <variant-id> [-p <v>]... [-o <out>]
                variants-get <package-name> <version-code> <variant-id> [-p <v>]... [-o <out>]
                variants-list <package-name> <version-code> [-p <v>]... [-o <out>]
        users
                create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <name> [-p <v>]...
                list <parent> [-p <v>]... [-o <out>]
                patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
  androidpublisher3 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `androidpublisher3-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/androidpublisher3-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/androidpublisher3-secret.json`, assuming that the required *androidpublisher* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `androidpublisher3 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
