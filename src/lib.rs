mod initial_watch_data;
mod session;
mod domand;
mod header;

pub mod nicovideo_rust {
    use serde_derive::Deserialize;
    use serde_derive::Serialize;
    use crate::domand;
    use crate::domand::DomandResponse;
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
        pub session: DomandResponse,
        pub initial_watch_data: InitialWatchData
    }

    /**
     * セッションとinitial_watch_dataを取得します。
     * 旧配信サーバー用
     * 2024年初頭頃の動画からdomandサーバーのみになるため使用不可。
     */
    pub async fn create_new_session(video_id: &str) ->  Result<NicoSession, &str> {
        let initial_watch_data = initial_watch_data::get_initial_watch_data(video_id).await.unwrap();

        match initial_watch_data.data.media.delivery {
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

    /**
     * 旧配信サーバーのセッションの維持を行います。
     * ~40000ms?間隔で送信しないとセッションが放棄されます。
     */
    pub async fn send_heart_beat(session: &NicoSession) {
        session::send_heart_beat(&session.session).await;
    }

    /**
     * セッションとinitial_watch_dataを取得します。
     * ニコニコ動画2023年新配信サーバー"domand"用
     */
    pub async fn create_new_domand_session(video_id: &str) ->  Result<NicoHlsSession, &str> {
        let initial_watch_data = initial_watch_data::get_initial_watch_data(video_id).await.unwrap();

        let session = domand::get_domand_session(&initial_watch_data).await.unwrap();
        Ok(NicoHlsSession {
            initial_watch_data,
            session
        })
    }
}
