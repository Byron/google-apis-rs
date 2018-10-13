<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/cli/README.md.mako'
DO NOT EDIT !
-->
The `admin1-directory` command-line interface *(CLI)* allows to use most features of the *Google directory* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *directory* API can be found at the
[official documentation site](https://developers.google.com/admin-sdk/directory/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-admin1_directory-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/admin1_directory-cli).

# Usage

This documentation was generated from the *directory* API at revision *20180917*. The CLI is at version *1.0.7*.

```bash
admin1-directory [options]
        asps
                delete <user-key> <code-id> [-p <v>]...
                get <user-key> <code-id> [-p <v>]... [-o <out>]
                list <user-key> [-p <v>]... [-o <out>]
        channels
                stop (-r <kv>)... [-p <v>]...
        chromeosdevices
                action <customer-id> <resource-id> (-r <kv>)... [-p <v>]...
                get <customer-id> <device-id> [-p <v>]... [-o <out>]
                list <customer-id> [-p <v>]... [-o <out>]
                move-devices-to-ou <customer-id> <org-unit-path> (-r <kv>)... [-p <v>]...
                patch <customer-id> <device-id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <customer-id> <device-id> (-r <kv>)... [-p <v>]... [-o <out>]
        customers
                get <customer-key> [-p <v>]... [-o <out>]
                patch <customer-key> (-r <kv>)... [-p <v>]... [-o <out>]
                update <customer-key> (-r <kv>)... [-p <v>]... [-o <out>]
        domain-aliases
                delete <customer> <domain-alias-name> [-p <v>]...
                get <customer> <domain-alias-name> [-p <v>]... [-o <out>]
                insert <customer> (-r <kv>)... [-p <v>]... [-o <out>]
                list <customer> [-p <v>]... [-o <out>]
        domains
                delete <customer> <domain-name> [-p <v>]...
                get <customer> <domain-name> [-p <v>]... [-o <out>]
                insert <customer> (-r <kv>)... [-p <v>]... [-o <out>]
                list <customer> [-p <v>]... [-o <out>]
        groups
                aliases-delete <group-key> <alias> [-p <v>]...
                aliases-insert <group-key> (-r <kv>)... [-p <v>]... [-o <out>]
                aliases-list <group-key> [-p <v>]... [-o <out>]
                delete <group-key> [-p <v>]...
                get <group-key> [-p <v>]... [-o <out>]
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                patch <group-key> (-r <kv>)... [-p <v>]... [-o <out>]
                update <group-key> (-r <kv>)... [-p <v>]... [-o <out>]
        members
                delete <group-key> <member-key> [-p <v>]...
                get <group-key> <member-key> [-p <v>]... [-o <out>]
                has-member <group-key> <member-key> [-p <v>]... [-o <out>]
                insert <group-key> (-r <kv>)... [-p <v>]... [-o <out>]
                list <group-key> [-p <v>]... [-o <out>]
                patch <group-key> <member-key> (-r <kv>)... [-p <v>]... [-o <out>]
                update <group-key> <member-key> (-r <kv>)... [-p <v>]... [-o <out>]
        mobiledevices
                action <customer-id> <resource-id> (-r <kv>)... [-p <v>]...
                delete <customer-id> <resource-id> [-p <v>]...
                get <customer-id> <resource-id> [-p <v>]... [-o <out>]
                list <customer-id> [-p <v>]... [-o <out>]
        notifications
                delete <customer> <notification-id> [-p <v>]...
                get <customer> <notification-id> [-p <v>]... [-o <out>]
                list <customer> [-p <v>]... [-o <out>]
                patch <customer> <notification-id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <customer> <notification-id> (-r <kv>)... [-p <v>]... [-o <out>]
        orgunits
                delete <customer-id> <org-unit-path>... [-p <v>]...
                get <customer-id> <org-unit-path>... [-p <v>]... [-o <out>]
                insert <customer-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <customer-id> [-p <v>]... [-o <out>]
                patch <customer-id> <org-unit-path>... (-r <kv>)... [-p <v>]... [-o <out>]
                update <customer-id> <org-unit-path>... (-r <kv>)... [-p <v>]... [-o <out>]
        privileges
                list <customer> [-p <v>]... [-o <out>]
        resolved-app-access-settings
                get-settings [-p <v>]... [-o <out>]
                list-trusted-apps [-p <v>]... [-o <out>]
        resources
                buildings-delete <customer> <building-id> [-p <v>]...
                buildings-get <customer> <building-id> [-p <v>]... [-o <out>]
                buildings-insert <customer> (-r <kv>)... [-p <v>]... [-o <out>]
                buildings-list <customer> [-p <v>]... [-o <out>]
                buildings-patch <customer> <building-id> (-r <kv>)... [-p <v>]... [-o <out>]
                buildings-update <customer> <building-id> (-r <kv>)... [-p <v>]... [-o <out>]
                calendars-delete <customer> <calendar-resource-id> [-p <v>]...
                calendars-get <customer> <calendar-resource-id> [-p <v>]... [-o <out>]
                calendars-insert <customer> (-r <kv>)... [-p <v>]... [-o <out>]
                calendars-list <customer> [-p <v>]... [-o <out>]
                calendars-patch <customer> <calendar-resource-id> (-r <kv>)... [-p <v>]... [-o <out>]
                calendars-update <customer> <calendar-resource-id> (-r <kv>)... [-p <v>]... [-o <out>]
                features-delete <customer> <feature-key> [-p <v>]...
                features-get <customer> <feature-key> [-p <v>]... [-o <out>]
                features-insert <customer> (-r <kv>)... [-p <v>]... [-o <out>]
                features-list <customer> [-p <v>]... [-o <out>]
                features-patch <customer> <feature-key> (-r <kv>)... [-p <v>]... [-o <out>]
                features-rename <customer> <old-name> (-r <kv>)... [-p <v>]...
                features-update <customer> <feature-key> (-r <kv>)... [-p <v>]... [-o <out>]
        role-assignments
                delete <customer> <role-assignment-id> [-p <v>]...
                get <customer> <role-assignment-id> [-p <v>]... [-o <out>]
                insert <customer> (-r <kv>)... [-p <v>]... [-o <out>]
                list <customer> [-p <v>]... [-o <out>]
        roles
                delete <customer> <role-id> [-p <v>]...
                get <customer> <role-id> [-p <v>]... [-o <out>]
                insert <customer> (-r <kv>)... [-p <v>]... [-o <out>]
                list <customer> [-p <v>]... [-o <out>]
                patch <customer> <role-id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <customer> <role-id> (-r <kv>)... [-p <v>]... [-o <out>]
        schemas
                delete <customer-id> <schema-key> [-p <v>]...
                get <customer-id> <schema-key> [-p <v>]... [-o <out>]
                insert <customer-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <customer-id> [-p <v>]... [-o <out>]
                patch <customer-id> <schema-key> (-r <kv>)... [-p <v>]... [-o <out>]
                update <customer-id> <schema-key> (-r <kv>)... [-p <v>]... [-o <out>]
        tokens
                delete <user-key> <client-id> [-p <v>]...
                get <user-key> <client-id> [-p <v>]... [-o <out>]
                list <user-key> [-p <v>]... [-o <out>]
        users
                aliases-delete <user-key> <alias> [-p <v>]...
                aliases-insert <user-key> (-r <kv>)... [-p <v>]... [-o <out>]
                aliases-list <user-key> [-p <v>]... [-o <out>]
                aliases-watch <user-key> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <user-key> [-p <v>]...
                get <user-key> [-p <v>]... [-o <out>]
                insert (-r <kv>)... [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                make-admin <user-key> (-r <kv>)... [-p <v>]...
                patch <user-key> (-r <kv>)... [-p <v>]... [-o <out>]
                photos-delete <user-key> [-p <v>]...
                photos-get <user-key> [-p <v>]... [-o <out>]
                photos-patch <user-key> (-r <kv>)... [-p <v>]... [-o <out>]
                photos-update <user-key> (-r <kv>)... [-p <v>]... [-o <out>]
                undelete <user-key> (-r <kv>)... [-p <v>]...
                update <user-key> (-r <kv>)... [-p <v>]... [-o <out>]
                watch (-r <kv>)... [-p <v>]... [-o <out>]
        verification-codes
                generate <user-key> [-p <v>]...
                invalidate <user-key> [-p <v>]...
                list <user-key> [-p <v>]... [-o <out>]
  admin1-directory --help

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