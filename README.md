# SPIR-V grammar rules

This is a proof-of-concept on how the rules currently only expressed in the _description_ section of each operand of the [SPIR-V Specification]() could be expressed in the machine readable grammar as well.


### Interesting parts

The `rules` subdirectory is structured exactly like the `include/spirv` directory of the original [SPIR-V-Headers](https://github.com/KhronosGroup/SPIRV-Headers) repository.
It contains the rules for `*.grammar.json` of the SPIR-V-Headers repository.

## Libs
There is a [serde](https://crates.io/crates/serde) based parser for the JSON rules file in `crates/spirv-grammar-rules`.

## Executables

### parse-rules.rs

Parses the first argument, assuming it's a rules file, and prints out the parsed rules. Call from the root via

`cargo run --example parse_rules -- rules/1.2/spirv.core.grammar-rules.json`

### generate-rules-file

Takes a grammar file and generates a rules template file. Call from root via

`cargo run --bin generate-rules-file -- spirv-headers/SPIRV-Headers/include/spirv/1.2/spirv.core.grammar.json`

It'll take care of creating a similar operand-naming (if possible) as well as matching the opcodes and opnames.
Finally, it will add all _well-known_ rules to the `rule_types` table.


### Contributing

All contributions are welcome. I ask for one thing:

If you add rules to an operation that has no rules _at-all_ at the moment, please take the time to add _all_ rules. Otherwise, tracking unimplemented ops becomes a nightmare.



### License

MIT / Apache-2.0 dual license
