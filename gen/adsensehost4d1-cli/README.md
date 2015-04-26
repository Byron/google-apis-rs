<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/cli/README.md.mako'
DO NOT EDIT !
-->
The `adsensehost4d1` command-line interface *(CLI)* allows to use most features of the *Google AdSense Host* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

# Usage

This documentation was generated from the *AdSense Host* API at revision *20150307*. The CLI is at version *0.1.0*.

```bash
  adsensehost4d1 [options] accounts adclients-get <account-id> <ad-client-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] accounts adclients-list <account-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] accounts adunits-delete <account-id> <ad-client-id> <ad-unit-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] accounts adunits-get <account-id> <ad-client-id> <ad-unit-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] accounts adunits-get-ad-code <account-id> <ad-client-id> <ad-unit-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] accounts adunits-insert <account-id> <ad-client-id> -r <kv>... [-p <v>...] [-o <out>]
  adsensehost4d1 [options] accounts adunits-list <account-id> <ad-client-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] accounts adunits-patch <account-id> <ad-client-id> <ad-unit-id> -r <kv>... [-p <v>...] [-o <out>]
  adsensehost4d1 [options] accounts adunits-update <account-id> <ad-client-id> -r <kv>... [-p <v>...] [-o <out>]
  adsensehost4d1 [options] accounts get <account-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] accounts list <filter-ad-client-id>... [-p <v>...] [-o <out>]
  adsensehost4d1 [options] accounts reports-generate <account-id> <start-date> <end-date> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] adclients get <ad-client-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] adclients list [-p <v>...] [-o <out>]
  adsensehost4d1 [options] associationsessions start <product-code>... <website-url> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] associationsessions verify <token> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] customchannels delete <ad-client-id> <custom-channel-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] customchannels get <ad-client-id> <custom-channel-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] customchannels insert <ad-client-id> -r <kv>... [-p <v>...] [-o <out>]
  adsensehost4d1 [options] customchannels list <ad-client-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] customchannels patch <ad-client-id> <custom-channel-id> -r <kv>... [-p <v>...] [-o <out>]
  adsensehost4d1 [options] customchannels update <ad-client-id> -r <kv>... [-p <v>...] [-o <out>]
  adsensehost4d1 [options] reports generate <start-date> <end-date> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] urlchannels delete <ad-client-id> <url-channel-id> [-p <v>...] [-o <out>]
  adsensehost4d1 [options] urlchannels insert <ad-client-id> -r <kv>... [-p <v>...] [-o <out>]
  adsensehost4d1 [options] urlchannels list <ad-client-id> [-p <v>...] [-o <out>]
  adsensehost4d1 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_adsensehost4d1_cli/index.html

Configuration:
  --scope <url>  
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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `adsensehost4d1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/adsensehost4d1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/adsensehost4d1-secret.json`, assuming that the required *adsensehost* API 
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

You may consider redirecting standard error into a file for ease of use, e.g. `adsensehost4d1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/