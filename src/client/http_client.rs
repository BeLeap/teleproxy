use http::Method;

pub struct HttpClient {
    pub target: String,
}

impl HttpClient {
    pub fn into_reqwest(self, method: Method, path: String) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.target, path);
        let url = url.parse::<reqwest::Url>().unwrap();
        client.request(method, url)
    }
}
