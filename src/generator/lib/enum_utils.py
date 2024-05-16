from typing import Any
from .rust_type import RustType
from .types import Base
from .util import Context, UNUSED_TYPE_MARKER, schema_markers, canonical_type_name, remove_invalid_chars_in_ident, \
    singular, activity_split


def is_property_enum(s: dict) -> bool:
    enum = s.get("enum")
    if enum is None:
        return False
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


def get_enum_variants(enum: dict) -> list[str]:
    return get_from_enum(enum, 'enum')


def get_enum_variants_descriptions(enum: dict) -> list[str]:
    return get_from_enum(enum, 'enumDescriptions')


def get_enum_default(enum: dict) -> str | None:
    return get_from_enum(enum, 'default')


def get_from_enum(enum: dict, key: str) -> list[Any] | None:
    if enum is None:
        return None

    variants = enum.get(key)
    if variants is not None:
        return variants

    nested_enum = enum.get('items')
    if nested_enum:
        return get_from_enum(nested_enum, key)
    nested_enum = enum.get('additionalProperties')
    if nested_enum:
        return get_from_enum(nested_enum, key)

    if key != 'default':
        print(f"could not find key '{key}' in enum:", enum)
    return None


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

        for pk, pv in properties.items():
            enum = get_enum_if_is_enum(name, pk, pv)
            if enum:
                enums[(name, pk)] = (name, pk, enum, pv)

    for name, v in c.fqan_map.items():
        name = activity_split(name)[1]
        parameters = v.get('parameters')
        if parameters:
            for pk, pv in parameters.items():
                enum = get_enum_if_is_enum(name, pk, pv)

                if enum:
                    enums[(name, pk)] = (name, pk, enum, pv)

    return list(enums.values())


def to_enum_variant_name(name: str) -> str:
    c_name = canonical_type_name(name)
    c_name = remove_invalid_chars_in_ident(c_name)
    return c_name
