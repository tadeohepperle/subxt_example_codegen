use std::fs;

use parity_scale_codec::Decode;
use subxt_codegen::RuntimeGenerator;
use subxt_example_codegen::{
    context::ExampleContext, format_code, format_scale_value_string, ExampleGenerator,
};
use subxt_metadata::Metadata;

/// Make sure you have a `polkadot.scale` file at the root of this project.
///
/// Use `cargo run` to just generate a file with all examples for `polkadot.scale` under `gen/polkadot.rs`.
/// You can copy this file into the empty `src/polkadot.rs` to have it included in the module tree and checked by rust analyzer.
///
/// Use `cargo run -- build` to make use of `trybuild`. It will build the generated `gen/polkadot.rs` file and report any errors.
///
/// The directory `./alternative_metadata` contains different metadata files that can quickly replace the `./polkadot.scale` to see if codegen still works.
fn main() -> anyhow::Result<()> {
    let c = format_code("type A = runtime_types::frame_system::AccountInfo<u32,runtime_types::pallet_balances::types::AccountData<u128>>;");

    let val = scale_value::value!({ base_block: { ref_time: 10351, proof_size: 0 }, max_block: { ref_time: 200, proof_size: 184 }, per_class: { normal: { 
        base_extrinsic: { ref_time: 107648000, proof_size: 0 }, max_extrinsic: Some ({ ref_time: 147, proof_size: 1365059}), max_total: Some ({ ref_time: 1500, proof_size: 138350580 }), reserved: Some ({ ref_time: 0, proof_size: 0 }) }, operational: { base_extrinsic: { ref_time: 107648000, proof_size: 0 }, max_extrinsic: Some ({ ref_time: 32323223, proof_size: 233223 }), max_total: Some ({ ref_time: 233223, proof_size: 184
         }), reserved: Some ({ ref_time: 50, proof_size: 46116 }) }, mandatory: { base_extrinsic: { ref_time: 107648000, proof_size: 0 }, max_extrinsic: None (), max_total: None (), reserved: None () } } });

    let s = val.to_string();

    let c = format_scale_value_string(&s);

    println!("{c}");
    return Ok(());

    println!("{c}");
    dbg!(c);

    let example_gen = ExampleGenerator::new_from_metadata_file("polkadot.scale")?;
    let tokens = example_gen.all_examples_wrapped()?;

    // you can also try something like this instead:
    // let tokens = example_gen.call_example_wrapped("Balances", "transfer")?;
    // let tokens = example_gen.custom_value_example_wrapped("Foo")?;

    let syn_tree = syn::parse_file(&tokens.to_string()).unwrap();
    let pretty = prettyplease::unparse(&syn_tree);
    fs::write("gen/generated.rs", pretty)?;

    // if executed with `cargo run -- build` we use trybuild to validate the generated code.
    // note: trybuild expects and executes `pub fn main()` in the generated code.
    // if args().skip(1).next() == Some("build".into()) {
    //     let test_cases = trybuild::TestCases::new();
    //     // necessary to let macro work in build:
    //     fs::copy(
    //         metadata_file,
    //         format!("target/tests/trybuild/subxt_example_codegen/{metadata_file}"),
    //     )?;
    //     test_cases.pass("gen/generated.rs");
    // }

    Ok(())
}
