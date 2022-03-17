pub mod api;

const BASE_URL: &str = "https://api.wit.ai";

pub fn client(token: String) -> api::Api {
    api::Api::new(BASE_URL.to_owned(), api::Authorization::Bearer(token))
}
