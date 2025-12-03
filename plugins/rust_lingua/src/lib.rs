use std::cell::LazyCell;

use extism_pdk::FnResult;
use extism_pdk::FromBytes;
use extism_pdk::HttpRequest;
use extism_pdk::Json;
use extism_pdk::ToBytes;
use extism_pdk::config;
use extism_pdk::http;
use extism_pdk::plugin_fn;
use extism_pdk::var;
use lemmy_api_common::comment::CommentInsertForm;
use lemmy_api_common::language::Language as LemmyLanguage;
use lemmy_api_common::language::LanguageId;
use lemmy_api_common::plugin::PluginMetadata;
use lemmy_api_common::post::PostInsertForm;
use lemmy_api_common::site::GetSiteResponse;
use lingua::Language;
use lingua::LanguageDetector;
use lingua::LanguageDetectorBuilder;
use serde::Deserialize;
use serde::Serialize;

// Returns info about the plugin which gets included in /api/v4/site
#[plugin_fn]
pub fn metadata() -> FnResult<Json<PluginMetadata>> {
    // initialize the detector because it takes a long time (~5s)
    LazyCell::<LanguageDetector>::force(&DETECTOR);

    Ok(Json(PluginMetadata::new(
        "Lingua",
        "https://github.com/LemmyNet/lemmy-plugins/",
        "Automatic language tagging for posts and comments",
    )))
}

// Usage: https://docs.rs/lingua/1.7.2/lingua/index.html
// There are various optimizations available, which could be exposed as plugin settings
const DETECTOR: LazyCell<LanguageDetector> =
    LazyCell::new(|| LanguageDetectorBuilder::from_all_languages().build());

#[plugin_fn]
pub fn local_post_before_create(
    Json(mut form): Json<PostInsertForm>,
) -> FnResult<Json<PostInsertForm>> {
    let content = format!("{} {}", form.name, form.body.clone().unwrap_or_default());
    detect_language(content, &mut form.language_id)?;
    Ok(Json(form))
}

#[plugin_fn]
pub fn local_comment_before_create(
    Json(mut form): Json<CommentInsertForm>,
) -> FnResult<Json<CommentInsertForm>> {
    detect_language(form.content.clone(), &mut form.language_id)?;
    Ok(Json(form))
}

#[plugin_fn]
pub fn federated_post_before_receive(
    Json(mut form): Json<PostInsertForm>,
) -> FnResult<Json<PostInsertForm>> {
    let content = format!("{} {}", form.name, form.body.clone().unwrap_or_default());
    detect_language(content, &mut form.language_id)?;
    Ok(Json(form))
}

#[plugin_fn]
pub fn federated_comment_before_receive(
    Json(mut form): Json<CommentInsertForm>,
) -> FnResult<Json<CommentInsertForm>> {
    detect_language(form.content.clone(), &mut form.language_id)?;
    Ok(Json(form))
}

fn detect_language(content: String, language_id: &mut Option<LanguageId>) -> FnResult<()> {
    if language_id.is_none() {
        let detected_language: Option<Language> = DETECTOR.detect_language_of(content);

        if let Some(detected_language) = detected_language {
            let all_langs = all_languages()?;
            let lang = all_langs
                .iter()
                .find(|l| l.code == detected_language.iso_code_639_1().to_string());
            *language_id = lang.map(|l| l.id);
        }
    }
    Ok(())
}

#[derive(Deserialize, Serialize, FromBytes, ToBytes)]
#[encoding(Json)]
struct AllLanguages(Vec<LemmyLanguage>);

fn all_languages() -> FnResult<Vec<LemmyLanguage>> {
    const KEY: &str = "all_languages";
    let langs = var::get::<AllLanguages>(KEY)?;
    if let Some(langs) = langs {
        Ok(langs.0)
    } else {
        let lemmy_url = config::get("lemmy_url")?.unwrap();
        let req = HttpRequest {
            url: format!("{lemmy_url}api/v4/site"),
            headers: Default::default(),
            method: Some("GET".to_string()),
        };
        let site: GetSiteResponse = http::request::<()>(&req, None)?.json()?;
        let langs = site.all_languages;
        var::set(KEY, AllLanguages(langs.clone()))?;
        Ok(langs)
    }
}
