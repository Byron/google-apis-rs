<%namespace name="util" file="../../lib/util.mako"/>\
<%
    from util import (hash_comment, new_context, method_default_scope, indent_all_but_first_by)
    from cli import (subcommand_md_filename, new_method_context, SPLIT_START, SPLIT_END, pretty, SCOPE_FLAG,
                     mangle_subcommand, is_request_value_property, FIELD_SEP)

    c = new_context(schemas, resources, context.get('methods'))
%>\
% for resource in sorted(c.rta_map.keys()):
% for method in sorted(c.rta_map[resource]):
<%
    mc = new_method_context(resource, method, c)
%>\
${SPLIT_START} ${subcommand_md_filename(resource, method)}
% if mc.m.description:
${mc.m.description}
% endif # show method description
% if mc.m.get('scopes'):
# Scopes

You will need authorization for \
% if len(mc.m.scopes) > 1:
at least one of the following scopes to make a valid call:

% for s in mc.m.scopes:
* *${s}*
% endfor
% else:
the *${mc.m.scopes[0]}* scope to make a valid call.
% endif # len(scopes) > 1

If unset, the scope for this method defaults to *${method_default_scope(mc.m)}*.
You can set the scope for this method like this: `${util.program_name()} --${SCOPE_FLAG} <scope> ${mangle_subcommand(resource)} ${mangle_subcommand(method)} ...`
% endif # have method scopes
<%
    rprops = [p for p in mc.required_props if not is_request_value_property(mc, p)]
%>\
% if rprops:
# Required Scalar ${len(rprops) > 1 and 'Arguments' or 'Argument'}
% for p in rprops:
* **<${mangle_subcommand(p.name)}\>**
    - ${p.get('description') or 'No description provided' | indent_all_but_first_by(2)}
% endfor  # each required property (which is not the request value)
% endif # have required properties
% if mc.request_value:
# Required Request Value

The request value is a data-structure with various fields. Each field may be a simple scalar or another data-structure.
In the latter case it is advised to set the field-cursor to the data-structure's field to specify values more concisely.

For example, a structure like this:
```
"scalar_int": 5,
"struct": {
    "scalar_float": 2.4
    "sub_struct": {
        "strings": ["baz", "bar"],
        "mapping": HashMap,
    }
}
"scalar_str": "foo",
```

can be set completely with the following arguments. Note how the cursor position is adjusted the respective fields:
```
-r scalar_int=2 -r struct -r scalar_float=4.2 -r sub_struct -r strings=first -r strings=second -r mapping=key=value -r ${FIELD_SEP} -r scalar_str=other
```

* The cursor position is always set relative to the current one, unless the field name starts with the '${FIELD_SEP}' character. Fields can be nested such as in `-r f${FIELD_SEP}s${FIELD_SEP}o` .
* **Lists** are always appended to, in the example, the list at `struct${FIELD_SEP}sub_struct${FIELD_SEP}strings` will have the value `["first", "second"]`.
* **Mappings** are set using the `key=value` form.
* You can also set nested fields without setting the cursor explicitly. For example, to set the mapping from the root, you would specify `-r struct${FIELD_SEP}sub_struct${FIELD_SEP}mapping=foo=bar`. In case the cursor is not at the root, you may explicitly drill down from the root using a leading '${FIELD_SEP}' character.

% endif # have request value
${SPLIT_END}
% endfor # each method
% endfor # each resource
