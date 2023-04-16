<%!
    from generator.lib.util import (put_and, rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment,
                      rb_type, mb_type, singular, hub_type, to_fqan, indent_all_but_first_by,
                      activity_rust_type, mangle_ident, activity_input_type, get_word,
                      split_camelcase_s, property, is_pod_property, TREF, IO_REQUEST,
                      schema_to_required_property, rust_copy_value_s, is_required_property,
                      hide_rust_doc_test, build_all_params, REQUEST_VALUE_PROPERTY_NAME, organize_params,
                      indent_by, to_rust_type, rnd_arg_val_for_type, extract_parts, mb_type_params_s,
                      hub_type_params_s, method_media_params, enclose_in, method_response,
                      CALL_BUILDER_MARKERT_TRAIT, pass_through, markdown_rust_block, parts_from_params,
                      DELEGATE_PROPERTY_NAME, struct_type_bounds_s, scope_url_to_variant,
                      re_find_replacements, ADD_PARAM_FN, ADD_PARAM_MEDIA_EXAMPLE, upload_action_fn, METHODS_RESOURCE,
                      method_name_to_variant, size_to_bytes, method_default_scope,
                      is_repeated_property, setter_fn_name, ADD_SCOPE_FN, ADD_SCOPES_FN, rust_doc_sanitize,
                      CLEAR_SCOPES_FN, items, string_impl)

    SIMPLE = "simple"
    RESUMABLE = "resumable"
    PROTOCOL_TYPE_MAP = {
        SIMPLE: "client::UploadProtocol::Simple",
        RESUMABLE: "client::UploadProtocol::Resumable"
    }

    def get_parts(part_prop):
        if not part_prop:
            return list()
        return extract_parts(part_prop.get('description', ''))

    def make_parts_desc(part_prop):

        parts = get_parts(part_prop)
        if not parts:
            return None
        part_desc = "**Settable Parts**\n\n"
        part_desc += ''.join('* *%s*\n' % part for part in parts)
        part_desc = part_desc[:-1]
        return part_desc
%>\
<%namespace name="util" file="../../../lib/util.mako"/>\
<%namespace name="lib" file="lib.mako"/>\

## Creates a method builder type
###############################################################################################
###############################################################################################
<%def name="new(resource, method, c)">\
<%
    hub_type_name = hub_type(schemas,util.canonical_name())
    m = c.fqan_map[to_fqan(c.rtc_map[resource], resource, method)]
    response_schema = method_response(c, m)

    # an identifier for a property. We prefix them to prevent clashes with the setters
    mb_tparams = mb_type_params_s(m)
    ThisType = mb_type(resource, method) + mb_tparams

    params, request_value = build_all_params(c, m)
    alt_param = None
    for p in params:
        if p.name == 'alt':
            alt_param = p
            break
        # end
    # end

    part_prop, parts = parts_from_params(params)
    part_desc = make_parts_desc(part_prop)
    parts = get_parts(part_prop)
