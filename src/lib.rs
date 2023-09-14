#![feature(try_trait_v2)]

//! This crate exposes a struct [ExampleGenerator] that is capable of generating code examples for calls, constants and storage entries from static metadata.
//! The generated code can look like this:
//!
//! ```rust,norun
//! #[subxt::subxt(runtime_metadata_path = "polkadot.scale")]
//! pub mod runtime {}
//! use runtime::runtime_types;
//! use subxt::{OnlineClient, PolkadotConfig};
//! use subxt_signer::sr25519::dev;
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> =
//!         ::subxt::utils::MultiAddress::Id(::subxt::utils::AccountId32([8; 32usize]));
//!     let value: ::core::primitive::u128 = 128;
//!     let payload = runtime::tx().balances().transfer(dest, value);
//!     let api = OnlineClient::<PolkadotConfig>::new().await?;
//!     let from = dev::alice();
//!     let events = api
//!         .tx()
//!         .sign_and_submit_then_watch_default(&payload, &from)
//!         .await?
//!         .wait_for_finalized_success()
//!         .await?;
//!     Ok(())
//! }
//! ```

use std::{borrow::Cow, fmt::Display};

use anyhow::{anyhow, Ok};
use context::{ExampleContext, FileOrUrl};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use scale_info::{form::PortableForm, TypeDef, Variant};
use subxt_codegen::{DerivesRegistry, TypeGenerator, TypeSubstitutes};
use subxt_metadata::{PalletMetadata, StorageEntryMetadata, StorageEntryType};
use values::{dynamic_type_example, static_type_example};

pub mod context;
/// empty mod, copy paste stuff in here to validate code quickly
mod generated;
pub mod values;
pub mod wasm_interface;
pub use wasm_interface::*;

/// The [ExampleGenerator] is a struct that can be used to generate code examples for various uses of subxt.
/// It is intended to be embedded into the WASM of a website, to create code snippets to be displayed.
/// It exposes these methods:
/// - `all_examples_wrapped()`
/// - `call_example_wrapped(pallet_name, call_name)`
/// - `storage_example_wrapped(, entry_name)`
/// - `constant_example_wrapped(constant_name, constant_name)`
///
/// which all generate some executable rust code that should contain all imports
pub struct ExampleGenerator<'a> {
    pub metadata: subxt::Metadata,
    pub context: Cow<'a, ExampleContext>,
}

impl<'a> ExampleGenerator<'a> {
    /// Constructs an [ExampleGenerator] from a file path that leads to some scale encoded metadata.
    pub fn fetch_from_context(context: ExampleContext) -> anyhow::Result<Self> {
        let metadata = context.file_or_url.fetch_metadata()?;
        Ok(Self {
            metadata: metadata.into(),
            context: Cow::Owned(context),
        })
    }

    pub fn new(metadata: subxt::Metadata, context: Cow<'a, ExampleContext>) -> Self {
        Self { metadata, context }
    }

    //////////////////////////////////////////////
    // top level example generators
    //////////////////////////////////////////////

    /// Creates code that contains with example code for all calls, cosntants and storage entries.
    /// If this code compiles, we can be pretty sure, that the code generation worked fine.
    /// Can be tested with e.g. trybuild.
    pub fn all_examples_wrapped(&self) -> anyhow::Result<TokenStream> {
        let mut call_examples: Vec<TokenStream> = vec![];
        let mut storage_examples: Vec<TokenStream> = vec![];
        let mut constant_examples: Vec<TokenStream> = vec![];

        for pallet in self.metadata.pallets() {
            if let Some(calls) = pallet.call_variants() {
                for call in calls {
                    let call_example = self.call_example(pallet.name(), &call.name)?;
                    call_examples.push(call_example);
                }
            }

            if let Some(storage) = pallet.storage() {
                for entry in storage.entries() {
                    let storage_example = self.storage_example(pallet.name(), entry.name())?;
                    storage_examples.push(storage_example);
                }
            }

            for constant in pallet.constants() {
                let constant_example = self.constant_example(pallet.name(), constant.name())?;
                constant_examples.push(constant_example);
            }
        }

        let mut custom_value_examples: Vec<TokenStream> = vec![];
        for custom_value in self.metadata.custom().iter() {
            let example = self
                .custom_value_example(custom_value.name())
                .unwrap_or_default();
            // Note: unwrap_or_default here just to be cautious: if a custom value has a key of "12", we cannot generate example code for it. That is okay. Some custom values can have weird keys and are skipped in codegen.
            custom_value_examples.push(example);
        }

        // we provide an empty main function, because `trybuild` attempts to run it. But we don't want to run code, just check that it compiles.

        let code = quote!(
            #(#call_examples)*
            #(#storage_examples)*
            #(#constant_examples)*
            #(#custom_value_examples)*
        );
        let wrapped = self.wrap_in_wrapper_fn(code);
        Ok(wrapped)
    }

