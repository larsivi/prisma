use serde;
use serde_json;
use crate::dml;

// This is a simple JSON serialization using Serde.
// The JSON format follows the DMMF spec, but is incomplete.

#[derive(Debug, serde::Serialize)]
pub struct Field {
    pub name: String,
    pub kind: String,
    pub dbName: Option<String>,
    pub arity: String,
    pub isUnique: bool,
    #[serde(rename = "type")]
    pub field_type: String
}

#[derive(Debug, serde::Serialize)]
pub struct Model {
    pub name: String,
    pub isEmbedded: bool,
    pub dbName: Option<String>,
    pub fields: Vec<Field>
}

#[derive(Debug, serde::Serialize)]
pub struct Datamodel {
    pub models: Vec<Model>
}

fn get_field_kind(field: &dml::Field) -> String {
    match field.field_type {
        dml::FieldType::Relation { to: _, to_field: _, name: _, on_delete: _} => String::from("relation"),
        dml::FieldType::Base(_) => String::from("scalar"),
        _ => unimplemented!("DMML does not support field type {:?}", field.field_type)
    }
}

fn type_to_string(scalar: &dml::ScalarType) -> String {
    match scalar {
        dml::ScalarType::Int => String::from("Int"),
        dml::ScalarType::Decimal => String::from("Decimal"),
        dml::ScalarType::Float => String::from("Float"),
        dml::ScalarType::Boolean => String::from("Boolean"),
        dml::ScalarType::String => String::from("String"),
        dml::ScalarType::DateTime => String::from("DateTime"),
        dml::ScalarType::Enum => panic!("Enum is an internally used type and should never be rendered.")
    }
}

fn get_field_type(field: &dml::Field) -> String {
    match &field.field_type {
        dml::FieldType::Relation { to: t, to_field: _, name: _, on_delete: _ } => t.clone(),
        dml::FieldType::Enum { enum_type: t } => t.clone(),
        dml::FieldType::Base(t) => type_to_string(t),
        dml::FieldType::ConnectorSpecific { base_type: t, connector_type: _ } => type_to_string(t)
    }
}

fn get_field_arity(field: &dml::Field) -> String {
    match field.arity {
        dml::FieldArity::Required => String::from("required"),
        dml::FieldArity::Optional => String::from("optional"),
        dml::FieldArity::List => String::from("list")
    }
}


pub fn field_to_dmmf(field: &dml::Field) -> Field {
    Field {
        name: field.name.clone(),
        kind: get_field_kind(field),
        dbName: field.database_name.clone(),
        arity: get_field_arity(field),
        isUnique: field.is_unique,
        field_type: get_field_type(field)
    }
}

pub fn model_to_dmmf(model: &dml::Model) -> Model {
    Model {
        name: model.name.clone(),
        dbName: model.database_name.clone(),
        isEmbedded: model.is_embedded,
        fields: model.fields.iter().map(&field_to_dmmf).collect()
    }
}

pub fn schema_to_dmmf(schema: &dml::Schema) -> Datamodel {
    let mut datamodel = Datamodel { models: vec![] };

    for obj in &schema.models {
        match obj {
            dml::ModelOrEnum::Enum(en) => unimplemented!("DMML has no enum support."),
            dml::ModelOrEnum::Model(model) => datamodel.models.push(model_to_dmmf(&model))
        }
    }

    return datamodel
}

pub fn render_to_dmmf(schema: &dml::Schema) -> String {
    let dmmf = schema_to_dmmf(schema);

    return serde_json::to_string_pretty(&dmmf).expect("Failed to render JSON");
}