%>\
% if 'description' in m:
${m.description | rust_doc_sanitize(documentationLink), rust_doc_comment}
///
% endif
% if m.get('supportsMediaDownload', False):
/// This method supports **media download**. To enable it, adjust the builder like this:
% if alt_param:
/// `.${mangle_ident(setter_fn_name(alt_param))}("media")`.
% else:
/// `${ADD_PARAM_MEDIA_EXAMPLE}`.
% endif
% if response_schema:
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `${response_schema.id}` structure that you would usually get. The latter will be a default value.
% endif
///
% endif ## supports media download
% if resource == METHODS_RESOURCE:
/// A builder for the *${method}* method.
% else:
/// A builder for the *${method}* method supported by a *${singular(resource)}* resource.
% endif
/// It is not used directly, but through a [`${rb_type(resource)}`] instance.
///
% if part_desc:
${part_desc | rust_doc_sanitize(documentationLink), rust_doc_comment}
///
% if m.get('scopes'):
/// # Scopes
///
/// You will need authorization for \
% if len(m.scopes) > 1:
at least one of the following scopes to make a valid call, possibly depending on *parts*:
///
% for s in m.scopes:
/// * *${s}*
% endfor
% else:
the *${m.scopes[0]}* scope to make a valid call.
% endif # len(scopes) > 1
///
/// The default scope will be `${scope_url_to_variant(name, method_default_scope(m), fully_qualified=True)}`.
% endif # have scopes
///
% endif
/// # Example
///
/// Instantiate a resource method builder
///
<%block filter="rust_doc_comment">\
${self.usage(resource, method, m, params, request_value, parts)}\
</%block>
pub struct ${ThisType}
    where ${struct_type_bounds_s()} {

    hub: &'a ${hub_type_name}${hub_type_params_s()},
## PROPERTIES ###############
% for p in params:
    ${property(p.name)}:\
    % if is_required_property(p):
 ${activity_rust_type(schemas, p, allow_optionals=False)},
    % else:
 ${activity_rust_type(schemas, p)},
    % endif
% endfor
## A generic map for additinal parameters. Sometimes you can set some that are documented online only
    ${api.properties.params}: HashMap<String, String>,
    % if method_default_scope(m):
## We need the scopes sorted, to not unnecessarily query new tokens
    ${api.properties.scopes}: BTreeSet<String>
    % endif
}

impl${mb_tparams} ${CALL_BUILDER_MARKERT_TRAIT} for ${ThisType} {}

