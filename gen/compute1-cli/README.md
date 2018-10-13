<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/cli/README.md.mako'
DO NOT EDIT !
-->
The `compute1` command-line interface *(CLI)* allows to use most features of the *Google compute* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *compute* API can be found at the
[official documentation site](https://developers.google.com/compute/docs/reference/latest/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-compute1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/compute1-cli).

# Usage

This documentation was generated from the *compute* API at revision *20180916*. The CLI is at version *1.0.7*.

```bash
compute1 [options]
        accelerator-types
                aggregated-list <project> [-p <v>]... [-o <out>]
                get <project> <zone> <accelerator-type> [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
        addresses
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <region> <address> [-p <v>]... [-o <out>]
                get <project> <region> <address> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
        autoscalers
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <zone> <autoscaler> [-p <v>]... [-o <out>]
                get <project> <zone> <autoscaler> [-p <v>]... [-o <out>]
                insert <project> <zone> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
                patch <project> <zone> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <zone> (-r <kv>)... [-p <v>]... [-o <out>]
        backend-buckets
                add-signed-url-key <project> <backend-bucket> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <project> <backend-bucket> [-p <v>]... [-o <out>]
                delete-signed-url-key <project> <backend-bucket> <key-name> [-p <v>]... [-o <out>]
                get <project> <backend-bucket> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                patch <project> <backend-bucket> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <backend-bucket> (-r <kv>)... [-p <v>]... [-o <out>]
        backend-services
                add-signed-url-key <project> <backend-service> (-r <kv>)... [-p <v>]... [-o <out>]
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <backend-service> [-p <v>]... [-o <out>]
                delete-signed-url-key <project> <backend-service> <key-name> [-p <v>]... [-o <out>]
                get <project> <backend-service> [-p <v>]... [-o <out>]
                get-health <project> <backend-service> (-r <kv>)... [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                patch <project> <backend-service> (-r <kv>)... [-p <v>]... [-o <out>]
                set-security-policy <project> <backend-service> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <backend-service> (-r <kv>)... [-p <v>]... [-o <out>]
        disk-types
                aggregated-list <project> [-p <v>]... [-o <out>]
                get <project> <zone> <disk-type> [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
        disks
                aggregated-list <project> [-p <v>]... [-o <out>]
                create-snapshot <project> <zone> <disk> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <project> <zone> <disk> [-p <v>]... [-o <out>]
                get <project> <zone> <disk> [-p <v>]... [-o <out>]
                insert <project> <zone> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
                resize <project> <zone> <disk> (-r <kv>)... [-p <v>]... [-o <out>]
                set-labels <project> <zone> <resource> (-r <kv>)... [-p <v>]... [-o <out>]
        firewalls
                delete <project> <firewall> [-p <v>]... [-o <out>]
                get <project> <firewall> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                patch <project> <firewall> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <firewall> (-r <kv>)... [-p <v>]... [-o <out>]
        forwarding-rules
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <region> <forwarding-rule> [-p <v>]... [-o <out>]
                get <project> <region> <forwarding-rule> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
                set-target <project> <region> <forwarding-rule> (-r <kv>)... [-p <v>]... [-o <out>]
        global-addresses
                delete <project> <address> [-p <v>]... [-o <out>]
                get <project> <address> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
        global-forwarding-rules
                delete <project> <forwarding-rule> [-p <v>]... [-o <out>]
                get <project> <forwarding-rule> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                set-target <project> <forwarding-rule> (-r <kv>)... [-p <v>]... [-o <out>]
        global-operations
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <operation> [-p <v>]...
                get <project> <operation> [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
        health-checks
                delete <project> <health-check> [-p <v>]... [-o <out>]
                get <project> <health-check> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                patch <project> <health-check> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <health-check> (-r <kv>)... [-p <v>]... [-o <out>]
        http-health-checks
                delete <project> <http-health-check> [-p <v>]... [-o <out>]
                get <project> <http-health-check> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                patch <project> <http-health-check> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <http-health-check> (-r <kv>)... [-p <v>]... [-o <out>]
        https-health-checks
                delete <project> <https-health-check> [-p <v>]... [-o <out>]
                get <project> <https-health-check> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                patch <project> <https-health-check> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <https-health-check> (-r <kv>)... [-p <v>]... [-o <out>]
        images
                delete <project> <image> [-p <v>]... [-o <out>]
                deprecate <project> <image> (-r <kv>)... [-p <v>]... [-o <out>]
                get <project> <image> [-p <v>]... [-o <out>]
                get-from-family <project> <family> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                set-labels <project> <resource> (-r <kv>)... [-p <v>]... [-o <out>]
        instance-group-managers
                abandon-instances <project> <zone> <instance-group-manager> (-r <kv>)... [-p <v>]... [-o <out>]
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <zone> <instance-group-manager> [-p <v>]... [-o <out>]
                delete-instances <project> <zone> <instance-group-manager> (-r <kv>)... [-p <v>]... [-o <out>]
                get <project> <zone> <instance-group-manager> [-p <v>]... [-o <out>]
                insert <project> <zone> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
                list-managed-instances <project> <zone> <instance-group-manager> [-p <v>]... [-o <out>]
                recreate-instances <project> <zone> <instance-group-manager> (-r <kv>)... [-p <v>]... [-o <out>]
                resize <project> <zone> <instance-group-manager> <size> [-p <v>]... [-o <out>]
                set-instance-template <project> <zone> <instance-group-manager> (-r <kv>)... [-p <v>]... [-o <out>]
                set-target-pools <project> <zone> <instance-group-manager> (-r <kv>)... [-p <v>]... [-o <out>]
        instance-groups
                add-instances <project> <zone> <instance-group> (-r <kv>)... [-p <v>]... [-o <out>]
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <zone> <instance-group> [-p <v>]... [-o <out>]
                get <project> <zone> <instance-group> [-p <v>]... [-o <out>]
                insert <project> <zone> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
                list-instances <project> <zone> <instance-group> (-r <kv>)... [-p <v>]... [-o <out>]
                remove-instances <project> <zone> <instance-group> (-r <kv>)... [-p <v>]... [-o <out>]
                set-named-ports <project> <zone> <instance-group> (-r <kv>)... [-p <v>]... [-o <out>]
        instance-templates
                delete <project> <instance-template> [-p <v>]... [-o <out>]
                get <project> <instance-template> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
        instances
                add-access-config <project> <zone> <instance> <network-interface> (-r <kv>)... [-p <v>]... [-o <out>]
                aggregated-list <project> [-p <v>]... [-o <out>]
                attach-disk <project> <zone> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <project> <zone> <instance> [-p <v>]... [-o <out>]
                delete-access-config <project> <zone> <instance> <access-config> <network-interface> [-p <v>]... [-o <out>]
                detach-disk <project> <zone> <instance> <device-name> [-p <v>]... [-o <out>]
                get <project> <zone> <instance> [-p <v>]... [-o <out>]
                get-serial-port-output <project> <zone> <instance> [-p <v>]... [-o <out>]
                insert <project> <zone> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
                list-referrers <project> <zone> <instance> [-p <v>]... [-o <out>]
                reset <project> <zone> <instance> [-p <v>]... [-o <out>]
                set-deletion-protection <project> <zone> <resource> [-p <v>]... [-o <out>]
                set-disk-auto-delete <project> <zone> <instance> <auto-delete> <device-name> [-p <v>]... [-o <out>]
                set-labels <project> <zone> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                set-machine-resources <project> <zone> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                set-machine-type <project> <zone> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                set-metadata <project> <zone> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                set-min-cpu-platform <project> <zone> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                set-scheduling <project> <zone> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                set-service-account <project> <zone> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                set-tags <project> <zone> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                simulate-maintenance-event <project> <zone> <instance> [-p <v>]... [-o <out>]
                start <project> <zone> <instance> [-p <v>]... [-o <out>]
                start-with-encryption-key <project> <zone> <instance> (-r <kv>)... [-p <v>]... [-o <out>]
                stop <project> <zone> <instance> [-p <v>]... [-o <out>]
                update-access-config <project> <zone> <instance> <network-interface> (-r <kv>)... [-p <v>]... [-o <out>]
                update-network-interface <project> <zone> <instance> <network-interface> (-r <kv>)... [-p <v>]... [-o <out>]
        interconnect-attachments
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <region> <interconnect-attachment> [-p <v>]... [-o <out>]
                get <project> <region> <interconnect-attachment> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
                patch <project> <region> <interconnect-attachment> (-r <kv>)... [-p <v>]... [-o <out>]
        interconnect-locations
                get <project> <interconnect-location> [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
        interconnects
                delete <project> <interconnect> [-p <v>]... [-o <out>]
                get <project> <interconnect> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                patch <project> <interconnect> (-r <kv>)... [-p <v>]... [-o <out>]
        license-codes
                get <project> <license-code> [-p <v>]... [-o <out>]
                test-iam-permissions <project> <resource> (-r <kv>)... [-p <v>]... [-o <out>]
        licenses
                delete <project> <license> [-p <v>]... [-o <out>]
                get <project> <license> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                test-iam-permissions <project> <resource> (-r <kv>)... [-p <v>]... [-o <out>]
        machine-types
                aggregated-list <project> [-p <v>]... [-o <out>]
                get <project> <zone> <machine-type> [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
        networks
                add-peering <project> <network> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <project> <network> [-p <v>]... [-o <out>]
                get <project> <network> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                patch <project> <network> (-r <kv>)... [-p <v>]... [-o <out>]
                remove-peering <project> <network> (-r <kv>)... [-p <v>]... [-o <out>]
                switch-to-custom-mode <project> <network> [-p <v>]... [-o <out>]
        node-groups
                add-nodes <project> <zone> <node-group> (-r <kv>)... [-p <v>]... [-o <out>]
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <zone> <node-group> [-p <v>]... [-o <out>]
                delete-nodes <project> <zone> <node-group> (-r <kv>)... [-p <v>]... [-o <out>]
                get <project> <zone> <node-group> [-p <v>]... [-o <out>]
                insert <project> <zone> <initial-node-count> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
                list-nodes <project> <zone> <node-group> [-p <v>]... [-o <out>]
                set-node-template <project> <zone> <node-group> (-r <kv>)... [-p <v>]... [-o <out>]
        node-templates
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <region> <node-template> [-p <v>]... [-o <out>]
                get <project> <region> <node-template> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
        node-types
                aggregated-list <project> [-p <v>]... [-o <out>]
                get <project> <zone> <node-type> [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
        projects
                disable-xpn-host <project> [-p <v>]... [-o <out>]
                disable-xpn-resource <project> (-r <kv>)... [-p <v>]... [-o <out>]
                enable-xpn-host <project> [-p <v>]... [-o <out>]
                enable-xpn-resource <project> (-r <kv>)... [-p <v>]... [-o <out>]
                get <project> [-p <v>]... [-o <out>]
                get-xpn-host <project> [-p <v>]... [-o <out>]
                get-xpn-resources <project> [-p <v>]... [-o <out>]
                list-xpn-hosts <project> (-r <kv>)... [-p <v>]... [-o <out>]
                move-disk <project> (-r <kv>)... [-p <v>]... [-o <out>]
                move-instance <project> (-r <kv>)... [-p <v>]... [-o <out>]
                set-common-instance-metadata <project> (-r <kv>)... [-p <v>]... [-o <out>]
                set-default-network-tier <project> (-r <kv>)... [-p <v>]... [-o <out>]
                set-usage-export-bucket <project> (-r <kv>)... [-p <v>]... [-o <out>]
        region-autoscalers
                delete <project> <region> <autoscaler> [-p <v>]... [-o <out>]
                get <project> <region> <autoscaler> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
                patch <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
        region-backend-services
                delete <project> <region> <backend-service> [-p <v>]... [-o <out>]
                get <project> <region> <backend-service> [-p <v>]... [-o <out>]
                get-health <project> <region> <backend-service> (-r <kv>)... [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
                patch <project> <region> <backend-service> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <region> <backend-service> (-r <kv>)... [-p <v>]... [-o <out>]
        region-commitments
                aggregated-list <project> [-p <v>]... [-o <out>]
                get <project> <region> <commitment> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
        region-disk-types
                get <project> <region> <disk-type> [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
        region-disks
                create-snapshot <project> <region> <disk> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <project> <region> <disk> [-p <v>]... [-o <out>]
                get <project> <region> <disk> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
                resize <project> <region> <disk> (-r <kv>)... [-p <v>]... [-o <out>]
                set-labels <project> <region> <resource> (-r <kv>)... [-p <v>]... [-o <out>]
                test-iam-permissions <project> <region> <resource> (-r <kv>)... [-p <v>]... [-o <out>]
        region-instance-group-managers
                abandon-instances <project> <region> <instance-group-manager> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <project> <region> <instance-group-manager> [-p <v>]... [-o <out>]
                delete-instances <project> <region> <instance-group-manager> (-r <kv>)... [-p <v>]... [-o <out>]
                get <project> <region> <instance-group-manager> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
                list-managed-instances <project> <region> <instance-group-manager> [-p <v>]... [-o <out>]
                recreate-instances <project> <region> <instance-group-manager> (-r <kv>)... [-p <v>]... [-o <out>]
                resize <project> <region> <instance-group-manager> <size> [-p <v>]... [-o <out>]
                set-instance-template <project> <region> <instance-group-manager> (-r <kv>)... [-p <v>]... [-o <out>]
                set-target-pools <project> <region> <instance-group-manager> (-r <kv>)... [-p <v>]... [-o <out>]
        region-instance-groups
                get <project> <region> <instance-group> [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
                list-instances <project> <region> <instance-group> (-r <kv>)... [-p <v>]... [-o <out>]
                set-named-ports <project> <region> <instance-group> (-r <kv>)... [-p <v>]... [-o <out>]
        region-operations
                delete <project> <region> <operation> [-p <v>]...
                get <project> <region> <operation> [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
        regions
                get <project> <region> [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
        routers
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <region> <router> [-p <v>]... [-o <out>]
                get <project> <region> <router> [-p <v>]... [-o <out>]
                get-router-status <project> <region> <router> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
                patch <project> <region> <router> (-r <kv>)... [-p <v>]... [-o <out>]
                preview <project> <region> <router> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <region> <router> (-r <kv>)... [-p <v>]... [-o <out>]
        routes
                delete <project> <route> [-p <v>]... [-o <out>]
                get <project> <route> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
        security-policies
                add-rule <project> <security-policy> (-r <kv>)... [-p <v>]... [-o <out>]
                delete <project> <security-policy> [-p <v>]... [-o <out>]
                get <project> <security-policy> [-p <v>]... [-o <out>]
                get-rule <project> <security-policy> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                patch <project> <security-policy> (-r <kv>)... [-p <v>]... [-o <out>]
                patch-rule <project> <security-policy> (-r <kv>)... [-p <v>]... [-o <out>]
                remove-rule <project> <security-policy> [-p <v>]... [-o <out>]
        snapshots
                delete <project> <snapshot> [-p <v>]... [-o <out>]
                get <project> <snapshot> [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                set-labels <project> <resource> (-r <kv>)... [-p <v>]... [-o <out>]
        ssl-certificates
                delete <project> <ssl-certificate> [-p <v>]... [-o <out>]
                get <project> <ssl-certificate> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
        ssl-policies
                delete <project> <ssl-policy> [-p <v>]... [-o <out>]
                get <project> <ssl-policy> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                list-available-features <project> [-p <v>]... [-o <out>]
                patch <project> <ssl-policy> (-r <kv>)... [-p <v>]... [-o <out>]
        subnetworks
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <region> <subnetwork> [-p <v>]... [-o <out>]
                expand-ip-cidr-range <project> <region> <subnetwork> (-r <kv>)... [-p <v>]... [-o <out>]
                get <project> <region> <subnetwork> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
                list-usable <project> [-p <v>]... [-o <out>]
                patch <project> <region> <subnetwork> (-r <kv>)... [-p <v>]... [-o <out>]
                set-private-ip-google-access <project> <region> <subnetwork> (-r <kv>)... [-p <v>]... [-o <out>]
        target-http-proxies
                delete <project> <target-http-proxy> [-p <v>]... [-o <out>]
                get <project> <target-http-proxy> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                set-url-map <project> <target-http-proxy> (-r <kv>)... [-p <v>]... [-o <out>]
        target-https-proxies
                delete <project> <target-https-proxy> [-p <v>]... [-o <out>]
                get <project> <target-https-proxy> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                set-quic-override <project> <target-https-proxy> (-r <kv>)... [-p <v>]... [-o <out>]
                set-ssl-certificates <project> <target-https-proxy> (-r <kv>)... [-p <v>]... [-o <out>]
                set-ssl-policy <project> <target-https-proxy> (-r <kv>)... [-p <v>]... [-o <out>]
                set-url-map <project> <target-https-proxy> (-r <kv>)... [-p <v>]... [-o <out>]
        target-instances
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <zone> <target-instance> [-p <v>]... [-o <out>]
                get <project> <zone> <target-instance> [-p <v>]... [-o <out>]
                insert <project> <zone> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
        target-pools
                add-health-check <project> <region> <target-pool> (-r <kv>)... [-p <v>]... [-o <out>]
                add-instance <project> <region> <target-pool> (-r <kv>)... [-p <v>]... [-o <out>]
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <region> <target-pool> [-p <v>]... [-o <out>]
                get <project> <region> <target-pool> [-p <v>]... [-o <out>]
                get-health <project> <region> <target-pool> (-r <kv>)... [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
                remove-health-check <project> <region> <target-pool> (-r <kv>)... [-p <v>]... [-o <out>]
                remove-instance <project> <region> <target-pool> (-r <kv>)... [-p <v>]... [-o <out>]
                set-backup <project> <region> <target-pool> (-r <kv>)... [-p <v>]... [-o <out>]
        target-ssl-proxies
                delete <project> <target-ssl-proxy> [-p <v>]... [-o <out>]
                get <project> <target-ssl-proxy> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                set-backend-service <project> <target-ssl-proxy> (-r <kv>)... [-p <v>]... [-o <out>]
                set-proxy-header <project> <target-ssl-proxy> (-r <kv>)... [-p <v>]... [-o <out>]
                set-ssl-certificates <project> <target-ssl-proxy> (-r <kv>)... [-p <v>]... [-o <out>]
                set-ssl-policy <project> <target-ssl-proxy> (-r <kv>)... [-p <v>]... [-o <out>]
        target-tcp-proxies
                delete <project> <target-tcp-proxy> [-p <v>]... [-o <out>]
                get <project> <target-tcp-proxy> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                set-backend-service <project> <target-tcp-proxy> (-r <kv>)... [-p <v>]... [-o <out>]
                set-proxy-header <project> <target-tcp-proxy> (-r <kv>)... [-p <v>]... [-o <out>]
        target-vpn-gateways
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <region> <target-vpn-gateway> [-p <v>]... [-o <out>]
                get <project> <region> <target-vpn-gateway> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
        url-maps
                delete <project> <url-map> [-p <v>]... [-o <out>]
                get <project> <url-map> [-p <v>]... [-o <out>]
                insert <project> (-r <kv>)... [-p <v>]... [-o <out>]
                invalidate-cache <project> <url-map> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
                patch <project> <url-map> (-r <kv>)... [-p <v>]... [-o <out>]
                update <project> <url-map> (-r <kv>)... [-p <v>]... [-o <out>]
                validate <project> <url-map> (-r <kv>)... [-p <v>]... [-o <out>]
        vpn-tunnels
                aggregated-list <project> [-p <v>]... [-o <out>]
                delete <project> <region> <vpn-tunnel> [-p <v>]... [-o <out>]
                get <project> <region> <vpn-tunnel> [-p <v>]... [-o <out>]
                insert <project> <region> (-r <kv>)... [-p <v>]... [-o <out>]
                list <project> <region> [-p <v>]... [-o <out>]
        zone-operations
                delete <project> <zone> <operation> [-p <v>]...
                get <project> <zone> <operation> [-p <v>]... [-o <out>]
                list <project> <zone> [-p <v>]... [-o <out>]
        zones
                get <project> <zone> [-p <v>]... [-o <out>]
                list <project> [-p <v>]... [-o <out>]
  compute1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `compute1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/compute1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/compute1-secret.json`, assuming that the required *compute* API 
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

You may consider redirecting standard error into a file for ease of use, e.g. `compute1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/