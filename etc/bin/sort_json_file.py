#!/usr/bin/env python

# Sort JSON file and print in fixed format
# This lowers git diffs when a file has only changed by whitespace.


import argparse
import json
import os


def main(json_file_path, skip_missing_file):
    if not os.path.isfile(json_file_path) and skip_missing_file:
        return

    with open(json_file_path, "r") as fh:
        loaded_json = json.load(fh)

    with open(json_file_path, "w") as fh:
        json.dump(loaded_json, fh, indent=4, sort_keys=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Sort and format JSON file.")
    parser.add_argument(
        "json_file", metavar="FILE", type=str, help="JSON file to sort in place."
    )
    parser.add_argument(
        "--skip-missing-file",
        default=False,
        action="store_true",
        help="Do not fail on missing file.",
    )

    args = parser.parse_args()
    main(args.json_file, args.skip_missing_file)
