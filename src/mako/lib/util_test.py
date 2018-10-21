#!/usr/bin/env python

import unittest


from util import to_api_version, library_name, re_find_replacements


class UtilsTest(unittest.TestCase):

    def test_to_version_ok(self):
        for v, want in (('v1.3', '1d3'),
                        ('v1', '1'),
                        ('directory_v1', '1_directory'),
                        ('directory_v1.3', '1d3_directory'),
                        ('v1beta2', '1_beta2'),
                        ('v1sandbox', '1_sandbox'),
                        ('v2.0', '2'),
                        ('v2.0.1', '2d0d1'),
                        ('v0.0', '0'),
                        ('v0.1.0', '0d1'),
                        ('v2.0beta3', '2_beta3'),
                        ('alpha', 'alpha'),
                        ('beta', 'beta'),
                        ('vm_beta', 'vm_beta')):
            res = to_api_version(v)
            self.assertEqual(res, want)

    def test_to_version_fail(self):
        for iv in ('some_branch_name', '1.3'):
            with self.assertRaises(AssertionError):
                to_api_version(iv)

    def test_library_name(self):
        for v, want in (('v1', 'oauth2_v1'),
                        ('v1.4', 'oauth2_v1d4'),
                        ('alpha', 'oauth2_alpha'),
                        ('beta', 'oauth2_beta'),
                        ('vm_beta', 'oauth2_vm_beta')):
            res = library_name('oauth2', v)
            self.assertEqual(res, want)

    def test_url_substitution(self):
        url = "https://www.googleapis.com/resumable/upload/groups/v1/groups/{groupId}/{foo}/archive"
        ms = list(re_find_replacements.finditer(url))
        self.assertEqual(len(ms), 2)
        self.assertEqual(ms[0].group(0), '{groupId}')
        self.assertEqual(ms[1].group(0), '{foo}')

        url = "customer/{customerId}/orgunits{/orgUnitPath*}"
        ms = list(re_find_replacements.findall(url))
        self.assertEqual(len(ms), 2)
        self.assertEqual(ms[0], '{customerId}')
        self.assertEqual(ms[1], '{/orgUnitPath*}')

        url = "{+project}/subscriptions"
        ms = list(re_find_replacements.findall(url))
        self.assertEqual(len(ms), 1)
        self.assertEqual(ms[0], '{+project}')


def main():
    unittest.main()


if __name__ == "__main__":
    main()
