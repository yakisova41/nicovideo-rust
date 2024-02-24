use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::initial_watch_data::InitialWatchData;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionPayload {
    session: SessionPayloadSession
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionPayloadSession {
    recipe_id: String,
    content_id: String,
    content_type: String,
    timing_constraint: String,
    content_uri: String,
    keep_method:KeepMethod,
    priority: i64,
    protocol: SessionPayloadProtocol,
    session_operation_auth: SessionOperationAuth,
    client_info: ClientInfo,
    content_auth: ContentAuth,
    content_src_id_sets: Vec<ContentSrcIdSet>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionPayloadProtocol {
    name: String,
    parameters: Parameters
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameters {
    http_parameters: HttpParameters
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct  HttpParameters {
    parameters: ParametersHttpParametersParameters
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParametersHttpParametersParameters {
    hls_parameters: HlsParameters
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HlsParameters {
    use_well_known_port: String,
    use_ssl: String,
    transfer_preset: String,
    segment_duration: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionOperationAuth {
    session_operation_auth_by_signature: SessionOperationAuthBySignature
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionOperationAuthBySignature {
    token:String,
    signature: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientInfo {
    player_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentAuth {
    auth_type: String,
    content_key_timeout: i64,
    service_id: String,
    service_user_id:String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentSrcIdSet {
    content_src_ids: Vec<ContentSrcId>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentSrcId {
    src_id_to_mux: SrcIdToMux
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SrcIdToMux {
    audio_src_ids: Vec<String>,
    video_src_ids: Vec<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeepMethod {
    heartbeat: HeartBeat
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct HeartBeat {
    lifetime: i64
}

pub fn payload_generator(initial_watch_data: &InitialWatchData) -> SessionPayload {
    let copy = initial_watch_data.clone();

    match copy.media.delivery {
        Some(delivery) => {
            let protocol = SessionPayloadProtocol {
                name: "http".to_owned(),
                parameters: Parameters {
                    http_parameters : HttpParameters {
                        parameters : ParametersHttpParametersParameters {
                            hls_parameters : HlsParameters {
                                use_well_known_port: "yes".to_owned(),
                                use_ssl: "yes".to_owned(),
                                transfer_preset: "".to_owned(),
                                segment_duration: 6000
                            }
                        }
                    }
                }
            };
        
            let session_operation_auth = SessionOperationAuth {
                session_operation_auth_by_signature: SessionOperationAuthBySignature {
                    token:delivery.movie.session.token,
                    signature:delivery.movie.session.signature
                },
            };
        
            let mut audio_src_ids:Vec<String> = Vec::new();
            for audio in delivery.movie.audios {
                if audio.is_available {
                    audio_src_ids.push(audio.id);
                }
            };
        
            let mut video_src_ids:Vec<String> = Vec::new();
            for video in delivery.movie.videos {
                if video.is_available {
                    video_src_ids.push(video.id);
                }
            };
        
        
            let mut content_src_ids: Vec<ContentSrcId> = Vec::new();
            content_src_ids.push(ContentSrcId {
                src_id_to_mux: SrcIdToMux {
                    audio_src_ids,
                    video_src_ids,
                }
            });
        
            let mut content_src_id_sets:Vec<ContentSrcIdSet> = Vec::new();
            content_src_id_sets.push(ContentSrcIdSet {
                content_src_ids
            });
        
            SessionPayload {
                session: SessionPayloadSession {
                    recipe_id: format!("nicovideo-{}", copy.video.id),
                    content_id: "out1".to_owned(),
                    content_type: "movie".to_owned(),
                    timing_constraint: "unlimited".to_owned(),
                    priority: 0,
                    protocol,            
                    session_operation_auth,
                    content_auth: ContentAuth {
                        auth_type: "ht2".to_owned(),
                        content_key_timeout: 600000,
                        service_id: "nicovideo".to_owned(),
                        service_user_id: delivery.movie.session.service_user_id,
                    },
                    content_uri: "".to_owned(),
                    keep_method: KeepMethod {
                        heartbeat: HeartBeat {
                            lifetime: 120000
                        }
                    },
                    client_info: ClientInfo {
                        player_id: delivery.movie.session.player_id
                    },
                    content_src_id_sets,
                }
            }
        },
        None => {
            panic!("")
        },
    }
}