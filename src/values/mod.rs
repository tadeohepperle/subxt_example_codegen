use proc_macro2::TokenStream;
use scale_info::PortableRegistry;
use subxt_codegen::TypeGenerator;

pub mod dynamic_values;
pub mod static_values;

pub struct ExampleValue {}

impl ExampleValue {}

pub fn static_type_example(id: u32, type_gen: &TypeGenerator) -> anyhow::Result<TokenStream> {
    static_values::type_example(id, type_gen)
}

pub fn dynamic_type_example(id: u32, types: &PortableRegistry) -> anyhow::Result<TokenStream> {
    dynamic_values::type_example(id, types)
}
