use crate::json::Value;
use extism_pdk::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Metadata {
    name: String,
    url: String,
    description: String,
}

// Returns info about the plugin which gets included in /api/v4/site
//go:wasmexport metadata
#[plugin_fn]
pub fn metadata() -> FnResult<Json<Metadata>> {
    Ok(Json(Metadata {
        name: "Test Plugin".to_string(),
        url: "https://example.com".to_string(),
        description: "Plugin to test Lemmy feature".to_string(),
    }))
}

#[plugin_fn]
pub fn post_vote(
    Json(vote): Json<HashMap<String, Value>>,
) -> FnResult<Json<HashMap<String, Value>>> {
    if vote.get("person_id").and_then(Value::as_i64) == Some(2)
        && vote.get("score").and_then(Value::as_i64) == Some(-1)
    {
        return Err(Error::msg("user is not allowed to downvote").into());
    }
    Ok(Json(vote))
}
