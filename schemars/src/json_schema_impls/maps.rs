use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;

macro_rules! map_impl {
    ($($desc:tt)+) => {
        impl $($desc)+
        where
            K: JsonSchema,
            V: JsonSchema,
        {
            fn is_referenceable() -> bool {
                true
            }

            fn schema_name() -> String {
                format!("Map_of_{}_to_{}", K::schema_name(), V::schema_name())
            }

            fn json_schema(gen: &mut SchemaGenerator) -> Schema {
                use crate::Map;

                let key_subschema = gen.subschema_for::<K>();
                let value_subschema = gen.subschema_for::<V>();

                let key_schema = gen.definitions().get(&K::schema_name());
                let value_schema = gen.definitions().get(&V::schema_name());

                let mut examples = serde_json::Map::default();
                if let (Some(key_schema), Some(value_schema)) = (key_schema, value_schema) {
                    let key_examples = key_schema
                        .clone()
                        .into_object()
                        .metadata
                        .map(|m| m.examples)
                        .unwrap_or_default();
                    let value_examples = value_schema
                        .clone()
                        .into_object()
                        .metadata
                        .map(|m| m.examples)
                        .unwrap_or_default();

                    for (k, v) in key_examples.into_iter().zip(value_examples.into_iter()) {
                        examples.insert(
                            k.as_str()
                                .map(ToString::to_string)
                                .unwrap_or_else(|| k.to_string()),
                            v,
                        );
                    }
                }

                let examples: serde_json::Value = examples.into();

                let meta = Metadata {
                    description: Some(format!(
                        "Map of [{k}](#section/{k}) to [{v}](#section/{v}).\n\n Please Check the type sections for more information.",
                        k = K::schema_name(),
                        v = V::schema_name(),
                    )),
                    examples: vec![examples],
                    ..Default::default()
                };

                let pattern_properties: Map<_, _> = [("".to_string(), value_subschema)].into();

                SchemaObject {
                    instance_type: Some(InstanceType::Object.into()),
                    object: Some(Box::new(ObjectValidation {
                        // additional_properties: Some(Box::new(value_subschema)),
                        pattern_properties,
                        property_names: Some(Box::new(key_subschema)),
                        ..Default::default()
                    })),
                    metadata: Some(Box::new(meta)),
                    ..Default::default()
                }
                .into()
            }
        }
    };
}

map_impl!(<K, V> JsonSchema for std::collections::BTreeMap<K, V>);
map_impl!(<K, V, H> JsonSchema for std::collections::HashMap<K, V, H>);
