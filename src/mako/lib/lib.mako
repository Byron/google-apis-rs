<%! from util import (activity_split, put_and, md_italic, split_camelcase_s, canonical_type_name, hub_type,
                      rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment, markdown_rust_block,
                      unindent_first_by, mangle_ident, mb_type, singular, scope_url_to_variant,
                      PART_MARKER_TRAIT, RESOURCE_MARKER_TRAIT, METHOD_BUILDER_MARKERT_TRAIT, 
                      find_fattest_resource, build_all_params, pass_through, parts_from_params,
                      REQUEST_MARKER_TRAIT, RESPONSE_MARKER_TRAIT, supports_scopes, to_api_version,
                      to_fqan, METHODS_RESOURCE, ADD_PARAM_MEDIA_EXAMPLE, PROTOCOL_TYPE_INFO, enclose_in,
                      upload_action_fn)  %>\
<%namespace name="util" file="util.mako"/>\
<%namespace name="mbuild" file="mbuild.mako"/>\

## If rust-doc is True, examples will be made to work for rust doc tests. Otherwise they are set 
## for github markdown.
###############################################################################################
###############################################################################################
<%def name="docs(c, rust_doc=True)">\
<%
    # fr == fattest resource, the fatter, the more important, right ?
    fr = find_fattest_resource(c)
    hub_url = 'struct.' + hub_type(c.schemas, util.canonical_name()) + '.html'
    method_builder_url = 'cmn/trait.' + METHOD_BUILDER_MARKERT_TRAIT + '.html'
    delegate_url = 'cmn/trait.Delegate.html'
    request_trait_url = 'cmn/trait.' + REQUEST_MARKER_TRAIT + '.html'
    response_trait_url = 'cmn/trait.' + RESPONSE_MARKER_TRAIT + '.html'
    part_trait_url = 'cmn/trait.' + PART_MARKER_TRAIT + '.html'

    doc_base_url = cargo.doc_base_url + '/' + util.crate_name() + '/'

    def link(name, url):
        lf = '[%s](%s)'
        if rust_doc:
            return lf % (name, url)
        for scheme in ('http', 'https'):
            if url.startswith(scheme + '://'):
                return lf % (name, url)
        return lf % (name, doc_base_url + url)


    api_version = to_api_version(version)
    if api_version[0].isdigit():
        api_version = 'v' + api_version


    upload_methods, download_methods, subscription_methods = list(), list(), list()
    for m in c.fqan_map.values():
        for array, param in ((download_methods, 'supportsMediaDownload'),
                             (upload_methods, 'supportsMediaUpload'),
                             (subscription_methods, 'supportsSubscription')):
            if m.get(param, False):
                array.append(m)
    # end for each method
    header_methods = (('Upload', upload_methods), ('Download', download_methods), ('Subscription', subscription_methods))
%>\
% if rust_doc:
This documentation was generated from *${util.canonical_name()}* crate version *${cargo.build_version}*.
The original source code can be found [on github](${cargo.repo_base_url}/tree/master/${directories.output}/${util.library_name()}).
% endif
# Features

% if len(c.rta_map) > 1:
Handle the following *Resources* with ease from the central ${link('hub', hub_url)} ... 
% elif METHODS_RESOURCE in c.rta_map:
Use the following functionality with ease from the central ${link('hub', hub_url)} ... 
% else:
It seems there is nothing you can do here ... .
% endif

% for r in sorted(c.rta_map.keys()):
% if r == METHODS_RESOURCE:
<% continue %>
% endif ## skip method resource
<%
    md_methods = list()
    for method in sorted(c.rta_map[r]):
        md_methods.append(link('*%s*' % split_camelcase_s(method),
                               'struct.%s.html' % mb_type(r, method)))
    md_resource = split_camelcase_s(r)
    sn = singular(canonical_type_name(r))

    if sn in schemas:
        md_resource = link(md_resource, 'struct.%s.html' % singular(canonical_type_name(r)))
%>\
* ${md_resource} (${put_and(md_methods)})
% endfor ## each resource activity

% if METHODS_RESOURCE in c.rta_map:
% if len(c.rta_map) > 1:
Other activities are ...

% endif
% for method in sorted(c.rta_map[METHODS_RESOURCE]):
* ${link(split_camelcase_s(method), 'struct.%s.html' % mb_type(METHODS_RESOURCE, method))}
% endfor
% endif

