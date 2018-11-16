/// we don't know that it works
///
/// we only know it might

extern crate chrono;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_urlencoded;
extern crate reqwest;
#[cfg(feature = "webhook")] extern crate iron;

use std::fmt;
use std::collections::HashMap;
use chrono::DateTime;
use serde::{de, de::{Visitor, Deserializer}, Deserialize};
use failure::Error;

/// Discord bots API root
pub static API: &str = "https://discordbots.org/api";

/// Snowflake wrapper for deserializing snowflake strings from api
#[derive(Serialize, Clone, Debug)]
pub struct Snowflake(u64);

#[derive(Debug, Fail)]
#[fail(display = "Error parsing parameter")]
pub struct ParamError;

/// Error when statuscode != OK
#[derive(Debug, Fail)]
#[fail(display = "Error parsing parameter")]
pub struct StatusError(reqwest::StatusCode);

impl StatusError {
    fn with(status: reqwest::StatusCode) -> Result<()> {
        if status == 200 {
            Ok(())
        } else {
            Err(StatusError(status).into())
        }
    }
}

///DBL simple user
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleUser {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub def_avatar: String,
}

/// Dbl user: https://discordbots.org/api/docs#users
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(flatten)]
    pub user: SimpleUser,

    pub bio: Option<String>,
    pub banner: Option<String>,
    pub social: HashMap<String, String>,
    pub color: Option<String>,
    pub supporter: bool,
    pub certified_dev: bool,
    #[serde(rename = "mod")]
    pub is_mod: bool,
    #[serde(rename = "webMod")]
    pub is_web_mod: bool,
    pub admin: bool
}

/// Dbl bot: https://discordbots.org/api/docs#bots
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    #[serde(flatten)]
    pub user: SimpleUser,

    pub lib: String,
    pub prefix: String,
    #[serde(rename = "shortdesc")]
    pub short_desc: String,
    #[serde(rename = "longdesc")]
    pub long_desc: Option<String>,
    pub tags: Vec<String>,
    pub website: Option<String>,
    pub support: Option<String>,
    pub github: Option<String>,
    pub owners: Vec<Snowflake>,
    pub invite: Option<String>,
    pub date: DateTime<chrono::Utc>,
    #[serde(rename = "certifiedBot")]
    pub certified: bool,
    pub vanity: Option<String>,
    pub points: usize
}

/// Dbl bot stats: https://discordbots.org/api/docs#bots
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BotStats {
    #[serde(default)]
    pub server_count: usize,
    pub shards: Vec<usize>,
    #[serde(default)]
    pub shard_count: usize
}

/// Server count enum
///
/// If your bot is sharded, you can give shard data here, or if its not, you can give a single usize for the total server count
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ServerCount {
    Sharded(Vec<usize>),
    Single(usize)
}

/// Use this to post stats:
///
/// ex. single sharded bot with 10 servers
/// ```rust
/// Client::new(token).post_stats(PostBotStats::new(ServerCount::Single(10)));
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostBotStats {
    pub server_count: ServerCount,
    pub shard_id: Option<usize>,
    pub shard_count: Option<usize>
}

impl PostBotStats {
    /// Makes a PostBotStats out of a ServerCount. You can set the shard id/count with methods
    pub fn new(server_count: ServerCount) -> Self {
        PostBotStats { server_count, shard_id: None, shard_count: None }
    }

    /// Set the shard id
    pub fn shard_id(self, id: usize) -> Self {
        PostBotStats { shard_id: Some(id), ..self }
    }

    /// Set the shard count
    pub fn shard_count(self, count: usize) -> Self {
        PostBotStats { shard_count: Some(count), ..self }
    }
}

type Result<T> = std::result::Result<T, Error>;

impl User {
    pub fn get(id: Snowflake) -> Result<Self> {
        let mut resp = reqwest::get(&format!("{}/users/{}", API, id))?;
        Ok(resp.json()?)
    }
}

/// The dbl client
///
/// Used for bot specific functions
pub struct Client {
    pub token: String,
    pub client: reqwest::Client
}

impl Client {
    /// Initialize a new dbl client from a token: https://discordbots.org/api/docs#reference
    pub fn new(token: &str) -> Self {
        Client {token: token.to_owned(), client: reqwest::Client::new()}
    }

