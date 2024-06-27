<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `content2-sandbox` command-line interface *(CLI)* allows to use most features of the *Google Shopping Content* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Shopping Content* API can be found at the
[official documentation site](https://developers.google.com/shopping-content).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-content2_sandbox-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/content2_sandbox-cli).

# Usage

This documentation was generated from the *Shopping Content* API at revision *20181009*. The CLI is at version *5.0.5*.

```bash
content2-sandbox [options]
        orderinvoices
                createchargeinvoice <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                createrefundinvoice <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
        orderpayments
                notifyauthapproved <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                notifyauthdeclined <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                notifycharge <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                notifyrefund <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
        orderreturns
                get <merchant-id> <return-id> [-p <v>]... [-o <out>]
                list <merchant-id> [-p <v>]... [-o <out>]
        orders
                acknowledge <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                advancetestorder <merchant-id> <order-id> [-p <v>]... [-o <out>]
                cancel <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                cancellineitem <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                canceltestorderbycustomer <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                createtestorder <merchant-id> (-r <kv>)... [-p <v>]... [-o <out>]
                createtestreturn <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                custombatch (-r <kv>)... [-p <v>]... [-o <out>]
                get <merchant-id> <order-id> [-p <v>]... [-o <out>]
                getbymerchantorderid <merchant-id> <merchant-order-id> [-p <v>]... [-o <out>]
                gettestordertemplate <merchant-id> <template-name> [-p <v>]... [-o <out>]
                instorerefundlineitem <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <merchant-id> [-p <v>]... [-o <out>]
                refund <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                rejectreturnlineitem <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                returnlineitem <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                returnrefundlineitem <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                setlineitemmetadata <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                shiplineitems <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                updatelineitemshippingdetails <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                updatemerchantorderid <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
                updateshipment <merchant-id> <order-id> (-r <kv>)... [-p <v>]... [-o <out>]
  content2-sandbox --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `content2-sandbox-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/content2-sandbox-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/content2-sandbox-secret.json`, assuming that the required *content* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `content2-sandbox --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
