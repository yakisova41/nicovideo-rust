mod initial_watch_data;
mod session;
use initial_watch_data::InitialWatchData;
use session::SessionResponse;

#[allow(dead_code)]
#[derive(Clone)]

pub struct NicoSession {
    pub session: SessionResponse,
    pub initial_watch_data: InitialWatchData
}


pub async fn create_new_session(video_id: &str) ->  NicoSession {
    let initial_watch_data = initial_watch_data::get_initial_watch_data(video_id).await.unwrap();
    let session = session::get_session(&initial_watch_data).await.unwrap();

    let active_session = NicoSession {
        initial_watch_data,
        session
    };

    active_session
}

pub async fn send_heart_beat(session: &NicoSession) {
    session::send_heart_beat(&session.session).await;
}