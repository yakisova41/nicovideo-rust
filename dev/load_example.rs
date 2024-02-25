use nicovideo_rust::nicovideo_rust;

#[tokio::main]
async fn main() {
    let session = nicovideo_rust::create_new_domand_session("sm9").await.unwrap();
    let copy = session.clone();

    //println!("M3U8 URI: {}",&session.session.data.content_url);
    //println!("session response headers: {}",serde_json::to_string(&session.session.headers).unwrap());
    let _m3u8Response = nicovideo_rust::fetch_by_domand_session(session, copy.session.data.content_url).await;
}