    fn get<U: reqwest::IntoUrl>(&self, u: U) -> reqwest::RequestBuilder {
        self.client.get(u).header("Authorization", self.token.to_owned())
    }

    fn post<U: reqwest::IntoUrl>(&self, u: U) -> reqwest::RequestBuilder {
        self.client.post(u).header("Authorization", self.token.to_owned())
    }

    /// Get the last 1k votes from your bot
    pub fn get_votes(&self) -> Result<Vec<SimpleUser>> {
        let mut resp = self.get(&format!("{}/bots/votes", API)).send()?;
        Ok(resp.json()?)
    }

    /// Check if a user has voted
    pub fn get_voted(&self, user: Snowflake) -> Result<bool> {
        let mut resp = self.get(&format!("{}/bots/check?userId={}", API, user)).send()?;
        let jsp: HashMap<String, i32> = resp.json()?;
        if *jsp.get("voted").ok_or(ParamError)? == 1 { Ok(true) } else { Ok(false) }
    }

    /// Get bot info
    pub fn get_bot(&self) -> Result<Bot> {
        Ok(self.get(&format!("{}/bots", API)).send()?.json()?)
    }

    /// Get bot stats
    pub fn get_stats(&self) -> Result<Bot> {
        Ok(self.get(&format!("{}/bots/stats", API)).send()?.json()?)
    }

    /// Post bot stats
    ///
    /// Check the docs on PostBotStats for more info
    pub fn post_stats(&self, pbs: PostBotStats) -> Result<()> {
        let status = self.post(&format!("{}/bots/stats", API)).json(&pbs).send()?.status();
        StatusError::with(status)
    }
}

/// To use this module, make sure to enable the webhook feature.
#[cfg(feature = "webhook")]
pub mod webhook {
    use std::collections::HashMap;
    use iron::{Handler, Request, Response, IronResult, status};
    use std::{sync::Mutex, io::Read};
    use super::{Snowflake};

    /// Webhook vote type
    /// Should always be upvote unless using the test function
    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub enum VoteType {
        Test,
        Upvote
    }

    /// A webhook message: https://discordbots.org/api/docs#webhooks
    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct WebHookVote {
        bot: Snowflake,
        user: Snowflake,
        #[serde(rename = "type")]
        vote_type: VoteType,
        is_weekend: bool,
        query: HashMap<String, String>
    }

    /// The webhook handler trait:
    ///
    /// ```rust
    /// struct MyHandler {
    ///     votes: i32
    /// }
    ///
    /// impl webhook::WebhookHandler for MyHandler {
    ///     fn handle(&mut self, whv: webhook::WebHookVote) {
    ///         self.votes += 1;
    ///     }
    /// }
    ///
    /// // later:
    /// mount.mount(webhook::iron_handler(MyHandler {votes: 0}));
    /// ```
    pub trait WebhookHandler {
        fn handle(&mut self, whv: WebHookVote);
    }

