<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `dfareporting3d2` command-line interface *(CLI)* allows to use most features of the *Google dfareporting* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *dfareporting* API can be found at the
[official documentation site](https://developers.google.com/doubleclick-advertisers/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-dfareporting3d2-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dfareporting3d2-cli).

# Usage

This documentation was generated from the *dfareporting* API at revision *20190531*. The CLI is at version *5.0.5*.

```bash
dfareporting3d2 [options]
        account-active-ad-summaries
                get <profile-id> <summary-account-id> [-p <v>]... [-o <out>]
        account-permission-groups
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        account-permissions
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        account-user-profiles
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        accounts
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        ads
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        advertiser-groups
                delete <profile-id> <id> [-p <v>]...
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        advertiser-landing-pages
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        advertisers
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        browsers
                list <profile-id> [-p <v>]... [-o <out>]
        campaign-creative-associations
                insert <profile-id> <campaign-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> <campaign-id> [-p <v>]... [-o <out>]
        campaigns
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        change-logs
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        cities
                list <profile-id> [-p <v>]... [-o <out>]
        connection-types
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        content-categories
                delete <profile-id> <id> [-p <v>]...
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        conversions
                batchinsert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                batchupdate <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        countries
                get <profile-id> <dart-id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        creative-assets
                insert <profile-id> <advertiser-id> (-r <kv>)... (-u simple -f <file> [-m <mime>]) [-p <v>]... [-o <out>]
        creative-field-values
                delete <profile-id> <creative-field-id> <id> [-p <v>]...
                get <profile-id> <creative-field-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> <creative-field-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> <creative-field-id> [-p <v>]... [-o <out>]
                patch <profile-id> <creative-field-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> <creative-field-id> (-r <kv>)... [-p <v>]... [-o <out>]
        creative-fields
                delete <profile-id> <id> [-p <v>]...
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        creative-groups
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        creatives
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        dimension-values
                query <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        directory-site-contacts
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        directory-sites
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        dynamic-targeting-keys
                delete <profile-id> <object-id> <name> <object-type> [-p <v>]...
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        event-tags
                delete <profile-id> <id> [-p <v>]...
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        files
                get <report-id> <file-id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        floodlight-activities
                delete <profile-id> <id> [-p <v>]...
                generatetag <profile-id> [-p <v>]... [-o <out>]
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        floodlight-activity-groups
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        floodlight-configurations
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        inventory-items
                get <profile-id> <project-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> <project-id> [-p <v>]... [-o <out>]
        languages
                list <profile-id> [-p <v>]... [-o <out>]
        metros
                list <profile-id> [-p <v>]... [-o <out>]
        mobile-apps
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        mobile-carriers
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        operating-system-versions
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        operating-systems
                get <profile-id> <dart-id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        order-documents
                get <profile-id> <project-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> <project-id> [-p <v>]... [-o <out>]
        orders
                get <profile-id> <project-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> <project-id> [-p <v>]... [-o <out>]
        placement-groups
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        placement-strategies
                delete <profile-id> <id> [-p <v>]...
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        placements
                generatetags <profile-id> [-p <v>]... [-o <out>]
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        platform-types
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        postal-codes
                get <profile-id> <code> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        projects
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        regions
                list <profile-id> [-p <v>]... [-o <out>]
        remarketing-list-shares
                get <profile-id> <remarketing-list-id> [-p <v>]... [-o <out>]
                patch <profile-id> <remarketing-list-id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        remarketing-lists
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> <advertiser-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        reports
                compatible-fields-query <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <profile-id> <report-id> [-p <v>]...
                files-get <profile-id> <report-id> <file-id> [-p <v>]... [-o <out>]
                files-list <profile-id> <report-id> [-p <v>]... [-o <out>]
                get <profile-id> <report-id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <report-id> (-r <kv>)... [-p <v>]... [-o <out>]
                run <profile-id> <report-id> [-p <v>]... [-o <out>]
                update <profile-id> <report-id> (-r <kv>)... [-p <v>]... [-o <out>]
        sites
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        sizes
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        subaccounts
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        targetable-remarketing-lists
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> <advertiser-id> [-p <v>]... [-o <out>]
        targeting-templates
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        user-profiles
                get <profile-id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
        user-role-permission-groups
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        user-role-permissions
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
        user-roles
                delete <profile-id> <id> [-p <v>]...
                get <profile-id> <id> [-p <v>]... [-o <out>]
                insert <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
                patch <profile-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <profile-id> (-r <kv>)... [-p <v>]... [-o <out>]
        video-formats
                get <profile-id> <id> [-p <v>]... [-o <out>]
                list <profile-id> [-p <v>]... [-o <out>]
  dfareporting3d2 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `dfareporting3d2-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/dfareporting3d2-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/dfareporting3d2-secret.json`, assuming that the required *dfareporting* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `dfareporting3d2 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
