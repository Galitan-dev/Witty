use std::fmt::Debug;

use super::super::Error;
use super::{Endpoint, ParseBody};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

pub fn path(e: &Endpoint) -> String {
    match e {
        Endpoint::Message(_) => "/message".to_owned(),
    }
}

pub fn params(e: &Endpoint) -> Vec<(String, String)> {
    match e {
        Endpoint::Message(msg) => vec![("q".to_owned(), msg.to_owned())],
    }
}

impl ParseBody<Message> for Endpoint {
    fn parse_body(&self, body: hyper::body::Bytes) -> Result<Message, Error> {
        let serde: Value = serde_json::from_slice(&body).map_err(|err| Error {
            name: "Bad Response".to_owned(),
            message: err.to_string(),
            code: 400,
            debug: format!("{:?}", body),
        })?;
        let user: Message = serde_json::from_slice(&body).map_err(|err| Error {
            name: "Bad Response".to_owned(),
            message: err.to_string(),
            code: 400,
            debug: format!("{:?}", serde),
        })?;
        Ok(user)
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Message {
    pub text: String,
    pub intents: Vec<Intent>,
    #[serde(deserialize_with = "parse_map_to_vec")]
    pub entities: Vec<Vec<Entity>>,
    #[serde(deserialize_with = "parse_map_to_vec")]
    pub traits: Vec<Trait>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Intent {
    pub id: String,
    pub name: String,
    pub confidence: f32,
}

#[derive(Deserialize, Debug)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub role: String,
    pub start: i16,
    pub end: i16,
    pub body: String,
    pub value: Value,
    pub confidence: f32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Trait {
    id: String,
    name: String,
    confidence: f32,
}

pub fn parse_map_to_vec<'de, D, R>(deserializer: D) -> Result<Vec<R>, D::Error>
where
    D: Deserializer<'de>,
    R: Deserialize<'de>,
{
    let map: Map<String, Value> = Map::deserialize(deserializer)?;
    let mut vec: Vec<R> = Vec::new();

    for (_key, value) in map {
        vec.push(R::deserialize(value).expect("Unable to deserialize value"));
    }

    Ok(vec)
}
