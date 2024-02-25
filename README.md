# nicovideo-rust
ニコニコ動画からセッション情報と動画情報を取得するだけのライブラリ。

## Usage
```rust
use nicovideo_rust::nicovideo_rust;

#[tokio::main]
async fn main() {
    // domand server
    let session = nicovideo_rust::create_new_domand_session("sm9").await.unwrap();
    println!("M3U8 URI: {}",&session.session.data.content_url);
    println!("session response headers: {}",serde_json::to_string(&session.session.headers).unwrap());

    // old server
    let session_old = nicovideo_rust::create_new_session("sm9").await.unwrap();
    println!("M3U8 URI: {}",&session_old.session.data.session.content_uri);
    println!("initial watch data response headers: {}",serde_json::to_string(&session_old.initial_watch_data.headers).unwrap());
}

```
