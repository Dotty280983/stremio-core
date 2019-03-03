use crate::state_types::{EnvFuture, Environment, Request};
use crate::types::addons::{ResourceRequest, ResourceResponse};
use crate::types::*;
use futures::{future, Future};
use std::error::Error;
use std::marker::PhantomData;
use super::AddonTransport;
use serde_json::json;
use serde_json::value::Value;

pub struct AddonLegacyTransport<T: Environment> {
    pub env: PhantomData<T>,
}
impl<T: Environment> AddonTransport for AddonLegacyTransport<T> {
    fn get(req: &ResourceRequest) -> EnvFuture<Box<ResourceResponse>> {
        let fetch_req = match build_legacy_req(req) {
            Ok(r) => r,
            Err(e) => return Box::new(future::err(e.into())),
        };

        match &req.resource_ref.resource as &str {
            "catalog" => Box::new(
                T::fetch_serde::<_, Vec<MetaPreview>>(fetch_req)
                    .map(|r| Box::new(ResourceResponse::Metas { metas: *r }))
            ),
            "meta" => Box::new(
                T::fetch_serde::<_, MetaItem>(fetch_req)
                    .map(|r| Box::new(ResourceResponse::Meta{ meta: *r }))
            ),
            // @TODO streams
            // @TODO better error
            _ => Box::new(future::err("legacy transport: unsupported response".into())),
        }
    }
}

fn build_legacy_req(req: &ResourceRequest) -> Result<Request<()>, Box<dyn Error>> {
    let q_json = match &req.resource_ref.resource as &str {
        // @TODO
        "catalog" => {
	    // Just follows the convention set out by stremboard
	    // L287 cffb94e4a9c57f5872e768eff25164b53f004a2b
            let sort = if req.resource_ref.id == "top" {
                Value::Null
            } else {
                json!({
                    req.resource_ref.id.to_owned(): -1,
                    "popularity": -1
                })
            };
            json!({
                "jsonrpc": "2.0",
                "method": "stream.find",
                "params": [Value::Null, {
                    "query": {
                        "genre": req.resource_ref.get_extra_first_val("genre"),
                    },
                    "sort": sort,
                    "limit": 100,
                    "skip": req.resource_ref.get_extra_first_val("skip"),
                }]
            })
        },
        "meta" => json!({}),
        "streams" => json!({}),
        // @TODO better error
        _ => return Err("legacy transport: unsupported resource".into()),
    };
    // NOTE: tihs is not using a URL safe base64 standard, which means that technically this is
    // not safe; however, the original implementation of stremio-addons work the same way,
    // so we're technically replicating a legacy bug on purpose
    // https://github.com/Stremio/stremio-addons/blob/v2.8.14/rpc.js#L53
    let param_str = base64::encode(&serde_json::to_string(&q_json)?);
    let url = format!("{}/q.json?b={}", &req.transport_url, param_str);
    Ok(Request::get(&url).body(())?)
}