    pub fn call_example_wrapped(
        &self,
        pallet_name: &str,
        call_name: &str,
    ) -> anyhow::Result<TokenStream> {
        let call_example = self.call_example(pallet_name, call_name)?;
        let wrapped = self.wrap_in_main(call_example);
        Ok(wrapped)
    }

    pub fn storage_example_wrapped(
        &self,
        pallet_name: &str,
        storage_item: &str,
    ) -> anyhow::Result<TokenStream> {
        let storage_example = self.storage_example(pallet_name, storage_item)?;
        let wrapped = self.wrap_in_main(storage_example);
        Ok(wrapped)
    }

    pub fn constant_example_wrapped(
        &self,
        pallet_name: &str,
        constant_name: &str,
    ) -> anyhow::Result<TokenStream> {
        let constant_example = self.constant_example(pallet_name, constant_name)?;
        let wrapped = self.wrap_in_main(constant_example);
        Ok(wrapped)
    }

    pub fn custom_value_example_wrapped(&self, name: &str) -> anyhow::Result<TokenStream> {
        let custom_value_example = self.custom_value_example(name)?;
        let wrapped = self.wrap_in_main(custom_value_example);
        Ok(wrapped)
    }

    //////////////////////////////////////////////
    // unwrapped example generators
    //////////////////////////////////////////////

    pub fn call_example(&self, pallet_name: &str, call_name: &str) -> anyhow::Result<TokenStream> {
        let type_gen = self.type_gen();
        let pallet = self
            .metadata
            .pallet_by_name(pallet_name)
            .ok_or(anyhow!("pallet {pallet_name} not found."))?;
        let call = pallet.call_variant_by_name(call_name).ok_or(anyhow!(
            "call {call_name} in pallet {pallet_name} not found."
        ))?;
        // defines: let payload = ...
        let payload_code = self.call_example_payload(&type_gen, &pallet, call)?;
        let code = quote!(
            #payload_code
            let api = OnlineClient::<PolkadotConfig>::new().await?;
            let from = dev::alice();
            let events = api
                .tx()
                .sign_and_submit_then_watch_default(&payload, &from)
                .await?
                .wait_for_finalized_success()
                .await?;
        );

        Ok(code)
    }

