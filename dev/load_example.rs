use nicovideo_rust::nicovideo_rust;

#[tokio::main]
async fn main() {
    let session = nicovideo_rust::create_new_domand_session("sm9").await.unwrap();
    println!("{}",serde_json::to_string(&session.session.headers).unwrap());
}

