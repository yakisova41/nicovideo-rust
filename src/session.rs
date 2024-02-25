use crate::initial_watch_data::InitialWatchData;
use reqwest::{self};
use serde_json::Value;
mod session_payload;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::header;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionResponse {
    pub meta: Meta,
    pub data: Data,
    pub headers: header::ParsedHeaderMap
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionFetchResponse {
    pub meta: Meta,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub status: i64,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub session: Session,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: String,
    #[serde(rename = "recipe_id")]
    pub recipe_id: String,
    #[serde(rename = "content_id")]
    pub content_id: String,
    #[serde(rename = "content_src_id_sets")]
    pub content_src_id_sets: Vec<ContentSrcIdSet>,
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "timing_constraint")]
    pub timing_constraint: String,
    #[serde(rename = "keep_method")]
    pub keep_method: KeepMethod,
    pub protocol: Protocol,
    #[serde(rename = "play_seek_time")]
    pub play_seek_time: i64,
    #[serde(rename = "play_speed")]
    pub play_speed: f64,
    #[serde(rename = "play_control_range")]
    pub play_control_range: PlayControlRange,
    #[serde(rename = "content_uri")]
    pub content_uri: String,
    #[serde(rename = "session_operation_auth")]
    pub session_operation_auth: SessionOperationAuth,
    #[serde(rename = "content_auth")]
    pub content_auth: ContentAuth,
    #[serde(rename = "runtime_info")]
    pub runtime_info: RuntimeInfo,
    #[serde(rename = "client_info")]
    pub client_info: ClientInfo,
    #[serde(rename = "created_time")]
    pub created_time: i64,
    #[serde(rename = "modified_time")]
    pub modified_time: i64,
    pub priority: f64,
    #[serde(rename = "content_route")]
    pub content_route: i64,
    pub version: String,
    #[serde(rename = "content_status")]
    pub content_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentSrcIdSet {
    #[serde(rename = "content_src_ids")]
    pub content_src_ids: Vec<ContentSrcId>,
    #[serde(rename = "allow_subset")]
    pub allow_subset: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentSrcId {
    #[serde(rename = "src_id_to_mux")]
    pub src_id_to_mux: SrcIdToMux,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SrcIdToMux {
    #[serde(rename = "video_src_ids")]
    pub video_src_ids: Vec<String>,
    #[serde(rename = "audio_src_ids")]
    pub audio_src_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeepMethod {
    pub heartbeat: Heartbeat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heartbeat {
    pub lifetime: i64,
    #[serde(rename = "onetime_token")]
    pub onetime_token: String,
    #[serde(rename = "deletion_timeout_on_no_stream")]
    pub deletion_timeout_on_no_stream: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    pub name: String,
    pub parameters: Parameters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
    #[serde(rename = "http_parameters")]
    pub http_parameters: HttpParameters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpParameters {
    pub method: String,
    pub parameters: Parameters2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters2 {
    #[serde(rename = "hls_parameters")]
    pub hls_parameters: HlsParameters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HlsParameters {
    #[serde(rename = "segment_duration")]
    pub segment_duration: i64,
    #[serde(rename = "total_duration")]
    pub total_duration: i64,
    #[serde(rename = "transfer_preset")]
    pub transfer_preset: String,
    #[serde(rename = "use_ssl")]
    pub use_ssl: String,
    #[serde(rename = "use_well_known_port")]
    pub use_well_known_port: String,
    #[serde(rename = "media_segment_format")]
    pub media_segment_format: String,
    pub encryption: Encryption,
    #[serde(rename = "separate_audio_stream")]
    pub separate_audio_stream: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encryption {
    pub empty: Empty,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Empty {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayControlRange {
    #[serde(rename = "max_play_speed")]
    pub max_play_speed: f64,
    #[serde(rename = "min_play_speed")]
    pub min_play_speed: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionOperationAuth {
    #[serde(rename = "session_operation_auth_by_signature")]
    pub session_operation_auth_by_signature: SessionOperationAuthBySignature,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionOperationAuthBySignature {
    #[serde(rename = "created_time")]
    pub created_time: i64,
    #[serde(rename = "expire_time")]
    pub expire_time: i64,
    pub token: String,
    pub signature: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentAuth {
    #[serde(rename = "auth_type")]
    pub auth_type: String,
    #[serde(rename = "max_content_count")]
    pub max_content_count: i64,
    #[serde(rename = "content_key_timeout")]
    pub content_key_timeout: i64,
    #[serde(rename = "service_id")]
    pub service_id: String,
    #[serde(rename = "service_user_id")]
    pub service_user_id: String,
    #[serde(rename = "content_auth_info")]
    pub content_auth_info: ContentAuthInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentAuthInfo {
    pub method: String,
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInfo {
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "execution_history")]
    pub execution_history: Vec<Value>,
    #[serde(rename = "thumbnailer_state")]
    pub thumbnailer_state: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    #[serde(rename = "player_id")]
    pub player_id: String,
    #[serde(rename = "remote_ip")]
    pub remote_ip: String,
    #[serde(rename = "tracking_info")]
    pub tracking_info: String,
}


pub async fn get_session(initial_watch_data: &InitialWatchData) -> Result<SessionResponse, &str> {
    let client = reqwest::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.append(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json"));
    headers.append(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));

    let payload_data = session_payload::payload_generator(initial_watch_data);
    let payload = serde_json::to_string(&payload_data).unwrap();
    
    match client.post("https://api.dmc.nico/api/sessions?_format=json")
        .headers(headers)
        .body(payload)
        .send()
        .await  {
        Ok(res) => {
            let headers = res.headers();
            let parsed_headers = header::parse_headers(headers);
            let text = res.text().await.unwrap();
            let session: SessionFetchResponse = serde_json::from_str(&text).unwrap();
            Ok(SessionResponse {
                data: session.data,
                meta: session.meta,
                headers: parsed_headers
            })
        },
        Err(_) => Err("")
    }
}

pub async fn send_heart_beat(session: &SessionResponse) {
    let copy_session = session.clone();
    let client = reqwest::Client::new();

    let payload_data = Data {
        session: copy_session.data.session
    };
    let payload = serde_json::to_string(&payload_data).unwrap();

    let _ = client
        .post(format!("https://api.dmc.nico/api/sessions/{}?_format=json&_method=PUT", session.data.session.id))
        .body(payload)
        .send()
        .await
        ;
}