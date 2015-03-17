<%!
    from util import (put_and, rust_test_fn_invisible, rust_doc_test_norun, rust_doc_comment,
                      rb_type, mb_type, singular, hub_type, to_fqan, indent_all_but_first_by,
                      method_params, activity_rust_type, mangle_ident, activity_input_type, get_word,
                      split_camelcase_s, property, is_pod_property, TREF, IO_REQUEST, 
                      schema_to_required_property, rust_copy_value_s, is_required_property,
                      hide_rust_doc_test, build_all_params, REQUEST_VALUE_PROPERTY_NAME, organize_params, 
                      indent_by, to_rust_type, rnd_arg_val_for_type, extract_parts, mb_type_params_s,
                      hub_type_params_s, method_media_params, enclose_in, mb_type_bounds, method_response,
                      METHOD_BUILDER_MARKERT_TRAIT, pass_through, markdown_rust_block, parts_from_params,
                      DELEGATE_PROPERTY_NAME, struct_type_bounds_s, supports_scopes, scope_url_to_variant,
                      re_find_replacements)

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

    def is_repeated_property(p):
        return p.get('repeated', False)

    def setter_fn_name(p):
        fn_name = p.name
        if is_repeated_property(p):
            fn_name = 'add_' + fn_name
        return fn_name

%>\
<%namespace name="util" file="util.mako"/>\
<%namespace name="lib" file="lib.mako"/>\

## Creates a method builder type
###############################################################################################
###############################################################################################
<%def name="new(resource, method, c)">\
<% 
    hub_type_name = hub_type(schemas,util.canonical_name())
    m = c.fqan_map[to_fqan(c.rtc_map[resource], resource, method)]
    # an identifier for a property. We prefix them to prevent clashes with the setters
    mb_tparams = mb_type_params_s(m)
    ThisType = mb_type(resource, method) + mb_tparams

    params, request_value = build_all_params(c, m)

    part_prop, parts = parts_from_params(params)
    part_desc = make_parts_desc(part_prop)
    parts = get_parts(part_prop)
%>\
% if 'description' in m:
${m.description | rust_doc_comment}
///
% endif
/// A builder for the *${method}* method supported by a *${singular(resource)}* resource.
/// It is not used directly, but through a `${rb_type(resource)}`.
///
% if part_desc:
${part_desc | rust_doc_comment}
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
% endif
% endif
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
    % if supports_scopes(auth):
## We need the scopes sorted, to not unnecessarily query new tokens
    ${api.properties.scopes}: BTreeMap<String, ()>
    % endif
}

impl${mb_tparams} cmn::${METHOD_BUILDER_MARKERT_TRAIT} for ${ThisType} {}

impl${mb_tparams} ${ThisType} where ${', '.join(mb_type_bounds())} {

${self._action_fn(c, resource, method, m, params, request_value, parts)}\

## SETTERS ###############
% for p in params:
${self._setter_fn(resource, method, m, p, part_prop, ThisType, c)}\
% endfor

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own 
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known paramters
    /// which have their own setter method. If done anyway, the request will fail.
    % if parameters:
    /// 
    /// # Additional Parameters
    ///
    % for opn, op in parameters.iteritems():
    /// * *${opn}* (${op.location}-${op.type}) - ${op.description}
    % endfor
    % endif
    pub fn param<T>(mut self, name: T, value: T) -> ${ThisType}
                                                        where T: Str {
        self.${api.properties.params}.insert(name.as_slice().to_string(), value.as_slice().to_string());
        self
    }

