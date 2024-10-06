#!/usr/bin/env python

import json
import unittest

from generator.lib.util import (
    library_name,
    re_find_replacements,
    to_api_version,
    to_rust_type,
)

from .test_data.discovery_document import DISCOVERY_DOC


class UtilsTest(unittest.TestCase):

    def test_to_version_ok(self):
        for v, want in (
            ("v1.3", "1d3"),
            ("v1", "1"),
            ("directory_v1", "1_directory"),
            ("directory_v1.3", "1d3_directory"),
            ("v1beta2", "1_beta2"),
            ("v1sandbox", "1_sandbox"),
            ("v2.0", "2"),
            ("v2.0.1", "2d0d1"),
            ("v0.0", "0"),
            ("v0.1.0", "0d1"),
            ("v2.0beta3", "2_beta3"),
            ("alpha", "alpha"),
            ("beta", "beta"),
            ("vm_beta", "vm_beta"),
        ):
            res = to_api_version(v)
            self.assertEqual(res, want)

    def test_to_version_fail(self):
        for iv in ("some_branch_name", "1.3"):
            with self.assertRaises(AssertionError):
                to_api_version(iv)

    def test_library_name(self):
        for v, want in (
            ("v1", "oauth2_v1"),
            ("v1.4", "oauth2_v1d4"),
            ("alpha", "oauth2_alpha"),
            ("beta", "oauth2_beta"),
            ("vm_beta", "oauth2_vm_beta"),
        ):
            res = library_name("oauth2", v)
            self.assertEqual(res, want)

    def test_url_substitution(self):
        url = "https://www.googleapis.com/resumable/upload/groups/v1/groups/{groupId}/{foo}/archive"
        ms = list(re_find_replacements.finditer(url))
        self.assertEqual(len(ms), 2)
        self.assertEqual(ms[0].group(0), "{groupId}")
        self.assertEqual(ms[1].group(0), "{foo}")

        url = "customer/{customerId}/orgunits{/orgUnitPath*}"
        ms = list(re_find_replacements.findall(url))
        self.assertEqual(len(ms), 2)
        self.assertEqual(ms[0], "{customerId}")
        self.assertEqual(ms[1], "{/orgUnitPath*}")

        url = "{+project}/subscriptions"
        ms = list(re_find_replacements.findall(url))
        self.assertEqual(len(ms), 1)
        self.assertEqual(ms[0], "{+project}")

    def test_to_rust_type(self):
        full_api_schema = json.loads(DISCOVERY_DOC)

        schemas = full_api_schema["schemas"]

        # Get class
        class_name = None
        property_name = None
        property_value = {"$ref": "Album"}
        rust_type = to_rust_type(
            schemas, class_name, property_name, property_value, allow_optionals=True
        )
        self.assertEqual(rust_type, "Option<Album>")

        # allow_optionals=False
        class_name = None
        property_name = None
        property_value = {"$ref": "Album"}
        rust_type = to_rust_type(
            schemas, class_name, property_name, property_value, allow_optionals=False
        )
        self.assertEqual(rust_type, "Album")

        # Get properties
        test_properties = (
            ("Album", "title", "String"),  # string
            ("Status", "code", "i32"),  # numeric
            ("Album", "mediaItemsCount", "i64"),  # numeric via "count" keyword
            ("Album", "isWriteable", "bool"),  # boolean
            ("Album", "shareInfo", "ShareInfo"),  # reference type
            ("SearchMediaItemsResponse", "mediaItems", "Vec<MediaItem>"),  # array
        )
        for class_name, property_name, expected in test_properties:
            property_value = schemas[class_name]["properties"][property_name]
            rust_type = to_rust_type(
                schemas,
                class_name,
                property_name,
                property_value,
                allow_optionals=False,
            )
            self.assertEqual(
                rust_type,
                expected,
                f"Parsed class: {class_name}, property: {property_name}",
            )

        # items reference
        class_name = "SearchMediaItemsResponse"
        property_name = "mediaItems"
        property_value = schemas[class_name]["properties"][property_name]
        rust_type = to_rust_type(
            schemas, class_name, property_name, property_value, allow_optionals=True
        )
        self.assertEqual(rust_type, "Option<Vec<MediaItem>>")

        # additionalProperties reference
        class_name = "Status"
        property_name = "details"
        property_value = schemas[class_name]["properties"][property_name]
        rust_type = to_rust_type(
            schemas, class_name, property_name, property_value, allow_optionals=True
        )
        self.assertEqual(rust_type, "Option<Vec<HashMap<String, serde_json::Value>>>")


def main():
    unittest.main()


if __name__ == "__main__":
    main()
