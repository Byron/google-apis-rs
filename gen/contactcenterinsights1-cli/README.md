<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `contactcenterinsights1` command-line interface *(CLI)* allows to use most features of the *Google Contactcenterinsights* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Contactcenterinsights* API can be found at the
[official documentation site](https://cloud.google.com/contact-center/insights/docs).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-contactcenterinsights1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/contactcenterinsights1-cli).

# Usage

This documentation was generated from the *Contactcenterinsights* API at revision *20251222*. The CLI is at version *7.0.0*.

```bash
contactcenterinsights1 [options]
        projects
                locations-analysis-rules-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-analysis-rules-delete <name> [-p <v>]... [-o <out>]
                locations-analysis-rules-get <name> [-p <v>]... [-o <out>]
                locations-analysis-rules-list <parent> [-p <v>]... [-o <out>]
                locations-analysis-rules-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-assessment-rules-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-assessment-rules-delete <name> [-p <v>]... [-o <out>]
                locations-assessment-rules-get <name> [-p <v>]... [-o <out>]
                locations-assessment-rules-list <parent> [-p <v>]... [-o <out>]
                locations-assessment-rules-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-assessments-appeal <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-assessments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-assessments-delete <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-assessments-finalize <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-assessments-get <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-assessments-list <parent> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-assessments-notes-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-assessments-notes-delete <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-assessments-notes-list <parent> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-assessments-notes-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-assessments-publish <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-calculate-stats <location> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-delete <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-feedback-labels-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-feedback-labels-delete <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-feedback-labels-get <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-feedback-labels-list <parent> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-feedback-labels-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-generate-signed-audio <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-get <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-conversations-list <parent> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-delete <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-get <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-list <parent> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-operations-cancel <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-operations-get <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-operations-list <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-query-metrics <location> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-query-performance-overview <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-search <parent> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-authorized-views-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-authorized-view-sets-delete <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-get <name> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-list <parent> [-p <v>]... [-o <out>]
                locations-authorized-view-sets-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-bulk-delete-feedback-labels <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-bulk-download-feedback-labels <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-bulk-upload-feedback-labels <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-analyses-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-analyses-delete <name> [-p <v>]... [-o <out>]
                locations-conversations-analyses-get <name> [-p <v>]... [-o <out>]
                locations-conversations-analyses-list <parent> [-p <v>]... [-o <out>]
                locations-conversations-assessments-appeal <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-assessments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-assessments-delete <name> [-p <v>]... [-o <out>]
                locations-conversations-assessments-finalize <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-assessments-get <name> [-p <v>]... [-o <out>]
                locations-conversations-assessments-list <parent> [-p <v>]... [-o <out>]
                locations-conversations-assessments-notes-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-assessments-notes-delete <name> [-p <v>]... [-o <out>]
                locations-conversations-assessments-notes-list <parent> [-p <v>]... [-o <out>]
                locations-conversations-assessments-notes-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-assessments-publish <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-bulk-analyze <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-bulk-delete <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-calculate-stats <location> [-p <v>]... [-o <out>]
                locations-conversations-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-delete <name> [-p <v>]... [-o <out>]
                locations-conversations-feedback-labels-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-feedback-labels-delete <name> [-p <v>]... [-o <out>]
                locations-conversations-feedback-labels-get <name> [-p <v>]... [-o <out>]
                locations-conversations-feedback-labels-list <parent> [-p <v>]... [-o <out>]
                locations-conversations-feedback-labels-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-generate-signed-audio <name> [-p <v>]... [-o <out>]
                locations-conversations-get <name> [-p <v>]... [-o <out>]
                locations-conversations-ingest <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-list <parent> [-p <v>]... [-o <out>]
                locations-conversations-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-sample <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-segments-bulk-analyze <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-conversations-upload <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-bulk-delete-feedback-labels <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-bulk-download-feedback-labels <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-bulk-upload-feedback-labels <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-conversations-bulk-delete <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-conversations-calculate-stats <location> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-conversations-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-conversations-feedback-labels-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-conversations-feedback-labels-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-conversations-feedback-labels-get <name> [-p <v>]... [-o <out>]
                locations-datasets-conversations-feedback-labels-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-conversations-feedback-labels-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-conversations-generate-signed-audio <name> [-p <v>]... [-o <out>]
                locations-datasets-conversations-get <name> [-p <v>]... [-o <out>]
                locations-datasets-conversations-ingest <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-conversations-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-conversations-sample <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-get <name> [-p <v>]... [-o <out>]
                locations-datasets-insightsdata-export <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-list-all-feedback-labels <parent> [-p <v>]... [-o <out>]
                locations-datasets-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-encryption-spec-initialize <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-get-encryption-spec <name> [-p <v>]... [-o <out>]
                locations-get-settings <name> [-p <v>]... [-o <out>]
                locations-insightsdata-export <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-issue-models-calculate-issue-model-stats <issue-model> [-p <v>]... [-o <out>]
                locations-issue-models-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-issue-models-delete <name> [-p <v>]... [-o <out>]
                locations-issue-models-deploy <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-issue-models-export <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-issue-models-get <name> [-p <v>]... [-o <out>]
                locations-issue-models-import <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-issue-models-issues-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-issue-models-issues-delete <name> [-p <v>]... [-o <out>]
                locations-issue-models-issues-get <name> [-p <v>]... [-o <out>]
                locations-issue-models-issues-list <parent> [-p <v>]... [-o <out>]
                locations-issue-models-issues-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-issue-models-list <parent> [-p <v>]... [-o <out>]
                locations-issue-models-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-issue-models-undeploy <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-list-all-feedback-labels <parent> [-p <v>]... [-o <out>]
                locations-operations-cancel <name> [-p <v>]... [-o <out>]
                locations-operations-get <name> [-p <v>]... [-o <out>]
                locations-operations-list <name> [-p <v>]... [-o <out>]
                locations-phrase-matchers-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-phrase-matchers-delete <name> [-p <v>]... [-o <out>]
                locations-phrase-matchers-get <name> [-p <v>]... [-o <out>]
                locations-phrase-matchers-list <parent> [-p <v>]... [-o <out>]
                locations-phrase-matchers-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-qa-question-tags-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-qa-question-tags-delete <name> [-p <v>]... [-o <out>]
                locations-qa-question-tags-get <name> [-p <v>]... [-o <out>]
                locations-qa-question-tags-list <parent> [-p <v>]... [-o <out>]
                locations-qa-question-tags-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-qa-scorecards-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-qa-scorecards-delete <name> [-p <v>]... [-o <out>]
                locations-qa-scorecards-get <name> [-p <v>]... [-o <out>]
                locations-qa-scorecards-list <parent> [-p <v>]... [-o <out>]
                locations-qa-scorecards-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-delete <name> [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-deploy <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-get <name> [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-list <parent> [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-qa-questions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-qa-questions-delete <name> [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-qa-questions-get <name> [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-qa-questions-list <parent> [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-qa-questions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-tune-qa-scorecard-revision <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-qa-scorecards-revisions-undeploy <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-query-metrics <location> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-query-performance-overview <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-update-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-views-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-views-delete <name> [-p <v>]... [-o <out>]
                locations-views-get <name> [-p <v>]... [-o <out>]
                locations-views-list <parent> [-p <v>]... [-o <out>]
                locations-views-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
  contactcenterinsights1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `contactcenterinsights1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/contactcenterinsights1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/contactcenterinsights1-secret.json`, assuming that the required *contactcenterinsights* API
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `contactcenterinsights1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
