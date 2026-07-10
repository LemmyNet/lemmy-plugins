use extism_pdk::*;
use lemmy_api_common::person::GetPersonDetailsResponse;
use lemmy_api_common::plugin::PluginMetadata;
use lemmy_api_common::post::PostLikeForm;
use serde::Deserialize;
use serde::de::IntoDeserializer;

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

#[derive(PartialEq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
enum VotingMode {
    #[default]
    All,
    Local,
    None,
}

fn parse_voting_mode(s: Option<String>) -> Result<VotingMode, serde::de::value::Error> {
    let s = s.unwrap_or("all".to_string());
    VotingMode::deserialize(s.into_deserializer())
}
#[plugin_fn]
pub fn post_before_vote(Json(vote): Json<PostLikeForm>) -> FnResult<Json<PostLikeForm>> {
    let lemmy_url = config::get("lemmy_url")?.unwrap();
    let person_id = vote.person_id.0;
    let is_upvote = vote.vote_is_upvote == Some(Some(true));
    let is_downvote = vote.vote_is_upvote == Some(Some(false));

    let upvote_mode: VotingMode = parse_voting_mode(config::get("upvote_mode")?)?;
    let downvote_mode: VotingMode = parse_voting_mode(config::get("downvote_mode")?)?;

    let req = HttpRequest {
        url: format!("{lemmy_url}api/v4/person?person_id={person_id}"),
        headers: Default::default(),
        method: Some("GET".to_string()),
    };
    let res: GetPersonDetailsResponse = http::request::<()>(&req, None)?.json()?;
    let is_local = res.person_view.person.local;
    if is_upvote
        && (upvote_mode == VotingMode::None || (upvote_mode == VotingMode::Local && !is_local))
    {
        return Err(Error::msg("upvote not allowed").into());
    }
    if is_downvote
        && (downvote_mode == VotingMode::None || (downvote_mode == VotingMode::Local && !is_local))
    {
        return Err(Error::msg("downvote not allowed").into());
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
