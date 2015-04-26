<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/cli/README.md.mako'
DO NOT EDIT !
-->
The `fusiontables2` command-line interface *(CLI)* allows to use most features of the *Google fusiontables* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

# Usage

This documentation was generated from the *fusiontables* API at revision *20150326*. The CLI is at version *0.1.0*.

```bash
  fusiontables2 [options] column delete <table-id> <column-id> [-p <v>...]
  fusiontables2 [options] column get <table-id> <column-id> [-p <v>...] [-o <out>]
  fusiontables2 [options] column insert <table-id> -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 [options] column list <table-id> [-p <v>...] [-o <out>]
  fusiontables2 [options] column patch <table-id> <column-id> -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 [options] column update <table-id> <column-id> -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 [options] query sql <sql> [-p <v>...] [-o <out>]
  fusiontables2 [options] query sql-get <sql> [-p <v>...] [-o <out>]
  fusiontables2 [options] style delete <table-id> <style-id> [-p <v>...]
  fusiontables2 [options] style get <table-id> <style-id> [-p <v>...] [-o <out>]
  fusiontables2 [options] style insert <table-id> -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 [options] style list <table-id> [-p <v>...] [-o <out>]
  fusiontables2 [options] style patch <table-id> <style-id> -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 [options] style update <table-id> <style-id> -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 [options] table copy <table-id> [-p <v>...] [-o <out>]
  fusiontables2 [options] table delete <table-id> [-p <v>...]
  fusiontables2 [options] table get <table-id> [-p <v>...] [-o <out>]
  fusiontables2 [options] table import-rows <table-id> -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  fusiontables2 [options] table import-table <name> -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  fusiontables2 [options] table insert -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 [options] table list [-p <v>...] [-o <out>]
  fusiontables2 [options] table patch <table-id> -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 [options] table replace-rows <table-id> -u (simple|resumable) <file> <mime> [-p <v>...] [-o <out>]
  fusiontables2 [options] table update <table-id> -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 [options] task delete <table-id> <task-id> [-p <v>...]
  fusiontables2 [options] task get <table-id> <task-id> [-p <v>...] [-o <out>]
  fusiontables2 [options] task list <table-id> [-p <v>...] [-o <out>]
  fusiontables2 [options] template delete <table-id> <template-id> [-p <v>...]
  fusiontables2 [options] template get <table-id> <template-id> [-p <v>...] [-o <out>]
  fusiontables2 [options] template insert <table-id> -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 [options] template list <table-id> [-p <v>...] [-o <out>]
  fusiontables2 [options] template patch <table-id> <template-id> -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 [options] template update <table-id> <template-id> -r <kv>... [-p <v>...] [-o <out>]
  fusiontables2 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_fusiontables2_cli/index.html

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `fusiontables2-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/fusiontables2-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/fusiontables2-secret.json`, assuming that the required *fusiontables* API 
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

You may consider redirecting standard error into a file for ease of use, e.g. `fusiontables2 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/