<%
    from generator.lib.util import (markdown_comment, new_context)
    from generator.lib.cli import (CONFIG_DIR, CONFIG_DIR_FLAG, SCOPE_FLAG, application_secret_path, DEBUG_FLAG)

    c = new_context(schemas, resources)
%>\
<%namespace name="util" file="../../lib/util.mako"/>\
<%namespace name="argparse" file="lib/argparse.mako"/>\
<%block filter="markdown_comment">\
<%util:gen_info source="${self.uri}" />\
</%block>
The `${util.program_name()}` command-line interface *(CLI)* allows to use most features of the *Google ${util.canonical_name()}* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.
% if documentationLink:

Everything else about the *${util.canonical_name()}* API can be found at the
[official documentation site](${documentationLink}).
% endif

# Installation and Source Code

Install the command-line interface with cargo using:

```bash
cargo install ${util.crate_name()}
```

Find the source code [on github](${util.github_source_root_url()}).

# Usage

This documentation was generated from the *${util.canonical_name()}* API at revision *${revision is UNDEFINED and '00000000' or revision}*. The CLI is at version *${cargo.build_version}*.

```bash
${argparse.grammar(c)}
```

# Configuration

The program will store all persistent data in the `${CONFIG_DIR}` directory in *JSON* files prefixed with `${util.program_name()}-`.  You can change the directory used to store configuration with the `--${CONFIG_DIR_FLAG}` flag on a per-invocation basis.

More information about the various kinds of persistent data are given in the following paragraphs.

# Authentication

Most APIs require a user to authenticate any request. If this is the case, the [scope][scopes] determines the
set of permissions granted. The granularity of these is usually no more than *read-only* or *full-access*.

If not set, the system will automatically select the smallest feasible scope, e.g. when invoking a
method that is read-only, it will ask only for a read-only scope.
You may use the `--${SCOPE_FLAG}` flag to specify a scope directly.
All applicable scopes are documented in the respective method's CLI documentation.

The first time a scope is used, the user is asked for permission. Follow the instructions given
by the CLI to grant permissions, or to decline.

If a scope was authenticated by the user, the respective information will be stored as *JSON* in the configuration
directory, e.g. `${CONFIG_DIR}/${util.program_name()}-token-<scope-hash>.json`. No manual management of these tokens
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
`${CONFIG_DIR}/${application_secret_path(util.program_name())}`, assuming that the required *${name}* API
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be
output to standard error *as-is*.

The `--${DEBUG_FLAG}` flag will print errors using the `Debug` representation to standard error.

You may consider redirecting standard error into a file for ease of use, e.g. `${util.program_name()} --${DEBUG_FLAG} <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/
