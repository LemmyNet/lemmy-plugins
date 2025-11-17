use extism_pdk::FnResult;
use extism_pdk::FromBytes;
use extism_pdk::HttpRequest;
use extism_pdk::Json;
use extism_pdk::ToBytes;
use extism_pdk::config;
use extism_pdk::http;
use extism_pdk::plugin_fn;
use extism_pdk::var;
use lemmy_api_common::language::Language as LemmyLanguage;
use lemmy_api_common::plugin::PluginMetadata;
use lemmy_api_common::post::PostInsertForm;
use lingua::Language;
use lingua::LanguageDetector;
use lingua::LanguageDetectorBuilder;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

// Returns info about the plugin which gets included in /api/v4/site
#[plugin_fn]
pub fn metadata() -> FnResult<Json<PluginMetadata>> {
    Ok(Json(PluginMetadata {
        name: "Lingua".to_string(),
        url: Url::parse("https://github.com/LemmyNet/lemmy-plugins/").unwrap(),
        description: "Automatic language tagging for posts and comments".to_string(),
    }))
}

#[plugin_fn]
pub fn local_post_before_create(
    Json(mut form): Json<PostInsertForm>,
) -> FnResult<Json<PostInsertForm>> {
    if form.language_id.is_some() {
        // already has a language, no need to analyze
        return Ok(Json(form));
    }

    let content = format!("{} {}", form.name, form.body.clone().unwrap_or_default());

    // Usage: https://docs.rs/lingua/1.7.2/lingua/index.html
    // There are various optimizations available, which could be exposed as plugin settings
    let detector: LanguageDetector = LanguageDetectorBuilder::from_all_languages().build();
    let detected_language: Option<Language> = detector.detect_language_of(content);

    if let Some(detected_language) = detected_language {
        let all_langs = all_languages()?;
        let lang = all_langs
            .iter()
            .find(|l| l.code == detected_language.iso_code_639_1().to_string());
        form.language_id = lang.map(|l| l.id);
    }
    Ok(Json(form))
}

#[derive(Deserialize, Serialize, FromBytes, ToBytes)]
#[encoding(Json)]
struct AllLanguages(Vec<LemmyLanguage>);

fn all_languages() -> FnResult<Vec<LemmyLanguage>> {
    const KEY: &'static str = "all_languages";
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
        let res: Vec<LemmyLanguage> = http::request::<()>(&req, None)?.json()?;
        var::set(KEY, AllLanguages(res.clone()))?;
        Ok(res)
    }
}
