from typing import Any
from .rust_type import RustType
from .types import Base
from .util import Context, UNUSED_TYPE_MARKER, schema_markers, canonical_type_name, remove_invalid_chars_in_ident, \
    singular, activity_split, items


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
        schema = remove_invalid_chars_in_ident(schema_name)
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

    element = enum.get(key)
    if element is not None:
        return element

    nested = get_from_enum(enum.get('items'), key)
    if nested:
        return nested
    nested = get_from_enum(enum.get('additionalProperties'), key)
    if nested:
        return nested

    if key != 'default':  # just a debugging help
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
    enums: dict[RustType, tuple[str, Any, RustType, Any]] = {}
    for name, s in items(c.schemas):
        if UNUSED_TYPE_MARKER in schema_markers(s, c, transitive=True):
            continue

        properties = s.get('properties')
        if not properties:
            continue

        for pk, pv in items(properties):
            add_to_enums_if_enum(name, pk, pv, enums)

    for name, v in items(c.fqan_map):
        name = activity_split(name)[1]
        parameters = v.get('parameters')
        if parameters:
            for pk, pv in items(parameters):
                add_to_enums_if_enum(name, pk, pv, enums)

    return list(enums.values())


def add_to_enums_if_enum(schema_name, property_name, property_value,
                         enums: dict[RustType, tuple[str, Any, RustType, Any]]):
    enum = get_enum_if_is_enum(schema_name, property_name, property_value)
    if enum:
        existing_enum = enums.get(enum)
        if existing_enum:
            if existing_enum[2] != enum:
                print('WARNING: duplicate enum entry. ', enum.name, schema_name, property_name, property_value)
                print('existing enum:                 ', existing_enum[2].name, existing_enum[0], existing_enum[1], existing_enum[3])
            return

        enums[enum] = (schema_name, property_name, enum, property_value)


def to_enum_variant_name(name: str) -> str:
    c_name = canonical_type_name(name)
    c_name = remove_invalid_chars_in_ident(c_name)
    return c_name
