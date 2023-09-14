//! Interface callable from JavaScript

use std::{convert::Infallible, fmt::Debug, ops::FromResidual};

use parity_scale_codec::Decode;
use serde::Serialize;
use subxt::{OfflineClient, OnlineClient, SubstrateConfig};
use subxt_metadata::{Metadata, PalletMetadata};
use wasm_bindgen::{convert::IntoWasmAbi, prelude::*};

use crate::{context::ExampleContext, ExampleGenerator};

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[wasm_bindgen]
pub fn make_pretty(code: &str) -> String {
    let syn_tree = syn::parse_file(&code).unwrap();
    let pretty = prettyplease::unparse(&syn_tree);
    pretty
}

#[wasm_bindgen]
pub struct Client {
    kind: ClientKind,
}

pub enum ClientKind {
    Offline {
        metadata_file_name: String,
        client: OfflineClient<SubstrateConfig>,
    },
    Online {
        url: String,
        client: OnlineClient<SubstrateConfig>,
    },
}

impl ClientKind {
    pub fn metadata(&self) -> subxt::Metadata {
        match self {
            ClientKind::Offline { client, .. } => client.metadata(),
            ClientKind::Online { client, .. } => client.metadata(),
        }
    }
}

#[wasm_bindgen]
impl Client {
    /// Creates an offline client from the metadata bytes and the name of the metadata file.
    ///
    /// genesis_hash and runtime_version are set to some unimportant default values and hopefully not used.
    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(
        metadata_file_name: &str,
        bytes: js_sys::Uint8Array,
    ) -> Result<Client, String> {
        // AFAIK no efficient way to just "view" the Uint8Array withput copying the bytes.
        let bytes = bytes.to_vec();
        let metadata: Metadata = Metadata::decode(&mut &bytes[..]).map_err(|e| format!("{e}"))?;

        let client = OfflineClient::<SubstrateConfig>::new(
            // should not matter much:
            Default::default(),
            // should not matter much:
            subxt::backend::RuntimeVersion {
                spec_version: 0,
                transaction_version: 0,
            },
            metadata,
        );

        Ok(Client {
            kind: ClientKind::Offline {
                metadata_file_name: metadata_file_name.into(),
                client,
            },
        })
    }

    /// Creates an OnlineClient from a given url
    #[wasm_bindgen(js_name = "fromUrl")]
    pub async fn from_url(url: &str) -> Result<Client, String> {
        console_log!("try to create client from url {url}");
        let client_or_err = subxt::OnlineClient::<SubstrateConfig>::from_url(url).await;
        match client_or_err {
            Ok(client) => Ok(Client {
                kind: ClientKind::Online {
                    url: url.into(),
                    client,
                },
            }),
            Err(err) => Err(format!("{err}")),
        }
    }

    #[wasm_bindgen(js_name = "metadataContent")]
    pub fn metadata_content(&self) -> JsValue {
        let metadata = self.kind.metadata();
        let contents = MetadataContent::from_metadata(&metadata);
        serde_wasm_bindgen::to_value(&contents).expect("should always work")
    }

    #[wasm_bindgen(js_name = "palletDocs")]
    pub fn pallet_docs(&self, pallet_name: &str) -> JsValue {
        let metadata = self.kind.metadata();
        let Some(pallet_metadata) = metadata.pallet_by_name(pallet_name) else {
            return JsValue::UNDEFINED;
        };
        serde_wasm_bindgen::to_value(pallet_metadata.docs()).expect("should always work")
    }

    #[wasm_bindgen(js_name = "palletContent")]
    pub fn pallet_content(&self, pallet_name: &str) -> JsValue {
        let metadata = self.kind.metadata();
        let Some(pallet_metadata) = metadata.pallet_by_name(pallet_name) else {
            console_log!("pallet {pallet_name} not found in metadata");
            return JsValue::UNDEFINED;
        };
        let pallet_content = PalletContent::from_pallet_metadata(pallet_metadata);
        console_log!("pallet {pallet_name} found, content: {pallet_content:?}");
        serde_wasm_bindgen::to_value(&pallet_content).expect("should always work")
    }

    #[wasm_bindgen(js_name = "callContent")]
    pub fn call_content(&self, pallet_name: &str, call_name: &str) -> MyJsValue {
        let metadata = self.kind.metadata();
        let pallet_metadata = metadata.pallet_by_name(pallet_name)?;
        let call = pallet_metadata.call_variant_by_name(call_name)?;

        let docs = &call.docs;

        // static code example
        let example_generator_static =
            ExampleGenerator::new(self.example_context(false), &metadata);
        let code_example_static =
            example_generator_static.call_example_wrapped(pallet_name, call_name)?;
        // dynamic code example
        let example_generator_dynamic =
            ExampleGenerator::new(self.example_context(true), &metadata);
        let code_example_dynamic =
            example_generator_dynamic.call_example_wrapped(pallet_name, call_name)?;
        let content: ItemContent = ItemContent {
            pallet_name,
            name: call_name,
            docs,
            code_example_static: &make_pretty(&code_example_static.to_string()),
            code_example_dynamic: &make_pretty(&code_example_dynamic.to_string()),
        };
        serde_wasm_bindgen::to_value(&content)
            .expect("should always work")
            .into()
    }

