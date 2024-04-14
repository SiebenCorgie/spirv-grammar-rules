use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

pub use serde;
pub use serde_json;

///Types as defined in 2.2.2 of the unified spec:
///<https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#_types>
//TODO: This should probably be split since a lot of those are overlaping.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Boolean,
    Integer,
    FloatingPoint,
    Numerical,
    Scalar,
    Vector,
    Matrix,
    Array,
    Structure,
    Aggregate,
    Composite,
    Image,
    Sampler,
    SampledImage,
    PhysicalPointer,
    LogicalPointer,
    Concrete,
    Abstract,
    Opaque,
    VariablePointer,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Rule {
    ///Declares that `operand` needs to
    /// have one of `ty`'s base type.
    BaseType {
        operand: String,
        base_types: Vec<Type>,
    },
    ///Declares that `operand` needs to have one of the
    /// concrete(?) types in `ty`.
    TypeConstraint {
        operand: String,
        ty: Vec<Type>,
    },
    ///Declares that the `operand` needs to have the
    ///same type as the result of this instruction.
    ResultEqualType(String),

    ///The amount of components (if a / b are vectors) must be equal.
    ComponentCountEqual {
        a: String,
        b: String,
    },

    ///The component width of `a` and `b` need to be equal.
    ComponentWidthEqual {
        a: String,
        b: String,
    },
    ///The component type of `a` and `b` need to be equal
    ComponentTypeEqual {
        a: String,
        b: String,
    },
    IsSigned {
        operand: String,
        is_signed: bool,
    },

    ComponentWidth {
        operand: String,
        allowed: Vec<u32>,
    },

    ComponentCount {
        operand: String,
        allowed: Vec<u32>,
    },

    ///Unknown rule that needs to be parsed at runtime.
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Instruction {
    pub opname: String,
    ///The opcode of that instruction in the [source_grammar](GrammarRules::source_grammar).
    pub opcode: u32,
    ///Binds names to the operands in the order of the operand definition of the source-grammar's
    /// `operand` field.
    ///
    /// So this _should_ also have the same length.
    pub operand_mapping: Vec<String>,
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GrammarRules {
    ///Name of the grammar file those rules are made for.
    pub source_grammar: String,
    /// list of rules that are used as well as their description.
    pub rule_types: HashMap<String, String>,
    ///All defined instructions and their rules.
    pub instructions: Vec<Instruction>,

    ///lookuptable, that lets us cache the intsruction lookup
    #[serde(skip)]
    #[serde(default)]
    pub lookup_opcode: HashMap<u32, usize>,
}

#[derive(Debug)]
pub enum GrammarError {
    FsError(std::io::Error),
    ParseError(serde_json::Error),
}

impl GrammarRules {
    pub fn new(src_grammar_file: String) -> Self {
        let mut rule_types = HashMap::new();
        rule_types.insert(
            "base_type".to_owned(),
            "Constraints the base type of this operand to be in this set of types".to_owned(),
        );

        rule_types.insert(
            "type_constrain".to_owned(),
            "The operand must be within the set of types as defined in 2.2.2.".to_owned(),
        );

        rule_types.insert(
            "result_equal_type".to_owned(),
            "Signals that an operand needs to have the same type as the result of that instruction"
                .to_owned(),
        );

        GrammarRules {
            source_grammar: src_grammar_file,
            rule_types,
            instructions: Vec::new(),
            lookup_opcode: HashMap::default(),
        }
    }

    pub fn lookup_opcode(&mut self, opcode: u32) -> Option<&Instruction> {
        let index = if let Some(known) = self.lookup_opcode.get(&opcode) {
            *known
        } else {
            let idx: usize = if let Some(opcode_inst_idx) = self
                .instructions
                .iter()
                .enumerate()
                .find_map(|(instidx, inst)| {
                    if inst.opcode == opcode {
                        Some(instidx)
                    } else {
                        None
                    }
                }) {
                self.lookup_opcode.insert(opcode, opcode_inst_idx);
                opcode_inst_idx
            } else {
                //no inststruction for opcode :(
                return None;
            };

            idx
        };

        self.instructions.get(index)
    }

    pub fn load_from_file<P: AsRef<Path>>(file: P) -> Result<Self, GrammarError> {
        let file_string = std::fs::read_to_string(file).map_err(|e| GrammarError::FsError(e))?;
        let loaded = serde_json::from_str(&file_string).map_err(|e| GrammarError::ParseError(e))?;
        Ok(loaded)
    }

    ///Loads the core grammar rules from memory
    pub fn load_core_grammar() -> Self {
        //TODO: not sure if we _want_ to bundle the rules with the lib,
        //      but right now thats pretty efficient :D
        static CORE_GRAMMAR_RULES: &'static str =
            include_str!("../../../rules/1.2/spirv.core.grammar-rules.json");

        //should always parse
        let mut rules: GrammarRules = serde_json::from_str(CORE_GRAMMAR_RULES).unwrap();

        //pre populate the lookup table
        for (instidx, inst) in rules.instructions.iter().enumerate() {
            rules.lookup_opcode.insert(inst.opcode, instidx);
        }

        rules
    }

    pub fn load_glsl_std_450_grammar() -> Self {
        static GLSL_GRAMMAR_RULES: &'static str =
            include_str!("../../../rules/1.2/extinst.glsl.std.450.grammar-rules.json");

        //should always parse
        let mut rules: GrammarRules = serde_json::from_str(GLSL_GRAMMAR_RULES).unwrap();

        //pre populate the lookup table
        for (instidx, inst) in rules.instructions.iter().enumerate() {
            rules.lookup_opcode.insert(inst.opcode, instidx);
        }

        rules
    }
}