    /// Constructs an iron handler out of a webhook handler
    pub fn iron_handler<T: 'static + WebhookHandler + Send + Sized>(t: T) -> impl Handler {
        WHIronHandler {
            webhook: Mutex::new(t)
        }
    }

    struct WHIronHandler<T: WebhookHandler + Sized> {
        webhook: Mutex<T>
    }

    impl<T: 'static + WebhookHandler + Send + Sized> Handler for WHIronHandler<T> {
        fn handle(&self, req: &mut Request) -> IronResult<Response> {
            let mut buf = Vec::new();
            let _ = &req.body.read_to_end(&mut buf);

            match serde_json::from_slice(buf.as_slice()).ok() {
                Some(x) => {
                    let mut hookhandler = self.webhook.lock().unwrap();
                    hookhandler.handle(x);

                    Ok(Response::with(status::Ok))
                },
                None => {
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
    }
}

pub mod widget {
    use std::collections::HashMap;

    /// Widget colors: https://discordbots.org/api/docs#widgets
    pub struct CustomizeWidget {
        pub color_map: HashMap<&'static str, String>,
        pub widget_type: Option<String>,
        pub no_avatar: bool,
    }

    //skip this
    //
    //
    //no, skip it
    //SKIP IT
    impl CustomizeWidget {
        /// Makes a new CustomizeWidget
        /// When setting colors, make sure not to include the hash
        pub fn new() -> Self {
            CustomizeWidget { color_map: HashMap::new(), widget_type: None, no_avatar: false }
        }

        /// Either owner, status, upvotes, servers, or lib
        pub fn widget_type(self, widget_type: String) -> Self {
            CustomizeWidget { widget_type: Some(widget_type), ..self }
        }

        pub fn no_avatar(self) -> Self {
            CustomizeWidget { no_avatar: true, ..self }
        }

        //TODO: use some other way

        pub fn top_color(&mut self, top_color: &str) {
            self.color_map.insert("topcolor", top_color.to_owned());
        }

        pub fn middle_color(&mut self, middle_color: &str) {
            self.color_map.insert("middlecolor", middle_color.to_owned());
        }

        pub fn username_color(&mut self, username_color: &str) {
            self.color_map.insert("usernamecolor", username_color.to_owned());
        }

        pub fn certified_color(&mut self, certified_color: &str) {
            self.color_map.insert("certifiedcolor", certified_color.to_owned());
        }

        pub fn data_color(&mut self, data_color: &str) {
            self.color_map.insert("datacolor", data_color.to_owned());
        }

        pub fn label_color(&mut self, label_color: &str) {
            self.color_map.insert("labelcolor", label_color.to_owned());
        }

        pub fn highlight_color(&mut self, highlight_color: &str) {
            self.color_map.insert("highlightcolor", highlight_color.to_owned());
        }

        pub fn avatar_bg(&mut self, avatar_bg: &str) {
            self.color_map.insert("avatarbg", avatar_bg.to_owned());
        }

        pub fn left_color(&mut self, left_color: &str) {
            self.color_map.insert("leftcolor", left_color.to_owned());
        }

        pub fn right_color(&mut self, right_color: &str) {
            self.color_map.insert("rightcolor", right_color.to_owned());
        }

        pub fn left_text_color(&mut self, left_text_color: &str) {
            self.color_map.insert("lefttextcolor", left_text_color.to_owned());
        }

        pub fn right_text_color(&mut self, right_text_color: &str) {
            self.color_map.insert("righttextcolor", right_text_color.to_owned());
        }
    }
}

impl Bot {
    /// Get a bots data by its id
    pub fn get(id: Snowflake) -> Result<Self> {
        Ok(reqwest::get(&format!("{}/bots/{}", API, id))?.json()?)
    }

    /// Gets a bots stats by id
    pub fn get_stats(id: Snowflake) -> Result<BotStats> {
        Ok(reqwest::get(&format!("{}/bots/{}/stats", API, id))?.json()?)
    }

    /// Get a bot's widget image url
    /// ``ext`` determines the extension, svg or png
    pub fn get_widget(id: Snowflake, ext: &str, customize: Option<widget::CustomizeWidget>) -> Result<String> {
        let mut s = format!("{}/widget", API);
        let params = customize.map(|x| {
            let mut params = x.color_map;

            if let Some(t) = x.widget_type {
                s.push_str(&format!("/{}", t))
            }

            if x.no_avatar {
                params.insert("noavatar", "true".to_owned());
            }

            params
        });

        s.push_str(&format!("/{}.{}", id, ext));

        if let Some(p) = params {
            s.push_str(&format!("?{}", serde_urlencoded::to_string(p)?));
        }

        Ok(s)
    }
}

pub mod listing {
    use super::*;

    /// Sorting options for bot listing
    ///
    /// To reverse, use .reverse instead of wrapping
    #[derive(Clone, Debug)]
    pub enum BotListingSort {
        Points,
        MonthlyPoints,
        Date,
        ServerCount,
        Reverse(Box<BotListingSort>)
    }

    impl BotListingSort {
        /// Reverses the sorting order
        pub fn reverse(self) -> Self {
            match self {
                BotListingSort::Reverse(x) => *x,
                x => BotListingSort::Reverse(Box::new(x))
            }
        }
    }

    impl ToString for BotListingSort {
        fn to_string(&self) -> String {
            match &self {
                BotListingSort::Points => "points".to_owned(),
                BotListingSort::MonthlyPoints => "monthly_points".to_owned(),
                BotListingSort::Date => "date".to_owned(),
                BotListingSort::ServerCount => "server_count".to_owned(),
                BotListingSort::Reverse(bls) => format!("-{}", bls.to_string())
            }
        }
    }

    use serde::ser::Serializer;
    fn serialize_botsort<S>(bls: &BotListingSort, s: S) -> std::result::Result<S::Ok, S::Error> where S: Serializer {
        s.collect_str(&bls.to_string())
    }

    /// Bot listing
    ///
    /// You can use this to search through dbl's bots
    #[derive(Serialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct BotListing {
        pub limit: i64,
        pub offset: i64,
        pub search: String,
        #[serde(serialize_with = "serialize_botsort")]
        pub sort: BotListingSort,
        pub fields: String
    }

    #[derive(Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct BotList {
        pub results: Vec<super::Bot>,
        pub limit: i64,
        pub offset: i64,
        pub count: usize,
        pub total: usize
    }

    impl BotListing {
        pub fn new() -> Self {
            BotListing { limit: 50, offset: 0, search: "".to_owned(), sort: BotListingSort::Points, fields: "".to_owned() }
        }

        pub fn search(self, search: String) -> Self {
            BotListing { search, ..self }
        }

        pub fn sort(self, sort: BotListingSort) -> Self {
            BotListing { sort, ..self }
        }

        pub fn limit(self, limit: i64) -> Self {
            BotListing { limit, ..self }
        }

        pub fn offset(self, offset: i64) -> Self {
            BotListing { offset, ..self }
        }

        pub fn fields(self, fields: &str) -> Self {
            BotListing { fields: fields.to_owned(), ..self }
        }

        /// Execute the BotListing and get the BotList
        pub fn exec(&self) -> Result<BotList> {
            let client = reqwest::Client::new();
            let mut resp = client.get(&format!("{}/bots", API)).query(self).send()?;

            Ok(resp.json()?)
        }
    }
}

