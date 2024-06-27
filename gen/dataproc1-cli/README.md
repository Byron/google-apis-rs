<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `dataproc1` command-line interface *(CLI)* allows to use most features of the *Google Dataproc* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Dataproc* API can be found at the
[official documentation site](https://cloud.google.com/dataproc/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-dataproc1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/dataproc1-cli).

# Usage

This documentation was generated from the *Dataproc* API at revision *20240617*. The CLI is at version *5.0.5*.

```bash
dataproc1 [options]
        projects
                locations-autoscaling-policies-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-autoscaling-policies-delete <name> [-p <v>]... [-o <out>]
                locations-autoscaling-policies-get <name> [-p <v>]... [-o <out>]
                locations-autoscaling-policies-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-autoscaling-policies-list <parent> [-p <v>]... [-o <out>]
                locations-autoscaling-policies-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-autoscaling-policies-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-autoscaling-policies-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-batches-analyze <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-batches-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-batches-delete <name> [-p <v>]... [-o <out>]
                locations-batches-get <name> [-p <v>]... [-o <out>]
                locations-batches-list <parent> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> [-p <v>]... [-o <out>]
                locations-operations-delete <name> [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <name> [-p <v>]... [-o <out>]
                locations-session-templates-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-session-templates-delete <name> [-p <v>]... [-o <out>]
                locations-session-templates-get <name> [-p <v>]... [-o <out>]
                locations-session-templates-list <parent> [-p <v>]... [-o <out>]
                locations-session-templates-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-sessions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-sessions-delete <name> [-p <v>]... [-o <out>]
                locations-sessions-get <name> [-p <v>]... [-o <out>]
                locations-sessions-list <parent> [-p <v>]... [-o <out>]
                locations-sessions-terminate <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-workflow-templates-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-workflow-templates-delete <name> [-p <v>]... [-o <out>]
                locations-workflow-templates-get <name> [-p <v>]... [-o <out>]
                locations-workflow-templates-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-workflow-templates-instantiate <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-workflow-templates-instantiate-inline <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-workflow-templates-list <parent> [-p <v>]... [-o <out>]
                locations-workflow-templates-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-workflow-templates-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-workflow-templates-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-autoscaling-policies-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-autoscaling-policies-delete <name> [-p <v>]... [-o <out>]
                regions-autoscaling-policies-get <name> [-p <v>]... [-o <out>]
                regions-autoscaling-policies-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-autoscaling-policies-list <parent> [-p <v>]... [-o <out>]
                regions-autoscaling-policies-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-autoscaling-policies-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-autoscaling-policies-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-create <project-id> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-delete <project-id> <region> <cluster-name> [-p <v>]... [-o <out>]
                regions-clusters-diagnose <project-id> <region> <cluster-name> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-get <project-id> <region> <cluster-name> [-p <v>]... [-o <out>]
                regions-clusters-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-inject-credentials <project> <region> <cluster> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-list <project-id> <region> [-p <v>]... [-o <out>]
                regions-clusters-node-groups-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-node-groups-get <name> [-p <v>]... [-o <out>]
                regions-clusters-node-groups-repair <name> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-node-groups-resize <name> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-patch <project-id> <region> <cluster-name> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-repair <project-id> <region> <cluster-name> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-start <project-id> <region> <cluster-name> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-stop <project-id> <region> <cluster-name> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-clusters-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-jobs-cancel <project-id> <region> <job-id> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-jobs-delete <project-id> <region> <job-id> [-p <v>]... [-o <out>]
                regions-jobs-get <project-id> <region> <job-id> [-p <v>]... [-o <out>]
                regions-jobs-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-jobs-list <project-id> <region> [-p <v>]... [-o <out>]
                regions-jobs-patch <project-id> <region> <job-id> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-jobs-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-jobs-submit <project-id> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-jobs-submit-as-operation <project-id> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-jobs-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-operations-cancel <name> [-p <v>]... [-o <out>]
                regions-operations-delete <name> [-p <v>]... [-o <out>]
                regions-operations-get <name> [-p <v>]... [-o <out>]
                regions-operations-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-operations-list <name> [-p <v>]... [-o <out>]
                regions-operations-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-operations-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-workflow-templates-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-workflow-templates-delete <name> [-p <v>]... [-o <out>]
                regions-workflow-templates-get <name> [-p <v>]... [-o <out>]
                regions-workflow-templates-get-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-workflow-templates-instantiate <name> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-workflow-templates-instantiate-inline <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-workflow-templates-list <parent> [-p <v>]... [-o <out>]
                regions-workflow-templates-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-workflow-templates-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                regions-workflow-templates-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
  dataproc1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `dataproc1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/dataproc1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/dataproc1-secret.json`, assuming that the required *dataproc* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `dataproc1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
