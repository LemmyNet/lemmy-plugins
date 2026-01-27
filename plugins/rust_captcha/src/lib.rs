use anyhow::anyhow;
use base64::{engine::general_purpose::STANDARD_NO_PAD as base64, Engine};
use captcha::generate;
use captcha::Captcha;
use captcha::Difficulty;
use extism_pdk::plugin_fn;
use extism_pdk::var;
use extism_pdk::FnResult;
use extism_pdk::Json;
use extism_pdk::ToBytes;
use lemmy_api_common::account::auth::CaptchaResponse;
use lemmy_api_common::plugin::PluginMetadata;
use serde::Deserialize;
use serde::Serialize;
use std::io::Cursor;
use uuid::Uuid;

// Returns info about the plugin which gets included in /api/v4/site
#[plugin_fn]
pub fn metadata() -> FnResult<Json<PluginMetadata>> {
    Ok(Json(PluginMetadata::new(
        "Captcha",
        "https://github.com/LemmyNet/lemmy-plugins/",
        "Captcha for signup",
    )))
}

#[plugin_fn]
pub fn get_captcha() -> FnResult<Json<CaptchaResponse>> {
    // TODO: add config option for difficulty
    let captcha = generate(Difficulty::Medium);

    let answer = captcha.chars_as_string();

    let png = captcha
        .as_base64()
        .ok_or(anyhow!("failed to create captcha image"))?;

    let wav = captcha_as_wav_base64(&captcha)?;
    let uuid = Uuid::new_v4().to_string();

    var::set(uuid.clone(), answer)?;

    Ok(Json(CaptchaResponse { png, wav, uuid }))
}

// TODO: this is currently defined twice, here and in lemmy
#[derive(ToBytes, Deserialize, Serialize)]
#[encoding(Json)]
struct CaptchaAnswer {
    answer: String,
    uuid: String,
}

#[plugin_fn]
pub fn validate_captcha(Json(form): Json<CaptchaAnswer>) -> FnResult<bool> {
    let answer: String = var::get(form.uuid)?.unwrap_or_default();
    return Ok(answer == form.answer);
}

/// Converts the captcha to a base64 encoded wav audio file
pub(crate) fn captcha_as_wav_base64(captcha: &Captcha) -> FnResult<String> {
    let letters = captcha.as_wav();

    // Decode each wav file, concatenate the samples
    let mut concat_samples: Vec<i16> = Vec::new();
    let mut any_header: Option<hound::WavSpec> = None;
    for letter in letters {
        let mut cursor = Cursor::new(letter.unwrap_or_default());
        let reader = hound::WavReader::new(&mut cursor)?;
        any_header = Some(reader.spec());
        let samples16 = reader
            .into_samples::<i16>()
            .collect::<Result<Vec<_>, _>>()?;
        concat_samples.extend(samples16);
    }

    // Encode the concatenated result as a wav file
    let mut output_buffer = Cursor::new(vec![]);
    if let Some(header) = any_header {
        let mut writer = hound::WavWriter::new(&mut output_buffer, header)?;
        let mut writer16 = writer.get_i16_writer(concat_samples.len().try_into()?);
        for sample in concat_samples {
            writer16.write_sample(sample);
        }
        writer16.flush()?;
        writer.finalize()?;

        Ok(base64.encode(output_buffer.into_inner()))
    } else {
        Err(anyhow!("Failed to create audio captcha"))?
    }
}
