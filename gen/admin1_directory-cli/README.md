<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/cli/README.md.mako'
DO NOT EDIT !
-->
The `admin1-directory` command-line interface *(CLI)* allows to use most features of the *Google directory* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

# Usage

This documentation was generated from the *directory* API at revision *20150314*. The CLI is at version *0.1.0*.

```bash
  admin1-directory [options] asps delete <user-key> <code-id> [-p <v>...]
  admin1-directory [options] asps get <user-key> <code-id> [-p <v>...] [-o <out>]
  admin1-directory [options] asps list <user-key> [-p <v>...] [-o <out>]
  admin1-directory [options] channels stop -r <kv>... [-p <v>...]
  admin1-directory [options] chromeosdevices get <customer-id> <device-id> [-p <v>...] [-o <out>]
  admin1-directory [options] chromeosdevices list <customer-id> [-p <v>...] [-o <out>]
  admin1-directory [options] chromeosdevices patch <customer-id> <device-id> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] chromeosdevices update <customer-id> <device-id> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] groups aliases-delete <group-key> <alias> [-p <v>...]
  admin1-directory [options] groups aliases-insert <group-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] groups aliases-list <group-key> [-p <v>...] [-o <out>]
  admin1-directory [options] groups delete <group-key> [-p <v>...]
  admin1-directory [options] groups get <group-key> [-p <v>...] [-o <out>]
  admin1-directory [options] groups insert -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] groups list [-p <v>...] [-o <out>]
  admin1-directory [options] groups patch <group-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] groups update <group-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] members delete <group-key> <member-key> [-p <v>...]
  admin1-directory [options] members get <group-key> <member-key> [-p <v>...] [-o <out>]
  admin1-directory [options] members insert <group-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] members list <group-key> [-p <v>...] [-o <out>]
  admin1-directory [options] members patch <group-key> <member-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] members update <group-key> <member-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] mobiledevices action <customer-id> <resource-id> -r <kv>... [-p <v>...]
  admin1-directory [options] mobiledevices delete <customer-id> <resource-id> [-p <v>...]
  admin1-directory [options] mobiledevices get <customer-id> <resource-id> [-p <v>...] [-o <out>]
  admin1-directory [options] mobiledevices list <customer-id> [-p <v>...] [-o <out>]
  admin1-directory [options] notifications delete <customer> <notification-id> [-p <v>...]
  admin1-directory [options] notifications get <customer> <notification-id> [-p <v>...] [-o <out>]
  admin1-directory [options] notifications list <customer> [-p <v>...] [-o <out>]
  admin1-directory [options] notifications patch <customer> <notification-id> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] notifications update <customer> <notification-id> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] orgunits delete <customer-id> <org-unit-path>... [-p <v>...]
  admin1-directory [options] orgunits get <customer-id> <org-unit-path>... [-p <v>...] [-o <out>]
  admin1-directory [options] orgunits insert <customer-id> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] orgunits list <customer-id> [-p <v>...] [-o <out>]
  admin1-directory [options] orgunits patch <customer-id> <org-unit-path>... -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] orgunits update <customer-id> <org-unit-path>... -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] schemas delete <customer-id> <schema-key> [-p <v>...]
  admin1-directory [options] schemas get <customer-id> <schema-key> [-p <v>...] [-o <out>]
  admin1-directory [options] schemas insert <customer-id> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] schemas list <customer-id> [-p <v>...] [-o <out>]
  admin1-directory [options] schemas patch <customer-id> <schema-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] schemas update <customer-id> <schema-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] tokens delete <user-key> <client-id> [-p <v>...]
  admin1-directory [options] tokens get <user-key> <client-id> [-p <v>...] [-o <out>]
  admin1-directory [options] tokens list <user-key> [-p <v>...] [-o <out>]
  admin1-directory [options] users aliases-delete <user-key> <alias> [-p <v>...]
  admin1-directory [options] users aliases-insert <user-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] users aliases-list <user-key> [-p <v>...] [-o <out>]
  admin1-directory [options] users aliases-watch <user-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] users delete <user-key> [-p <v>...]
  admin1-directory [options] users get <user-key> [-p <v>...] [-o <out>]
  admin1-directory [options] users insert -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] users list [-p <v>...] [-o <out>]
  admin1-directory [options] users make-admin <user-key> -r <kv>... [-p <v>...]
  admin1-directory [options] users patch <user-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] users photos-delete <user-key> [-p <v>...]
  admin1-directory [options] users photos-get <user-key> [-p <v>...] [-o <out>]
  admin1-directory [options] users photos-patch <user-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] users photos-update <user-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] users undelete <user-key> -r <kv>... [-p <v>...]
  admin1-directory [options] users update <user-key> -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] users watch -r <kv>... [-p <v>...] [-o <out>]
  admin1-directory [options] verification-codes generate <user-key> [-p <v>...]
  admin1-directory [options] verification-codes invalidate <user-key> [-p <v>...]
  admin1-directory [options] verification-codes list <user-key> [-p <v>...] [-o <out>]
  admin1-directory --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_admin1_directory_cli/index.html

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `admin1-directory-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/admin1-directory-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/admin1-directory-secret.json`, assuming that the required *admin* API 
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

You may consider redirecting standard error into a file for ease of use, e.g. `admin1-directory --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/