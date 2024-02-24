use reqwest;
use scraper::{Html, Selector};
use serde_json::Value;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialWatchData {
    pub ads: Value,
    pub category: Value,
    pub channel: Value,
    pub client: Client,
    pub comment: Comment,
    pub community: Value,
    pub easy_comment: EasyComment,
    pub external: External,
    pub genre: Genre,
    pub marquee: Marquee,
    pub media: Media,
    pub ok_reason: String,
    pub owner: Value,
    pub payment: Payment,
    pub pc_watch_page: PcWatchPage,
    pub player: Player,
    pub ppv: Value,
    pub ranking: Ranking,
    pub series: Option<Series>,
    pub smartphone: Value,
    pub system: System,
    pub tag: Tag,
    pub video: Video5,
    pub video_ads: VideoAds,
    pub video_live: Value,
    pub viewer: Option<Viewer4>,
    pub waku: Waku,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub nicosid: String,
    pub watch_id: String,
    pub watch_track_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub server: Server,
    pub keys: Keys,
    pub layers: Vec<Layer>,
    pub threads: Vec<Thread>,
    pub ng: Ng,
    pub is_attention_required: bool,
    pub nv_comment: NvComment,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keys {
    pub user_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub index: i64,
    pub is_translucent: bool,
    pub thread_ids: Vec<ThreadId>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadId {
    pub id: i64,
    pub fork: i64,
    pub fork_label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub id: i64,
    pub fork: i64,
    pub fork_label: String,
    pub video_id: String,
    pub is_active: bool,
    pub is_default_post_target: bool,
    pub is_easy_comment_post_target: bool,
    pub is_leaf_required: bool,
    pub is_owner_thread: bool,
    pub is_threadkey_required: bool,
    pub threadkey: Value,
    #[serde(rename = "is184Forced")]
    pub is184forced: bool,
    pub has_nicoscript: bool,
    pub label: String,
    pub postkey_status: i64,
    pub server: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ng {
    pub ng_score: NgScore,
    pub channel: Vec<Value>,
    pub owner: Vec<Value>,
    pub viewer: Option<Viewer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NgScore {
    pub is_disabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
    pub revision: i64,
    pub count: i64,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "type")]
    pub type_field: String,
    pub source: String,
    pub registered_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NvComment {
    pub thread_key: String,
    pub server: String,
    pub params: Params,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub targets: Vec<Target>,
    pub language: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub id: String,
    pub fork: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EasyComment {
    pub phrases: Vec<Phrase>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phrase {
    pub text: String,
    pub nicodic: Option<Nicodic>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nicodic {
    pub title: String,
    pub view_title: String,
    pub summary: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct External {
    pub commons: Commons,
    pub ichiba: Ichiba,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commons {
    pub has_content_tree: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ichiba {
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub key: String,
    pub label: String,
    pub is_immoral: bool,
    pub is_disabled: bool,
    pub is_not_set: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Marquee {
    pub is_disabled: bool,
    pub tag_related_lead: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub domand: Domand,
    pub delivery: Option<Delivery>,
    pub delivery_legacy: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domand {
    pub videos: Vec<Video>,
    pub audios: Vec<Audio>,
    pub is_storyboard_available: bool,
    pub access_right_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: String,
    pub is_available: bool,
    pub label: String,
    pub bit_rate: i64,
    pub width: i64,
    pub height: i64,
    pub quality_level: i64,
    pub recommended_highest_audio_quality_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub id: String,
    pub is_available: bool,
    pub bit_rate: i64,
    pub sampling_rate: i64,
    pub integrated_loudness: f64,
    pub true_peak: f64,
    pub quality_level: i64,
    pub loudness_collection: Vec<LoudnessCollection>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoudnessCollection {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delivery {
    pub recipe_id: String,
    pub encryption: Value,
    pub movie: Movie,
    pub storyboard: Value,
    pub tracking_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    pub content_id: String,
    pub audios: Vec<Audio2>,
    pub videos: Vec<Video2>,
    pub session: Session,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio2 {
    pub id: String,
    pub is_available: bool,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub bitrate: i64,
    pub sampling_rate: i64,
    pub loudness: Loudness,
    pub level_index: i64,
    pub loudness_collection: Vec<LoudnessCollection2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Loudness {
    pub integrated_loudness: f64,
    pub true_peak: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoudnessCollection2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video2 {
    pub id: String,
    pub is_available: bool,
    pub metadata: Metadata2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata2 {
    pub label: String,
    pub bitrate: i64,
    pub resolution: Resolution,
    pub level_index: i64,
    pub recommended_highest_audio_level_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub recipe_id: String,
    pub player_id: String,
    pub videos: Vec<String>,
    pub audios: Vec<String>,
    pub movies: Vec<Value>,
    pub protocols: Vec<String>,
    pub auth_types: AuthTypes,
    pub service_user_id: String,
    pub token: String,
    pub signature: String,
    pub content_id: String,
    pub heartbeat_lifetime: i64,
    pub content_key_timeout: i64,
    pub priority: f64,
    pub transfer_presets: Vec<Value>,
    pub urls: Vec<Url>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthTypes {
    pub http: String,
    pub hls: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub url: String,
    pub is_well_known_port: bool,
    pub is_ssl: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub video: Video3,
    pub preview: Preview,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video3 {
    pub is_ppv: bool,
    pub is_admission: bool,
    pub is_continuation_benefit: bool,
    pub is_premium: bool,
    pub watchable_user_type: String,
    pub commentable_user_type: String,
    pub billing_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub ppv: Ppv,
    pub admission: Admission,
    pub continuation_benefit: ContinuationBenefit,
    pub premium: Premium,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ppv {
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Admission {
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuationBenefit {
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Premium {
    pub is_enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PcWatchPage {
    pub tag_related_banner: Value,
    pub video_end: VideoEnd,
    pub show_owner_menu: bool,
    pub show_owner_thread_co_editing_link: bool,
    pub show_mymemory_editing_link: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoEnd {
    pub banner_in: Value,
    pub overlay: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub initial_playback: Value,
    pub comment: Comment2,
    pub layer_mode: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment2 {
    pub is_default_invisible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ranking {
    pub genre: Option<Genre2>,
    pub popular_tag: Vec<PopularTag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre2 {
    pub rank: i64,
    pub genre: String,
    pub date_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PopularTag {
    pub tag: String,
    pub regularized_tag: String,
    pub rank: i64,
    pub genre: String,
    pub date_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub thumbnail_url: String,
    pub video: Video4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video4 {
    pub prev: Prev,
    pub next: Option<Next>,
    pub first: First,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prev {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub title: String,
    pub registered_at: String,
    pub count: Count,
    pub thumbnail: Thumbnail,
    pub duration: i64,
    pub short_description: String,
    pub latest_comment_summary: String,
    pub is_channel_video: bool,
    pub is_payment_required: bool,
    pub playback_position: Value,
    pub owner: Owner,
    pub require_sensitive_masking: bool,
    pub video_live: Value,
    pub is_muted: bool,
    #[serde(rename = "9d091f87")]
    pub n9d091f87: bool,
    pub acf68865: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub view: i64,
    pub comment: i64,
    pub mylist: i64,
    pub like: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String,
    pub middle_url: String,
    pub large_url: String,
    pub listing_url: String,
    pub n_hd_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub owner_type: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub visibility: String,
    pub id: String,
    pub name: Value,
    pub icon_url: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Next {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub title: String,
    pub registered_at: String,
    pub count: Count2,
    pub thumbnail: Thumbnail2,
    pub duration: i64,
    pub short_description: String,
    pub latest_comment_summary: String,
    pub is_channel_video: bool,
    pub is_payment_required: bool,
    pub playback_position: Value,
    pub owner: Owner2,
    pub require_sensitive_masking: bool,
    pub video_live: Value,
    pub is_muted: bool,
    #[serde(rename = "9d091f87")]
    pub n9d091f87: bool,
    pub acf68865: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Count2 {
    pub view: i64,
    pub comment: i64,
    pub mylist: i64,
    pub like: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail2 {
    pub url: String,
    pub middle_url: String,
    pub large_url: String,
    pub listing_url: String,
    pub n_hd_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner2 {
    pub owner_type: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub visibility: String,
    pub id: String,
    pub name: Value,
    pub icon_url: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct First {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub title: String,
    pub registered_at: String,
    pub count: Count3,
    pub thumbnail: Thumbnail3,
    pub duration: i64,
    pub short_description: String,
    pub latest_comment_summary: String,
    pub is_channel_video: bool,
    pub is_payment_required: bool,
    pub playback_position: Value,
    pub owner: Owner3,
    pub require_sensitive_masking: bool,
    pub video_live: Value,
    pub is_muted: bool,
    #[serde(rename = "9d091f87")]
    pub n9d091f87: bool,
    pub acf68865: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Count3 {
    pub view: i64,
    pub comment: i64,
    pub mylist: i64,
    pub like: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail3 {
    pub url: String,
    pub middle_url: String,
    pub large_url: String,
    pub listing_url: String,
    pub n_hd_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner3 {
    pub owner_type: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub visibility: String,
    pub id: String,
    pub name: Value,
    pub icon_url: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub server_time: String,
    pub is_peak_time: bool,
    pub is_stella_alive: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub items: Vec<Item2>,
    #[serde(rename = "hasR18Tag")]
    pub has_r18tag: bool,
    pub is_published_nicoscript: bool,
    pub edit: Edit,
    pub viewer: Option<Viewer2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item2 {
    pub name: String,
    pub is_category: bool,
    pub is_category_candidate: bool,
    pub is_nicodic_article_exists: bool,
    pub is_locked: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edit {
    pub is_editable: bool,
    pub uneditable_reason: Value,
    pub edit_key: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer2 {
    pub is_editable: bool,
    pub uneditable_reason: Value,
    pub edit_key: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video5 {
    pub id: String,
    pub title: String,
    pub description: String,
    pub count: Count4,
    pub duration: i64,
    pub thumbnail: Thumbnail4,
    pub rating: Rating,
    pub registered_at: String,
    pub is_private: bool,
    pub is_deleted: bool,
    pub is_no_banner: bool,
    pub is_authentication_required: bool,
    pub is_embed_player_allowed: bool,
    pub is_gift_allowed: bool,
    pub viewer: Option<Viewer3>,
    pub watchable_user_type_for_payment: String,
    pub commentable_user_type_for_payment: String,
    #[serde(rename = "9d091f87")]
    pub n9d091f87: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Count4 {
    pub view: i64,
    pub comment: i64,
    pub mylist: i64,
    pub like: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail4 {
    pub url: String,
    pub middle_url: Value,
    pub large_url: Value,
    pub player: String,
    pub ogp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub is_adult: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer3 {
    pub is_owner: bool,
    pub like: Like,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    pub is_liked: bool,
    pub count: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoAds {
    pub additional_params: AdditionalParams,
    pub items: Vec<Item3>,
    pub reason: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalParams {
    pub video_id: String,
    pub video_duration: i64,
    #[serde(rename = "isAdultRatingNG")]
    pub is_adult_rating_ng: bool,
    pub is_authentication_required: bool,
    pub is_r18: bool,
    pub nicosid: String,
    pub lang: String,
    pub watch_track_id: String,
    //pub genre: String,
    //pub gender: String,
    //pub age: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item3 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub timing_ms: Value,
    pub additional_params: AdditionalParams2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalParams2 {
    pub linear_type: String,
    pub ad_idx: i64,
    pub skip_type: i64,
    pub skippable_type: i64,
    pub pod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer4 {
    pub id: i64,
    pub nickname: String,
    pub is_premium: bool,
    pub allow_sensitive_contents: bool,
    pub existence: Existence,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Existence {
    pub age: i64,
    pub prefecture: String,
    pub sex: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Waku {
    pub information: Value,
    pub bg_images: Vec<Value>,
    pub add_contents: Value,
    pub add_video: Value,
    pub tag_related_banner: Value,
    pub tag_related_marquee: Value,
}



fn parse_initial_watch_data(text : &str) -> Result<InitialWatchData, std::string::String>  {
    let document = Html::parse_document(&text);
    let selector = Selector::parse("#js-initial-watch-data").unwrap();


    let elem = document.select(&selector).last().unwrap();

    match elem.attr("data-api-data") {
        Some(apidata_text) => {
            let apidata: InitialWatchData = serde_json::from_str(&apidata_text).unwrap();
            Ok(apidata)
        },
        None => {
            Err("The #js-initial-watch-data element is missing the \"data-api-data\" attribute.".to_string())
        },
    }
}

pub async fn get_initial_watch_data(video_id: &str) -> Result<InitialWatchData, String> {
    match reqwest::get(format!("https://www.nicovideo.jp/watch/{}", video_id)).await {
        Ok(res) => {
            let text = res.text().await.unwrap();
            let parsed = parse_initial_watch_data(&text).unwrap();
            Ok(parsed)
        },
        Err(_e) => Err("e".to_string())
    }
}