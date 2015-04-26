<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/cli/README.md.mako'
DO NOT EDIT !
-->
The `dfareporting2d1` command-line interface *(CLI)* allows to use most features of the *Google dfareporting* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

# Usage

This documentation was generated from the *dfareporting* API at revision *20150326*. The CLI is at version *0.1.0*.

```bash
  dfareporting2d1 [options] account-active-ad-summaries get <profile-id> <summary-account-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] account-permission-groups get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] account-permission-groups list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] account-permissions get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] account-permissions list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] account-user-profiles get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] account-user-profiles insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] account-user-profiles list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] account-user-profiles patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] account-user-profiles update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] accounts get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] accounts list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] accounts patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] accounts update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] ads get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] ads insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] ads list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] ads patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] ads update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] advertiser-groups delete <profile-id> <id> [-p <v>...]
  dfareporting2d1 [options] advertiser-groups get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] advertiser-groups insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] advertiser-groups list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] advertiser-groups patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] advertiser-groups update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] advertisers get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] advertisers insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] advertisers list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] advertisers patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] advertisers update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] browsers list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] campaign-creative-associations insert <profile-id> <campaign-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] campaign-creative-associations list <profile-id> <campaign-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] campaigns get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] campaigns insert <profile-id> <default-landing-page-name> <default-landing-page-url> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] campaigns list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] campaigns patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] campaigns update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] change-logs get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] change-logs list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] cities list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] connection-types get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] connection-types list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] content-categories delete <profile-id> <id> [-p <v>...]
  dfareporting2d1 [options] content-categories get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] content-categories insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] content-categories list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] content-categories patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] content-categories update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] countries get <profile-id> <dart-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] countries list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-assets insert <profile-id> <advertiser-id> -r <kv>... -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-field-values delete <profile-id> <creative-field-id> <id> [-p <v>...]
  dfareporting2d1 [options] creative-field-values get <profile-id> <creative-field-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-field-values insert <profile-id> <creative-field-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-field-values list <profile-id> <creative-field-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-field-values patch <profile-id> <creative-field-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-field-values update <profile-id> <creative-field-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-fields delete <profile-id> <id> [-p <v>...]
  dfareporting2d1 [options] creative-fields get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-fields insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-fields list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-fields patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-fields update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-groups get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-groups insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-groups list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-groups patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creative-groups update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creatives get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creatives insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creatives list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creatives patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] creatives update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] dimension-values query <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] directory-site-contacts get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] directory-site-contacts list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] directory-sites get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] directory-sites insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] directory-sites list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] event-tags delete <profile-id> <id> [-p <v>...]
  dfareporting2d1 [options] event-tags get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] event-tags insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] event-tags list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] event-tags patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] event-tags update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] files get <report-id> <file-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] files list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-activities delete <profile-id> <id> [-p <v>...]
  dfareporting2d1 [options] floodlight-activities generatetag <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-activities get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-activities insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-activities list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-activities patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-activities update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-activity-groups delete <profile-id> <id> [-p <v>...]
  dfareporting2d1 [options] floodlight-activity-groups get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-activity-groups insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-activity-groups list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-activity-groups patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-activity-groups update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-configurations get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-configurations list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-configurations patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] floodlight-configurations update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] inventory-items get <profile-id> <project-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] inventory-items list <profile-id> <project-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] landing-pages delete <profile-id> <campaign-id> <id> [-p <v>...]
  dfareporting2d1 [options] landing-pages get <profile-id> <campaign-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] landing-pages insert <profile-id> <campaign-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] landing-pages list <profile-id> <campaign-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] landing-pages patch <profile-id> <campaign-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] landing-pages update <profile-id> <campaign-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] metros list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] mobile-carriers get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] mobile-carriers list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] operating-system-versions get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] operating-system-versions list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] operating-systems get <profile-id> <dart-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] operating-systems list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] order-documents get <profile-id> <project-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] order-documents list <profile-id> <project-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] orders get <profile-id> <project-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] orders list <profile-id> <project-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placement-groups get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placement-groups insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placement-groups list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placement-groups patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placement-groups update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placement-strategies delete <profile-id> <id> [-p <v>...]
  dfareporting2d1 [options] placement-strategies get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placement-strategies insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placement-strategies list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placement-strategies patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placement-strategies update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placements generatetags <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placements get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placements insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placements list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placements patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] placements update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] platform-types get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] platform-types list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] postal-codes get <profile-id> <code> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] postal-codes list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] projects get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] projects list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] regions list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] remarketing-list-shares get <profile-id> <remarketing-list-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] remarketing-list-shares patch <profile-id> <remarketing-list-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] remarketing-list-shares update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] remarketing-lists get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] remarketing-lists insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] remarketing-lists list <profile-id> <advertiser-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] remarketing-lists patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] remarketing-lists update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] reports compatible-fields-query <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] reports delete <profile-id> <report-id> [-p <v>...]
  dfareporting2d1 [options] reports files-get <profile-id> <report-id> <file-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] reports files-list <profile-id> <report-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] reports get <profile-id> <report-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] reports insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] reports list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] reports patch <profile-id> <report-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] reports run <profile-id> <report-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] reports update <profile-id> <report-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] sites get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] sites insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] sites list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] sites patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] sites update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] sizes get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] sizes insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] sizes list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] subaccounts get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] subaccounts insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] subaccounts list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] subaccounts patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] subaccounts update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] targetable-remarketing-lists get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] targetable-remarketing-lists list <profile-id> <advertiser-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] user-profiles get <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] user-profiles list [-p <v>...] [-o <out>]
  dfareporting2d1 [options] user-role-permission-groups get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] user-role-permission-groups list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] user-role-permissions get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] user-role-permissions list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] user-roles delete <profile-id> <id> [-p <v>...]
  dfareporting2d1 [options] user-roles get <profile-id> <id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] user-roles insert <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] user-roles list <profile-id> [-p <v>...] [-o <out>]
  dfareporting2d1 [options] user-roles patch <profile-id> <id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 [options] user-roles update <profile-id> -r <kv>... [-p <v>...] [-o <out>]
  dfareporting2d1 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_dfareporting2d1_cli/index.html

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `dfareporting2d1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/dfareporting2d1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/dfareporting2d1-secret.json`, assuming that the required *dfareporting* API 
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

You may consider redirecting standard error into a file for ease of use, e.g. `dfareporting2d1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/