% for method_type, methods in header_methods:
% if methods:
${method_type} supported by ...

% for m in methods:
<% 
    _, resource, method = activity_split(m.id)
    name_parts = [split_camelcase_s(method)]
    if resource != METHODS_RESOURCE:
        name_parts.append(split_camelcase_s(resource))
%>\
* ${link('*%s*' % ' '.join(name_parts), 'struct.%s.html' % mb_type(resource, method))}
% endfor ## for each method

% endif  ## if methods
% endfor ## for each method type

% if documentationLink:
Everything else about the *${util.canonical_name()}* *${api_version}* API can be found at the
[official documentation site](${documentationLink}).
% endif
% if rust_doc:

Not what you are looking for ? Find all other google APIs in their Rust [documentation index](../index.html).
% endif

# Structure of this Library

The API is structured into the following primary items:

* **${link('Hub', hub_url)}**
    * a central object to maintain state and allow accessing all *Activities*
* **${link('Resources', 'cmn/trait.' + RESOURCE_MARKER_TRAIT + '.html')}**
    * primary types that you can apply *Activities* to
    * a collection of properties and *Parts*
    * **${link('Parts', part_trait_url)}**
        * a collection of properties
        * never directly used in *Activities*
* **${link('Activities', method_builder_url)}**
    * operations to apply to *Resources*

Generally speaking, you can invoke *Activities* like this:

```Rust,ignore
let r = hub.resource().activity(...).${api.terms.action}()
```

% if fr:
Or specifically ...

```ignore
% for an, a in c.sta_map[fr.id].iteritems():
<% category, resource, activity = activity_split(an) %>\
let r = hub.${mangle_ident(resource)}().${mangle_ident(activity)}(...).${api.terms.action}()
% endfor
```
% endif

The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
The `${api.terms.action}()` method performs the actual communication with the server and returns the respective result.

# Usage

${'##'} Setting up your Project

To use this library, you would put the following lines into your `Cargo.toml` file:

```toml
[dependencies]
${util.crate_name()} = "${cargo.build_version}"
```

${'##'} A complete example

${self.hub_usage_example(c, rust_doc, fr=fr)}\

${'##'} Handling Errors

All errors produced by the system are provided either as ${link('Result', 'cmn/enum.Result.html')} enumeration as return value of 
the ${api.terms.action}() methods, or handed as possibly intermediate results to either the 
${link('Hub Delegate', delegate_url)}, or the ${link('Authenticator Delegate', urls.authenticator_delegate)}.

When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
makes the system potentially resilient to all kinds of errors.

${'##'} About Uploads and Downlods
If a method supports downloads, the response body, which is part of the ${link('Result', 'cmn/enum.Result.html')}, should be
read by you to obtain the media.
If such a method also supports a ${link('Response Result', 'cmn/trait.ResponseResult.html')}, it will return that by default.
You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
this call: `${ADD_PARAM_MEDIA_EXAMPLE}`.

Methods supporting uploads can do so using up to ${len(PROTOCOL_TYPE_INFO)} different protocols: 
${put_and(md_italic(PROTOCOL_TYPE_INFO.keys()))}. The distinctiveness of each is represented by customized 
`${api.terms.action}(...)` methods, which are then named ${put_and(enclose_in('`', ("%s(...)" % upload_action_fn(api.terms.upload_action, v['suffix']) for v in PROTOCOL_TYPE_INFO.values())))} respectively.

${'##'} About Customization/Callbacks

You may alter the way an `${api.terms.action}()` method is called by providing a ${link('delegate', delegate_url)} to the 
${link('Method Builder', method_builder_url)} before making the final `${api.terms.action}()` call. 
Respective methods will be called to provide progress information, as well as determine whether the system should 
retry on failure.

The ${link('delegate trait', delegate_url)} is default-implemented, allowing you to customize it with minimal effort.

${'##'} About Parts

All structures provided by this library are made to be ${link('enocodable', request_trait_url)} and 
${link('decodable', response_trait_url)} via json. Optionals are used to indicate that partial requests are responses are valid.
Most optionals are are considered ${link('Parts', part_trait_url)} which are identifyable by name, which will be sent to 
the server to indicate either the set parts of the request or the desired parts in the response.

