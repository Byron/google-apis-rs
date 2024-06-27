<!---
DO NOT EDIT !
This file was generated automatically from 'src/generator/templates/cli/README.md.mako'
DO NOT EDIT !
-->
The `classroom1` command-line interface *(CLI)* allows to use most features of the *Google classroom* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *classroom* API can be found at the
[official documentation site](https://developers.google.com/classroom/).

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install google-classroom1-cli
```

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/main/gen/classroom1-cli).

# Usage

This documentation was generated from the *classroom* API at revision *20240617*. The CLI is at version *5.0.5*.

```bash
classroom1 [options]
        courses
                aliases-create <course-id> (-r <kv>)... [-p <v>]... [-o <out>]
                aliases-delete <course-id> <alias> [-p <v>]... [-o <out>]
                aliases-list <course-id> [-p <v>]... [-o <out>]
                announcements-add-on-attachments-create <course-id> <item-id> (-r <kv>)... [-p <v>]... [-o <out>]
                announcements-add-on-attachments-delete <course-id> <item-id> <attachment-id> [-p <v>]... [-o <out>]
                announcements-add-on-attachments-get <course-id> <item-id> <attachment-id> [-p <v>]... [-o <out>]
                announcements-add-on-attachments-list <course-id> <item-id> [-p <v>]... [-o <out>]
                announcements-add-on-attachments-patch <course-id> <item-id> <attachment-id> (-r <kv>)... [-p <v>]... [-o <out>]
                announcements-create <course-id> (-r <kv>)... [-p <v>]... [-o <out>]
                announcements-delete <course-id> <id> [-p <v>]... [-o <out>]
                announcements-get <course-id> <id> [-p <v>]... [-o <out>]
                announcements-get-add-on-context <course-id> <item-id> [-p <v>]... [-o <out>]
                announcements-list <course-id> [-p <v>]... [-o <out>]
                announcements-modify-assignees <course-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                announcements-patch <course-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-add-on-attachments-create <course-id> <item-id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-add-on-attachments-delete <course-id> <item-id> <attachment-id> [-p <v>]... [-o <out>]
                course-work-add-on-attachments-get <course-id> <item-id> <attachment-id> [-p <v>]... [-o <out>]
                course-work-add-on-attachments-list <course-id> <item-id> [-p <v>]... [-o <out>]
                course-work-add-on-attachments-patch <course-id> <item-id> <attachment-id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-add-on-attachments-student-submissions-get <course-id> <item-id> <attachment-id> <submission-id> [-p <v>]... [-o <out>]
                course-work-add-on-attachments-student-submissions-patch <course-id> <item-id> <attachment-id> <submission-id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-create <course-id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-delete <course-id> <id> [-p <v>]... [-o <out>]
                course-work-get <course-id> <id> [-p <v>]... [-o <out>]
                course-work-get-add-on-context <course-id> <item-id> [-p <v>]... [-o <out>]
                course-work-list <course-id> [-p <v>]... [-o <out>]
                course-work-modify-assignees <course-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-patch <course-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-student-submissions-get <course-id> <course-work-id> <id> [-p <v>]... [-o <out>]
                course-work-student-submissions-list <course-id> <course-work-id> [-p <v>]... [-o <out>]
                course-work-student-submissions-modify-attachments <course-id> <course-work-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-student-submissions-patch <course-id> <course-work-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-student-submissions-reclaim <course-id> <course-work-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-student-submissions-return <course-id> <course-work-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-student-submissions-turn-in <course-id> <course-work-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-materials-add-on-attachments-create <course-id> <item-id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-materials-add-on-attachments-delete <course-id> <item-id> <attachment-id> [-p <v>]... [-o <out>]
                course-work-materials-add-on-attachments-get <course-id> <item-id> <attachment-id> [-p <v>]... [-o <out>]
                course-work-materials-add-on-attachments-list <course-id> <item-id> [-p <v>]... [-o <out>]
                course-work-materials-add-on-attachments-patch <course-id> <item-id> <attachment-id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-materials-create <course-id> (-r <kv>)... [-p <v>]... [-o <out>]
                course-work-materials-delete <course-id> <id> [-p <v>]... [-o <out>]
                course-work-materials-get <course-id> <id> [-p <v>]... [-o <out>]
                course-work-materials-get-add-on-context <course-id> <item-id> [-p <v>]... [-o <out>]
                course-work-materials-list <course-id> [-p <v>]... [-o <out>]
                course-work-materials-patch <course-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                create (-r <kv>)... [-p <v>]... [-o <out>]
                delete <id> [-p <v>]... [-o <out>]
                get <id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
                patch <id> (-r <kv>)... [-p <v>]... [-o <out>]
                posts-add-on-attachments-create <course-id> <post-id> (-r <kv>)... [-p <v>]... [-o <out>]
                posts-add-on-attachments-delete <course-id> <post-id> <attachment-id> [-p <v>]... [-o <out>]
                posts-add-on-attachments-get <course-id> <post-id> <attachment-id> [-p <v>]... [-o <out>]
                posts-add-on-attachments-list <course-id> <post-id> [-p <v>]... [-o <out>]
                posts-add-on-attachments-patch <course-id> <post-id> <attachment-id> (-r <kv>)... [-p <v>]... [-o <out>]
                posts-add-on-attachments-student-submissions-get <course-id> <post-id> <attachment-id> <submission-id> [-p <v>]... [-o <out>]
                posts-add-on-attachments-student-submissions-patch <course-id> <post-id> <attachment-id> <submission-id> (-r <kv>)... [-p <v>]... [-o <out>]
                posts-get-add-on-context <course-id> <post-id> [-p <v>]... [-o <out>]
                students-create <course-id> (-r <kv>)... [-p <v>]... [-o <out>]
                students-delete <course-id> <user-id> [-p <v>]... [-o <out>]
                students-get <course-id> <user-id> [-p <v>]... [-o <out>]
                students-list <course-id> [-p <v>]... [-o <out>]
                teachers-create <course-id> (-r <kv>)... [-p <v>]... [-o <out>]
                teachers-delete <course-id> <user-id> [-p <v>]... [-o <out>]
                teachers-get <course-id> <user-id> [-p <v>]... [-o <out>]
                teachers-list <course-id> [-p <v>]... [-o <out>]
                topics-create <course-id> (-r <kv>)... [-p <v>]... [-o <out>]
                topics-delete <course-id> <id> [-p <v>]... [-o <out>]
                topics-get <course-id> <id> [-p <v>]... [-o <out>]
                topics-list <course-id> [-p <v>]... [-o <out>]
                topics-patch <course-id> <id> (-r <kv>)... [-p <v>]... [-o <out>]
                update <id> (-r <kv>)... [-p <v>]... [-o <out>]
        invitations
                accept <id> [-p <v>]... [-o <out>]
                create (-r <kv>)... [-p <v>]... [-o <out>]
                delete <id> [-p <v>]... [-o <out>]
                get <id> [-p <v>]... [-o <out>]
                list [-p <v>]... [-o <out>]
        registrations
                create (-r <kv>)... [-p <v>]... [-o <out>]
                delete <registration-id> [-p <v>]... [-o <out>]
        user-profiles
                get <user-id> [-p <v>]... [-o <out>]
                guardian-invitations-create <student-id> (-r <kv>)... [-p <v>]... [-o <out>]
                guardian-invitations-get <student-id> <invitation-id> [-p <v>]... [-o <out>]
                guardian-invitations-list <student-id> [-p <v>]... [-o <out>]
                guardian-invitations-patch <student-id> <invitation-id> (-r <kv>)... [-p <v>]... [-o <out>]
                guardians-delete <student-id> <guardian-id> [-p <v>]... [-o <out>]
                guardians-get <student-id> <guardian-id> [-p <v>]... [-o <out>]
                guardians-list <student-id> [-p <v>]... [-o <out>]
  classroom1 --help

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

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `classroom1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

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
directory, e.g. `~/.google-service-cli/classroom1-token-<scope-hash>.json`. No manual management of these tokens
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
`~/.google-service-cli/classroom1-secret.json`, assuming that the required *classroom* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `classroom1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
