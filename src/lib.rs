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

use std::{collections::BTreeSet, fmt::Display};

use anyhow::{anyhow, Ok};
use heck::ToSnakeCase;
use proc_macro2::{Ident, Punct, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use scale_info::{
    form::PortableForm, Field, PortableRegistry, Type, TypeDef, TypeDefArray, TypeDefCompact,
    TypeDefPrimitive, TypeDefSequence, TypeParameter, Variant,
};
use subxt::ext::codec::Decode;
use subxt_codegen::{
    CratePath, DerivesRegistry, RuntimeGenerator, TypeDefGen, TypeGenerator, TypeSubstitutes,
};
use subxt_metadata::{Metadata, PalletMetadata, StorageEntryMetadata, StorageEntryType};

pub enum FileOrUrl {
    File(String),
    Url(String),
}

/// empty mod, copy paste stuff in here to validate code quickly
mod generated;

/// The [ExampleGenerator] is a struct that can be used to generate code examples for various uses of subxt.
/// It is intended to be embedded into the WASM of a website, to create code snippets to be displayed.
/// It exposes these methods:
/// - `all_examples_wrapped()`
/// - `call_example_wrapped(pallet_name, call_name)`
/// - `storage_example_wrapped(, entry_name)`
/// - `constant_example_wrapped(constant_name, constant_name)`
///
/// which all generate some executable rust code that should contain all imports
pub struct ExampleGenerator {
    pub metadata: subxt_metadata::Metadata,
    /// currently not used. Will be used later to inform how the subxt macro is generated.
    /// Can also be used used later to make for an async constructor that fetches the metadata from this location.
    file_or_url: FileOrUrl,
}

impl ExampleGenerator {
    /// Constructs an [ExampleGenerator] from a file path that leads to some scale encoded metadata.
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let bytes = std::fs::read(path)?;
        let mut metadata = Metadata::decode(&mut &bytes[..])?;
        RuntimeGenerator::ensure_unique_type_paths(&mut metadata);
        Ok(Self {
            metadata,
            file_or_url: FileOrUrl::File(path.into()),
        })
    }

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
        let code = wrap_with_imports(
            quote!(
                #(#call_examples)*
                #(#storage_examples)*
                #(#constant_examples)*
                #(#custom_value_examples)*
            ),
            "wrapper",
            quote!(
                pub fn main() {}
            ),
            &self.file_or_url,
        );

        Ok(code)
    }

    /// create an executable example (with ) with the specified call
    pub fn call_example_wrapped(
        &self,
        pallet_name: &str,
        call_name: &str,
    ) -> anyhow::Result<TokenStream> {
        let call_example = self.call_example(pallet_name, call_name)?;
        let code = wrap_with_imports(
            quote!(
                #call_example
            ),
            "main",
            quote!(),
            &self.file_or_url,
        );

        Ok(code)
    }

    pub fn call_example(&self, pallet_name: &str, call_name: &str) -> anyhow::Result<TokenStream> {
        let type_gen = self.new_type_gen();
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

    /// the returned code defines: `let payload = ...`
    fn call_example_payload(
        &self,
        type_gen: &TypeGenerator,
        pallet: &PalletMetadata,
        call: &Variant<PortableForm>,
    ) -> anyhow::Result<TokenStream> {
        let pallet_name = format_ident!("{}", pallet.name().to_snake_case());
        let call_name = format_ident!("{}", call.name.to_snake_case());

        let variable_iter = call.fields.iter().map(|field| {
            let name: &str = field
                .name
                .as_ref()
                .expect("only named fields should be call arguments");
            let type_path =
                type_gen.resolve_field_type_path(field.ty.id, &[], field.type_name.as_deref());
            (name, field.ty.id, type_path)
        });

        let (field_names, field_declarations) =
            variable_names_and_declarations(type_gen, variable_iter)?;

        let code = quote!(
            #(#field_declarations);*
            let payload = runtime::tx().#pallet_name().#call_name( #(#field_names),*);
        );
        Ok(code)
    }

    pub fn storage_example_wrapped(
        &self,
        pallet_name: &str,
        storage_item: &str,
    ) -> anyhow::Result<TokenStream> {
        let storage_example = self.storage_example(pallet_name, storage_item)?;
        let code = wrap_with_imports(
            quote!(
                #storage_example
            ),
            "main",
            quote!(#[tokio::main]),
            &self.file_or_url,
        );
        Ok(code)
    }

    fn storage_example(
        &self,
        pallet_name: &str,
        storage_item: &str,
    ) -> anyhow::Result<TokenStream> {
        let type_gen = self.new_type_gen();
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

        let value_type = entry.entry_type().value_ty();
        let value_type_path = type_gen.resolve_type_path(value_type);

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

    /// the returned code defines: `let storage_query = ...`
    fn storage_example_query(
        &self,
        type_gen: &TypeGenerator,
        pallet: &PalletMetadata,
        entry: &StorageEntryMetadata,
    ) -> anyhow::Result<TokenStream> {
        let pallet_name = format_ident!("{}", pallet.name().to_snake_case());
        let entry_name = format_ident!("{}", entry.name().to_snake_case());

        let variables_iter = storage_entry_key_ty_ids(type_gen, entry)
            .into_iter()
            .enumerate()
            .map(|(i, type_id)| {
                let variable_name = format!("key_{i}");
                let type_path = type_gen.resolve_type_path(type_id);
                (variable_name, type_id, type_path)
            });
        let (variable_names, variable_declarations) =
            variable_names_and_declarations(type_gen, variables_iter)?;

        let code = quote!(
            #(#variable_declarations);*
            let storage_query = runtime::storage().#pallet_name().#entry_name( #(#variable_names),*);
        );
        Ok(code)
    }

    pub fn constant_example_wrapped(
        &self,
        pallet_name: &str,
        constant_name: &str,
    ) -> anyhow::Result<TokenStream> {
        let constant_example = self.constant_example(pallet_name, constant_name)?;
        let code = wrap_with_imports(
            quote!(
                #constant_example
            ),
            "main",
            quote!(#[tokio::main]),
            &self.file_or_url,
        );
        Ok(code)
    }

    fn constant_example(
        &self,
        pallet_name: &str,
        constant_name: &str,
    ) -> anyhow::Result<TokenStream> {
        let type_gen = self.new_type_gen();
        let pallet = self
            .metadata
            .pallet_by_name(pallet_name)
            .ok_or_else(|| anyhow!("pallet {pallet_name} not found."))?;
        let constant = pallet.constant_by_name(constant_name).ok_or_else(|| {
            anyhow!("constant {constant_name} in pallet {pallet_name} not found.")
        })?;
        let constant_type_path = type_gen.resolve_type_path(constant.ty());

        let pallet_name = format_ident!("{}", pallet.name().to_snake_case());
        let constant_name = format_ident!("{}", constant.name().to_snake_case());

        let code = quote!(
            let constant_query = runtime::constants().#pallet_name().#constant_name();
            let api = OnlineClient::<PolkadotConfig>::new().await?;
            let value : #constant_type_path = api.constants().at(&constant_query)?;
        );

        Ok(code)
    }

    /// create an executable example (with ) with the specified call
    pub fn custom_value_example_wrapped(&self, name: &str) -> anyhow::Result<TokenStream> {
        let custom_value_example = self.custom_value_example(name)?;
        let code = wrap_with_imports(
            quote!(
                #custom_value_example
            ),
            "main",
            quote!(),
            &self.file_or_url,
        );

        Ok(code)
    }

    fn custom_value_example(&self, name: &str) -> anyhow::Result<TokenStream> {
        let type_gen = self.new_type_gen();
        let custom_metadata = self.metadata.custom();
        let custom_value = custom_metadata
            .get(name)
            .ok_or_else(|| anyhow!("custom value {name} not found."))?;
        let snake_case_name = custom_value.name().to_snake_case();
        let name = syn::parse_str::<syn::Ident>(&snake_case_name)?;

        let custom_value_type = type_gen.resolve_type_path(custom_value.type_id());

        let code = quote!(
            let value_address = runtime::custom().#name();
            let api = OnlineClient::<PolkadotConfig>::new().await?;
            let custom_value : #custom_value_type = api.custom_values().at(&value_address)?;
        );

        Ok(code)
    }

    fn new_type_gen(&self) -> TypeGenerator {
        TypeGenerator::new(
            self.metadata.types(),
            "runtime_types",
            TypeSubstitutes::with_default_substitutes(&Default::default()),
            DerivesRegistry::with_default_derives(&Default::default()),
            Default::default(),
            true,
        )
    }
}

pub enum CompactMode {
    // explicitely stating Compact(u32)
    Expl,
    // compact encoded via attribute #[codec(compact)]
    Attr,
}

/// Takes some inner code and adds:
/// - imports for some subxt types
/// - static interface via subxt macro referencing polkadot.scale
/// - an async wrapper function that wraps #inner
fn wrap_with_imports(
    inner: impl ToTokens,
    wrapper_fn_name: &str,
    code_before_wrapper: impl ToTokens,
    file_or_url: &FileOrUrl,
) -> TokenStream {
    let attr_macro = match file_or_url {
        FileOrUrl::File(path) => {
            quote!(#[subxt::subxt(runtime_metadata_path = #path)])
        }
        FileOrUrl::Url(url) => {
            quote!(#[subxt::subxt(runtime_metadata_url = #url)])
        }
    };

    let wrapper_fn_ident = format_ident!("{}", wrapper_fn_name);
    quote!(
        #attr_macro
        pub mod runtime {}
        use runtime::runtime_types;

        use subxt::{OnlineClient, PolkadotConfig};
        use subxt_signer::sr25519::dev;

        #code_before_wrapper
        async fn #wrapper_fn_ident() -> Result<(), Box<dyn std::error::Error>> {
            #inner
            Ok(())
        }


    )
}

fn storage_entry_key_ty_ids(type_gen: &TypeGenerator, entry: &StorageEntryMetadata) -> Vec<u32> {
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
fn variable_names_and_declarations<'a>(
    type_gen: &TypeGenerator,
    variables: impl Iterator<Item = (impl Display, u32, impl ToTokens)>,
) -> anyhow::Result<(Vec<Ident>, Vec<TokenStream>)> {
    let mut variable_names: Vec<Ident> = vec![];
    let mut variable_declarations: Vec<TokenStream> = vec![];

    for (variable_name, type_id, type_path) in variables {
        let variable_name = format_ident!("{variable_name}");
        let type_example = type_example(type_gen, type_id, CompactMode::Attr)?;
        // put it together as a variable declaration
        let declaration = quote!(let #variable_name : #type_path = #type_example;);
        variable_names.push(variable_name);
        variable_declarations.push(declaration);
    }

    Ok((variable_names, variable_declarations))
}

fn type_example(
    type_gen: &TypeGenerator,
    id: u32,
    compact_mode: CompactMode,
) -> anyhow::Result<TokenStream> {
    let ty = type_gen.resolve_type(id);
    type_def_example(type_gen, id, &ty, compact_mode)
}

fn type_def_example(
    type_gen: &TypeGenerator,
    id: u32,
    ty: &Type<PortableForm>,
    compact_mode: CompactMode,
) -> anyhow::Result<TokenStream> {
    match &ty.type_def {
        scale_info::TypeDef::Composite(def) => {
            let struct_path = resolve_type_path_omit_generics(type_gen, id);
            let gen_for_unsused_params =
                TypeDefGen::from_type(ty, type_gen, &CratePath::default(), false)
                    .expect("should work");
            let fields: TokenStream = fields_example(
                type_gen,
                &def.fields,
                gen_for_unsused_params.has_unused_type_params(),
            )?;
            Ok(quote!(#struct_path #fields))
        }
        scale_info::TypeDef::Variant(def) => {
            // just take first variant:
            let enum_path = resolve_type_path_omit_generics(type_gen, id);
            let first_variant = def
                .variants
                .first()
                .ok_or(anyhow!("variant type should have at least one variant"))?;
            let variant_ident = format_ident!("{}", &first_variant.name);
            // Technically we also need for phantom types here, but that is quite difficult at the moment, because we only want to check for a single variant, and TypeDefGen does not support that right now
            // So for now, we set it to false.
            let fields = fields_example(type_gen, &first_variant.fields, false)?;
            Ok(quote!(#enum_path:: #variant_ident #fields))
        }
        scale_info::TypeDef::Sequence(def) => {
            // return a Vec with 2 elements:
            let inner_ty = type_gen.resolve_type(def.type_param.id);
            let item_code =
                type_def_example(type_gen, def.type_param.id, &inner_ty, CompactMode::Expl)?;
            let vec_code = quote!(vec![#item_code, #item_code]);
            Ok(vec_code)
        }
        scale_info::TypeDef::Array(def) => {
            let inner_ty = type_gen.resolve_type(def.type_param.id);
            let item_code =
                type_def_example(type_gen, def.type_param.id, &inner_ty, CompactMode::Expl)?;
            let inner_is_copy = type_def_is_copy(type_gen, &inner_ty.type_def);
            let len = def.len as usize;
            let arr_code = if inner_is_copy {
                // if the item_code is an expression that is `Copy` we can use short init syntax:
                quote!([#item_code;#len])
            } else {
                // otherwise we need to duplicate the item_code `len` times:
                let item_iter = (0..len).map(|_| &item_code);
                quote!([#(#item_iter),*])
            };
            Ok(arr_code)
        }
        scale_info::TypeDef::Tuple(def) => {
            let mut fields: Vec<TokenStream> = vec![];
            for f in &def.fields {
                let value = type_example(type_gen, f.id, CompactMode::Expl)?;
                fields.push(value)
            }
            Ok(quote!(( #(#fields),* )))
        }
        scale_info::TypeDef::Primitive(def) => Ok(primitive_example(def)),
        scale_info::TypeDef::Compact(def) => {
            // there are actually two possibilities here:
            // 1. the value is not actually compact but just tagged with { #[codec(compact)] number: u8 } in the type definition.
            // --> give a normal primitive as a type example, e.g. 8
            // 2. the value is actually like (Compact<u8>, String) in the type definition.
            // --> give compact type example, e.g. Compact(8)

            // How to find out? In structs, we are gonna be in case 1, otherwise (inside a tuple, array or vec) where the #[codec(compact)] is not possible, we are in case 2.
            // `explicit_compact` flag is used to indicate we are in case 2.

            let inner_code = type_example(type_gen, def.type_param.id, CompactMode::Expl)?;
            // I used this originally, but it turns out the compact part should be omitted:

            let code = match compact_mode {
                CompactMode::Expl => {
                    let compact_path = resolve_type_path_omit_generics(type_gen, id);
                    quote!(#compact_path(#inner_code))
                }
                CompactMode::Attr => inner_code,
            };

            Ok(code)
        }
        scale_info::TypeDef::BitSequence(_def) => {
            Ok(quote!(subxt::utils::bits::DecodedBits::from_iter([
                true, false, false
            ])))
        }
    }
}

fn fields_example(
    type_gen: &TypeGenerator,
    fields: &[Field<PortableForm>],
    has_unused_type_params: bool,
) -> anyhow::Result<TokenStream> {
    let all_named = fields.iter().all(|f| f.name.is_some());
    let all_unnamed = fields.iter().all(|f| f.name.is_none());
    match (all_named, all_unnamed) {
        (true, false) => {
            // all fields named
            let mut field_idents_and_values: Vec<TokenStream> = vec![];
            for f in fields {
                let name = f.name.as_ref().expect("safe because of check above; qed");
                let ident = format_ident!("{name}");
                let value_code = type_example(type_gen, f.ty.id, CompactMode::Attr)?;
                field_idents_and_values.push(quote!(#ident : #value_code));
            }
            // maybe add phantom data to struct / named composite enum
            let maybe_phantom = if has_unused_type_params {
                quote!( __subxt_unused_type_params: ::core::marker::PhantomData )
            } else {
                quote!()
            };
            Ok(quote!({ #(#field_idents_and_values ,)* #maybe_phantom }))
        }
        (false, true) => {
            // all fields unnamed
            let mut field_values: Vec<TokenStream> = vec![];
            for f in fields {
                let value_code = type_example(type_gen, f.ty.id, CompactMode::Attr)?;
                field_values.push(value_code);
            }
            // maybe add phantom data to struct / named composite enum
            let maybe_phantom = if has_unused_type_params {
                quote!(::core::marker::PhantomData)
            } else {
                quote!()
            };
            Ok(quote!(( #(#field_values ,)* #maybe_phantom )))
        }
        (true, true) => {
            // no fields
            Ok(quote!())
        }
        (false, false) => {
            // mixed fields
            Err(anyhow!("mixed fields in struct def"))
        }
    }
}

fn primitive_example(def: &TypeDefPrimitive) -> TokenStream {
    match def {
        TypeDefPrimitive::Bool => quote!(false),
        TypeDefPrimitive::Char => quote!('c'),
        TypeDefPrimitive::Str => quote!("Hello".into()),
        TypeDefPrimitive::U8 => quote!(8),
        TypeDefPrimitive::U16 => quote!(16),
        TypeDefPrimitive::U32 => quote!(32),
        TypeDefPrimitive::U64 => quote!(64),
        TypeDefPrimitive::U128 => quote!(128),
        TypeDefPrimitive::U256 => quote!(256),
        TypeDefPrimitive::I8 => quote!(-8),
        TypeDefPrimitive::I16 => quote!(-16),
        TypeDefPrimitive::I32 => quote!(-32),
        TypeDefPrimitive::I64 => quote!(-64),
        TypeDefPrimitive::I128 => quote!(-128),
        TypeDefPrimitive::I256 => quote!(-256),
    }
}

/// Simple Heuristics. Just makes array initialization shorter if is `Copy`.
fn type_def_is_copy(type_gen: &TypeGenerator, ty: &TypeDef<PortableForm>) -> bool {
    match ty {
        TypeDef::Primitive(def) => match def {
            TypeDefPrimitive::Str => false,
            _ => true,
        },
        scale_info::TypeDef::Array(def) => {
            let item_type = type_gen.resolve_type(def.type_param.id);
            def.len <= 32 && type_def_is_copy(type_gen, &item_type.type_def)
        }
        scale_info::TypeDef::Tuple(def) => def.fields.iter().all(|f| {
            let ty = type_gen.resolve_type(f.id);
            type_def_is_copy(type_gen, &ty.type_def)
        }),

        scale_info::TypeDef::Compact(def) => {
            let ty = type_gen.resolve_type(def.type_param.id);
            type_def_is_copy(type_gen, &ty.type_def)
        }
        _ => false,
    }
}

/// Converts e.g. HashMap<u8, u16> => HashMap
///
/// This is a workaround, should probably be handled with syn::Expr somehow
fn resolve_type_path_omit_generics(type_gen: &TypeGenerator, id: u32) -> TokenStream {
    let path = type_gen.resolve_type_path(id);
    let path: TokenStream = path
        .to_token_stream()
        .into_iter()
        .take_while(|t| match t {
            TokenTree::Punct(p) => p.as_char() != '<',
            _ => true,
        })
        .collect();
    path
}
