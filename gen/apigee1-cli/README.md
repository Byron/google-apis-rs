<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `apigee1` command-line interface *(CLI)* allows to use most features of the *Google Apigee* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *Apigee* API can be found at the
[official documentation site](https://cloud.google.com/apigee-api-management/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-apigee1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/apigee1-cli).

# Usage

This documentation was generated from the *Apigee* API at revision *20240620*. The CLI is at version *5.0.5*.

```bash
apigee1 [options]
        hybrid
                issuers-list <name> [-p <v>]... [-o <out>]
        organizations
                analytics-datastores-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                analytics-datastores-delete <name> [-p <v>]... [-o <out>]
                analytics-datastores-get <name> [-p <v>]... [-o <out>]
                analytics-datastores-list <parent> [-p <v>]... [-o <out>]
                analytics-datastores-test <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                analytics-datastores-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                apiproducts-attributes <name> (-r <kv>)... [-p <v>]... [-o <out>]
                apiproducts-attributes-delete <name> [-p <v>]... [-o <out>]
                apiproducts-attributes-get <name> [-p <v>]... [-o <out>]
                apiproducts-attributes-list <parent> [-p <v>]... [-o <out>]
                apiproducts-attributes-update-api-product-attribute <name> (-r <kv>)... [-p <v>]... [-o <out>]
                apiproducts-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                apiproducts-delete <name> [-p <v>]... [-o <out>]
                apiproducts-get <name> [-p <v>]... [-o <out>]
                apiproducts-list <parent> [-p <v>]... [-o <out>]
                apiproducts-rateplans-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                apiproducts-rateplans-delete <name> [-p <v>]... [-o <out>]
                apiproducts-rateplans-get <name> [-p <v>]... [-o <out>]
                apiproducts-rateplans-list <parent> [-p <v>]... [-o <out>]
                apiproducts-rateplans-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                apiproducts-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                apis-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                apis-delete <name> [-p <v>]... [-o <out>]
                apis-deployments-list <parent> [-p <v>]... [-o <out>]
                apis-get <name> [-p <v>]... [-o <out>]
                apis-keyvaluemaps-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                apis-keyvaluemaps-delete <name> [-p <v>]... [-o <out>]
                apis-keyvaluemaps-entries-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                apis-keyvaluemaps-entries-delete <name> [-p <v>]... [-o <out>]
                apis-keyvaluemaps-entries-get <name> [-p <v>]... [-o <out>]
                apis-keyvaluemaps-entries-list <parent> [-p <v>]... [-o <out>]
                apis-keyvaluemaps-entries-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                apis-list <parent> [-p <v>]... [-o <out>]
                apis-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                apis-revisions-delete <name> [-p <v>]... [-o <out>]
                apis-revisions-deployments-list <parent> [-p <v>]... [-o <out>]
                apis-revisions-get <name> [-p <v>]... [-o <out>]
                apis-revisions-update-api-proxy-revision <name> (-r <kv>)... [-p <v>]... [-o <out>]
                appgroups-apps-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                appgroups-apps-delete <name> [-p <v>]... [-o <out>]
                appgroups-apps-get <name> [-p <v>]... [-o <out>]
                appgroups-apps-keys-apiproducts-delete <name> [-p <v>]... [-o <out>]
                appgroups-apps-keys-apiproducts-update-app-group-app-key-api-product <name> [-p <v>]... [-o <out>]
                appgroups-apps-keys-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                appgroups-apps-keys-delete <name> [-p <v>]... [-o <out>]
                appgroups-apps-keys-get <name> [-p <v>]... [-o <out>]
                appgroups-apps-keys-update-app-group-app-key <name> (-r <kv>)... [-p <v>]... [-o <out>]
                appgroups-apps-list <parent> [-p <v>]... [-o <out>]
                appgroups-apps-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                appgroups-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                appgroups-delete <name> [-p <v>]... [-o <out>]
                appgroups-get <name> [-p <v>]... [-o <out>]
                appgroups-list <parent> [-p <v>]... [-o <out>]
                appgroups-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                apps-get <name> [-p <v>]... [-o <out>]
                apps-list <parent> [-p <v>]... [-o <out>]
                create (-r <kv>)... [-p <v>]... [-o <out>]
                datacollectors-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                datacollectors-delete <name> [-p <v>]... [-o <out>]
                datacollectors-get <name> [-p <v>]... [-o <out>]
                datacollectors-list <parent> [-p <v>]... [-o <out>]
                datacollectors-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <name> [-p <v>]... [-o <out>]
                deployments-list <parent> [-p <v>]... [-o <out>]
                developers-apps-attributes <name> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-apps-attributes-delete <name> [-p <v>]... [-o <out>]
                developers-apps-attributes-get <name> [-p <v>]... [-o <out>]
                developers-apps-attributes-list <parent> [-p <v>]... [-o <out>]
                developers-apps-attributes-update-developer-app-attribute <name> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-apps-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-apps-delete <name> [-p <v>]... [-o <out>]
                developers-apps-generate-key-pair-or-update-developer-app-status <name> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-apps-get <name> [-p <v>]... [-o <out>]
                developers-apps-keys-apiproducts-delete <name> [-p <v>]... [-o <out>]
                developers-apps-keys-apiproducts-update-developer-app-key-api-product <name> [-p <v>]... [-o <out>]
                developers-apps-keys-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-apps-keys-create-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-apps-keys-delete <name> [-p <v>]... [-o <out>]
                developers-apps-keys-get <name> [-p <v>]... [-o <out>]
                developers-apps-keys-replace-developer-app-key <name> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-apps-keys-update-developer-app-key <name> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-apps-list <parent> [-p <v>]... [-o <out>]
                developers-apps-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-attributes <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-attributes-delete <name> [-p <v>]... [-o <out>]
                developers-attributes-get <name> [-p <v>]... [-o <out>]
                developers-attributes-list <parent> [-p <v>]... [-o <out>]
                developers-attributes-update-developer-attribute <name> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-balance-adjust <name> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-balance-credit <name> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-delete <name> [-p <v>]... [-o <out>]
                developers-get <name> [-p <v>]... [-o <out>]
                developers-get-balance <name> [-p <v>]... [-o <out>]
                developers-get-monetization-config <name> [-p <v>]... [-o <out>]
                developers-list <parent> [-p <v>]... [-o <out>]
                developers-set-developer-status <name> [-p <v>]... [-o <out>]
                developers-subscriptions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-subscriptions-expire <name> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-subscriptions-get <name> [-p <v>]... [-o <out>]
                developers-subscriptions-list <parent> [-p <v>]... [-o <out>]
                developers-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                developers-update-monetization-config <name> (-r <kv>)... [-p <v>]... [-o <out>]
                endpoint-attachments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                endpoint-attachments-delete <name> [-p <v>]... [-o <out>]
                endpoint-attachments-get <name> [-p <v>]... [-o <out>]
                endpoint-attachments-list <parent> [-p <v>]... [-o <out>]
                envgroups-attachments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                envgroups-attachments-delete <name> [-p <v>]... [-o <out>]
                envgroups-attachments-get <name> [-p <v>]... [-o <out>]
                envgroups-attachments-list <parent> [-p <v>]... [-o <out>]
                envgroups-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                envgroups-delete <name> [-p <v>]... [-o <out>]
                envgroups-get <name> [-p <v>]... [-o <out>]
                envgroups-get-deployed-ingress-config <name> [-p <v>]... [-o <out>]
                envgroups-list <parent> [-p <v>]... [-o <out>]
                envgroups-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-addons-config-set-addon-enablement <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-analytics-admin-get-schemav2 <name> [-p <v>]... [-o <out>]
                environments-analytics-exports-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-analytics-exports-get <name> [-p <v>]... [-o <out>]
                environments-analytics-exports-list <parent> [-p <v>]... [-o <out>]
                environments-apis-deployments-list <parent> [-p <v>]... [-o <out>]
                environments-apis-revisions-debugsessions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-apis-revisions-debugsessions-data-get <name> [-p <v>]... [-o <out>]
                environments-apis-revisions-debugsessions-delete-data <name> [-p <v>]... [-o <out>]
                environments-apis-revisions-debugsessions-get <name> [-p <v>]... [-o <out>]
                environments-apis-revisions-debugsessions-list <parent> [-p <v>]... [-o <out>]
                environments-apis-revisions-deploy <name> [-p <v>]... [-o <out>]
                environments-apis-revisions-deployments-generate-deploy-change-report <name> [-p <v>]... [-o <out>]
                environments-apis-revisions-deployments-generate-undeploy-change-report <name> [-p <v>]... [-o <out>]
                environments-apis-revisions-get-deployments <name> [-p <v>]... [-o <out>]
                environments-apis-revisions-undeploy <name> [-p <v>]... [-o <out>]
                environments-archive-deployments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-archive-deployments-delete <name> [-p <v>]... [-o <out>]
                environments-archive-deployments-generate-download-url <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-archive-deployments-generate-upload-url <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-archive-deployments-get <name> [-p <v>]... [-o <out>]
                environments-archive-deployments-list <parent> [-p <v>]... [-o <out>]
                environments-archive-deployments-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-caches-delete <name> [-p <v>]... [-o <out>]
                environments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-delete <name> [-p <v>]... [-o <out>]
                environments-deployments-list <parent> [-p <v>]... [-o <out>]
                environments-flowhooks-attach-shared-flow-to-flow-hook <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-flowhooks-detach-shared-flow-from-flow-hook <name> [-p <v>]... [-o <out>]
                environments-flowhooks-get <name> [-p <v>]... [-o <out>]
                environments-get <name> [-p <v>]... [-o <out>]
                environments-get-addons-config <name> [-p <v>]... [-o <out>]
                environments-get-api-security-runtime-config <name> [-p <v>]... [-o <out>]
                environments-get-debugmask <name> [-p <v>]... [-o <out>]
                environments-get-deployed-config <name> [-p <v>]... [-o <out>]
                environments-get-iam-policy <resource> [-p <v>]... [-o <out>]
                environments-get-security-actions-config <name> [-p <v>]... [-o <out>]
                environments-get-trace-config <name> [-p <v>]... [-o <out>]
                environments-keystores-aliases-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-keystores-aliases-csr <name> [-p <v>]... [-o <out>]
                environments-keystores-aliases-delete <name> [-p <v>]... [-o <out>]
                environments-keystores-aliases-get <name> [-p <v>]... [-o <out>]
                environments-keystores-aliases-get-certificate <name> [-p <v>]... [-o <out>]
                environments-keystores-aliases-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-keystores-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-keystores-delete <name> [-p <v>]... [-o <out>]
                environments-keystores-get <name> [-p <v>]... [-o <out>]
                environments-keyvaluemaps-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-keyvaluemaps-delete <name> [-p <v>]... [-o <out>]
                environments-keyvaluemaps-entries-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-keyvaluemaps-entries-delete <name> [-p <v>]... [-o <out>]
                environments-keyvaluemaps-entries-get <name> [-p <v>]... [-o <out>]
                environments-keyvaluemaps-entries-list <parent> [-p <v>]... [-o <out>]
                environments-keyvaluemaps-entries-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-modify-environment <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-optimized-stats-get <name> [-p <v>]... [-o <out>]
                environments-queries-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-queries-get <name> [-p <v>]... [-o <out>]
                environments-queries-get-result <name> [-p <v>]... [-o <out>]
                environments-queries-get-resulturl <name> [-p <v>]... [-o <out>]
                environments-queries-list <parent> [-p <v>]... [-o <out>]
                environments-references-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-references-delete <name> [-p <v>]... [-o <out>]
                environments-references-get <name> [-p <v>]... [-o <out>]
                environments-references-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-resourcefiles-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-resourcefiles-delete <parent> <type> <name> [-p <v>]... [-o <out>]
                environments-resourcefiles-get <parent> <type> <name> [-p <v>]... [-o <out>]
                environments-resourcefiles-list <parent> [-p <v>]... [-o <out>]
                environments-resourcefiles-list-environment-resources <parent> <type> [-p <v>]... [-o <out>]
                environments-resourcefiles-update <parent> <type> <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-security-actions-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-security-actions-disable <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-security-actions-enable <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-security-actions-get <name> [-p <v>]... [-o <out>]
                environments-security-actions-list <parent> [-p <v>]... [-o <out>]
                environments-security-incidents-batch-update <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-security-incidents-get <name> [-p <v>]... [-o <out>]
                environments-security-incidents-list <parent> [-p <v>]... [-o <out>]
                environments-security-incidents-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-security-reports-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-security-reports-get <name> [-p <v>]... [-o <out>]
                environments-security-reports-get-result <name> [-p <v>]... [-o <out>]
                environments-security-reports-get-result-view <name> [-p <v>]... [-o <out>]
                environments-security-reports-list <parent> [-p <v>]... [-o <out>]
                environments-security-stats-query-tabular-stats <orgenv> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-security-stats-query-time-series-stats <orgenv> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-set-iam-policy <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-sharedflows-deployments-list <parent> [-p <v>]... [-o <out>]
                environments-sharedflows-revisions-deploy <name> [-p <v>]... [-o <out>]
                environments-sharedflows-revisions-get-deployments <name> [-p <v>]... [-o <out>]
                environments-sharedflows-revisions-undeploy <name> [-p <v>]... [-o <out>]
                environments-stats-get <name> [-p <v>]... [-o <out>]
                environments-subscribe <parent> [-p <v>]... [-o <out>]
                environments-targetservers-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-targetservers-delete <name> [-p <v>]... [-o <out>]
                environments-targetservers-get <name> [-p <v>]... [-o <out>]
                environments-targetservers-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-test-iam-permissions <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-trace-config-overrides-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-trace-config-overrides-delete <name> [-p <v>]... [-o <out>]
                environments-trace-config-overrides-get <name> [-p <v>]... [-o <out>]
                environments-trace-config-overrides-list <parent> [-p <v>]... [-o <out>]
                environments-trace-config-overrides-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-unsubscribe <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-update-debugmask <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-update-environment <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-update-security-actions-config <name> (-r <kv>)... [-p <v>]... [-o <out>]
                environments-update-trace-config <name> (-r <kv>)... [-p <v>]... [-o <out>]
                get <name> [-p <v>]... [-o <out>]
                get-deployed-ingress-config <name> [-p <v>]... [-o <out>]
                get-project-mapping <name> [-p <v>]... [-o <out>]
                get-runtime-config <name> [-p <v>]... [-o <out>]
                get-security-settings <name> [-p <v>]... [-o <out>]
                get-sync-authorization <name> (-r <kv>)... [-p <v>]... [-o <out>]
                host-queries-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                host-queries-get <name> [-p <v>]... [-o <out>]
                host-queries-get-result <name> [-p <v>]... [-o <out>]
                host-queries-get-result-view <name> [-p <v>]... [-o <out>]
                host-queries-list <parent> [-p <v>]... [-o <out>]
                host-security-reports-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                host-security-reports-get <name> [-p <v>]... [-o <out>]
                host-security-reports-get-result <name> [-p <v>]... [-o <out>]
                host-security-reports-get-result-view <name> [-p <v>]... [-o <out>]
                host-security-reports-list <parent> [-p <v>]... [-o <out>]
                host-stats-get <name> [-p <v>]... [-o <out>]
                instances-attachments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-attachments-delete <name> [-p <v>]... [-o <out>]
                instances-attachments-get <name> [-p <v>]... [-o <out>]
                instances-attachments-list <parent> [-p <v>]... [-o <out>]
                instances-canaryevaluations-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-canaryevaluations-get <name> [-p <v>]... [-o <out>]
                instances-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-delete <name> [-p <v>]... [-o <out>]
                instances-get <name> [-p <v>]... [-o <out>]
                instances-list <parent> [-p <v>]... [-o <out>]
                instances-nat-addresses-activate <name> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-nat-addresses-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-nat-addresses-delete <name> [-p <v>]... [-o <out>]
                instances-nat-addresses-get <name> [-p <v>]... [-o <out>]
                instances-nat-addresses-list <parent> [-p <v>]... [-o <out>]
                instances-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                instances-report-status <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                keyvaluemaps-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                keyvaluemaps-delete <name> [-p <v>]... [-o <out>]
                keyvaluemaps-entries-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                keyvaluemaps-entries-delete <name> [-p <v>]... [-o <out>]
                keyvaluemaps-entries-get <name> [-p <v>]... [-o <out>]
                keyvaluemaps-entries-list <parent> [-p <v>]... [-o <out>]
                keyvaluemaps-entries-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                list <parent> [-p <v>]... [-o <out>]
                operations-get <name> [-p <v>]... [-o <out>]
                operations-list <name> [-p <v>]... [-o <out>]
                optimized-host-stats-get <name> [-p <v>]... [-o <out>]
                reports-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                reports-delete <name> [-p <v>]... [-o <out>]
                reports-get <name> [-p <v>]... [-o <out>]
                reports-list <parent> [-p <v>]... [-o <out>]
                reports-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                security-assessment-results-batch-compute <name> (-r <kv>)... [-p <v>]... [-o <out>]
                security-profiles-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                security-profiles-delete <name> [-p <v>]... [-o <out>]
                security-profiles-environments-compute-environment-scores <profile-environment> (-r <kv>)... [-p <v>]... [-o <out>]
                security-profiles-environments-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                security-profiles-environments-delete <name> [-p <v>]... [-o <out>]
                security-profiles-get <name> [-p <v>]... [-o <out>]
                security-profiles-list <parent> [-p <v>]... [-o <out>]
                security-profiles-list-revisions <name> [-p <v>]... [-o <out>]
                security-profiles-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                set-addons <org> (-r <kv>)... [-p <v>]... [-o <out>]
                set-sync-authorization <name> (-r <kv>)... [-p <v>]... [-o <out>]
                sharedflows-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                sharedflows-delete <name> [-p <v>]... [-o <out>]
                sharedflows-deployments-list <parent> [-p <v>]... [-o <out>]
                sharedflows-get <name> [-p <v>]... [-o <out>]
                sharedflows-list <parent> [-p <v>]... [-o <out>]
                sharedflows-revisions-delete <name> [-p <v>]... [-o <out>]
                sharedflows-revisions-deployments-list <parent> [-p <v>]... [-o <out>]
                sharedflows-revisions-get <name> [-p <v>]... [-o <out>]
                sharedflows-revisions-update-shared-flow-revision <name> (-r <kv>)... [-p <v>]... [-o <out>]
                sites-apicategories-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                sites-apicategories-delete <name> [-p <v>]... [-o <out>]
                sites-apicategories-get <name> [-p <v>]... [-o <out>]
                sites-apicategories-list <parent> [-p <v>]... [-o <out>]
                sites-apicategories-patch <name> (-r <kv>)... [-p <v>]... [-o <out>]
                sites-apidocs-create <parent> (-r <kv>)... [-p <v>]... [-o <out>]
                sites-apidocs-delete <name> [-p <v>]... [-o <out>]
                sites-apidocs-get <name> [-p <v>]... [-o <out>]
                sites-apidocs-get-documentation <name> [-p <v>]... [-o <out>]
                sites-apidocs-list <parent> [-p <v>]... [-o <out>]
                sites-apidocs-update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                sites-apidocs-update-documentation <name> (-r <kv>)... [-p <v>]... [-o <out>]
                update <name> (-r <kv>)... [-p <v>]... [-o <out>]
                update-security-settings <name> (-r <kv>)... [-p <v>]... [-o <out>]
        projects
                provision-organization <project> (-r <kv>)... [-p <v>]... [-o <out>]
  apigee1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `apigee1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/apigee1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/apigee1-secret.json`, assuming that the required *apigee* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `apigee1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
