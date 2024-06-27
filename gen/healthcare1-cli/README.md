<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `healthcare1` command-line interface *(CLI)* allows to use most features of the *Google Cloud Healthcare* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Cloud Healthcare* API can be found at the
[official documentation site](https://cloud.google.com/healthcare).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-healthcare1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/healthcare1-cli).

# Usage

This documentation was generated from the *Cloud Healthcare* API at revision *20240605*. The CLI is at version *5.0.5*.

```bash
healthcare1 [options]
        projects
                locations-datasets-consent-stores-attribute-definitions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-attribute-definitions-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-attribute-definitions-get <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-attribute-definitions-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-attribute-definitions-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-check-data-access <consent-store> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consent-artifacts-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consent-artifacts-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consent-artifacts-get <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consent-artifacts-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consents-activate <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consents-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consents-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consents-delete-revision <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consents-get <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consents-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consents-list-revisions <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consents-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consents-reject <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-consents-revoke <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-evaluate-user-consents <consent-store> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-get <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-query-accessible-data <consent-store> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-user-data-mappings-archive <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-user-data-mappings-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-user-data-mappings-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-user-data-mappings-get <name> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-user-data-mappings-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-consent-stores-user-data-mappings-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-data-mapper-workspaces-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-datasets-data-mapper-workspaces-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-data-mapper-workspaces-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-deidentify <source-dataset> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-deidentify <source-store> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-dicom-web-studies-get-study-metrics <study> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-dicom-web-studies-series-get-series-metrics <series> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-export <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-get <name> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-get-dicom-store-metrics <name> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-import <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-search-for-instances <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-search-for-series <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-search-for-studies <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-store-instances <parent> <dicom-web-path> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-delete <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-retrieve-metadata <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-retrieve-study <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-search-for-instances <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-search-for-series <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-series-delete <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-series-instances-delete <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-series-instances-frames-retrieve-frames <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-series-instances-frames-retrieve-rendered <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-series-instances-retrieve-instance <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-series-instances-retrieve-metadata <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-series-instances-retrieve-rendered <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-series-retrieve-metadata <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-series-retrieve-series <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-series-search-for-instances <parent> <dicom-web-path> [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-studies-store-instances <parent> <dicom-web-path> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-dicom-stores-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-deidentify <source-store> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-export <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir--patient-everything <name> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir--resource-purge <name> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir--resource-validate <parent> <type> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-capabilities <name> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-conditional-delete <parent> <type> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-conditional-patch <parent> <type> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-conditional-update <parent> <type> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-create <parent> <type> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-execute-bundle <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-history <name> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-read <name> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-search <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-search-type <parent> <resource-type> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-fhir-vread <name> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-get <name> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-get-fhir-store-metrics <name> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-import <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-rollback <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-fhir-stores-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-get <name> [-p <v>]... [-o <out>]
                locations-datasets-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-export <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-get <name> [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-get-hl7v2-store-metrics <name> [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-get-iam-policy <resource> [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-import <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-messages-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-messages-delete <name> [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-messages-get <name> [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-messages-ingest <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-messages-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-messages-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-hl7-v2-stores-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-list <parent> [-p <v>]... [-o <out>]
                locations-datasets-operations-cancel <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-operations-get <name> [-p <v>]... [-o <out>]
                locations-datasets-operations-list <name> [-p <v>]... [-o <out>]
                locations-datasets-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-datasets-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                locations-get <name> [-p <v>]... [-o <out>]
                locations-list <name> [-p <v>]... [-o <out>]
                locations-services-nlp-analyze-entities <nlp-service> (-r <kv>)... [-p <v>]... [-o <out>]
  healthcare1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `healthcare1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/healthcare1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/healthcare1-secret.json`, assuming that the required *healthcare* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `healthcare1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
