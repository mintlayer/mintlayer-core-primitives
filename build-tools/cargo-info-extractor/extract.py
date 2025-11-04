#!/usr/bin/env python3
'''
A simple program that extracts certain info from Cargo.toml and prints it to stdout.
To be used in CI.
'''

import argparse
import pathlib
import tomllib


ROOT_DIR = pathlib.Path(__file__).resolve().parent.parent.parent
ROOT_CARGO_TOML = ROOT_DIR.joinpath("Cargo.toml")


def get_rust_version(cargo_toml_root):
    version = cargo_toml_root["package"]["rust-version"]

    if len(version.split('.')) == 2:
        version = version + '.0'

    return version


def main():
    parser = argparse.ArgumentParser()
    mutex_group = parser.add_mutually_exclusive_group(required=True)
    mutex_group.add_argument('--rust-version', action='store_true', help='extract Rust version')
    args = parser.parse_args()

    with open(ROOT_CARGO_TOML, "rb") as file:
        cargo_toml_root = tomllib.load(file)

    if args.rust_version:
        result = get_rust_version(cargo_toml_root)
        print(result)


if __name__ == "__main__":
    main()
