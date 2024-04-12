fn main() {
    let rules_file = std::env::args()
        .skip(1)
        .next()
        .expect("Expect a rules file as argument!");
    let file_content = std::fs::read_to_string(&rules_file).expect("Could not read rules file!");

    let parsed_rules: spirv_grammar_rules::Rules =
        serde_json::from_str(&file_content).expect("Could not parse rules file!");

    println!("Rules of {rules_file}:\n{:#?}", parsed_rules);
}
