use reqwest::{Client, Error, Method, Response};

pub struct Builder {
    method: Option<Method>,
    url: String,
    queries: Vec<(String, String)>,
    headers: Vec<(String, String)>,
}

impl Builder {
    pub fn new(url: &str) -> Self {
        Builder {
            method: None,
            url: url.to_string(),
            queries: Vec::new(),
            headers: Vec::new(),
        }
    }

    pub fn select(mut self, column: &str) -> Self {
        self.method = Some(Method::GET);
        self.queries.push(("select".to_string(), column.to_string()));
        self
    }

    pub async fn execute(self) -> Result<Response, Error> {
        let mut req = Client::new().request(
            self.method.unwrap(),
            &self.url,
        );
        for (k, v) in &self.headers {
            req = req.header(k, v);
        }
        req = req.query(&self.queries);

        let resp = req.send()
           .await?;

        Ok(resp)
    }
}
