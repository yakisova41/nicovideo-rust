use crate::initial_watch_data::InitialWatchData;
use reqwest;
use reqwest::header;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HlsResponse {
    pub meta: Meta,
    pub data: Data
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub status: i64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub content_url: String,
    pub create_time: String,
    pub expire_time: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HlsPayload {
    pub outputs: Vec<Vec<String>>
}

pub async fn get_hls(initial_watch_data: &InitialWatchData) -> Result<HlsResponse, &str> {
    let client = reqwest::Client::new();

    let mut outputs: Vec<Vec<String>> = Vec::new();
    let mut outputs_inner: Vec<String> = Vec::new();

    let copy = initial_watch_data.clone();

    let mut video_src_ids:Vec<String> = Vec::new();
    for video in copy.media.domand.videos {
        if video.is_available {
            video_src_ids.push(video.id);
        }
    };
    outputs_inner.push(video_src_ids.last().unwrap().to_string());

    let mut audio_src_ids:Vec<String> = Vec::new();
    for audio in copy.media.domand.audios {
        if audio.is_available {
            audio_src_ids.push(audio.id);
        }
    };
    outputs_inner.push(audio_src_ids.last().unwrap().to_string());

    outputs.push(outputs_inner);

    let payload_data = HlsPayload {
        outputs
    };
    let payload = serde_json::to_string(&payload_data).unwrap();

    let mut headers = header::HeaderMap::new();
    headers.insert("x-access-right-key", header::HeaderValue::from_str(&initial_watch_data.media.domand.access_right_key).unwrap());
    headers.insert("x-frontend-id", header::HeaderValue::from_str( "6").unwrap());
    headers.insert("x-frontend-version", header::HeaderValue::from_str("0").unwrap());
    headers.insert("x-request-with", header::HeaderValue::from_str("https://www.nicovideo.jp").unwrap());


    match client
        .post(
        format!("https://nvapi.nicovideo.jp/v1/watch/{}/access-rights/hls?actionTrackId={}",
                initial_watch_data.video.id,
                initial_watch_data.client.watch_track_id
            )
        )
        .body(payload)
        .headers(headers)
        .send()
        .await {
        Ok(res) => {
            let text = res.text().await.unwrap();
            let data:HlsResponse = serde_json::from_str(&text).unwrap();
            Ok(data)
        },
        Err(_) => Err("")
    }

}