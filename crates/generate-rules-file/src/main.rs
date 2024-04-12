#![feature(string_remove_matches)]
use std::{ffi::OsString, str::FromStr};

use spirv_grammar_rules::{
    serde_json::{self, Value},
    GrammarRules, Instruction,
};

fn main() {
    let src_grammar_file = std::env::args()
        .skip(1)
        .next()
        .expect("Expected a source file argument!");
    let src_path = std::path::PathBuf::from_str(&src_grammar_file).unwrap();
    let grammar_string = std::fs::read_to_string(&src_path).expect("Could not read grammar file!");
    let src_grammar: Value =
        serde_json::from_str(&grammar_string).expect("Could not parse grammar into json");

    let grammar_file_name = src_path
        .file_name()
        .unwrap_or(&OsString::from("grammar.json"))
        .to_str()
        .unwrap()
        .to_owned();
    let rules_file_name = grammar_file_name
        .replace("grammar", "grammar-rules")
        .to_owned();

    //now setup the Grammar rules based on the `src_grammar`, add all the known rules and finally collect all
    //instructions into the rules.
    let mut rules = GrammarRules::new(grammar_file_name);

    let instruction_table = src_grammar
        .get("instructions")
        .expect("Could not get \"instructions\" table from source grammar!");

    fill_instructions(&mut rules.instructions, &instruction_table);

    let rules_file_string =
        serde_json::to_string_pretty(&rules).expect("Could not serialize the rules to JSON");
    std::fs::write(rules_file_name, rules_file_string).expect("Could not write rules to file");
}

fn fill_instructions(instructions: &mut Vec<Instruction>, instruction_array: &Value) {
    if let Value::Array(insts) = instruction_array {
        for inst in insts {
            let mut opname: String = inst.get("opname").unwrap().to_string();
            //Don't ask :D
            opname.remove_matches("\"");
            opname.remove_matches("\'");
            let opcode: u32 = str::parse(&inst.get("opcode").unwrap().to_string()).unwrap();
            let operand_mapping = if let Some(operands) = inst.get("operands") {
                let operand_mapping = if let Value::Array(operands) = operands {
                    let mut mapping = Vec::with_capacity(operands.len());
                    for (opidx, op) in operands.iter().enumerate() {
                        let name = if let Some(opname) = op.get("name") {
                            let mut name = opname.to_string();
                            name.remove_matches("\"");
                            name.remove_matches("\'");
                            name
                        } else {
                            //try to derive a name from the _kind_, otherwise use _unnamed_
                            if let Some(kind) = op.get("kind") {
                                let mut name = format!("{}_{opidx}", kind.to_string());
                                name.remove_matches("\"");
                                name.remove_matches("\'");
                                name
                            } else {
                                format!("Unnamed {opidx}")
                            }
                        };

                        mapping.push(name);
                    }
                    mapping
                } else {
                    Vec::with_capacity(0)
                };

                operand_mapping
            } else {
                Vec::with_capacity(0)
            };

            instructions.push(Instruction {
                opname,
                opcode,
                operand_mapping,
                rules: Vec::with_capacity(0),
            })
        }
    } else {
        panic!("Expected instruction table to be array!");
    }
}
