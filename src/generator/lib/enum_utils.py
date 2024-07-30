from typing import Any
from .rust_type import RustType
from .types import Base
from .util import Context, UNUSED_TYPE_MARKER, schema_markers, canonical_type_name, remove_invalid_chars_in_ident, \
    activity_split, items, Enum, EnumVariant


def get_enum_from_dict(name: RustType, s: dict) -> Enum:
    description: str | None = get_from_enum(s, 'description')
    (variants, is_any_variant_deprecated) = _get_enum_variants(s)
    default: EnumVariant | str | None = get_enum_default(s)
    default_variant: EnumVariant | None = None
    if default is not None:
        for variant in variants:
            if variant.name == default:
                default_variant = variant
                break

    return Enum(name, description, variants, default_variant, is_any_variant_deprecated)


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


def _get_enum_variants(enum: dict) -> tuple[list[EnumVariant], bool]:
    variants = get_from_enum(enum, 'enum')
    descriptions = get_from_enum(enum, 'enumDescriptions')
    if not descriptions:
        descriptions = variants

    result = []
    is_any_variant_deprecated = False
    for variant in descriptions:
        if is_enum_variant_deprecated(variant):
            is_any_variant_deprecated = True
            break

    for i in range(len(variants)):
        variant = variants[i]
        description = None
        if descriptions:
            description = descriptions[i]

        if variant is None:
            continue

        is_deprecated = is_enum_variant_deprecated(description)
        deprecation_message = None
        if is_deprecated:
            deprecation_message = description

        name = to_enum_variant_name(variant, not is_any_variant_deprecated)
        result.append(EnumVariant(name, variant, description, is_deprecated, deprecation_message))

    return result, is_any_variant_deprecated


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

    return None


def get_enum_if_is_enum(k, property_name: str, property_value: dict) -> Enum | None:
    if property_value is None:
        return None
    if is_property_enum(property_value):
        return get_enum_from_dict(get_enum_type(k, property_name), property_value)

    return get_enum_if_is_enum(k, property_name, _get_inner_enum(property_value))


def _get_inner_enum(pv: dict):
    nested = pv.get('items')
    if nested and is_property_enum(nested):
        return nested

    nested = pv.get('additionalProperties')
    if nested and is_property_enum(nested):
        return nested

    return None


def find_enums_in_context(c: Context) -> list[Enum]:
    enums: dict[RustType, tuple[str, Any, Enum]] = {}
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

    result = []
    for enum in enums.values():
        result.append(enum[2])

    return result


def add_to_enums_if_enum(schema_name, property_name, property_value,
                         enums: dict[RustType, tuple[str, Any, Enum]]):
    enum: Enum | None = get_enum_if_is_enum(schema_name, property_name, property_value)
    if enum:
        existing_enum = enums.get(enum.ty)
        if existing_enum:
            if existing_enum[2].ty.name != enum.ty.name or existing_enum[2].variants != enum.variants:
                print('='*80)
                print('WARNING: duplicate enum entry. ', enum.ty, schema_name, property_name, 'enum: \n\n', enum)
                print()
                print('existing enum:                 ', existing_enum[2].ty, existing_enum[0], existing_enum[1],
                      'enum: \n\n', existing_enum[2])
                print('=' * 80)
            return

        enums[enum.ty] = (schema_name, property_name, enum)


def to_enum_variant_name(name: str, make_camel_case: bool = True) -> str:
    if make_camel_case:
        name = canonical_type_name(name)

    name = remove_invalid_chars_in_ident(name)
    return name


def is_enum_variant_deprecated(description: str | None) -> bool:
    if description is None:
        return False

    s = description.lower()
    if 'deprecated' in s:
        return True

    return False
