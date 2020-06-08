use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error, Method, Response,
};

pub struct Builder {
    method: Method,
    url: String,
    schema: Option<String>,
    // Need this to allow access from `filter.rs`
    pub(crate) queries: Vec<(String, String)>,
    headers: HeaderMap,
    body: Option<String>,
    is_rpc: bool,
}

// TODO: Test Unicode support
impl Builder {
    pub fn new<T>(url: T, schema: Option<String>) -> Self
    where
        T: Into<String>,
    {
        let mut builder = Builder {
            method: Method::GET,
            url: url.into(),
            schema,
            queries: Vec::new(),
            headers: HeaderMap::new(),
            body: None,
            is_rpc: false,
        };
        builder
            .headers
            .insert("Accept", HeaderValue::from_static("application/json"));
        builder
    }

    pub fn auth<T>(mut self, token: T) -> Self
    where
        T: Into<String>,
    {
        self.headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token.into())).unwrap(),
        );
        self
    }

    // TODO: Renaming, casting, & JSON column examples
    // TODO: Resource embedding example
    pub fn select<T>(mut self, columns: T) -> Self
    where
        T: Into<String>,
    {
        self.method = Method::GET;
        self.queries.push(("select".to_string(), columns.into()));
        self
    }

    pub fn order<T>(mut self, columns: T) -> Self
    where
        T: Into<String>,
    {
        self.queries.push(("order".to_string(), columns.into()));
        self
    }

    pub fn limit(mut self, count: usize) -> Self {
        self.headers
            .insert("Range-Unit", HeaderValue::from_static("items"));
        self.headers.insert(
            "Range",
            HeaderValue::from_str(&format!("0-{}", count - 1)).unwrap(),
        );
        self
    }

    pub fn range(mut self, low: usize, high: usize) -> Self {
        self.headers
            .insert("Range-Unit", HeaderValue::from_static("items"));
        self.headers.insert(
            "Range",
            HeaderValue::from_str(&format!("{}-{}", low, high)).unwrap(),
        );
        self
    }

    fn count(mut self, method: &str) -> Self {
        self.headers
            .insert("Range-Unit", HeaderValue::from_static("items"));
        // Value is irrelevant, we just want the size
        self.headers
            .insert("Range", HeaderValue::from_static("0-0"));
        self.headers.insert(
            "Prefer",
            HeaderValue::from_str(&format!("count={}", method)).unwrap(),
        );
        self
    }

    pub fn exact_count(self) -> Self {
        self.count("exact")
    }

    pub fn planned_count(self) -> Self {
        self.count("planned")
    }

    pub fn estimated_count(self) -> Self {
        self.count("estimated")
    }

    pub fn single(mut self) -> Self {
        self.headers.insert(
            "Accept",
            HeaderValue::from_static("application/vnd.pgrst.object+json"),
        );
        self
    }

    pub fn insert<T>(mut self, body: T) -> Self
    where
        T: Into<String>,
    {
        self.method = Method::POST;
        self.headers
            .insert("Prefer", HeaderValue::from_static("return=representation"));
        self.body = Some(body.into());
        self
    }

    pub fn upsert<T>(mut self, body: T) -> Self
    where
        T: Into<String>,
    {
        self.method = Method::POST;
        self.headers.insert(
            "Prefer",
            HeaderValue::from_static("return=representation,resolution=merge-duplicates"),
        );
        self.body = Some(body.into());
        self
    }

    pub fn update<T>(mut self, body: T) -> Self
    where
        T: Into<String>,
    {
        self.method = Method::PATCH;
        self.headers
            .insert("Prefer", HeaderValue::from_static("return=representation"));
        self.body = Some(body.into());
        self
    }

    pub fn delete(mut self) -> Self {
        self.method = Method::DELETE;
        self.headers
            .insert("Prefer", HeaderValue::from_static("return=representation"));
        self
    }

    pub fn rpc<T>(mut self, params: T) -> Self
    where
        T: Into<String>,
    {
        self.method = Method::POST;
        self.body = Some(params.into());
        self.is_rpc = true;
        self
    }

    pub async fn execute(mut self) -> Result<Response, Error> {
        let mut req = Client::new().request(self.method.clone(), &self.url);
        if let Some(schema) = self.schema {
            let key = if self.method == Method::GET || self.method == Method::HEAD {
                "Accept-Profile"
            } else {
                "Content-Profile"
            };
            self.headers
                .insert(key, HeaderValue::from_str(&schema).unwrap());
        }
        if self.method != Method::GET && self.method != Method::HEAD {
            self.headers
                .insert("Content-Type", HeaderValue::from_static("application/json"));
        }
        req = req.headers(self.headers).query(&self.queries);
        if let Some(body) = self.body {
            req = req.body(body);
        }

        req.send().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE_URL: &str = "http://localhost:3000/table";
    const RPC_URL: &str = "http://localhost:3000/rpc";

    #[test]
    fn only_accept_json() {
        let builder = Builder::new(TABLE_URL, None);
        assert_eq!(
            builder.headers.get("Accept").unwrap(),
            HeaderValue::from_static("application/json")
        );
    }

    #[test]
    fn auth_with_token() {
        let builder = Builder::new(TABLE_URL, None).auth("$Up3rS3crET");
        assert_eq!(
            builder.headers.get("Authorization").unwrap(),
            HeaderValue::from_static("Bearer $Up3rS3crET")
        );
    }

    #[test]
    fn select_assert_query() {
        let builder = Builder::new(TABLE_URL, None).select("some_table");
        assert_eq!(builder.method, Method::GET);
        assert_eq!(
            builder
                .queries
                .contains(&("select".to_string(), "some_table".to_string())),
            true
        );
    }

    #[test]
    fn order_assert_query() {
        let builder = Builder::new(TABLE_URL, None).order("id");
        assert_eq!(
            builder
                .queries
                .contains(&("order".to_string(), "id".to_string())),
            true
        );
    }

    #[test]
    fn limit_assert_range_header() {
        let builder = Builder::new(TABLE_URL, None).limit(20);
        assert_eq!(
            builder.headers.get("Range").unwrap(),
            HeaderValue::from_static("0-19")
        );
    }

    #[test]
    fn range_assert_range_header() {
        let builder = Builder::new(TABLE_URL, None).range(10, 20);
        assert_eq!(
            builder.headers.get("Range").unwrap(),
            HeaderValue::from_static("10-20")
        );
    }

    #[test]
    fn single_assert_accept_header() {
        let builder = Builder::new(TABLE_URL, None).single();
        assert_eq!(
            builder.headers.get("Accept").unwrap(),
            HeaderValue::from_static("application/vnd.pgrst.object+json")
        );
    }

    #[test]
    fn upsert_assert_prefer_header() {
        let builder = Builder::new(TABLE_URL, None).upsert("ignored");
        assert_eq!(
            builder.headers.get("Prefer").unwrap(),
            HeaderValue::from_static("return=representation,resolution=merge-duplicates")
        );
    }

    #[test]
    fn not_rpc_should_not_have_flag() {
        let builder = Builder::new(TABLE_URL, None).select("ignored");
        assert_eq!(builder.is_rpc, false);
    }

    #[test]
    fn rpc_should_have_body_and_flag() {
        let builder = Builder::new(RPC_URL, None).rpc("{\"a\": 1, \"b\": 2}");
        assert_eq!(builder.body.unwrap(), "{\"a\": 1, \"b\": 2}");
        assert_eq!(builder.is_rpc, true);
    }

    #[test]
    fn chain_filters() -> Result<(), Box<dyn std::error::Error>> {
        let builder = Builder::new(TABLE_URL, None)
            .eq("username", "supabot")
            .neq("message", "hello world")
            .gte("channel_id", "1")
            .select("*");

        let queries = builder.queries;
        assert_eq!(queries.len(), 4);
        assert!(queries.contains(&("username".into(), "eq.supabot".into())));
        assert!(queries.contains(&("message".into(), "neq.hello world".into())));
        assert!(queries.contains(&("channel_id".into(), "gte.1".into())));

        Ok(())
    }
}
