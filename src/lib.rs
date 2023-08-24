use std::collections::BTreeSet;

use anyhow::{anyhow, Ok};
use heck::ToSnakeCase;
use proc_macro2::{Ident, Punct, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use scale_info::{
    form::PortableForm, Field, PortableRegistry, Type, TypeDef, TypeDefArray, TypeDefCompact,
    TypeDefPrimitive, TypeDefSequence, TypeParameter,
};
use subxt::ext::codec::Decode;
use subxt_codegen::{CratePath, DerivesRegistry, TypeGenerator, TypeSubstitutes};
use subxt_metadata::Metadata;

pub enum FileOrUrl {
    File(String),
    Url(String),
}

pub mod polkadot;

pub struct ExampleGenerator {
    metadata: subxt_metadata::Metadata,
    file_or_url: FileOrUrl,
}

impl ExampleGenerator {
    pub fn polkadot() -> Self {
        let polkadot_scale_path = "polkadot.scale";
        let bytes = std::fs::read(polkadot_scale_path).expect("works");
        let metadata = Metadata::decode(&mut &bytes[..]).expect("works");
        Self {
            metadata,
            file_or_url: FileOrUrl::File(polkadot_scale_path.into()),
        }
    }

    pub fn file_with_all_examples(&self) -> anyhow::Result<TokenStream> {
        let mut call_examples: Vec<TokenStream> = vec![];
        for p in self.metadata.pallets() {
            if let Some(variants) = p.call_variants() {
                for c in variants {
                    let call_example = self.call_example(p.name(), &c.name)?;
                    call_examples.push(call_example);
                    // break 'o;
                }
            }
        }
        let imports_and_static_interface = self.imports_and_static_interface();

        let code = quote!(
            #imports_and_static_interface

            // main is run by trybuild, but we dont want to run anything
            pub fn main() {}

            // we just need to make sure this compiles
            async fn wrapper() -> Result<(), Box<dyn std::error::Error>> {
                #(#call_examples)*

                Ok(())
            }
        );
        Ok(code)
    }

    fn call_example_payload(
        &self,
        type_gen: &TypeGenerator,
        pallet_name: &str,
        call_name: &str,
    ) -> anyhow::Result<TokenStream> {
        let pallet_metadata = self
            .metadata
            .pallet_by_name(pallet_name)
            .expect("should be there");
        let pallet_name = format_ident!("{}", pallet_name.to_snake_case());
        let call = pallet_metadata
            .call_variant_by_name(call_name)
            .expect("should be there");
        let call_name = format_ident!("{}", call_name.to_snake_case());

        let mut field_names: Vec<Ident> = vec![];
        let mut field_declarations: Vec<TokenStream> = vec![];

        for field in &call.fields {
            let field_name = field
                .name
                .as_ref()
                .ok_or(anyhow!("only named fields should make up a call"))?;
            let field_name = format_ident!("{field_name}");
            field_names.push(field_name.clone());

            // get the field type path:
            // todo!("shorten the path, we do not want ::std::vec::Vec<::core::primitive::u8> in examples, Vec<u8> is good enough")
            let field_type_path =
                type_gen.resolve_field_type_path(field.ty.id, &[], field.type_name.as_deref());

            let field_type_example = type_example(type_gen, field.ty.id)?;

            // put it together as a variable declaration
            let declaration = quote!(let #field_name : #field_type_path = #field_type_example;);
            field_declarations.push(declaration);
        }

        let code = quote!(
            #(#field_declarations);*
            let payload = polkadot::tx().#pallet_name().#call_name( #(#field_names),*);
        );
        Ok(code)
    }

    pub fn call_example(&self, pallet_name: &str, call_name: &str) -> anyhow::Result<TokenStream> {
        let type_gen = self.new_type_gen();
        let payload_code = self.call_example_payload(&type_gen, pallet_name, call_name)?;
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

    pub fn new_type_gen(&self) -> TypeGenerator {
        TypeGenerator::new(
            self.metadata.types(),
            "runtime_types",
            TypeSubstitutes::with_default_substitutes(&Default::default()),
            DerivesRegistry::with_default_derives(&Default::default()),
            Default::default(),
            true,
        )
    }

    fn imports_and_static_interface(&self) -> TokenStream {
        quote!(
            #[subxt::subxt(runtime_metadata_path = "polkadot.scale")]
            pub mod polkadot {}
            use polkadot::runtime_types;

            use subxt::{OnlineClient, PolkadotConfig};
            use subxt_signer::sr25519::dev;
        )
    }
}

fn type_example(type_gen: &TypeGenerator, id: u32) -> anyhow::Result<TokenStream> {
    let ty = type_gen.resolve_type(id);
    type_def_example(type_gen, id, &ty)
}

fn type_def_example(
    type_gen: &TypeGenerator,
    id: u32,
    ty: &Type<PortableForm>,
) -> anyhow::Result<TokenStream> {
    let unused_type_params = ty_has_unused_type_parameters(type_gen, ty);
    match &ty.type_def {
        scale_info::TypeDef::Composite(def) => {
            let struct_path = resolve_type_path_omit_generics(type_gen, id);
            let fields: TokenStream = fields_example(type_gen, &def.fields, unused_type_params)?;
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
            let fields = fields_example(type_gen, &first_variant.fields, unused_type_params)?;
            Ok(quote!(#enum_path:: #variant_ident #fields))
        }
        scale_info::TypeDef::Sequence(def) => {
            // return a Vec with 2 elements:
            let inner_ty = type_gen.resolve_type(def.type_param.id);
            let item_code = type_def_example(type_gen, def.type_param.id, &inner_ty)?;
            let vec_code = quote!(vec![#item_code, #item_code]);
            Ok(vec_code)
        }
        scale_info::TypeDef::Array(def) => {
            let inner_ty = type_gen.resolve_type(def.type_param.id);
            let item_code = type_def_example(type_gen, def.type_param.id, &inner_ty)?;
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
                let value = type_example(type_gen, f.id)?;
                fields.push(value)
            }
            Ok(quote!(( #(#fields),* )))
        }
        scale_info::TypeDef::Primitive(def) => Ok(primitive_example(def)),
        scale_info::TypeDef::Compact(def) => {
            let inner_code = type_example(type_gen, def.type_param.id)?;
            // I used this originally, but it turns out the compact part should be omitted:

            // let compact_path = resolve_type_path_omit_generics(type_gen, id);
            // Ok(quote!(#compact_path(#inner_code)))
            Ok(inner_code)
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
    unused_type_params: bool,
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
                let value_code = type_example(type_gen, f.ty.id)?;
                field_idents_and_values.push(quote!(#ident : #value_code));
            }
            // maybe add phantom data to struct / named composite enum
            let maybe_phantom = if unused_type_params {
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
                let value_code = type_example(type_gen, f.ty.id)?;
                field_values.push(value_code);
            }
            Ok(quote!(( #(#field_values ,)* )))
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

/// Simple Heuristics. Just makes array initialization easier if is `Copy`.
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

// /// This is a workaround, should probably be handled with syn::Expr
// ///
// /// e.g. HashMap<u8, u16> => HashMap
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

/// I know this function has a lot of code duplication, but it should suffice for this prototype.
///
/// It is mainly used to check if some phantom data needs to be added to a struct/enum
fn ty_has_unused_type_parameters(type_gen: &TypeGenerator, ty: &Type<PortableForm>) -> bool {
    let mut unused: BTreeSet<TypeParameter<PortableForm>> =
        BTreeSet::from_iter(ty.type_params.iter().cloned());

    match &ty.type_def {
        TypeDef::Composite(t) => {
            for f in &t.fields {
                if let Some(type_name) = &f.type_name {
                    let field_as_type_param = TypeParameter {
                        name: type_name.clone(), // not so nice, I know
                        ty: Some(f.ty),
                    };
                    unused.remove(&field_as_type_param);
                }

                let ty = type_gen.resolve_type(f.ty.id);
                for param in &ty.type_params {
                    unused.remove(param);
                }
            }
        }
        TypeDef::Variant(v) => {
            for v in &v.variants {
                for f in &v.fields {
                    if let Some(type_name) = &f.type_name {
                        let field_as_type_param = TypeParameter {
                            name: type_name.clone(), // not so nice, I know
                            ty: Some(f.ty),
                        };
                        unused.remove(&field_as_type_param);
                    }

                    let ty = type_gen.resolve_type(f.ty.id);
                    for param in &ty.type_params {
                        unused.remove(param);
                    }
                }
            }
        }
        TypeDef::Tuple(e) => {
            for f in &e.fields {
                let ty = type_gen.resolve_type(f.id);

                for param in &ty.type_params {
                    unused.remove(param);
                }
            }
        }
        TypeDef::Sequence(TypeDefSequence { type_param })
        | TypeDef::Array(TypeDefArray { type_param, .. })
        | TypeDef::Compact(TypeDefCompact { type_param }) => {
            let ty = type_gen.resolve_type(type_param.id);
            for param in &ty.type_params {
                unused.remove(param);
            }
        }
        TypeDef::Primitive(_) | TypeDef::BitSequence(_) => {}
    };

    let s = format!("{:?}", ty);
    if s.contains("Equivocation") {
        dbg!(ty);
    }

    !unused.is_empty()
}
