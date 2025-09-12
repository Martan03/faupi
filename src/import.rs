use oas3::{
    OpenApiV3Spec,
    spec::{ObjectSchema, Operation, Schema, SchemaType, SchemaTypeSet},
};

use crate::{
    args::import::Import,
    error::Result,
    specs::{
        body::{Mapping, body::Body},
        method::Method,
        response::Response,
        spec::Spec,
        specs_struct::Specs,
        status_code::StatusCode,
    },
};

impl Import {
    pub fn oas3_to_specs(oas: OpenApiV3Spec) -> Result<Specs> {
        let Some(paths) = oas.paths.clone() else {
            return Ok(Specs::default());
        };

        let mut specs = Specs::default();
        for (url, item) in paths.iter() {
            for (method, op) in item.methods() {
                let res = Self::oper_to_res(op, &oas)?;
                let spec = Spec {
                    method: Method::try_from(method)?,
                    url: url.clone(),
                    response: res,
                };
                specs.0.push(spec);
            }
        }

        Ok(specs)
    }

    fn oper_to_res(op: &Operation, oas: &OpenApiV3Spec) -> Result<Response> {
        let responses = op.responses(oas);
        let Some((status, res)) = responses.iter().next() else {
            return Ok(Response::default());
        };

        let mut body = Body::Null;
        if let Some(media) = res.content.get("application/json")
            && let Some(schema) = media.schema(oas)?
        {
            body = Self::traverse_object(&schema, oas)?;
        }

        Ok(Response {
            status: StatusCode::try_from(status.as_str())?,
            delay: None,
            body,
        })
    }

    fn traverse_object(
        obj: &ObjectSchema,
        spec: &OpenApiV3Spec,
    ) -> Result<Body> {
        let ty = match &obj.schema_type {
            Some(SchemaTypeSet::Single(ty)) => ty.clone(),
            Some(SchemaTypeSet::Multiple(items)) => {
                items.first().copied().unwrap_or(SchemaType::Null)
            }
            None => return Ok(Body::Null),
        };

        match ty {
            SchemaType::Array => Self::parse_array(obj, spec),
            SchemaType::Object => Self::parse_object(obj, spec),
            SchemaType::String => Ok(Body::String(String::new())),
            SchemaType::Integer => {
                Ok(Body::Number(serde_yaml::Number::from(0)))
            }
            SchemaType::Number => {
                Ok(Body::Number(serde_yaml::Number::from(0.0)))
            }
            SchemaType::Boolean => Ok(Body::Bool(true)),
            SchemaType::Null => Ok(Body::Null),
        }
    }

    fn traverse_schema(schema: &Schema, spec: &OpenApiV3Spec) -> Result<Body> {
        match schema {
            Schema::Boolean(b) => Ok(Body::Bool(b.0)),
            Schema::Object(obj_ref) => {
                let obj = obj_ref.resolve(spec)?;
                Self::traverse_object(&obj, spec)
            }
        }
    }

    fn parse_array(obj: &ObjectSchema, spec: &OpenApiV3Spec) -> Result<Body> {
        if let Some(items) = &obj.items {
            let body = Self::traverse_schema(items, spec)?;
            Ok(Body::Sequence(vec![body]))
        } else if !obj.prefix_items.is_empty() {
            let mut items = vec![];
            for item in obj.prefix_items.iter() {
                let item = item.resolve(spec)?;
                items.push(Self::traverse_object(&item, spec)?);
            }
            Ok(Body::Sequence(items))
        } else {
            Ok(Body::Sequence(vec![]))
        }
    }

    fn parse_object(obj: &ObjectSchema, spec: &OpenApiV3Spec) -> Result<Body> {
        let mut map = Mapping::new();
        for (key, prop) in obj.properties.iter() {
            let prop = prop.resolve(spec)?;
            let body = Self::traverse_object(&prop, spec)?;
            map.insert(Body::String(key.clone()), body);
        }
        Ok(Body::Mapping(map))
    }
}
