from typing import Any
from .rust_type import RustType
from .types import Base
from .util import Context, UNUSED_TYPE_MARKER, schema_markers, canonical_type_name, remove_invalid_chars_in_ident, \
    singular, activity_split


def is_property_enum(s: dict) -> bool:
    enum = s.get("enum")
    enum_description = s.get("enumDescriptions")
    if enum is None:
        return False
    if enum_description is None:
        print("could not find enumDescriptions for enum in s", s)
    return True


def get_enum_type(schema_name: str, property_name: str) -> RustType:
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


def get_enum_if_is_enum(k, property_name: str, property_value: dict) -> RustType | None:
    if property_value is None:
        return None
    if is_property_enum(property_value):
        return get_enum_type(k, property_name)

    return get_enum_if_is_enum(k, property_name, _get_inner_enum(property_value))


def _get_inner_enum(pv: dict):
    items = pv.get('items')
    if items and is_property_enum(items):
        return items
    additional = pv.get('additionalProperties')
    if additional and is_property_enum(additional):
        return additional

    return None



def find_enums_in_context(c: Context) -> list[tuple[str, Any, RustType, Any]]:
    enums:  dict[tuple, tuple[str, Any, RustType, Any]] = {}
    for name, s in c.schemas.items():
        if UNUSED_TYPE_MARKER in schema_markers(s, c, transitive=True):
            continue

        properties = s.get('properties')
        if not properties:
            continue

        try:
            p = properties.to_dict()
            vals = p.values()
            keys = p.keys()

            for pv, pk in zip(vals, keys):
                enum = get_enum_if_is_enum(name, pk, pv)
                if enum:
                    enums[(name, pk)] = (name, pk, enum, pv)
        except TypeError as e:
            print('exception in find_enums_in_context:', e)
            print('name:', name)
            print('s:', s)
            print('props:', properties)
            print('props type:', type(properties))
            raise e

    for name, v in c.fqan_map.items():
        # print(k)
        name = activity_split(name)[1]
        if v.get('parameters'):
            for pk, pv in v.parameters.items():
                enum = get_enum_if_is_enum(name, pk, pv)

                if enum:
                    enums[(name, pk)] = (name, pk, enum, pv)

    return list(enums.values())


def to_enum_variant_name(name: str) -> str:
    c_name = canonical_type_name(name)
    c_name = remove_invalid_chars_in_ident(c_name)
    return c_name