${'##'} About Builder Arguments

Using ${link('method builders', method_builder_url)}, you are able to prepare an action call by repeatedly calling it's methods.
These will always take a single argument, for which the following statements are true.

* [PODs][wiki-pod] are handed by copy
* strings are passed as `&str`
* ${link('request values', request_trait_url)} are borrowed

Arguments will always be copied or cloned into the builder, to make them independent of their original life times.

[wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
[builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
[google-go-api]: https://github.com/google/google-api-go-client
</%def>

## Sets up a hub ready for use. You must wrap it into a test function for it to work
## Needs test_prelude.
###############################################################################################
###############################################################################################
<%def name="test_hub(hub_type, comments=True)">\
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use ${util.library_name()}::${hub_type};

% if comments:
// Get an ApplicationSecret instance by some means. It contains the `client_id` and 
// `client_secret`, among other things.
% endif
let secret: ApplicationSecret = Default::default();
% if comments:
// Instantiate the authenticator. It will choose a suitable authentication flow for you, 
// unless you replace  `None` with the desired Flow.
// Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
// what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
// retrieve them from storage.
% endif
let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                              hyper::Client::new(),
                              <MemoryStorage as Default>::default(), None);
let mut hub = ${hub_type}::new(hyper::Client::new(), auth);\
</%def>

## You will still have to set the filter for your comment type - either nothing, or rust_doc_comment !
###############################################################################################
###############################################################################################
<%def name="hub_usage_example(c, rust_doc=True, fr=None)">\
<% 
    test_filter = rust_test_fn_invisible
    main_filter = rust_doc_test_norun
    if not rust_doc:
        test_filter = pass_through
        main_filter = markdown_rust_block

    if fr is None:
        fr = find_fattest_resource(c)
    if fr is not None:
        fqan = None
        last_param_count = None
        for fqan in c.sta_map[fr.id]:
            category, aresource, amethod = activity_split(fqan)
            # Cannot use fqan directly, as it might need remapping thanks to 'special case' resource.
            # see METHODS_RESOURCE for more information
            am = c.fqan_map[to_fqan(category, aresource, amethod)]
            build_all_params(c, am)
            aparams, arequest_value = build_all_params(c, am)

            if last_param_count is None or len(aparams) > last_param_count:
                m, resource, method, params, request_value = am, aresource or category, amethod, aparams, arequest_value
                last_param_count = len(aparams)
        # end for each fn to test
        part_prop, parts = parts_from_params(params)
    # end fill in values
%>\
% if fr:
${mbuild.usage(resource, method, m, params, request_value, parts, show_all=True, rust_doc=rust_doc, handle_result=True)}\
% else:
<%block filter="main_filter">\
${util.test_prelude()}\

<%block filter="test_filter">\
${self.test_hub(hub_type(c.schemas, util.canonical_name()))}
</%block>
</%block>
% endif
</%def>

###############################################################################################
###############################################################################################
<%def name="license()">\
# License
The **${util.library_name()}** library was generated by ${put_and(copyright.authors)}, and is placed 
under the *${copyright.license_abbrev}* license.
You can read the full text at the repository's [license file][repo-license].

[repo-license]: ${cargo.repo_base_url + '/LICENSE.md'}
</%def>


## Builds the scope-enum for the API
## It's possible there is no scope enum if there is no auth information
###############################################################################################
###############################################################################################
<%def name="scope_enum()">\
% if not supports_scopes(auth):
<% return '' %>\
% endif
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
% for url, scope in auth.oauth2.scopes.items():
    ${scope.description | rust_doc_comment}
    ${scope_url_to_variant(name, url, fully_qualified=False)},
    % if not loop.last:

    % endif
% endfor
}

impl Str for Scope {
    fn as_slice(&self) -> &str {
        match *self {
            % for url in auth.oauth2.scopes.keys():
            ${scope_url_to_variant(name, url)} => "${url}",
            % endfor
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
<%
            default_url = None
            shortest_url = None
            for url in auth.oauth2.scopes.keys():
                if not default_url and 'readonly' in url:
                    default_url = url
                if not shortest_url or len(shortest_url) > len(url):
                    shortest_url = url
            # end for each url
            default_url = default_url or shortest_url
%>\
        ${scope_url_to_variant(name, default_url)}
    }
}
</%def>