impl Into<u64> for Snowflake {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for Snowflake {
    fn from(u: u64) -> Self {
        Snowflake(u)
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, fmtter: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtter, "{}", self.0)
    }
}

struct SnowflakeVisitor;

impl <'de> Visitor<'de> for SnowflakeVisitor {
    type Value = Snowflake;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a snowflake as a string")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
    {
        let i: u64 = value.parse().map_err(|_| de::Error::invalid_type(de::Unexpected::Str(value), &self))?;
        Ok(Snowflake(i))
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Snowflake, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(SnowflakeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bot() {
        let ok = Bot::get(510114241307607051.into()).unwrap();
        assert_eq!(ok.user.username, "Skype Bot");
    }

    #[test]
    fn get_bot_stats() {
        Bot::get_stats(510114241307607051.into()).unwrap();
    }

    #[test]
    fn get_widget() {
        let str = Bot::get_widget(510114241307607051.into(), "svg", None).unwrap();
        assert_eq!(str, "https://discordbots.org/api/widget/510114241307607051.svg");
    }

    #[test]
    fn customize_widget() {
        use widget::*;

        let mut cwig = CustomizeWidget::new().no_avatar();
        cwig.certified_color("000000");

        let s = Bot::get_widget(510114241307607051.into(), "svg", Some(cwig)).unwrap();

        let out = ["https://discordbots.org/api/widget/510114241307607051.svg?certifiedcolor=000000&noavatar=true".to_owned(),
            "https://discordbots.org/api/widget/510114241307607051.svg?noavatar=true&certifiedcolor=000000".to_owned()];

        assert_eq!(true, out.contains(&s));
    }

    /// i know these arent good things to test
    /// o well
    #[test]
    fn list_bots() {
        use listing::*;

        let res = BotListing::new().search("shibe".to_owned()).exec().unwrap();
        let uname = &res.results.first().unwrap().user.username;

        assert_eq!(uname, "Shibe Bot");

        let res = BotListing::new().search("shibe".to_owned()).sort(BotListingSort::Points.reverse()).exec().unwrap();
        let uname = &res.results.first().unwrap().user.username;

        assert_eq!(uname, "Birb Bot");
    }
}