    % if supports_scopes(auth):
    /// Identifies the authorization scope for the method you are building.
    /// 
    /// Use this method to actively specify which scope should be used, instead of relying on the 
    /// automated algorithm which simply prefers read-only scopes over those who are not.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// 
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T>(mut self, scope: T) -> ${ThisType} 
                                                        where T: Str {
        self.${api.properties.scopes}.insert(scope.as_slice().to_string(), ());
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
    /// Sets the *${split_camelcase_s(p.name)}* ${get_word(p, 'location')}property to the given value.
    ///
    % if show_part_info(m, p):
    /// Even though the *parts* list is automatically derived from *Resource* passed in 
    /// during instantiation and indicates which values you are passing, the response would contain the very same parts.
    /// This may not always be desirable, as you can obtain (newly generated) parts you cannot pass in,
    /// like statistics that are generated server side. Therefore you should use this method to specify 
    /// the parts you provide in addition to the ones you want in the response.
    % elif is_required_property(p):
    /// Even though the property as already been set when instantiating this call, 
    /// we provide this method for API completeness.
    % endif
    % if part_desc:
    ///
    ${part_desc | rust_doc_comment, indent_all_but_first_by(1)}
    % endif
    /// 
    % if 'description' in p:
    ${p.description | rust_doc_comment, indent_all_but_first_by(1)}
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
    required_args = request_value and ['&' + rb_name] or []
    for p in required_props:
        # could also just skip the first element, but ... let's be safe
        if request_value and request_value.id == p.get(TREF):
            continue
        v = rvfrt(p.name, p)
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
        action_name = api.terms.upload_action + media_params[index].type.suffix
    else:
        action_name = api.terms.action
    action_args = media_params and media_params[-1].type.example_value or ''

    random_value_warning = "Values shown here are possibly random and not representative !"

    hide_filter = show_all and pass_through or hide_rust_doc_test
    test_block_filter = rust_doc and rust_doc_test_norun or markdown_rust_block
    test_fn_filter = rust_doc and rust_test_fn_invisible or pass_through
%>\
<%block filter="test_block_filter">\
${capture(util.test_prelude) | hide_filter}\
% if request_value:
use ${util.library_name()}::${request_value.id};
% endif
% if handle_result:
use ${util.library_name()}::cmn::Result;
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
let mut ${rb_name}: ${request_value.id} = Default::default();
% for spn, sp in request_value.get('properties', dict()).iteritems():
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
## ${to_rust_type(request_value.id, spn, sp, allow_optionals=False)}
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

${'.' + action_name | indent_by(13)}(${action_args});
% if handle_result:

match result {
    Result::HttpError(err) => println!("HTTPERROR: {:?}", err),
    Result::MissingAPIKey => println!("Missing API Key"),
    Result::MissingToken => println!("Missing Token"),
    Result::Failure(_) => println!("General Failure (Response doesn't print)"),
    Result::FieldClash(clashed_field) => println!("FIELD CLASH: {:?}", clashed_field),
    Result::Success(_) => println!("Success (value doesn't print)"),
}
% endif
</%block>
</%block>\
</%def>


## create an entire 'api.terms.action' method
###############################################################################################
###############################################################################################
<%def name="_action_fn(c, resource, method, m, params, request_value, parts)">\
<%
    import os.path
    join_url = lambda b, e: b.strip('/') + e
    media_params = method_media_params(m)

    type_params = ''
    where = ''
    qualifier = 'pub '
    add_args = ''
    rtype = 'cmn::Result<hyper::client::Response>'
    response_schema = method_response(c, m)

    if response_schema:
        rtype = 'cmn::Result<(hyper::client::Response, %s)>' % (response_schema.id)

    possible_urls = [m.path]
    if media_params:
        stripped = lambda s: s.strip().strip(',')
        qualifier = ''
        for p in media_params:
            type_params += p.type.param + ', '
            where += p.type.param + ': ' + p.type.where + ', '
            add_args += p.type.arg_name + ': ' + ('Option<(%s, u64, mime::Mime)>' % p.type.param) + ', '
            possible_urls.append(p.path)
        # end for each param
        where = ' where ' + stripped(where)
        type_params = '<' + stripped(type_params) + '>'
        add_args = ', ' + stripped(add_args)
    # end handle media params

    action_fn = qualifier + 'fn ' + api.terms.action + type_params + ('(mut self%s)' % add_args) + ' -> ' + rtype + where

    field_params = [p for p in params if p.get('is_query_param', True)]

    paddfields = 'self.' + api.properties.params

    delegate = 'self.' + property(DELEGATE_PROPERTY_NAME)
    delegate_call = delegate + '.as_mut().unwrap()'
    auth_call = 'self.hub.auth.borrow_mut()'
    client = '(*self.hub.client.borrow_mut()).borrow_mut()'

    if supports_scopes(auth):
        all_scopes = sorted(auth.oauth2.scopes.keys())
        default_scope = all_scopes[0]
        if m.httpMethod in ('HEAD', 'GET', 'OPTIONS', 'TRACE'):
            for scope in all_scopes:
                if 'readonly' in scope:
                    default_scope = scope
                    break
            # end for each scope
        # end try to find read-only default scope
    # end handle default scope

    # s = '{foo}' -> ('{foo}', 'foo') -> (find_this, replace_with)
    seen = set()
    replacements = list()
    all_required_param_name = set(p.name for p in params if is_required_property(p))
    for possible_url in possible_urls:
        for s in re_find_replacements.findall(possible_url):
            if s in seen: continue
            seen.add(s)
            sn = s[1:-1]
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
    /// Perform the operation you have build so far.
    ${action_fn} {
        use std::io::Read;
        let mut params: Vec<(&str, String)> = Vec::with_capacity(${len(params)} + ${paddfields}.len());
        % for p in field_params:
<%
    pname = 'self.' + property(p.name)    # property identifier
%>\
        ## parts can also be derived from the request, but we do that only if it's not set
        % if p.name == 'part' and request_value:
        % if not is_required_property(p):
        if ${pname}.is_none() {
            ${pname} = Some(self.${property(REQUEST_VALUE_PROPERTY_NAME)}.to_parts());
        }
        % else:
        if ${pname}.len() == 0 {
            ${pname} = self.${property(REQUEST_VALUE_PROPERTY_NAME)}.to_parts();
        }
        % endif
        % endif
        % if p.get('repeated', False):
        if ${pname}.len() > 0 {
            let mut s = String::new();
            for f in ${pname}.iter() {
                s.push_str(&("/".to_string() + f));
            }
            params.push(("${p.name}", s));
        }
        % elif not is_required_property(p):
        if ${pname}.is_some() {
            params.push(("${p.name}", ${pname}.unwrap().to_string()));
        }
        % else:
        params.push(("${p.name}", ${pname}.to_string()));
        % endif
        % endfor
        ## Additional params - may not overlap with optional params
        for &field in [${', '.join(enclose_in('"', (p.name for p in field_params)))}].iter() {
            if ${paddfields}.contains_key(field) {
                return cmn::Result::FieldClash(field);
            }
        }
        for (name, value) in ${paddfields}.iter() {
            params.push((&name, value.clone()));
        }

        % if media_params:
        let (mut url, protocol) = \
            % for mp in media_params:
            % if loop.first:
if \
            % else:
else if \
            % endif
${mp.type.arg_name}.is_some() {
                ("${join_url(rootUrl, mp.path)}".to_string(), "${mp.protocol}")
            } \
            % endfor
else { 
                unreachable!() 
        };
        params.push(("uploadType", protocol.to_string()));
        % else:
        let mut url = "${baseUrl}${m.path}".to_string();
        % endif
        % if not supports_scopes(auth):
        <% 
            assert 'key' in parameters, "Expected 'key' parameter if there are no scopes"
        %>
        let mut key = ${auth_call}.api_key();
        if key.is_none() && ${delegate}.is_some() {
            key = ${delegate_call}.api_key();
        }
        if key.is_some() {
            params.push(("key", key.unwrap()));
        } else {
            return cmn::Result::MissingAPIKey
        }
        % else:
        if self.${api.properties.scopes}.len() == 0 {
            self.${api.properties.scopes}.insert(${scope_url_to_variant(name, default_scope, fully_qualified=True)}.as_slice().to_string(), ());
        }
        % endif

        % if replacements:
        for &(find_this, param_name) in [${', '.join('("%s", "%s")' % r for r in replacements)}].iter() {
            let mut replace_with: Option<<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        % endif
        
        if params.len() > 0 {
            url.push('?');
            url.push_str(&url::form_urlencoded::serialize(params.iter().map(|t| (t.0, t.1.as_slice()))));
        }

        loop {
            % if supports_scopes(auth):
            let mut token = ${auth_call}.token(self.${api.properties.scopes}.keys());
            if token.is_none() && ${delegate}.is_some() {
                token = ${delegate_call}.token();
            }
            if token.is_none() {
                return cmn::Result::MissingToken
            }
            let auth_header = hyper::header::Authorization(token.unwrap().access_token);
            % endif
            if ${delegate}.is_some() {
                ${delegate_call}.pre_request("${m.id}");
            }
            match ${client}.request(hyper::method::Method::Extension("${m.httpMethod}".to_string()), url.as_slice())
               .header(hyper::header::UserAgent("google-api-rust-client/${cargo.build_version}".to_string()))
               % if supports_scopes(auth):
               .header(auth_header)
               % endif
               % if request_value:
               .header(hyper::header::ContentType(mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, Default::default())))
               .body(json::encode(&self.${property(REQUEST_VALUE_PROPERTY_NAME)}).unwrap().as_slice())
               % endif
               .send() {
                Err(err) => {
                    if ${delegate}.is_some() {
                        match ${delegate_call}.http_error(&err) {
                            oauth2::Retry::Abort => return cmn::Result::HttpError(err),
                            oauth2::Retry::After(d) => {
                                sleep(d);
                                continue;
                            }
                        }
                    } else {
                        return cmn::Result::HttpError(err);
                    }
                }
                Ok(mut res) => {
                    if !res.status.is_success() {
                        if ${delegate}.is_some() {
                            let mut json_err = String::new();
                            res.read_to_string(&mut json_err).ok();
                            let error_info: cmn::JsonServerError = json::decode(&json_err).unwrap();
                            if let oauth2::Retry::After(d) = ${delegate_call}.http_failure(&res, error_info) {
                                sleep(d);
                                continue;
                            }
                        }
                        return cmn::Result::Failure(res)
                    }
                % if response_schema:
                    let mut json_response = String::new();
                    res.read_to_string(&mut json_response).ok();
                    let result_value = (res, json::decode(&json_response).unwrap());
                % else:
                    let result_value = res;
                % endif
                    return cmn::Result::Success(result_value)
                }
            }
        }
    }

    % for p in media_params:
<% 
        none_type = 'None::<(' + p.type.default + ', u64, mime::Mime)>' 
%>\
    ${p.description | rust_doc_comment, indent_all_but_first_by(1)}
    ///
    % for item_name, item in p.info.iteritems():
    /// * *${split_camelcase_s(item_name)}*: ${isinstance(item, (list, tuple)) and put_and(enclose_in("'", item)) or str(item)}
    % endfor
    pub fn ${api.terms.upload_action}${p.type.suffix}<${p.type.param}>(self, ${p.type.arg_name}: ${p.type.param}, size: u64, mime_type: mime::Mime) -> ${rtype}
                where ${p.type.param}: ${p.type.where} {
        self.${api.terms.action}(\
        % for _ in range(0, loop.index):
${none_type}, \
        % endfor
Some((${p.type.arg_name}, size, mime_type)), \
        % for _ in range(loop.index+1, len(media_params)):
${none_type}, \
        % endfor
)
    }
    % endfor
</%def>