# Mintlayer Core primitive types

Here we have certain primitive types copied from Mintlayer Core and intended for use
in hardware wallets' firmware/apps in `no_std` mode.

Notes:
- The types in this repository are not always identical to those from Mintlayer Core, however
they are encode-compatible with them.
- Ideally, we should get rid of the code duplication and, instead, extract the Mintlayer Core's
"foundational" crates [^1] into a separate repository and make them usable (in a limited way)
in `no_std` mode.

[^1]: iI.e. `common` and `crypto` as well as the utility crates that they depend on - `utils`,
`serialization`, `logging` etc).