    #[wasm_bindgen(js_name = "storageEntryContent")]
    pub fn storage_entry_content(&self, pallet_name: &str, entry_name: &str) -> MyJsValue {
        let metadata = self.kind.metadata();
        let pallet_metadata = metadata.pallet_by_name(pallet_name)?;
        let storage = pallet_metadata.storage()?;
        let entry = storage.entry_by_name(entry_name)?;

        let docs = entry.docs();

        // static code example
        let example_generator_static =
            ExampleGenerator::new(self.example_context(false), &metadata);
        let code_example_static =
            example_generator_static.storage_example_wrapped(pallet_name, entry_name)?;

        // dynamic code example
        let example_generator_dynamic =
            ExampleGenerator::new(self.example_context(true), &metadata);
        let code_example_dynamic =
            example_generator_dynamic.storage_example_wrapped(pallet_name, entry_name)?;

        let content: ItemContent = ItemContent {
            pallet_name,
            name: entry_name,
            docs,
            code_example_static: &make_pretty(&code_example_static.to_string()),
            code_example_dynamic: &make_pretty(&code_example_dynamic.to_string()),
        };
        serde_wasm_bindgen::to_value(&content)
            .expect("should always work")
            .into()
    }

    fn example_context(&self, dynamic: bool) -> ExampleContext {
        match &self.kind {
            ClientKind::Offline {
                metadata_file_name, ..
            } => ExampleContext::from_file(metadata_file_name, dynamic),
            ClientKind::Online { url, .. } => ExampleContext::from_url(url, dynamic),
        }
    }
}

/// New-Type struct to implement short circuiting with `FromResidual` trait.
pub struct MyJsValue(JsValue);

impl FromResidual<std::option::Option<Infallible>> for MyJsValue {
    fn from_residual(residual: std::option::Option<Infallible>) -> Self {
        MyJsValue(JsValue::UNDEFINED)
    }
}

impl<T: Debug> FromResidual<Result<Infallible, T>> for MyJsValue {
    fn from_residual(residual: Result<Infallible, T>) -> Self {
        console_log!("Error: {residual:?}. Returning JsValue::UNDEFINED");
        MyJsValue(JsValue::UNDEFINED)
    }
}

impl From<JsValue> for MyJsValue {
    fn from(value: JsValue) -> Self {
        Self(value)
    }
}

impl wasm_bindgen::describe::WasmDescribe for MyJsValue {
    fn describe() {
        JsValue::describe()
    }
}

impl IntoWasmAbi for MyJsValue {
    type Abi = <JsValue as IntoWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        self.0.into_abi()
    }
}

#[derive(Serialize)]
pub struct MetadataContent<'a> {
    pallets: Vec<PalletContent<'a>>,
    runtime_apis: Vec<&'a str>,
    // note: String here, because I had lifetime issues otherwise
    custom_values: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct PalletContent<'a> {
    pub name: &'a str,
    pub calls: Vec<&'a str>,
    pub storage_entries: Vec<&'a str>,
    pub constants: Vec<&'a str>,
}

#[derive(Serialize)]
pub struct ItemContent<'a> {
    pub pallet_name: &'a str,
    pub name: &'a str,
    pub docs: &'a [String],
    pub code_example_static: &'a str,
    pub code_example_dynamic: &'a str,
}

impl<'a> MetadataContent<'a> {
    pub fn from_metadata(metadata: &'a subxt_metadata::Metadata) -> MetadataContent<'a> {
        let pallets = metadata
            .pallets()
            .map(|p| PalletContent::from_pallet_metadata(p))
            .collect();

        let runtime_apis: Vec<&str> = metadata.runtime_api_traits().map(|e| e.name()).collect();
        let custom_values: Vec<String> = metadata
            .custom()
            .iter()
            .map(|e| e.name().to_string())
            .collect();

        MetadataContent {
            pallets,
            runtime_apis,
            custom_values,
        }
    }
}

impl<'a> PalletContent<'a> {
    pub fn from_pallet_metadata(pallet_metadata: PalletMetadata<'a>) -> PalletContent<'a> {
        let name = pallet_metadata.name();
        let calls: Vec<&str> = if let Some(c) = pallet_metadata.call_variants() {
            c.iter().map(|c| c.name.as_str()).collect()
        } else {
            vec![]
        };
        let storage_items = if let Some(s) = pallet_metadata.storage() {
            s.entries().iter().map(|s| s.name()).collect()
        } else {
            vec![]
        };
        let constants = pallet_metadata.constants().map(|c| c.name()).collect();

        PalletContent {
            name,
            calls,
            storage_entries: storage_items,
            constants,
        }
    }
}

/*
Pallets:
 - assaasa
 - sasaass
 - assasas
 - assasas
   - Extrinsics
     - hahahah
     - ahshshh
     - ashashh
   - Storage Items
   - Constants

Custom Values
Runtime APIS
*/
