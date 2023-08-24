use std::fs;

use subxt::utils::bits::DecodedBits;
use subxt_example_codegen::ExampleGenerator;

fn main() -> anyhow::Result<()> {
    let example_gen = ExampleGenerator::polkadot();
    let t = format!("{:#?}", example_gen.metadata.types());
    fs::write("type reg", t);

    let tokens = example_gen.file_with_all_examples()?;
    let syn_tree = syn::parse_file(&tokens.to_string()).unwrap();
    let pretty = prettyplease::unparse(&syn_tree);
    fs::write("gen/polkadot.rs", pretty)?;

    // let t = trybuild::TestCases::new();
    // fs::copy(
    //     "polkadot.scale",
    //     "target/tests/trybuild/subxt_example_codegen/polkadot.scale",
    // )?;
    // t.pass("gen/polkadot.rs");
    Ok(())
}

// mismatched types
// expected struct `DecodedBits<u8, subxt::utils::bits::Lsb0>`
//    found struct `BitVec<_, _>
