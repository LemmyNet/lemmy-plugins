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
    let is_downvote = vote.vote_is_upvote == Some(Some(false));

    let allowed_downvote_instances = config::get("allowed_downvote_instances")?.unwrap_or_default();
    let allowed_downvote_instances = allowed_downvote_instances.split(",").collect::<Vec<_>>();

    let req = HttpRequest {
        url: format!("{lemmy_url}api/v4/person?person_id={person_id}"),
        headers: Default::default(),
        method: Some("GET".to_string()),
    };
    let res: GetPersonDetailsResponse = http::request::<()>(&req, None)?.json()?;
    let person_is_local = res.person_view.person.local;
    if is_downvote && !person_is_local && !allowed_downvote_instances.is_empty() {
        let person_instance = res.person_view.person.ap_id.domain().unwrap();
        if !allowed_downvote_instances.contains(&person_instance) {
            return Err(Error::msg(format!("downvote not allowed from {person_instance}")).into());
        }
    }

    let min_posts_for_downvote: i32 = config::get("min_posts_for_downvote")?
        .as_deref()
        .map(str::parse)
        .unwrap_or(Ok(-1))?;
    let person_post_count = res.person_view.person.post_count;
    if min_posts_for_downvote != -1 && person_post_count < min_posts_for_downvote && is_downvote {
        return Err(Error::msg("user is not allowed to downvote").into());
    }
    Ok(Json(vote))
}