impl${mb_tparams} ${ThisType}
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{
% if api.get('no_upload_prefix') is not None and ThisType.startswith(api.no_upload_prefix):
${self._action_fn(c, resource, method, m, params, request_value, parts, doit_without_upload = True)}\
% endif

${self._action_fn(c, resource, method, m, params, request_value, parts)}\

## SETTERS ###############
% for p in params:
${self._setter_fn(resource, method, m, p, part_prop, ThisType, c)}\
% endfor

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    % if parameters:
    ///
    /// # Additional Parameters
    ///
    % for opn, op in list((opn, op) for (opn, op) in parameters.items() if opn not in [p.name for p in params]):
    /// * *${opn}* (${op.location}-${op.type}) - ${op.description}
    % endfor
    % endif
    pub fn ${ADD_PARAM_FN}<T>(mut self, name: T, value: T) -> ${ThisType}
                                                        where T: AsRef<str> {
        self.${api.properties.params}.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    % if method_default_scope(m):
    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`${scope_url_to_variant(name, method_default_scope(m), fully_qualified=True)}`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn ${ADD_SCOPE_FN}<St>(mut self, scope: St) -> ${ThisType}
                                                        where St: AsRef<str> {
        self.${api.properties.scopes}.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::${ADD_SCOPE_FN}()`] for details.
    pub fn ${ADD_SCOPES_FN}<I, St>(mut self, scopes: I) -> ${ThisType}
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self.${api.properties.scopes}
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::${ADD_PARAM_FN}()`]
    /// for details).
    pub fn ${CLEAR_SCOPES_FN}(mut self) -> ${ThisType} {
        self.${api.properties.scopes}.clear();
        self
    }
    % endif
}
</%def>


## creates a setter for the call builder
###############################################################################################
###############################################################################################
<%def name="_setter_fn(resource, method, m, p, part_prop, ThisType, c)">\
<%
    InType = activity_input_type(schemas, p)

    if is_repeated_property(p):
        p.repeated = False
        InType = activity_input_type(schemas, p)
        p.repeated = True

    def show_part_info(m, p):
        if p.name != 'part':
            return False
        if not (m.get('request') and m.get('response')):
            return False
        return m.request.get(TREF, 'first') == m.response.get(TREF, 'second')

    value_name = 'new_value'
    new_value_copied = rust_copy_value_s(value_name, InType, p)
    if not is_required_property(p) and not is_repeated_property(p):
        new_value_copied = 'Some(%s)' % new_value_copied

    part_desc = None
    if part_prop is not None and p.name in ('part', REQUEST_VALUE_PROPERTY_NAME):
        part_desc = make_parts_desc(part_prop)
    # end part description
%>\
    % if 'description' in p:
    ${p.description | rust_doc_sanitize(documentationLink), rust_doc_comment, indent_all_but_first_by(1)}
    % endif
    % if is_repeated_property(p):
    ///
    /// Append the given value to the *${split_camelcase_s(p.name)}* ${get_word(p, 'location')}property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    % else:
    ///
    /// Sets the *${split_camelcase_s(p.name)}* ${get_word(p, 'location')}property to the given value.
    % endif
    % if show_part_info(m, p):
    ///
    /// Even though the *parts* list is automatically derived from *Resource* passed in
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify
    /// the parts you provide in addition to the ones you want in the response.
    % elif is_required_property(p):
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    % endif
    % if part_desc:
    ///
    ${part_desc | rust_doc_sanitize(documentationLink), rust_doc_comment, indent_all_but_first_by(1)}
    % endif
    pub fn ${mangle_ident(setter_fn_name(p))}(mut self, ${value_name}: ${InType}) -> ${ThisType} {
        % if p.get('repeated', False):
        self.${property(p.name)}.push(${new_value_copied});
        % else:
        self.${property(p.name)} = ${new_value_copied};
        % endif
        self
    }
</%def>


## creates usage docs the method builder
## show_all: If True, we will show all comments and hide no prelude. It's good to build a complete,
## documented example for a given method.
###############################################################################################
###############################################################################################
<%def name="usage(resource, method, m, params, request_value, parts=None, show_all=False, rust_doc=True, handle_result=False)">\
<%
    hub_type_name = hub_type(schemas, util.canonical_name())
    required_props, optional_props, part_prop = organize_params(params, request_value)
    is_string_value = lambda v: v.endswith('"')

    # to rust value
    def trv(spn, sp, sn=None):
        prev = sp.get('repeated', False)
        sp.repeated = False
        res = to_rust_type(schemas, sn, spn, sp, allow_optionals=False)
        sp.repeated = prev
        return res
    # rvfrt = random value for rust type
    rvfrt = lambda spn, sp, sn=None: rnd_arg_val_for_type(trv(spn, sp, sn))

    rb_name = 'req'   # name of request binding
    required_args = request_value and [rb_name] or []
    for p in required_props:
        # could also just skip the first element, but ... let's be safe
        if request_value and request_value.id == p.get(TREF):
            continue
        v = rnd_arg_val_for_type(activity_input_type(schemas, p))
        # we chose to replace random strings with their meaning, as indicated by the name !
        if is_string_value(v):
            v = '"%s"' % p.name
        required_args.append(v)
    # end for each required property
    required_args = ', '.join(required_args)

    media_params = method_media_params(m)

    if media_params:
        # index 0 == Simple (usually)
        # index 1 == Resumable
        # propose standard upload for smaller media. Also means we get to test different code-paths
        index = -1
        if media_params[-1].max_size < 100*1024*1024:
            index = 0
        action_name = upload_action_fn(api.terms.upload_action, media_params[index].type.suffix)
    else:
        action_name = api.terms.action
    action_args = media_params and media_params[-1].type.example_value or ''

    random_value_warning = "Values shown here are possibly random and not representative !"

    hide_filter = show_all and pass_through or hide_rust_doc_test
    test_block_filter = rust_doc and rust_doc_test_norun or markdown_rust_block
    test_fn_filter = rust_doc and rust_test_fn_invisible or pass_through

    if request_value:
        request_value_type = request_value.id
%>\
<%block filter="test_block_filter">\
${capture(util.test_prelude) | hide_filter}\
% if request_value:
use ${util.library_name()}::api::${request_value_type};
% endif
% if handle_result:
use ${util.library_name()}::{Result, Error};
% endif
% if media_params:
use std::fs;
% endif
<%block filter="test_fn_filter">\
${capture(lib.test_hub, hub_type_name, comments=show_all) | hide_filter}
% if request_value:
// As the method needs a request, you would usually fill it with the desired information
// into the respective structure. Some of the parts shown here might not be applicable !
// ${random_value_warning}
let mut ${rb_name} = ${request_value_type}::default();
% for spn, sp in items(request_value.get('properties', dict())):
% if parts is not None and spn not in parts:
<% continue %>
% endif
<%
    rtn = trv(spn, sp, request_value.id)
    assignment = rnd_arg_val_for_type(rtn)
    if is_string_value(assignment):
        assignment = assignment + '.to_string()'
    if assignment.endswith('default()'):
        assignment = assignment[1:] # cut & - it's not ok in this case :)!
        assignment += '; // is %s' % rtn
    else:
        assignment = 'Some(%s);' % assignment
%>\
${rb_name}.${mangle_ident(spn)} = ${assignment}
% endfor

% endif
// You can configure optional parameters by calling the respective setters at will, and
// execute the final call using `${action_name}(${action_args and '...' or ''})`.
% if optional_props:
// ${random_value_warning}
% endif
let result = hub.${mangle_ident(resource)}().${mangle_ident(method)}(${required_args})\
% for p in optional_props:
% if p.get('skip_example', False):
<% continue %>
% endif

<%block  filter="indent_by(13)">\
.${mangle_ident(setter_fn_name(p))}(${rvfrt(p.name, p)})\
</%block>\
% endfor

${'.' + action_name | indent_by(13)}(${action_args}).await;
% if handle_result:

match result {
    Err(e) => match e {
        // The Error enum provides details about what exactly happened.
        // You can also just use its `Debug`, `Display` or `Error` traits
         Error::HttpError(_)
        |Error::Io(_)
        |Error::MissingAPIKey
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
    },
    Ok(res) => println!("Success: {:?}", res),
}
% endif
</%block>
</%block>\
</%def>


## create an entire 'api.terms.action' method
###############################################################################################
###############################################################################################
<%def name="_action_fn(c, resource, method, m, params, request_value, parts, doit_without_upload = False)">\
<%
    import os.path
    join_url = lambda b, e: b.strip('/') + e
    if doit_without_upload:
        media_params = []
    else:
        media_params = method_media_params(m)

    type_params = ''
    where = ''
    qualifier = 'pub '
    add_args = ''
    rtype = 'client::Result<hyper::Response<hyper::body::Body>>'
    response_schema = method_response(c, m)

    supports_download = m.get('supportsMediaDownload', False);
    reserved_params = []
    if response_schema:
        if not supports_download:
            reserved_params = ['alt']
        rtype = 'client::Result<(hyper::Response<hyper::body::Body>, %s)>' % (response_schema.id)

    mtype_param = 'RS'

    possible_urls = [m.path]
    simple_media_param = None
    resumable_media_param = None
    if media_params:
        type_params = '<%s>' % mtype_param
        qualifier = ''
        where = '\n\t\twhere ' + mtype_param + ': client::ReadSeek'
        add_args = (', mut reader: %s, reader_mime_type: mime::Mime' % mtype_param) + ", protocol: client::UploadProtocol"
        for p in media_params:
            if p.protocol == SIMPLE:
                simple_media_param = p
            elif p.protocol == RESUMABLE:
                resumable_media_param = p
    # end handle media params

    if doit_without_upload:
        action_fn = qualifier + 'async fn ' + "doit_without_upload" + type_params + '(mut self)' + ' -> ' + rtype + where
    else:
        action_fn = qualifier + 'async fn ' + api.terms.action + type_params + ('(mut self%s)' % add_args) + ' -> ' + rtype + where

    field_params = [p for p in params if p.get('is_query_param', True)]

    paddfields = 'self.' + api.properties.params

    delegate = 'self.' + property(DELEGATE_PROPERTY_NAME)
    delegate_finish = 'dlg.finished'
    auth_call = 'self.hub.auth'

    default_scope = method_default_scope(m)

    # s = '{foo}' -> ('{foo}', 'foo') -> (find_this, replace_with)
    seen = set()
    replacements = list()
    all_required_param_name = set(p.name for p in params if is_required_property(p))
    MULTI_SLASH = 'multi-slash-prefix'
    URL_ENCODE = 'url-encode'

    READER_SEEK = "let size = reader.seek(io::SeekFrom::End(0)).unwrap();\nreader.seek(io::SeekFrom::Start(0)).unwrap();\n"
    if media_params:
        max_size = media_params[0].max_size
        if max_size > 0:
            READER_SEEK += "if size > %i {\n\treturn Err(client::Error::UploadSizeLimitExceeded(size, %i))\n}" % (max_size, max_size)

    special_cases = set()
    for possible_url in possible_urls:
        for s in re_find_replacements.findall(possible_url):
            if s in seen: continue
            seen.add(s)
            sn = s[1:-1]

            # NOTE: We only handle the cases that are actually used in the schemas. If this shouldn't
            # be worth it anymore (i.e. too many cases), then we should use a uri-template library
            # to handle this at runtime, possibly, or use a python uri-template library, to more easily
            # handle the required cases. Whatever is less work, I guess.
            if sn.startswith('/') and sn.endswith('*'):
                sn = sn[1:-1]
                special_cases.add(MULTI_SLASH)
            elif sn.startswith('+'):
                sn = sn[1:]
                special_cases.add(URL_ENCODE)
            assert sn in all_required_param_name, "Expected param '%s' to be in required parameter list for substitution" % sn
            replacements.append((s, sn))
        # end for each found substitution
        # Assure we can substitue everything
        for s, d in replacements:
            possible_url = possible_url.replace(s, d)
        assert '{' not in possible_url, "Failed to replace all fields in '%s', have to parse expressions" % possible_url
    # end for each possible url
    del seen
%>
    % if doit_without_upload:
    /// Perform the operation you have build so far, but without uploading. This is used to e.g. renaming or updating the description for a file
    % else:
    /// Perform the operation you have build so far.
    % endif
    ${action_fn} {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = ${delegate}.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "${m.id}",
                               http_method: ${method_name_to_variant(m.httpMethod)} });

        ## TODO: Should go into validation function?
        ## Additional params - may not overlap with optional params
        for &field in [${', '.join(enclose_in('"', reserved_params + [p.name for p in field_params]))}].iter() {
            if ${paddfields}.contains_key(field) {
                ${delegate_finish}(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(${len(params) + len(reserved_params)} + ${paddfields}.len());
<%
    if media_params and 'mediaUpload' in m:
        upload_type_map = dict()
        for mp in media_params:
            if mp.protocol == SIMPLE:
                upload_type_map[mp.protocol] = m.mediaUpload.protocols.simple.multipart and 'multipart' or 'media'
                break
        # for each media param
    # end build media param map
%>\
        % for p in field_params:
<%
    pname = 'self.' + property(p.name)    # property identifier
    to_string_impl = string_impl(p)
%>\
        ## parts can also be derived from the request, but we do that only if it's not set
        % if p.name == 'part' and request_value:
        % if is_repeated_property(p):
        if ${pname}.is_empty() {
            ${pname}.push(self.${property(REQUEST_VALUE_PROPERTY_NAME)}.to_parts());
        }
        % else:
        % if not is_required_property(p):
        if ${pname}.is_none() {
            ${pname} = Some(self.${property(REQUEST_VALUE_PROPERTY_NAME)}.to_parts());
        }
        % else:
        if ${pname}.len() == 0 {
            ${pname} = self.${property(REQUEST_VALUE_PROPERTY_NAME)}.to_parts();
        }
        % endif ## not is_required_property(p)
        % endif is_repeated_property(p):
        % endif ## p.name == 'part' and request_value:
        % if p.get('repeated', False):
        if ${pname}.len() > 0 {
            for f in ${pname}.iter() {
                params.push("${p.name}", ${to_string_impl("f")});
            }
        }
        % elif not is_required_property(p):
        if let Some(value) = ${pname}.as_ref() {
            params.push("${p.name}", ${to_string_impl("value")});
        }
        % else:
        params.push("${p.name}", ${to_string_impl(pname)});
        % endif
        % endfor

        params.extend(${paddfields}.iter());

        % if response_schema:
        % if supports_download:
        let (alt_field_missing, enable_resource_parsing) = {
            if let Some(value) = params.get("alt") {
                (false, value == "json")
            } else {
                (true, true)
            }
        };
        if alt_field_missing {
            params.push("alt", "json");
        }
        % else:
        params.push("alt", "json");
        % endif ## supportsMediaDownload
        % endif ## response schema
        % if media_params:
        let (mut url, upload_type) =
            % for mp in media_params:
            % if loop.first:
            if \
            % else:
else if \
            % endif
protocol == ${PROTOCOL_TYPE_MAP[mp.protocol]} {
                (self.hub._root_url.clone() + "${mp.path.lstrip('/')}", "${upload_type_map.get(mp.protocol, mp.protocol)}")
            } \
            % endfor
else {
                unreachable!()
            };
        params.push("uploadType", upload_type);
        % else:
        let mut url = self.hub._base_url.clone() + "${m.path}";
        % endif
        % if not default_scope:
        % if no_auth is UNDEFINED:
        <%
            assert 'key' in parameters, "Expected 'key' parameter if there are no scopes"
        %>
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                ${delegate_finish}(false);
                return Err(client::Error::MissingAPIKey)
            }
        }
        % endif
        % else:
        if self.${api.properties.scopes}.is_empty() {
            self.${api.properties.scopes}.insert(${scope_url_to_variant(name, default_scope, fully_qualified=True)}.as_ref().to_string());
        }
        % endif

        ## Handle URI Templates
        % if replacements:
        for &(find_this, param_name) in [${', '.join('("%s", "%s")' % r for r in replacements)}].iter() {
            url = params.uri_replacement(url, param_name, find_this, ${"true" if URL_ENCODE in special_cases else "false"});
        }
        ## Remove all used parameters
        {
            let to_remove = [${', '.join(reversed(['"%s"' % r[1] for r in replacements]))}];
            params.remove_params(&to_remove);
        }
        % endif

        let url = params.parse_with_url(&url);

        % if request_value:
        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self.${property(REQUEST_VALUE_PROPERTY_NAME)}).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
        % endif

        % if resumable_media_param:
        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;
        % endif

        loop {
            % if default_scope:
            let token = match ${auth_call}.get_token(&self.${api.properties.scopes}.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            ${delegate_finish}(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            % endif
            % if request_value:
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            % endif
            let mut req_result = {
            % if resumable_media_param:
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    Ok(hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Location", upload_url.as_ref().unwrap().clone())
                        .body(hyper::body::Body::empty())
                        .unwrap())
                } else {
            % endif
<%block filter="indent_by(resumable_media_param and 4 or 0)">\
            % if request_value and simple_media_param:
                let mut mp_reader: client::MultiPartReader = Default::default();
                let (mut body_reader, content_type) = match protocol {
                    ${PROTOCOL_TYPE_MAP[simple_media_param.protocol]} => {
                        mp_reader.reserve_exact(2);
                        ${READER_SEEK | indent_all_but_first_by(5)}
                        mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                 .add_part(&mut reader, size, reader_mime_type.clone());
                        (&mut mp_reader as &mut (dyn io::Read + Send), client::MultiPartReader::mime_type())
                    },
                    _ => (&mut request_value_reader as &mut (dyn io::Read + Send), json_mime_type.clone()),
                };
            % endif
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(${method_name_to_variant(m.httpMethod)})
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                % if default_scope:
                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }
                % endif

                % if resumable_media_param:
                upload_url_from_server = true;
                if protocol == ${PROTOCOL_TYPE_MAP[resumable_media_param.protocol]} {
                    req_builder = req_builder.header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                }
                % endif

                % if request_value:
                    % if not simple_media_param:
                        let request = req_builder
                        .header(CONTENT_TYPE, json_mime_type.to_string())
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()))\
                    % else:
                        let mut body_reader_bytes = vec![];
                        body_reader.read_to_end(&mut body_reader_bytes).unwrap();
                        let request = req_builder
                            .header(CONTENT_TYPE, content_type.to_string())
                            .body(hyper::body::Body::from(body_reader_bytes))\
                    % endif ## not simple_media_param
                % else:
                    % if simple_media_param:
                        let request = if protocol == ${PROTOCOL_TYPE_MAP[simple_media_param.protocol]} {
                            ${READER_SEEK | indent_all_but_first_by(4)}
                            let mut bytes = Vec::with_capacity(size as usize);
                            reader.read_to_end(&mut bytes)?;
                            req_builder.header(CONTENT_TYPE, reader_mime_type.to_string())
                                     .header(CONTENT_LENGTH, size)
                                     .body(hyper::body::Body::from(bytes))
                        } else {
                            req_builder.body(hyper::body::Body::from(Vec::new()))
                        }\
                    % else:
                        let request = req_builder
                        .body(hyper::body::Body::empty())\
                    % endif
                % endif
;

                client.request(request.unwrap()).await

</%block>\
                % if resumable_media_param:
            }
                % endif
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    ${delegate_finish}(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d).await;
                            continue;
                        }

                        ${delegate_finish}(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
                        }
                    }
                    % if resumable_media_param:
                    if protocol == ${PROTOCOL_TYPE_MAP[resumable_media_param.protocol]} {
                        ${READER_SEEK | indent_all_but_first_by(6)}
                        let upload_result = {
                            let url_str = &res.headers().get("Location").expect("LOCATION header is part of protocol").to_str().unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            client::ResumableUploadHelper {
                                client: &self.hub.client,
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &${auth_call},
                                user_agent: &self.hub._user_agent,
                                // TODO: Check this assumption
                                auth_header: format!("Bearer {}", token.ok_or_else(|| client::Error::MissingToken("resumable upload requires token".into()))?.as_str()),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload().await
                        };
                        match upload_result {
                            None => {
                                ${delegate_finish}(false);
                                return Err(client::Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                ## Do not ask the delgate again, as it was asked by the helper !
                                ${delegate_finish}(false);
                                return Err(client::Error::HttpError(err))
                            }
                            ## Now the result contains the actual resource, if any ... it will be
                            ## decoded next
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    ## delegate was called in upload() already - don't tell him again
                                    dlg.store_upload_url(None);
                                    ${delegate_finish}(false);
                                    return Err(client::Error::Failure(res))
                                }
                            }
                        }
                    }
                    % endif
                % if response_schema:
                    ## If 'alt' is not json, we cannot attempt to decode the response
                    let result_value = \
                    % if supports_download:
if enable_resource_parsing \
                    % endif
{
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    }\
                    % if supports_download:
 else { (res, Default::default()) }\
                    % endif
;
                % else:
                    let result_value = res;
                % endif

                    ${delegate_finish}(true);
                    return Ok(result_value)
                }
            }
        }
    }

    % for p in media_params:
    ${p.description | rust_doc_sanitize(documentationLink), rust_doc_comment, indent_all_but_first_by(1)}
    ///
    % for item_name, item in p.info.items():
    /// * *${split_camelcase_s(item_name)}*: ${isinstance(item, (list, tuple)) and put_and(enclose_in("'", item)) or str(item)}
    % endfor
    pub async fn ${upload_action_fn(api.terms.upload_action, p.type.suffix)}<${mtype_param}>(self, ${p.type.arg_name}: ${mtype_param}, mime_type: mime::Mime) -> ${rtype}
                where ${mtype_param}: client::ReadSeek {
        self.${api.terms.action}(${p.type.arg_name}, mime_type, ${PROTOCOL_TYPE_MAP[p.protocol]}).await
    }
    % endfor
</%def>
