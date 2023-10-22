from .rust_type import RustType
from .types import Base
from .util import Context, UNUSED_TYPE_MARKER, schema_markers, canonical_type_name, remove_invalid_chars_in_ident, \
    singular


def is_property_enum(s: dict) -> bool:
    enum = s.get("enum")
    enum_description = s.get("enumDescriptions")
    if enum is None:
        return False
    if enum_description is None:
        print("could not find enumDescriptions for enum in s", s)
    return True


def get_enum_type(schema_name: str, property_name: str, t, s) -> RustType:
    if schema_name is None:
        # print('schema_name was None:', property_name, t)
        schema_name = ""
    if len(schema_name) == 0:
        schema = ""
    else:
        schema = remove_invalid_chars_in_ident(singular(schema_name))
    name = canonical_type_name(schema
                               + remove_invalid_chars_in_ident(canonical_type_name(property_name))
                               + "Enum")
    # print(name, '\n')
    return Base(name)


def _add_enum_value(k, pk, pv, enums: dict):
    if is_property_enum(pv):
        enum_type = get_enum_type(k, pk, None, None)
        # print('enum type:', enum_type)
        enums[enum_type] = (k, pk, enum_type, pv)
        return enums

    inner_pv = _get_inner_enum(pv)
    if inner_pv:
        return _add_enum_value(k, pk, inner_pv, enums)

    return enums


def _get_inner_enum(pv: dict):
    items = pv.get('items')
    if items and is_property_enum(items):
        return items
    additional = pv.get('additionalProperties')
    if additional and is_property_enum(additional):
        return additional

    return None


def _parse_method_id(method_id: str, c: Context) -> str:
    parts = method_id.split('.')
    if len(parts) != 3:
        return method_id
    context = c.rtc_map.get(parts[1])

    if context is None or parts[0] != context:
        print('context was not equal to first part: ', context, parts[0], method_id)
        return method_id

    methods = c.rta_map.get(parts[1])
    if methods is None or parts[2] not in methods:
        print('third part was not in methods', methods, parts[2], method_id)

    return parts[1]


def find_enums_in_context(c: Context) -> list:
    enums = {}
    for k, s in c.schemas.items():
        if UNUSED_TYPE_MARKER not in schema_markers(s, c, transitive=True):
            # properties = s.get('properties')
            # printed_name = False
            if s.properties:
                for pv, pk in zip(s.properties.values(), s.properties.keys()):
                    enums = _add_enum_value(k, pk, pv, enums)

    for k, v in c.fqan_map.items():
        # print(k)
        k = _parse_method_id(k, c)
        if v.parameters:
            for pk, pv in v.parameters.items():
                enums = _add_enum_value(k, pk, pv, enums)
    return list(enums.values())


def to_enum_variant_name(name: str) -> str:
    c_name = canonical_type_name(name)
    c_name = remove_invalid_chars_in_ident(c_name)
    return c_name
