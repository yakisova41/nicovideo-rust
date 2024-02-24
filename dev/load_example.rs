use nicovideo_rust::nicovideo_rust;

#[tokio::main]
async fn main() {
    let session = nicovideo_rust::create_new_hls_session("sm9").await.unwrap();
    println!("{}", session.session.data.content_url);
}

