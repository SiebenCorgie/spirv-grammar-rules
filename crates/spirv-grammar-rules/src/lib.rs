use std::collections::HashMap;

use serde::Deserialize;

///Types as defined in 2.2.2 of the unified spec:
///<https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#_types>
//TODO: This should probably be split since a lot of those are overlaping.
#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub enum Rule {
    ///Declares that `operand` needs to
    /// have one of `ty`'s base type.
    #[serde(alias = "base_type")]
    BaseType {
        operand: String,
        base_types: Vec<Type>,
    },
    ///Declares that `operand` needs to have one of the
    /// concrete(?) types in `ty`.
    #[serde(alias = "type_constrain")]
    TypeConstraine { operand: String, ty: Vec<Type> },
    ///Declares that the `operand` needs to have the
    ///same type as the result of this instruction.
    #[serde(alias = "result_equal_type")]
    ResultTypeEqual(String),
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct GrammarRules {
    ///Name of the grammar file those rules are made for.
    pub source_grammar: String,
    /// list of rules that are used as well as their description.
    pub rule_types: HashMap<String, String>,
    ///All defined instructions and their rules.
    pub instructions: Vec<Instruction>,
}
