use reqwest::{Client, Error, Method, Response};

pub struct Builder {
    method: Option<Method>,
    url: String,
    queries: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl Builder {
    // TODO: Switching schema
    pub fn new(url: &str) -> Self {
        Builder {
            method: None,
            url: url.to_string(),
            queries: Vec::new(),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn select(mut self, column: &str) -> Self {
        self.method = Some(Method::GET);
        let column = column.chars().filter(|c| !c.is_whitespace()).collect();
        self.queries.push(("select".to_string(), column));
        self
    }

    // TODO: Write-only tables
    // TODO: URL-encoded payload
    // TODO: Allow specifying columns
    pub fn insert(mut self, body: &str) -> Self {
        self.method = Some(Method::POST);
        self.headers
            .push(("Prefer".to_string(), "return=representation".to_string()));
        self.body = Some(body.to_string());
        self
    }

    pub fn insert_csv(mut self, body: &str) -> Self {
        self.headers
            .push(("Content-Type".to_string(), "text/csv".to_string()));
        self.insert(body)
    }

    // TODO: Allow Prefer: resolution=ignore-duplicates
    // TODO: on_conflict (make UPSERT work on UNIQUE columns)
    pub fn upsert(mut self, body: &str) -> Self {
        self.method = Some(Method::POST);
        self.headers
            .push(("Prefer".to_string(),
                   "return=representation; resolution=merge-duplicates".to_string()));
        self.body = Some(body.to_string());
        self
    }

    pub fn single_upsert(mut self, primary_column: &str, key: &str, body: &str) -> Self {
        self.method = Some(Method::PUT);
        self.headers
            .push(("Prefer".to_string(), "return=representation".to_string()));
        self.queries.push((primary_column.to_string(),
                           format!("eq.{}", key)));
        self.body = Some(body.to_string());
        self
    }

    pub fn update(mut self, body: &str) -> Self {
        self.method = Some(Method::PATCH);
        self.headers
            .push(("Prefer".to_string(), "return=representation".to_string()));
        self.body = Some(body.to_string());
        self
    }

    pub fn delete(mut self) -> Self {
        self.method = Some(Method::DELETE);
        self.headers
            .push(("Prefer".to_string(), "return=representation".to_string()));
        self
    }

    pub async fn execute(self) -> Result<Response, Error> {
        let mut req = Client::new().request(self.method.unwrap(), &self.url);
        for (k, v) in &self.headers {
            req = req.header(k, v);
        }
        req = req.query(&self.queries);
        if let Some(body) = self.body {
            req = req.body(body);
        }

        let resp = req.send().await?;

        Ok(resp)
    }
}
