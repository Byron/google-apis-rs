<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `androidenterprise1` command-line interface *(CLI)* allows to use most features of the *Google Android Enterprise* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Android Enterprise* API can be found at the
[official documentation site](https://developers.google.com/android/work/play/emm-api).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-androidenterprise1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/androidenterprise1-cli).

# Usage

This documentation was generated from the *Android Enterprise* API at revision *20240625*. The CLI is at version *5.0.5*.

```bash
androidenterprise1 [options]
        devices
                force-report-upload <enterprise-id> <user-id> <device-id> [-p <v>]...
                get <enterprise-id> <user-id> <device-id> [-p <v>]... [-o <out>]
                get-state <enterprise-id> <user-id> <device-id> [-p <v>]... [-o <out>]
                list <enterprise-id> <user-id> [-p <v>]... [-o <out>]
                set-state <enterprise-id> <user-id> <device-id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <enterprise-id> <user-id> <device-id> (-r <kv>)... [-p <v>]... [-o <out>]
        enterprises
                acknowledge-notification-set [-p <v>]...
                complete-signup [-p <v>]... [-o <out>]
                create-enrollment-token <enterprise-id> [-p <v>]... [-o <out>]
                create-web-token <enterprise-id> (-r <kv>)... [-p <v>]... [-o <out>]
                enroll <token> (-r <kv>)... [-p <v>]... [-o <out>]
                generate-signup-url [-p <v>]... [-o <out>]
                get <enterprise-id> [-p <v>]... [-o <out>]
                get-service-account <enterprise-id> [-p <v>]... [-o <out>]
                get-store-layout <enterprise-id> [-p <v>]... [-o <out>]
                list <domain> [-p <v>]... [-o <out>]
                pull-notification-set [-p <v>]... [-o <out>]
                send-test-push-notification <enterprise-id> [-p <v>]... [-o <out>]
                set-account <enterprise-id> (-r <kv>)... [-p <v>]... [-o <out>]
                set-store-layout <enterprise-id> (-r <kv>)... [-p <v>]... [-o <out>]
                unenroll <enterprise-id> [-p <v>]...
        entitlements
                delete <enterprise-id> <user-id> <entitlement-id> [-p <v>]...
                get <enterprise-id> <user-id> <entitlement-id> [-p <v>]... [-o <out>]
                list <enterprise-id> <user-id> [-p <v>]... [-o <out>]
                update <enterprise-id> <user-id> <entitlement-id> (-r <kv>)... [-p <v>]... [-o <out>]
        grouplicenses
                get <enterprise-id> <group-license-id> [-p <v>]... [-o <out>]
                list <enterprise-id> [-p <v>]... [-o <out>]
        grouplicenseusers
                list <enterprise-id> <group-license-id> [-p <v>]... [-o <out>]
        installs
                delete <enterprise-id> <user-id> <device-id> <install-id> [-p <v>]...
                get <enterprise-id> <user-id> <device-id> <install-id> [-p <v>]... [-o <out>]
                list <enterprise-id> <user-id> <device-id> [-p <v>]... [-o <out>]
                update <enterprise-id> <user-id> <device-id> <install-id> (-r <kv>)... [-p <v>]... [-o <out>]
        managedconfigurationsfordevice
                delete <enterprise-id> <user-id> <device-id> <managed-configuration-for-device-id> [-p <v>]...
                get <enterprise-id> <user-id> <device-id> <managed-configuration-for-device-id> [-p <v>]... [-o <out>]
                list <enterprise-id> <user-id> <device-id> [-p <v>]... [-o <out>]
                update <enterprise-id> <user-id> <device-id> <managed-configuration-for-device-id> (-r <kv>)... [-p <v>]... [-o <out>]
        managedconfigurationsforuser
                delete <enterprise-id> <user-id> <managed-configuration-for-user-id> [-p <v>]...
                get <enterprise-id> <user-id> <managed-configuration-for-user-id> [-p <v>]... [-o <out>]
                list <enterprise-id> <user-id> [-p <v>]... [-o <out>]
                update <enterprise-id> <user-id> <managed-configuration-for-user-id> (-r <kv>)... [-p <v>]... [-o <out>]
        managedconfigurationssettings
                list <enterprise-id> <product-id> [-p <v>]... [-o <out>]
        permissions
                get <permission-id> [-p <v>]... [-o <out>]
        products
                approve <enterprise-id> <product-id> (-r <kv>)... [-p <v>]...
                generate-approval-url <enterprise-id> <product-id> [-p <v>]... [-o <out>]
                get <enterprise-id> <product-id> [-p <v>]... [-o <out>]
                get-app-restrictions-schema <enterprise-id> <product-id> [-p <v>]... [-o <out>]
                get-permissions <enterprise-id> <product-id> [-p <v>]... [-o <out>]
                list <enterprise-id> [-p <v>]... [-o <out>]
                unapprove <enterprise-id> <product-id> [-p <v>]...
        serviceaccountkeys
                delete <enterprise-id> <key-id> [-p <v>]...
                insert <enterprise-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <enterprise-id> [-p <v>]... [-o <out>]
        storelayoutclusters
                delete <enterprise-id> <page-id> <cluster-id> [-p <v>]...
                get <enterprise-id> <page-id> <cluster-id> [-p <v>]... [-o <out>]
                insert <enterprise-id> <page-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <enterprise-id> <page-id> [-p <v>]... [-o <out>]
                update <enterprise-id> <page-id> <cluster-id> (-r <kv>)... [-p <v>]... [-o <out>]
        storelayoutpages
                delete <enterprise-id> <page-id> [-p <v>]...
                get <enterprise-id> <page-id> [-p <v>]... [-o <out>]
                insert <enterprise-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <enterprise-id> [-p <v>]... [-o <out>]
                update <enterprise-id> <page-id> (-r <kv>)... [-p <v>]... [-o <out>]
        users
                delete <enterprise-id> <user-id> [-p <v>]...
                generate-authentication-token <enterprise-id> <user-id> [-p <v>]... [-o <out>]
                get <enterprise-id> <user-id> [-p <v>]... [-o <out>]
                get-available-product-set <enterprise-id> <user-id> [-p <v>]... [-o <out>]
                insert <enterprise-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <enterprise-id> <email> [-p <v>]... [-o <out>]
                revoke-device-access <enterprise-id> <user-id> [-p <v>]...
                set-available-product-set <enterprise-id> <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <enterprise-id> <user-id> (-r <kv>)... [-p <v>]... [-o <out>]
        webapps
                delete <enterprise-id> <web-app-id> [-p <v>]...
                get <enterprise-id> <web-app-id> [-p <v>]... [-o <out>]
                insert <enterprise-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <enterprise-id> [-p <v>]... [-o <out>]
                update <enterprise-id> <web-app-id> (-r <kv>)... [-p <v>]... [-o <out>]
  androidenterprise1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `androidenterprise1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/androidenterprise1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/androidenterprise1-secret.json`, assuming that the required *androidenterprise* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `androidenterprise1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
