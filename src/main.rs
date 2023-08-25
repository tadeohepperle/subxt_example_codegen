use std::{env::args, fs};

use subxt::utils::bits::DecodedBits;
use subxt_example_codegen::ExampleGenerator;

/// Make sure you have a `polkadot.scale` file at the root of this project.
///
/// Use `cargo run` to just generate a file with all examples for `polkadot.scale` under `gen/polkadot.rs`.
/// You can copy this file into the empty `src/polkadot.rs` to have it included in the module tree and checked by rust analyzer.
///
/// Use `cargo run -- build` to make use of `trybuild`. It will build the generated `gen/polkadot.rs` file and report any errors.
///
/// The directory `./alternative_metadata` contains different metadata files that can quickly replace the `./polkadot.scale` to see if codegen still works.
fn main() -> anyhow::Result<()> {
    let example_gen = ExampleGenerator::polkadot();
    let tokens = example_gen.all_examples_wrapped()?;
    // you can also try something like this instead:
    // let tokens = example_gen.call_example_wrapped("Balances", "transfer")?;

    let syn_tree = syn::parse_file(&tokens.to_string()).unwrap();
    let pretty = prettyplease::unparse(&syn_tree);
    fs::write("gen/polkadot.rs", pretty)?;

    // if executed with `cargo run -- build` we use trybuild to validate the generated code.
    // note: trybuild expects and executes `pub fn main()` in the generated code.
    if args().skip(1).next() == Some("build".into()) {
        let test_cases = trybuild::TestCases::new();
        // necessary to let macro work in build:
        fs::copy(
            "polkadot.scale",
            "target/tests/trybuild/subxt_example_codegen/polkadot.scale",
        )?;
        test_cases.pass("gen/polkadot.rs");
    }

    Ok(())
}