    fn storage_example(
        &self,
        pallet_name: &str,
        storage_item: &str,
    ) -> anyhow::Result<TokenStream> {
        let type_gen = self.type_gen();
        let pallet = self
            .metadata
            .pallet_by_name(pallet_name)
            .ok_or_else(|| anyhow!("pallet {pallet_name} not found."))?;
        let entry = pallet
            .storage()
            .expect("should be there")
            .entry_by_name(storage_item)
            .ok_or_else(|| {
                anyhow!("storage_entry {storage_item} in pallet {pallet_name} not found.")
            })?;

        let storage_query_code = self.storage_example_query(&type_gen, &pallet, entry)?;

        let value_type_path = if self.context.dynamic {
            quote!(DecodedValueThunk)
        } else {
            let value_type = entry.entry_type().value_ty();
            let value_type_path = type_gen.resolve_type_path(value_type);
            quote!(#value_type_path)
        };

        let code = quote!(
            #storage_query_code
            let api = OnlineClient::<PolkadotConfig>::new().await?;
            let result : Option<#value_type_path> = api
                .storage()
                .at_latest()
                .await?
                .fetch(&storage_query)
                .await?;
        );

        Ok(code)
    }

    fn constant_example(
        &self,
        pallet_name: &str,
        constant_name: &str,
    ) -> anyhow::Result<TokenStream> {
        let type_gen = self.type_gen();
        let pallet = self
            .metadata
            .pallet_by_name(pallet_name)
            .ok_or_else(|| anyhow!("pallet {pallet_name} not found."))?;
        let constant = pallet.constant_by_name(constant_name).ok_or_else(|| {
            anyhow!("constant {constant_name} in pallet {pallet_name} not found.")
        })?;

        let code = if self.context.dynamic {
            let pallet_name = pallet.name();
            let constant_name = constant.name();
            quote!(
                let constant_query = subxt::constants::dynamic(#pallet_name, #constant_name);
                let api = OnlineClient::<PolkadotConfig>::new().await?;
                let value : DecodedValueThunk = api.constants().at(&constant_query)?;
                // Todo: the value can be also decoded into the static type. Show here?
            )
        } else {
            let pallet_name = format_ident!("{}", pallet.name().to_snake_case());
            let constant_name = format_ident!("{}", constant.name().to_snake_case());
            let constant_type_path = type_gen.resolve_type_path(constant.ty());
            quote!(
                let constant_query = runtime::constants().#pallet_name().#constant_name();
                let api = OnlineClient::<PolkadotConfig>::new().await?;
                let value : #constant_type_path = api.constants().at(&constant_query)?;
            )
        };

        Ok(code)
    }

    fn custom_value_example(&self, name: &str) -> anyhow::Result<TokenStream> {
        let type_gen = self.type_gen();
        let custom_metadata = self.metadata.custom();
        let custom_value = custom_metadata
            .get(name)
            .ok_or_else(|| anyhow!("custom value {name} not found."))?;

        let custom_value_type = type_gen.resolve_type_path(custom_value.type_id());
        let interface_name = &self.context.inter_face_ident;

        let address_expr = if self.context.dynamic {
            let name = custom_value.name();
            quote!(#name)
        } else {
            let snake_case_name = custom_value.name().to_snake_case();
            let name = syn::parse_str::<syn::Ident>(&snake_case_name)?;
            quote!(#interface_name::custom().#name())
        };

        let code = quote!(
            let value_address = #address_expr;
            let api = OnlineClient::<PolkadotConfig>::new().await?;
            let custom_value : #custom_value_type = api.custom_values().at(&value_address)?;
        );

        Ok(code)
    }

    //////////////////////////////////////////////
    // helper functions for example generators
    //////////////////////////////////////////////

    /// the returned code defines: `let payload = ...`
    fn call_example_payload(
        &self,
        type_gen: &TypeGenerator,
        pallet: &PalletMetadata,
        call: &Variant<PortableForm>,
    ) -> anyhow::Result<TokenStream> {
        let variable_iter = call.fields.iter().map(|field| {
            let name: &str = field
                .name
                .as_ref()
                .expect("only named fields should be call arguments");
            let type_path =
                type_gen.resolve_field_type_path(field.ty.id, &[], field.type_name.as_deref());
            (name, field.ty.id, type_path)
        });

        let (variable_names, variable_declarations) =
            variable_names_and_declarations(type_gen, variable_iter, &self.context)?;

        let payload_expr = if self.context.dynamic {
            let pallet_name = pallet.name();
            let call_name = &call.name;
            let args_vec = variable_names_to_scale_value_vec(variable_names);
            quote!(subxt::tx::dynamic(#pallet_name, #call_name, #args_vec))
        } else {
            let pallet_name = format_ident!("{}", pallet.name().to_snake_case());
            let call_name = format_ident!("{}", call.name.to_snake_case());
            quote!(runtime::tx().#pallet_name().#call_name( #(#variable_names),*))
        };

        let code = quote!(
            #(#variable_declarations);*
            let payload = #payload_expr;
        );

        Ok(code)
    }

    /// the returned code defines: `let storage_query = ...`
    fn storage_example_query(
        &self,
        type_gen: &TypeGenerator,
        pallet: &PalletMetadata,
        entry: &StorageEntryMetadata,
    ) -> anyhow::Result<TokenStream> {
        let variables_iter = storage_entry_key_ty_ids(type_gen, entry)
            .into_iter()
            .enumerate()
            .map(|(i, type_id)| {
                let variable_name = format!("key_{i}");
                let type_path = type_gen.resolve_type_path(type_id);
                (variable_name, type_id, type_path)
            });
        let (variable_names, variable_declarations) =
            variable_names_and_declarations(type_gen, variables_iter, &self.context)?;

        let query_expr = if self.context.dynamic {
            let pallet_name = pallet.name();
            let entry_name = entry.name();

            let key_vec = variable_names_to_scale_value_vec(variable_names);

            quote!(subxt::storage::dynamic(#pallet_name, #entry_name, #key_vec))
        } else {
            let pallet_name = format_ident!("{}", pallet.name().to_snake_case());
            let entry_name = format_ident!("{}", entry.name().to_snake_case());
            quote!(runtime::storage().#pallet_name().#entry_name( #(#variable_names),*))
        };

        let code = quote!(
            #(#variable_declarations);*
            let storage_query = #query_expr;
        );

        Ok(code)
    }

    //////////////////////////////////////////////
    // utils
    //////////////////////////////////////////////

    fn type_gen(&self) -> TypeGenerator {
        TypeGenerator::new(
            self.metadata.types(),
            "runtime_types",
            TypeSubstitutes::with_default_substitutes(&Default::default()),
            DerivesRegistry::with_default_derives(&Default::default()),
            Default::default(),
            true,
        )
    }

    fn wrap_in_wrapper_fn(&self, code: TokenStream) -> TokenStream {
        wrap_with_imports(
            code,
            "wrapper",
            quote!(
                pub fn main() {}
            ),
            &self.context,
        )
    }

    fn wrap_in_main(&self, code: TokenStream) -> TokenStream {
        wrap_with_imports(code, "main", quote!(), &self.context)
    }
}

/// necessary, because for empty vecs, the type cannot be deduced.
fn variable_names_to_scale_value_vec(variable_names: Vec<Ident>) -> TokenStream {
    if variable_names.is_empty() {
        quote!(Vec::<Value>::new())
    } else {
        quote!(vec![ #(#variable_names),*])
    }
}

/// Takes some inner code and adds:
/// - imports for some subxt types
/// - static interface via subxt macro referencing polkadot.scale
/// - an async wrapper function that wraps #inner
fn wrap_with_imports(
    inner: impl ToTokens,
    wrapper_fn_name: &str,
    code_before_wrapper: impl ToTokens,
    context: &ExampleContext,
) -> TokenStream {
    let attr_macro = match &context.file_or_url {
        FileOrUrl::File(path) => {
            quote!(#[subxt::subxt(runtime_metadata_path = #path)])
        }
        FileOrUrl::Url(url) => {
            quote!(#[subxt::subxt(runtime_metadata_url = #url)])
        }
    };

    let wrapper_fn_ident = format_ident!("{}", wrapper_fn_name);
    let interface_name = &context.inter_face_ident;

    let dynamic_static_imports = if context.dynamic {
        quote!(
            #![recursion_limit = "256"]
            use subxt::ext::scale_bits::bits;
            use subxt::ext::scale_value::{value, Value};
            use subxt::dynamic::DecodedValueThunk;
        )
    } else {
        quote!(
            #attr_macro
            pub mod #interface_name {}
            use #interface_name::runtime_types;
        )
    };

    quote!(
        #dynamic_static_imports

        use subxt::{OnlineClient, PolkadotConfig};
        use subxt_signer::sr25519::dev;

        #code_before_wrapper
        async fn #wrapper_fn_ident() -> Result<(), Box<dyn std::error::Error>> {
            #inner
            Ok(())
        }


    )
}

pub fn storage_entry_key_ty_ids(
    type_gen: &TypeGenerator,
    entry: &StorageEntryMetadata,
) -> Vec<u32> {
    match entry.entry_type() {
        StorageEntryType::Plain(_) => vec![],
        StorageEntryType::Map { key_ty, .. } => {
            match &type_gen.resolve_type(*key_ty).type_def {
                // An N-map; return each of the keys separately.
                TypeDef::Tuple(tuple) => tuple.fields.iter().map(|ty| ty.id).collect::<Vec<_>>(),
                // A map with a single key; return the single key.
                _ => vec![*key_ty],
            }
        }
    }
}

/// The iterator item is: (variable_name, type_id, type_path)
fn variable_names_and_declarations(
    type_gen: &TypeGenerator,
    variables: impl Iterator<Item = (impl Display, u32, impl ToTokens)>,
    context: &ExampleContext,
) -> anyhow::Result<(Vec<Ident>, Vec<TokenStream>)> {
    let mut variable_names: Vec<Ident> = vec![];
    let mut variable_declarations: Vec<TokenStream> = vec![];

    for (variable_name, type_id, type_path) in variables {
        let variable_name = format_ident!("{variable_name}");
        let type_example = if context.dynamic {
            dynamic_type_example(type_id, type_gen.types())?
        } else {
            static_type_example(type_id, type_gen)?
        };

        let type_path = if context.dynamic {
            quote!(Value)
        } else {
            type_path.to_token_stream()
        };

        // put it together as a variable declaration
        let declaration = quote!(let #variable_name : #type_path = #type_example;);
        variable_names.push(variable_name);
        variable_declarations.push(declaration);
    }

    Ok((variable_names, variable_declarations))
}
