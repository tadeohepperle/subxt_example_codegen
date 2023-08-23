use std::fs;

use subxt_example_codegen::ExampleGenerator;

fn main() -> anyhow::Result<()> {
    let example_gen = ExampleGenerator::polkadot();
    let all = example_gen.file_with_all_examples()?;
    let syn_tree = syn::parse_file(&all.to_string()).unwrap();
    let pretty = prettyplease::unparse(&syn_tree);
    fs::write("gen/polkadot.rs", pretty)?;

    fs::copy(
        "polkadot.scale",
        "target/tests/trybuild/subxt_example_codegen/polkadot.scale",
    )?;
    let t = trybuild::TestCases::new();
    t.pass("gen/polkadot.rs");
    Ok(())
}
