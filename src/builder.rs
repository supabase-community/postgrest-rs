extern crate reqwest;

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
// TODO: Embedded resources
impl Builder {
    pub fn new(url: &str, schema: Option<String>) -> Self {
        let mut builder = Builder {
            method: Method::GET,
            url: url.to_string(),
            schema,
            headers: HeaderMap::new(),
            ..Default::default()
        };
        builder
            .headers
            .insert("Accept", HeaderValue::from_static("application/json"));
        builder
    }

    pub fn auth(mut self, token: &str) -> Self {
        self.headers.append(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        self
    }

    // TODO: Multiple columns
    // TODO: Renaming columns
    // TODO: Casting columns
    // TODO: JSON columns
    // TODO: Computed (virtual) columns
    // TODO: Investigate character corner cases (Unicode, [ .,:()])
    pub fn select(mut self, column: &str) -> Self {
        self.method = Method::GET;
        self.queries
            .push(("select".to_string(), column.to_string()));
        self
    }

    // TODO: desc/asc
    // TODO: nullsfirst/nullslast
    // TODO: Multiple columns
    // TODO: Computed columns
    pub fn order(mut self, column: &str) -> Self {
        self.queries.push(("order".to_string(), column.to_string()));
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
    pub fn insert(mut self, body: &str) -> Self {
        self.method = Method::POST;
        self.headers
            .insert("Prefer", HeaderValue::from_static("return=representation"));
        self.body = Some(body.to_string());
        self
    }

    pub fn insert_csv(mut self, body: &str) -> Self {
        self.headers
            .insert("Content-Type", HeaderValue::from_static("text/csv"));
        self.insert(body)
    }

    // TODO: Allow Prefer: resolution=ignore-duplicates
    // TODO: on_conflict (make UPSERT work on UNIQUE columns)
    pub fn upsert(mut self, body: &str) -> Self {
        self.method = Method::POST;
        self.headers.append(
            "Prefer",
            // Maybe check if this works as intended...
            HeaderValue::from_static("return=representation; resolution=merge-duplicates"),
        );
        self.body = Some(body.to_string());
        self
    }

    pub fn single_upsert(mut self, primary_column: &str, key: &str, body: &str) -> Self {
        self.method = Method::PUT;
        self.headers
            .append("Prefer", HeaderValue::from_static("return=representation"));
        self.queries
            .push((primary_column.to_string(), format!("eq.{}", key)));
        self.body = Some(body.to_string());
        self
    }

    pub fn update(mut self, body: &str) -> Self {
        self.method = Method::PATCH;
        self.headers
            .append("Prefer", HeaderValue::from_static("return=representation"));
        self.body = Some(body.to_string());
        self
    }

    pub fn delete(mut self) -> Self {
        self.method = Method::DELETE;
        self.headers
            .append("Prefer", HeaderValue::from_static("return=representation"));
        self
    }

    pub fn rpc(mut self, params: &str) -> Self {
        self.method = Method::POST;
        self.body = Some(params.to_string());
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
    use crate::Postgrest;

    const REST_URL: &str = "http://localhost:3000";

    #[test]
    fn only_accept_json() {
        let client = Postgrest::new(REST_URL);
        let builder = client.from("users");
        assert_eq!(
            builder.headers.get("Accept").unwrap(),
            HeaderValue::from_static("application/json")
        );
    }

    #[test]
    fn auth_with_token() {
        let client = Postgrest::new(REST_URL);
        let builder = client.from("users").auth("$Up3rS3crET");
        assert_eq!(
            builder.headers.get("Authentication").unwrap(),
            HeaderValue::from_static("Bearer $Up3rS3crET")
        );
    }

    #[test]
    fn select_assert_query() {
        let client = Postgrest::new(REST_URL);
        let builder = client.from("users").select("some_table");
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
        let client = Postgrest::new(REST_URL);
        let builder = client.from("users").order("id");
        assert_eq!(
            builder
                .queries
                .contains(&("order".to_string(), "id".to_string())),
            true
        );
    }

    #[test]
    fn limit_assert_range_header() {
        let client = Postgrest::new(REST_URL);
        let builder = client.from("users").limit(20);
        assert_eq!(
            builder.headers.get("Range").unwrap(),
            HeaderValue::from_static("0-19")
        );
    }

    #[test]
    fn range_assert_range_header() {
        let client = Postgrest::new(REST_URL);
        let builder = client.from("users").range(10, 20);
        assert_eq!(
            builder.headers.get("Range").unwrap(),
            HeaderValue::from_static("10-20")
        );
    }

    #[test]
    fn single_assert_accept_header() {
        let client = Postgrest::new(REST_URL);
        let builder = client.from("users").single();
        assert_eq!(
            builder.headers.get("Accept").unwrap(),
            HeaderValue::from_static("application/vnd.pgrst.object+json")
        );
    }

    #[test]
    fn insert_csv_assert_content_type() {
        let client = Postgrest::new(REST_URL);
        let builder = client.from("users").insert_csv("ignored");
        assert_eq!(
            builder.headers.get("Content-Type").unwrap(),
            HeaderValue::from_static("text/csv")
        );
    }

    #[test]
    fn upsert_assert_prefer_header() {
        let client = Postgrest::new(REST_URL);
        let builder = client.from("users").upsert("ignored");
        assert_eq!(
            builder.headers.get("Prefer").unwrap(),
            HeaderValue::from_static("return=representation; resolution=merge-duplicates")
        );
    }

    #[test]
    fn single_upsert_assert_prefer_header() {
        let client = Postgrest::new(REST_URL);
        let builder = client
            .from("users")
            .single_upsert("ignored", "ignored", "ignored");
        assert_eq!(
            builder.headers.get("Prefer").unwrap(),
            HeaderValue::from_static("return=representation")
        );
    }

    // filters...

    #[test]
    fn not_rpc_should_not_have_flag() {
        let client = Postgrest::new(REST_URL);
        let builder = client.from("users").select("column");
        assert_eq!(builder.is_rpc, false);
    }

    #[test]
    fn rpc_should_have_body_and_flag() {
        let client = Postgrest::new(REST_URL);
        let builder = client.from("users").rpc("{\"a\": 1, \"b\": 2}");
        assert_eq!(builder.body.unwrap(), "{\"a\": 1, \"b\": 2}");
        assert_eq!(builder.is_rpc, true);
    }
}
