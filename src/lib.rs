mod initial_watch_data;
mod session;
mod domand;
mod header;

pub mod nicovideo_rust {
    use reqwest::Response;
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
    pub struct NicoDomandSession {
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
    pub async fn create_new_domand_session(video_id: &str) ->  Result<NicoDomandSession, &str> {
        let initial_watch_data = initial_watch_data::get_initial_watch_data(video_id).await.unwrap();

        let session = domand::get_domand_session(&initial_watch_data).await.unwrap();
        Ok(NicoDomandSession {
            initial_watch_data,
            session
        })
    }

    /**
     * domand
     * HLSにアクセスする際のreqwestへのクッキー付与を代わりにするラッパー
     */
    pub async fn fetch_by_domand_session(session: NicoDomandSession, url: String) -> Response {
        let client = reqwest::Client::new();

        let i_raw_cookie = session.initial_watch_data.headers.get("set-cookie").unwrap();
        let i_split = i_raw_cookie.split(";").collect::<Vec<_>>();
        let i_cookie = format!("{}{}", i_split.get(0).unwrap(), ";");

        let s_raw_cookie = session.session.headers.get("set-cookie").unwrap();
        let s_split = s_raw_cookie.split(";").collect::<Vec<_>>();
        let s_cookie = format!("{}{}", s_split.get(0).unwrap(), ";");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("cookie", reqwest::header::HeaderValue::from_str(&format!("{}{}", &i_cookie, &s_cookie)).unwrap());

        let res = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .unwrap();

        res
    }
}
