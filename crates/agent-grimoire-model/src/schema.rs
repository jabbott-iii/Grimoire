//! JSON Schemas generated from the Rust types, the single source of truth for `schemas/`.

use std::collections::BTreeMap;

use schemars::generate::SchemaSettings;
use schemars::{JsonSchema, Schema, SchemaGenerator};

use crate::{AssetManifest, Bundle, Lockfile, ProjectConfig};

/// One generated schema and the file it is written to under `schemas/`.
#[derive(Debug, Clone)]
pub struct SchemaDocument {
    /// File name under `schemas/`.
    pub file_name: &'static str,
    /// The schema.
    pub schema: Schema,
}

/// Generates the schema for `T`, pinned to draft 2020-12 so that a schemars
/// upgrade cannot silently change the published draft.
fn generate<T: JsonSchema>() -> Schema {
    SchemaSettings::draft2020_12()
        .into_generator()
        .into_root_schema_for::<T>()
}

/// Every schema Grimoire publishes, in a fixed order.
#[must_use]
pub fn documents() -> Vec<SchemaDocument> {
    vec![
        SchemaDocument {
            file_name: "asset.schema.json",
            schema: generate::<AssetManifest>(),
        },
        SchemaDocument {
            file_name: "bundle.schema.json",
            schema: generate::<Bundle>(),
        },
        SchemaDocument {
            file_name: "grimoire.schema.json",
            schema: generate::<ProjectConfig>(),
        },
        SchemaDocument {
            file_name: "grimoire-lock.schema.json",
            schema: generate::<Lockfile>(),
        },
    ]
}

/// The schema of a map whose keys must match `K`'s schema.
///
/// schemars only carries a key's `pattern` over to the map schema, so it drops
/// length limits and enum keys. This adds `propertyNames` so the schema
/// rejects what the parser rejects.
pub(crate) fn keyed_map<K: JsonSchema, V: JsonSchema>(generator: &mut SchemaGenerator) -> Schema {
    let mut schema = <BTreeMap<String, V>>::json_schema(generator);
    let keys = generator.subschema_for::<K>().to_value();
    schema.insert("propertyNames".into(), keys);
    schema
}
