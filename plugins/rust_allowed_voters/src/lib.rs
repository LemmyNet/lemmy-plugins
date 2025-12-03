use extism_pdk::*;
use lemmy_api_common::person::GetPersonDetailsResponse;
use lemmy_api_common::plugin::PluginMetadata;
use lemmy_api_common::post::PostLikeForm;

// Returns info about the plugin which gets included in /api/v4/site
//go:wasmexport metadata
#[plugin_fn]
pub fn metadata() -> FnResult<Json<PluginMetadata>> {
    Ok(Json(PluginMetadata::new(
        "Allowed Voters",
        "https://github.com/LemmyNet/lemmy-plugins/",
        "Prevent users with few posts from voting",
    )))
}

#[plugin_fn]
pub fn post_before_vote(Json(vote): Json<PostLikeForm>) -> FnResult<Json<PostLikeForm>> {
    let lemmy_url = config::get("lemmy_url")?.unwrap();
    let person_id = vote.person_id.0;
    let req = HttpRequest {
        url: format!("{lemmy_url}api/v4/person?person_id={person_id}"),
        headers: Default::default(),
        method: Some("GET".to_string()),
    };
    let res: GetPersonDetailsResponse = http::request::<()>(&req, None)?.json()?;
    let person_post_count = res.person_view.person.post_count;
    let is_upvote = vote.vote_is_upvote;
    if person_post_count < 5 && !is_upvote {
        return Err(Error::msg("user is not allowed to downvote").into());
    }
    Ok(Json(vote))
}
