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

# Note: running `do_checks.sh` may need a Rust version that is higher than the one we can use
# for compilation. In particular, at the time of writing this, installing the latest cargo-deny
# requires Rust 1.88.
# TODO: put it elsewhere?
RUST_VERSION_FOR_CHECKS = "1.88.0"


def get_rust_version_from_cargo_toml(cargo_toml_root):
    version = cargo_toml_root["package"]["rust-version"]

    if len(version.split('.')) == 2:
        version = version + '.0'

    return version


def main():
    parser = argparse.ArgumentParser(
        # Use a bigger max_help_position, so that each parameter's help fits into one line.
        formatter_class=lambda prog: argparse.HelpFormatter(prog, max_help_position=30)
    )
    mutex_group = parser.add_mutually_exclusive_group(required=True)
    mutex_group.add_argument(
        '--rust-version',
        action='store_true',
        help='extract Rust version from Cargo.toml; this is the version that should be used for compilation'
    )
    mutex_group.add_argument(
        '--rust-version-for-checks',
        action='store_true',
        help='return the Rust version needed to run do_checks.sh'
    )
    args = parser.parse_args()

    with open(ROOT_CARGO_TOML, "rb") as file:
        cargo_toml_root = tomllib.load(file)

    if args.rust_version:
        result = get_rust_version_from_cargo_toml(cargo_toml_root)
        print(result)
    elif args.rust_version_for_checks:
        print(RUST_VERSION_FOR_CHECKS)


if __name__ == "__main__":
    main()
