use endpoints::{Endpoint, ParseBody};
use hyper::{client::HttpConnector, Body, Request};
use hyper_tls::HttpsConnector;
use itertools::Itertools;
use std::fmt::Debug;

use self::endpoints::message::Message;

pub mod endpoints;

#[allow(dead_code)]
pub enum Authorization {
    Bot(String),
    Bearer(String),
}

impl Authorization {
    pub fn to_string(&self) -> String {
        match self {
            Authorization::Bot(token) => format!("Bot {}", token),
            Authorization::Bearer(token) => format!("Bearer {}", token),
        }
    }
}

pub struct Api {
    base_url: String,
    auth: Authorization,
    client: hyper::Client<HttpsConnector<HttpConnector>, Body>,
}

impl Api {
    pub fn new(base_url: String, auth: Authorization) -> Self {
        Self {
            base_url: base_url,
            auth,
            client: hyper::Client::builder().build(HttpsConnector::new()),
        }
    }

    fn build_url(&self, path: String, params: Vec<(String, String)>) -> String {
        format!(
            "{}{}{}",
            self.base_url,
            path,
            if params.len() == 0 {
                "".to_owned()
            } else {
                format!(
                    "?{}",
                    Itertools::intersperse(
                        params.iter().cloned().map(|(key, value)| format!(
                            "{}={}",
                            key,
                            urlencoding::encode(&value)
                        )),
                        "&".to_owned()
                    )
                    .collect::<String>()
                )
            }
        )
    }

    pub async fn call<R>(&self, e: Endpoint) -> Result<R, Error>
    where
        Endpoint: ParseBody<R>,
    {
        let auth = &self.auth;

        let req = Request::builder()
            .header("authorization", auth.to_string())
            .method(e.method())
            .uri(self.build_url(e.path(), e.params()))
            .body(e.body())
            .map_err(|err| Error {
                name: "Bad Request".to_owned(),
                message: err.to_string(),
                code: 400,
                debug: format!("{:?}", e),
            })?;

        let res = self.client.request(req).await.map_err(|err| Error {
            name: "Service Unavailable".to_owned(),
            message: err.to_string(),
            code: 503,
            debug: format!("{:?}", e),
        })?;

        let status = res.status();
        let body = hyper::body::to_bytes(res.into_body())
            .await
            .map_err(|err| Error {
                name: "Bad Response".to_owned(),
                message: err.to_string(),
                code: 500,
                debug: format!("{:?}", e),
            })?;

        if status == 200 {
            e.parse_body(body)
        } else {
            let serde: serde_json::Value = serde_json::from_slice(&body).map_err(|err| Error {
                name: "Bad Response".to_owned(),
                message: err.to_string(),
                code: 500,
                debug: format!("{:?}", body),
            })?;

            Err(Error {
                name: serde.get("code").unwrap().as_str().unwrap().to_owned(),
                message: serde.get("error").unwrap().as_str().unwrap().to_owned(),
                code: status.as_u16() as i16,
                debug: format!("{:?}", serde),
            })
        }
    }

    pub async fn message<'a>(&self, msg: String) -> Result<Message, Error> {
        self.call(Endpoint::Message(msg)).await
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Error {
    name: String,
    message: String,
    code: i16,
    debug: String,
}
