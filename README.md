# SPIR-V grammar rules

This is a proof-of-concept on how the rules currently only expressed in the _description_ section of each operand of the [SPIR-V Specification]() could be expressed in the machine readable grammar as well.


### Interesting parts

The `rules` subdirectory is structured exactly like the `include/spirv` directory of the original [SPIR-V-Headers](https://github.com/KhronosGroup/SPIRV-Headers) repository.
It contains the rules for `*.grammar.json` of the SPIR-V-Headers repository.

The Rust program can patch the original `*.grammar.json` file with the correlating `*.grammar-rules.json` and outputs a combined `*.grammar.json` file. 



### Contributing

All contributions are welcome. However I ask for one thing:

If you add rules to an operation that has no rules _at-all_ at the moment. Please take the time to add _all_ rules. Otherwise tracking unimplemented ops becomes a nightmare.



### License

None(?): I hope there there might be an official version of this, or this will be patched into the official SPIR-V spec at some point.
