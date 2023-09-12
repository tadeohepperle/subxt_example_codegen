//! Interface callable from JavaScript

use anyhow::anyhow;
use parity_scale_codec::Decode;
use serde::Serialize;
use subxt::{OfflineClient, OnlineClient, SubstrateConfig};
use subxt_metadata::Metadata;
use wasm_bindgen::prelude::*;

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
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
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
}

#[derive(Serialize)]
pub struct MetadataContent<'a> {
    pallets: Vec<PalletContent<'a>>,
    runtime_apis: Vec<&'a str>,
    // note: String here, because I had lifetime issues otherwise
    custom_values: Vec<String>,
}

#[derive(Serialize)]
pub struct PalletContent<'a> {
    pub name: &'a str,
    pub calls: Vec<&'a str>,
    pub storage_entries: Vec<&'a str>,
    pub constants: Vec<&'a str>,
}

impl<'a> MetadataContent<'a> {
    pub fn from_metadata(metadata: &'a subxt_metadata::Metadata) -> MetadataContent<'a> {
        let pallets = metadata
            .pallets()
            .map(|p| {
                let name = p.name();
                let calls: Vec<&str> = if let Some(c) = p.call_variants() {
                    c.iter().map(|c| c.name.as_str()).collect()
                } else {
                    vec![]
                };
                let storage_items = if let Some(s) = p.storage() {
                    s.entries().iter().map(|s| s.name()).collect()
                } else {
                    vec![]
                };
                let constants = p.constants().map(|c| c.name()).collect();

                PalletContent {
                    name,
                    calls,
                    storage_entries: storage_items,
                    constants,
                }
            })
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
