<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/cli/README.md.mako'
DO NOT EDIT !
-->
The `compute1` command-line interface *(CLI)* allows to use most features of the *Google compute* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

# Usage

This documentation was generated from the *compute* API at revision *20150326*. The CLI is at version *0.1.0*.

```bash
  compute1 [options] addresses aggregated-list <project> [-p <v>...] [-o <out>]
  compute1 [options] addresses delete <project> <region> <address> [-p <v>...] [-o <out>]
  compute1 [options] addresses get <project> <region> <address> [-p <v>...] [-o <out>]
  compute1 [options] addresses insert <project> <region> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] addresses list <project> <region> [-p <v>...] [-o <out>]
  compute1 [options] backend-services delete <project> <backend-service> [-p <v>...] [-o <out>]
  compute1 [options] backend-services get <project> <backend-service> [-p <v>...] [-o <out>]
  compute1 [options] backend-services get-health <project> <backend-service> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] backend-services insert <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] backend-services list <project> [-p <v>...] [-o <out>]
  compute1 [options] backend-services patch <project> <backend-service> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] backend-services update <project> <backend-service> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] disk-types aggregated-list <project> [-p <v>...] [-o <out>]
  compute1 [options] disk-types get <project> <zone> <disk-type> [-p <v>...] [-o <out>]
  compute1 [options] disk-types list <project> <zone> [-p <v>...] [-o <out>]
  compute1 [options] disks aggregated-list <project> [-p <v>...] [-o <out>]
  compute1 [options] disks create-snapshot <project> <zone> <disk> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] disks delete <project> <zone> <disk> [-p <v>...] [-o <out>]
  compute1 [options] disks get <project> <zone> <disk> [-p <v>...] [-o <out>]
  compute1 [options] disks insert <project> <zone> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] disks list <project> <zone> [-p <v>...] [-o <out>]
  compute1 [options] firewalls delete <project> <firewall> [-p <v>...] [-o <out>]
  compute1 [options] firewalls get <project> <firewall> [-p <v>...] [-o <out>]
  compute1 [options] firewalls insert <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] firewalls list <project> [-p <v>...] [-o <out>]
  compute1 [options] firewalls patch <project> <firewall> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] firewalls update <project> <firewall> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] forwarding-rules aggregated-list <project> [-p <v>...] [-o <out>]
  compute1 [options] forwarding-rules delete <project> <region> <forwarding-rule> [-p <v>...] [-o <out>]
  compute1 [options] forwarding-rules get <project> <region> <forwarding-rule> [-p <v>...] [-o <out>]
  compute1 [options] forwarding-rules insert <project> <region> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] forwarding-rules list <project> <region> [-p <v>...] [-o <out>]
  compute1 [options] forwarding-rules set-target <project> <region> <forwarding-rule> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] global-addresses delete <project> <address> [-p <v>...] [-o <out>]
  compute1 [options] global-addresses get <project> <address> [-p <v>...] [-o <out>]
  compute1 [options] global-addresses insert <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] global-addresses list <project> [-p <v>...] [-o <out>]
  compute1 [options] global-forwarding-rules delete <project> <forwarding-rule> [-p <v>...] [-o <out>]
  compute1 [options] global-forwarding-rules get <project> <forwarding-rule> [-p <v>...] [-o <out>]
  compute1 [options] global-forwarding-rules insert <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] global-forwarding-rules list <project> [-p <v>...] [-o <out>]
  compute1 [options] global-forwarding-rules set-target <project> <forwarding-rule> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] global-operations aggregated-list <project> [-p <v>...] [-o <out>]
  compute1 [options] global-operations delete <project> <operation> [-p <v>...]
  compute1 [options] global-operations get <project> <operation> [-p <v>...] [-o <out>]
  compute1 [options] global-operations list <project> [-p <v>...] [-o <out>]
  compute1 [options] http-health-checks delete <project> <http-health-check> [-p <v>...] [-o <out>]
  compute1 [options] http-health-checks get <project> <http-health-check> [-p <v>...] [-o <out>]
  compute1 [options] http-health-checks insert <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] http-health-checks list <project> [-p <v>...] [-o <out>]
  compute1 [options] http-health-checks patch <project> <http-health-check> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] http-health-checks update <project> <http-health-check> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] images delete <project> <image> [-p <v>...] [-o <out>]
  compute1 [options] images deprecate <project> <image> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] images get <project> <image> [-p <v>...] [-o <out>]
  compute1 [options] images insert <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] images list <project> [-p <v>...] [-o <out>]
  compute1 [options] instance-templates delete <project> <instance-template> [-p <v>...] [-o <out>]
  compute1 [options] instance-templates get <project> <instance-template> [-p <v>...] [-o <out>]
  compute1 [options] instance-templates insert <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] instance-templates list <project> [-p <v>...] [-o <out>]
  compute1 [options] instances add-access-config <project> <zone> <instance> <network-interface> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] instances aggregated-list <project> [-p <v>...] [-o <out>]
  compute1 [options] instances attach-disk <project> <zone> <instance> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] instances delete <project> <zone> <instance> [-p <v>...] [-o <out>]
  compute1 [options] instances delete-access-config <project> <zone> <instance> <access-config> <network-interface> [-p <v>...] [-o <out>]
  compute1 [options] instances detach-disk <project> <zone> <instance> <device-name> [-p <v>...] [-o <out>]
  compute1 [options] instances get <project> <zone> <instance> [-p <v>...] [-o <out>]
  compute1 [options] instances get-serial-port-output <project> <zone> <instance> [-p <v>...] [-o <out>]
  compute1 [options] instances insert <project> <zone> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] instances list <project> <zone> [-p <v>...] [-o <out>]
  compute1 [options] instances reset <project> <zone> <instance> [-p <v>...] [-o <out>]
  compute1 [options] instances set-disk-auto-delete <project> <zone> <instance> <auto-delete> <device-name> [-p <v>...] [-o <out>]
  compute1 [options] instances set-metadata <project> <zone> <instance> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] instances set-scheduling <project> <zone> <instance> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] instances set-tags <project> <zone> <instance> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] instances start <project> <zone> <instance> [-p <v>...] [-o <out>]
  compute1 [options] instances stop <project> <zone> <instance> [-p <v>...] [-o <out>]
  compute1 [options] licenses get <project> <license> [-p <v>...] [-o <out>]
  compute1 [options] machine-types aggregated-list <project> [-p <v>...] [-o <out>]
  compute1 [options] machine-types get <project> <zone> <machine-type> [-p <v>...] [-o <out>]
  compute1 [options] machine-types list <project> <zone> [-p <v>...] [-o <out>]
  compute1 [options] networks delete <project> <network> [-p <v>...] [-o <out>]
  compute1 [options] networks get <project> <network> [-p <v>...] [-o <out>]
  compute1 [options] networks insert <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] networks list <project> [-p <v>...] [-o <out>]
  compute1 [options] projects get <project> [-p <v>...] [-o <out>]
  compute1 [options] projects move-disk <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] projects move-instance <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] projects set-common-instance-metadata <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] projects set-usage-export-bucket <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] region-operations delete <project> <region> <operation> [-p <v>...]
  compute1 [options] region-operations get <project> <region> <operation> [-p <v>...] [-o <out>]
  compute1 [options] region-operations list <project> <region> [-p <v>...] [-o <out>]
  compute1 [options] regions get <project> <region> [-p <v>...] [-o <out>]
  compute1 [options] regions list <project> [-p <v>...] [-o <out>]
  compute1 [options] routes delete <project> <route> [-p <v>...] [-o <out>]
  compute1 [options] routes get <project> <route> [-p <v>...] [-o <out>]
  compute1 [options] routes insert <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] routes list <project> [-p <v>...] [-o <out>]
  compute1 [options] snapshots delete <project> <snapshot> [-p <v>...] [-o <out>]
  compute1 [options] snapshots get <project> <snapshot> [-p <v>...] [-o <out>]
  compute1 [options] snapshots list <project> [-p <v>...] [-o <out>]
  compute1 [options] target-http-proxies delete <project> <target-http-proxy> [-p <v>...] [-o <out>]
  compute1 [options] target-http-proxies get <project> <target-http-proxy> [-p <v>...] [-o <out>]
  compute1 [options] target-http-proxies insert <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] target-http-proxies list <project> [-p <v>...] [-o <out>]
  compute1 [options] target-http-proxies set-url-map <project> <target-http-proxy> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] target-instances aggregated-list <project> [-p <v>...] [-o <out>]
  compute1 [options] target-instances delete <project> <zone> <target-instance> [-p <v>...] [-o <out>]
  compute1 [options] target-instances get <project> <zone> <target-instance> [-p <v>...] [-o <out>]
  compute1 [options] target-instances insert <project> <zone> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] target-instances list <project> <zone> [-p <v>...] [-o <out>]
  compute1 [options] target-pools add-health-check <project> <region> <target-pool> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] target-pools add-instance <project> <region> <target-pool> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] target-pools aggregated-list <project> [-p <v>...] [-o <out>]
  compute1 [options] target-pools delete <project> <region> <target-pool> [-p <v>...] [-o <out>]
  compute1 [options] target-pools get <project> <region> <target-pool> [-p <v>...] [-o <out>]
  compute1 [options] target-pools get-health <project> <region> <target-pool> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] target-pools insert <project> <region> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] target-pools list <project> <region> [-p <v>...] [-o <out>]
  compute1 [options] target-pools remove-health-check <project> <region> <target-pool> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] target-pools remove-instance <project> <region> <target-pool> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] target-pools set-backup <project> <region> <target-pool> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] target-vpn-gateways aggregated-list <project> [-p <v>...] [-o <out>]
  compute1 [options] target-vpn-gateways delete <project> <region> <target-vpn-gateway> [-p <v>...] [-o <out>]
  compute1 [options] target-vpn-gateways get <project> <region> <target-vpn-gateway> [-p <v>...] [-o <out>]
  compute1 [options] target-vpn-gateways insert <project> <region> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] target-vpn-gateways list <project> <region> [-p <v>...] [-o <out>]
  compute1 [options] url-maps delete <project> <url-map> [-p <v>...] [-o <out>]
  compute1 [options] url-maps get <project> <url-map> [-p <v>...] [-o <out>]
  compute1 [options] url-maps insert <project> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] url-maps list <project> [-p <v>...] [-o <out>]
  compute1 [options] url-maps patch <project> <url-map> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] url-maps update <project> <url-map> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] url-maps validate <project> <url-map> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] vpn-tunnels aggregated-list <project> [-p <v>...] [-o <out>]
  compute1 [options] vpn-tunnels delete <project> <region> <vpn-tunnel> [-p <v>...] [-o <out>]
  compute1 [options] vpn-tunnels get <project> <region> <vpn-tunnel> [-p <v>...] [-o <out>]
  compute1 [options] vpn-tunnels insert <project> <region> -r <kv>... [-p <v>...] [-o <out>]
  compute1 [options] vpn-tunnels list <project> <region> [-p <v>...] [-o <out>]
  compute1 [options] zone-operations delete <project> <zone> <operation> [-p <v>...]
  compute1 [options] zone-operations get <project> <zone> <operation> [-p <v>...] [-o <out>]
  compute1 [options] zone-operations list <project> <zone> [-p <v>...] [-o <out>]
  compute1 [options] zones get <project> <zone> [-p <v>...] [-o <out>]
  compute1 [options] zones list <project> [-p <v>...] [-o <out>]
  compute1 --help

All documentation details can be found at
http://byron.github.io/google-apis-rs/google_compute1_cli/index.html

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