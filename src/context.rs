use std::path::Path;

use quote::format_ident;
use subxt::ext::codec::Decode;
use subxt_codegen::RuntimeGenerator;
use subxt_metadata::Metadata;

pub struct ExampleContext {
    pub dynamic: bool,
    pub inter_face_ident: syn::Ident,
    pub file_or_url: FileOrUrl,
}

impl ExampleContext {
    pub fn from_file(file_path: &str, dynamic: bool) -> Self {
        ExampleContext {
            dynamic,
            inter_face_ident: format_ident!("runtime"),
            file_or_url: FileOrUrl::File(file_path.into()),
        }
    }
}

pub enum FileOrUrl {
    File(String),
    Url(String),
}

impl FileOrUrl {
    pub fn fetch_metadata(&self) -> anyhow::Result<Metadata> {
        match &self {
            FileOrUrl::File(path) => {
                let bytes = std::fs::read(path)?;
                let mut metadata = Metadata::decode(&mut &bytes[..])?;
                RuntimeGenerator::ensure_unique_type_paths(&mut metadata);
                Ok(metadata)
            }
            FileOrUrl::Url(_) => todo!("not yet implemented"),
        }
    }
}
