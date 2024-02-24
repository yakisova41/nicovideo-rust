mod initial_watch_data;
mod session;
mod hls;

pub mod nicovideo_rust {
    use serde_derive::Deserialize;
    use serde_derive::Serialize;
    use crate::hls;
    use crate::hls::HlsResponse;
    use crate::{initial_watch_data::{self, InitialWatchData}, session::{self, SessionResponse}};

    #[allow(dead_code)]
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct NicoSession {
        pub session: SessionResponse,
        pub initial_watch_data: InitialWatchData
    }

    #[allow(dead_code)]
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct NicoHlsSession {
        pub session: HlsResponse,
        pub initial_watch_data: InitialWatchData
    }

    pub async fn create_new_session(video_id: &str) ->  Result<NicoSession, &str> {
        let initial_watch_data = initial_watch_data::get_initial_watch_data(video_id).await.unwrap();

        match initial_watch_data.media.delivery {
            Some(_) => {
                let session = session::get_session(&initial_watch_data).await.unwrap();
                Ok(NicoSession {
                    initial_watch_data,
                    session
                })
            },
            None => {
                Err("There is no delivery in the format of initial-watch-data")
            },
        }
    }

    pub async fn create_new_hls_session(video_id: &str) ->  Result<NicoHlsSession, &str> {
        let initial_watch_data = initial_watch_data::get_initial_watch_data(video_id).await.unwrap();

        let session = hls::get_hls(&initial_watch_data).await.unwrap();
        Ok(NicoHlsSession {
            initial_watch_data,
            session
        })
    }

    pub async fn send_heart_beat(session: &NicoSession) {
        session::send_heart_beat(&session.session).await;
    }    
}
