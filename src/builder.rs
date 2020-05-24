use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error, Method, Response,
};

#[derive(Default)]
pub struct Builder {
    method: Method,
    url: String,
    schema: Option<String>,
    pub(crate) queries: Vec<(String, String)>,
    headers: HeaderMap,
    body: Option<String>,
    is_rpc: bool,
}

// TODO: Complex filters (not, and, or)
// TODO: Exact, planned, estimated count (HEAD verb)
// TODO: Response format
// TODO: Resource embedding (embedded filters, etc.)
// TODO: Content-Type (text/csv, etc.)
// TODO: Reject update/delete w/o filters
impl Builder {
    pub fn new<S>(url: S, schema: Option<String>) -> Self
    where
        S: Into<String>,
    {
        let mut builder = Builder {
            method: Method::GET,
            url: url.into(),
            schema,
            headers: HeaderMap::new(),
            ..Default::default()
        };
        builder
            .headers
            .insert("Accept", HeaderValue::from_static("application/json"));
        builder
    }

    pub fn auth<S>(mut self, token: S) -> Self
    where
        S: Into<String>,
    {
        self.headers.append(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token.into())).unwrap(),
        );
        self
    }

    // TODO: Multiple columns
    // TODO: Renaming columns
    // TODO: Casting columns
    // TODO: JSON columns
    // TODO: Computed (virtual) columns
    // TODO: Investigate character corner cases (Unicode, [ .,:()])
    pub fn select<S>(mut self, column: S) -> Self
    where
        S: Into<String>,
    {
        self.method = Method::GET;
        self.queries.push(("select".to_string(), column.into()));
        self
    }

    // TODO: desc/asc
    // TODO: nullsfirst/nullslast
    // TODO: Multiple columns
    // TODO: Computed columns
    pub fn order<S>(mut self, column: S) -> Self
    where
        S: Into<String>,
    {
        self.queries.push(("order".to_string(), column.into()));
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

    pub fn single(mut self) -> Self {
        self.headers.insert(
            "Accept",
            HeaderValue::from_static("application/vnd.pgrst.object+json"),
        );
        self
    }

    // TODO: Write-only tables
    // TODO: URL-encoded payload
    // TODO: Allow specifying columns
    pub fn insert<S>(mut self, body: S) -> Self
    where
        S: Into<String>,
    {
        self.method = Method::POST;
        self.headers
            .insert("Prefer", HeaderValue::from_static("return=representation"));
        self.body = Some(body.into());
        self
    }

    // TODO: Allow Prefer: resolution=ignore-duplicates
    // TODO: on_conflict (make UPSERT work on UNIQUE columns)
    pub fn upsert<S>(mut self, body: S) -> Self
    where
        S: Into<String>,
    {
        self.method = Method::POST;
        self.headers.insert(
            "Prefer",
            HeaderValue::from_static("return=representation,resolution=merge-duplicates"),
        );
        self.body = Some(body.into());
        self
    }

    pub fn single_upsert<S, T, U>(mut self, primary_column: S, key: T, body: U) -> Self
    where
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
    {
        self.method = Method::PUT;
        self.headers
            .insert("Prefer", HeaderValue::from_static("return=representation"));
        self.queries
            .push((primary_column.into(), format!("eq.{}", key.into())));
        self.body = Some(body.into());
        self
    }

    pub fn update<S>(mut self, body: S) -> Self
    where
        S: Into<String>,
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

    pub fn rpc<S>(mut self, params: S) -> Self
    where
        S: Into<String>,
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
                .append(key, HeaderValue::from_str(&schema).unwrap());
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
    const RPC_URL: &str = "http://localhost/rpc";

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
    fn single_upsert_assert_prefer_header() {
        let builder = Builder::new(TABLE_URL, None).single_upsert("ignored", "ignored", "ignored");
        assert_eq!(
            builder.headers.get("Prefer").unwrap(),
            HeaderValue::from_static("return=representation")
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
