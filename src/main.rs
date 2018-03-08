extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate hyper;

use reqwest::Url;
use hyper::header::Headers;

header! { (XChatWorkToken, "X-ChatWorkToken") => [String] }

#[derive(Serialize)]
struct PostMessageRequest {
    body: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum PostMessageResponse {
    Error { errors: Vec<String> },
    MessageId { message_id: String },
}

#[derive(Debug)]
struct MessageId {
    message_id: String,
}

#[derive(Debug)]
enum PostMessageError {
    Reqwest(reqwest::Error),
    UrlParse(url::ParseError),
    API(Vec<String>),
}

impl From<reqwest::Error> for PostMessageError {
    fn from(e: reqwest::Error) -> PostMessageError {
        PostMessageError::Reqwest(e)
    }
}

impl From<url::ParseError> for PostMessageError {
    fn from(e: url::ParseError) -> PostMessageError {
        PostMessageError::UrlParse(e)
    }
}

fn main() {
    let (room_id, body) = parse_args().unwrap();
    let token = env_chatwork_token().unwrap();
    let response = post_message(&token, room_id, &body).unwrap();
    println!("{:?}", response);
}

fn env_chatwork_token() -> Result<std::string::String, String> {
    std::env::var("CHATWORK_API_TOKEN")
        .map_err(|_| "CHATWORK_API_TOKEN environment variable not present".to_string())
}

fn parse_args() -> Result<(u32, String), String> {
    let mut args = std::env::args();
    args.next(); // プログラムの名前なので無視します
    let room_id = match args.next() {
        Some(s) => s.parse::<u32>()
            .or(Err("arg1 expected number for room_id".to_string())),
        None => Err("arg1 expected room_id, found None".to_string()),
    }?;

    let body = match args.next() {
        Some(s) => s,
        None => return Err("args2 expected body, found None".to_string()),
    };
    Ok((room_id, body))
}

fn post_message_url(room_id: u32) -> Result<Url, url::ParseError> {
    let url_str = format!("https://api.chatwork.com/v2/rooms/{}/messages", room_id);
    Url::parse(&url_str)
}

fn chatwork_api_headers(token: &str) -> Headers {
    let mut headers = Headers::new();
    headers.set(XChatWorkToken(token.to_string()));
    headers
}

fn request_chatwork_api<T: serde::Serialize, JSON: serde::de::DeserializeOwned>
    (url: Url,
     headers: Headers,
     body: &T)
     -> Result<JSON, reqwest::Error> {
    reqwest::Client::new()
        .post(url)
        .form(body)
        .headers(headers)
        .send()?
        .json()
}

fn post_message(token: &str, room_id: u32, body: &str) -> Result<MessageId, PostMessageError> {
    let body = PostMessageRequest { body: body.to_owned() };
    let url = post_message_url(room_id)?;
    let headers = chatwork_api_headers(token);
    let response = request_chatwork_api(url, headers, &body)?;
    match response {
        PostMessageResponse::Error { errors } => Err(PostMessageError::API(errors)),
        PostMessageResponse::MessageId { message_id } => Ok(MessageId { message_id: message_id }),
    }
}
