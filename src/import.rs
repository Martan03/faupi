use oas3::{
    OpenApiV3Spec,
    spec::{ObjectSchema, Operation, Schema, SchemaType, SchemaTypeSet},
};

use crate::{
    args::import::Import,
    error::Result,
    specs::{
        body::{Mapping, body::Body, type_constraint::TypeConstraint},
        method::Method,
        mock_config::MockConfig,
        response::Response,
        spec::Spec,
        status_code::StatusCode,
    },
};

impl Import {
    pub fn oas3_to_specs(oas: OpenApiV3Spec) -> Result<MockConfig> {
        let Some(paths) = oas.paths.clone() else {
            return Ok(MockConfig::default());
        };

        let mut specs = MockConfig::default();
        for (url, item) in paths.iter() {
            for (method, op) in item.methods() {
                let req = Self::oper_to_req(op, &oas)?;
                let res = Self::oper_to_res(op, &oas)?;

                let spec = Spec {
                    method: Method::try_from(method)?,
                    url: url.clone(),
                    request: req,
                    response: res,
                };
                specs.specs.push(spec);
            }
        }

        Ok(specs)
    }

    fn oper_to_req(
        op: &Operation,
        oas: &OpenApiV3Spec,
    ) -> Result<Option<Body>> {
        let Some(req) = &op.request_body else {
            return Ok(None);
        };

        let body = req.resolve(oas)?;
        if let Some(media) = body.content.get("application/json")
            && let Some(schema) = media.schema(oas)?
        {
            let body = Self::traverse_object(&schema, oas, true)?;
            return Ok(Some(body));
        }
        Ok(None)
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
            body = Self::traverse_object(&schema, oas, false)?;
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
        is_req: bool,
    ) -> Result<Body> {
        let ty = match &obj.schema_type {
            Some(SchemaTypeSet::Single(ty)) => ty.clone(),
            Some(SchemaTypeSet::Multiple(items)) => {
                items.first().copied().unwrap_or(SchemaType::Null)
            }
            None => return Ok(Body::Null),
        };

        match ty {
            SchemaType::Array => Self::parse_array(obj, spec, is_req),
            SchemaType::Object => Self::parse_object(obj, spec, is_req),
            SchemaType::String => Ok(if is_req {
                Body::Constraint(TypeConstraint::new("string", None))
            } else {
                Body::String(String::new())
            }),
            SchemaType::Integer => Ok(if is_req {
                Body::Constraint(TypeConstraint::new("number", None))
            } else {
                Body::Number(serde_yaml::Number::from(0))
            }),
            SchemaType::Number => Ok(if is_req {
                Body::Constraint(TypeConstraint::new("number", None))
            } else {
                Body::Number(serde_yaml::Number::from(0.0))
            }),
            SchemaType::Boolean => Ok(if is_req {
                Body::Constraint(TypeConstraint::new("boolean", None))
            } else {
                Body::Bool(true)
            }),
            SchemaType::Null => Ok(if is_req {
                Body::Constraint(TypeConstraint::new("any", None))
            } else {
                Body::Null
            }),
        }
    }

    fn traverse_schema(
        schema: &Schema,
        spec: &OpenApiV3Spec,
        is_req: bool,
    ) -> Result<Body> {
        match schema {
            Schema::Boolean(b) => Ok(if is_req {
                Body::Constraint(TypeConstraint::new("boolean", None))
            } else {
                Body::Bool(b.0)
            }),
            Schema::Object(obj_ref) => {
                let obj = obj_ref.resolve(spec)?;
                Self::traverse_object(&obj, spec, is_req)
            }
        }
    }

    fn parse_array(
        obj: &ObjectSchema,
        spec: &OpenApiV3Spec,
        is_req: bool,
    ) -> Result<Body> {
        if let Some(items) = &obj.items {
            let body = Self::traverse_schema(items, spec, is_req)?;
            Ok(Body::Sequence(vec![body]))
        } else if !obj.prefix_items.is_empty() {
            let mut items = vec![];
            for item in obj.prefix_items.iter() {
                let item = item.resolve(spec)?;
                items.push(Self::traverse_object(&item, spec, is_req)?);
            }
            Ok(Body::Sequence(items))
        } else {
            Ok(Body::Sequence(vec![]))
        }
    }

    fn parse_object(
        obj: &ObjectSchema,
        spec: &OpenApiV3Spec,
        is_req: bool,
    ) -> Result<Body> {
        let mut map = Mapping::new();
        for (key, prop) in obj.properties.iter() {
            let prop = prop.resolve(spec)?;
            let body = Self::traverse_object(&prop, spec, is_req)?;
            map.insert(Body::String(key.clone()), body);
        }
        Ok(Body::Mapping(map))
    }
}
