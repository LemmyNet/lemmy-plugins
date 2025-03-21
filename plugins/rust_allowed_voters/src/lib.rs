use crate::json::Value;
use extism_pdk::*;
use lemmy_api_common::person::GetPersonDetailsResponse;
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
    let lemmy_url = config::get("lemmy_url")?.unwrap();
    let person_id = vote.get("person_id").unwrap();
    let req = HttpRequest {
        url: format!("{lemmy_url}api/v4/person?person_id={person_id}"),
        headers: Default::default(),
        method: Some("GET".to_string()),
    };
    let res: GetPersonDetailsResponse = http::request::<()>(&req, None)?.json()?;
    let person_post_count = res.person_view.person.post_count;
    let vote_score = vote.get("like_score").and_then(Value::as_i64).unwrap();
    if person_post_count < 5 && vote_score == -1 {
        return Err(Error::msg("user is not allowed to downvote").into());
    }
    Ok(Json(vote))
}
