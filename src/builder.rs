use std::marker;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error, Method, Response,
};

pub enum Start {}
pub enum Finish {}

pub struct Builder<Stage = Start> {
    method: Method,
    url: String,
    schema: Option<String>,
    // Need this to allow access from `filter.rs`
    pub(super) queries: Vec<(String, String)>,
    headers: HeaderMap,
    body: Option<String>,
    is_rpc: bool,
    stage: marker::PhantomData<Stage>,
}

impl Builder<Start> {
    /// Creates a new `Builder` with the specified `schema`.
    pub fn new<T>(url: T, schema: Option<String>, headers: HeaderMap) -> Self
    where
        T: Into<String>,
    {
        let mut builder = Builder {
            method: Method::GET,
            url: url.into(),
            schema,
            queries: Vec::new(),
            headers,
            body: None,
            is_rpc: false,
            stage: marker::PhantomData,
        };
        builder
            .headers
            .insert("Accept", HeaderValue::from_static("application/json"));
        builder
    }

    /// Performs horizontal filtering with SELECT.
    ///
    /// # Note
    ///
    /// `columns` is whitespace-sensitive, so you need to omit them unless your
    /// column name contains whitespaces.
    ///
    /// # Example
    ///
    /// Simple example:
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// let resp = client
    ///     .from("table")
    ///     .select("*")
    ///     .execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Renaming columns:
    ///
    /// ```
    /// # use postgrest::Postgrest;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Postgrest::new("https://your.postgrest.endpoint");
    /// let resp = client
    ///     .from("users")
    ///     .select("name:very_very_long_column_name")
    ///     .execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Casting columns:
    ///
    /// ```
    /// # use postgrest::Postgrest;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Postgrest::new("https://your.postgrest.endpoint");
    /// let resp = client
    ///     .from("users")
    ///     .select("age::text")
    ///     .execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// SELECTing JSON fields:
    ///
    /// ```
    /// # use postgrest::Postgrest;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Postgrest::new("https://your.postgrest.endpoint");
    /// let resp = client
    ///     .from("users")
    ///     .select("id,json_data->phones->0->>number")
    ///     .execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Embedded filters (assume there is a foreign key constraint between
    /// tables `users` and `tweets`):
    ///
    /// ```
    /// # use postgrest::Postgrest;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Postgrest::new("https://your.postgrest.endpoint");
    /// let resp = client
    ///     .from("users")
    ///     .select("*,tweets(*)")
    ///     .execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn select<T>(mut self, columns: T) -> Builder<Finish>
    where
        T: Into<String>,
    {
        self.queries.push(("select".to_string(), columns.into()));
        Builder {
            method: Method::GET,
            url: self.url,
            schema: self.schema,
            queries: self.queries,
            headers: self.headers,
            body: self.body,
            is_rpc: self.is_rpc,
            stage: marker::PhantomData,
        }
    }

    /// Performs an INSERT of the `body` (in JSON) into the table.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("users")
    ///     .insert(r#"[{ "username": "soedirgo", "status": "online" },
    ///                 { "username": "jose", "status": "offline" }]"#);
    /// ```
    pub fn insert<T>(mut self, body: T) -> Builder<Finish>
    where
        T: Into<String>,
    {
        self.headers
            .insert("Prefer", HeaderValue::from_static("return=representation"));
        Builder {
            method: Method::POST,
            url: self.url,
            schema: self.schema,
            queries: self.queries,
            headers: self.headers,
            body: Some(body.into()),
            is_rpc: self.is_rpc,
            stage: marker::PhantomData,
        }
    }

    /// Performs an upsert of the `body` (in JSON) into the table.
    ///
    /// # Note
    ///
    /// This merges duplicates by default. Ignoring duplicates is possible via
    /// PostgREST, but is currently unsupported.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("users")
    ///     .upsert(r#"[{ "username": "soedirgo", "status": "online" },
    ///                 { "username": "jose", "status": "offline" }]"#);
    /// ```
    pub fn upsert<T>(mut self, body: T) -> Builder<Finish>
    where
        T: Into<String>,
    {
        self.headers.insert(
            "Prefer",
            HeaderValue::from_static("return=representation,resolution=merge-duplicates"),
        );
        Builder {
            method: Method::POST,
            url: self.url,
            schema: self.schema,
            queries: self.queries,
            headers: self.headers,
            body: Some(body.into()),
            is_rpc: self.is_rpc,
            stage: marker::PhantomData,
        }
    }

    /// Performs an UPDATE using the `body` (in JSON) on the table.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("users")
    ///     .eq("username", "soedirgo")
    ///     .update(r#"{ "status": "offline" }"#);
    /// ```
    pub fn update<T>(mut self, body: T) -> Builder<Finish>
    where
        T: Into<String>,
    {
        self.method = Method::PATCH;
        self.headers
            .insert("Prefer", HeaderValue::from_static("return=representation"));
        Builder {
            method: Method::PATCH,
            url: self.url,
            schema: self.schema,
            queries: self.queries,
            headers: self.headers,
            body: Some(body.into()),
            is_rpc: self.is_rpc,
            stage: marker::PhantomData,
        }
    }

    /// Performs a DELETE on the table.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("users")
    ///     .eq("username", "soedirgo")
    ///     .delete();
    /// ```
    pub fn delete(mut self) -> Builder<Finish> {
        self.headers
            .insert("Prefer", HeaderValue::from_static("return=representation"));
        Builder {
            method: Method::DELETE,
            url: self.url,
            schema: self.schema,
            queries: self.queries,
            headers: self.headers,
            body: self.body,
            is_rpc: self.is_rpc,
            stage: marker::PhantomData,
        }
    }
}

impl Builder<Finish> {
    /// Executes the PostgREST request.
    pub async fn execute(mut self) -> Result<Response, Error> {
        if let Some(schema) = self.schema {
            let key = if let Method::GET | Method::HEAD = self.method {
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

        let mut req = Client::new()
            .request(self.method.clone(), &self.url)
            .headers(self.headers)
            .query(&self.queries);

        if let Some(body) = self.body {
            req = req.body(body);
        }

        req.send().await
    }
}

// TODO: Test Unicode support
impl<Stage> Builder<Stage> {
    /// Authenticates the request with JWT.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("table")
    ///     .auth("supers.ecretjw.ttoken");
    /// ```
    pub fn auth<T>(mut self, token: T) -> Self
    where
        T: AsRef<str>,
    {
        self.headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token.as_ref())).unwrap(),
        );
        self
    }

    /// Orders the result with the specified `columns`.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("users")
    ///     .select("*")
    ///     .order("username.desc.nullsfirst,age_range");
    /// ```
    pub fn order<T>(mut self, columns: T) -> Self
    where
        T: Into<String>,
    {
        self.queries.push(("order".to_string(), columns.into()));
        self
    }

    /// Limits the result with the specified `count`.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("users")
    ///     .select("*")
    ///     .limit(20);
    /// ```
    pub fn limit(mut self, count: usize) -> Self {
        self.headers
            .insert("Range-Unit", HeaderValue::from_static("items"));
        self.headers.insert(
            "Range",
            HeaderValue::from_str(&format!("0-{}", count - 1)).unwrap(),
        );
        self
    }

    /// Limits the result to rows within the specified range, inclusive.
    ///
    /// # Example
    ///
    /// This retrieves the 2nd to 5th entries in the result:
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("users")
    ///     .select("*")
    ///     .range(1, 4);
    /// ```
    pub fn range(mut self, low: usize, high: usize) -> Self {
        self.headers
            .insert("Range-Unit", HeaderValue::from_static("items"));
        self.headers.insert(
            "Range",
            HeaderValue::from_str(&format!("{}-{}", low, high)).unwrap(),
        );
        self
    }

    #[doc(hidden)]
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

    /// Retrieves the (accurate) total size of the result.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("users")
    ///     .select("*")
    ///     .exact_count();
    /// ```
    pub fn exact_count(self) -> Self {
        self.count("exact")
    }

    /// Estimates the total size of the result using PostgreSQL statistics. This
    /// is faster than using `exact_count()`.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("users")
    ///     .select("*")
    ///     .planned_count();
    /// ```
    pub fn planned_count(self) -> Self {
        self.count("planned")
    }

    /// Retrieves the total size of the result using some heuristics:
    /// `exact_count` for smaller sizes, `planned_count` for larger sizes.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("users")
    ///     .select("*")
    ///     .estimated_count();
    /// ```
    pub fn estimated_count(self) -> Self {
        self.count("estimated")
    }

    /// Retrieves only one row from the result.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint");
    /// client
    ///     .from("users")
    ///     .select("*")
    ///     .single();
    /// ```
    pub fn single(mut self) -> Self {
        self.headers.insert(
            "Accept",
            HeaderValue::from_static("application/vnd.pgrst.object+json"),
        );
        self
    }

    /// Performs a stored procedure call. This should only be used through the
    /// `rpc()` method in `Postgrest`.
    pub(super) fn rpc<T>(self, params: T) -> Builder<Finish>
    where
        T: Into<String>,
    {
        Builder {
            method: Method::POST,
            url: self.url,
            schema: self.schema,
            queries: self.queries,
            headers: self.headers,
            body: Some(params.into()),
            is_rpc: true,
            stage: marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE_URL: &str = "http://localhost:3000/table";
    const RPC_URL: &str = "http://localhost:3000/rpc";

    #[test]
    fn only_accept_json() {
        let builder = Builder::new(TABLE_URL, None, HeaderMap::new()).select("ds");
        assert_eq!(
            builder.headers.get("Accept").unwrap(),
            HeaderValue::from_static("application/json")
        );
    }

    #[test]
    fn auth_with_token() {
        let builder = Builder::new(TABLE_URL, None, HeaderMap::new()).auth("$Up3rS3crET");
        assert_eq!(
            builder.headers.get("Authorization").unwrap(),
            HeaderValue::from_static("Bearer $Up3rS3crET")
        );
    }

    #[test]
    fn select_assert_query() {
        let builder = Builder::new(TABLE_URL, None, HeaderMap::new()).select("some_table");
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
        let builder = Builder::new(TABLE_URL, None, HeaderMap::new()).order("id");
        assert_eq!(
            builder
                .queries
                .contains(&("order".to_string(), "id".to_string())),
            true
        );
    }

    #[test]
    fn limit_assert_range_header() {
        let builder = Builder::new(TABLE_URL, None, HeaderMap::new()).limit(20);
        assert_eq!(
            builder.headers.get("Range").unwrap(),
            HeaderValue::from_static("0-19")
        );
    }

    #[test]
    fn range_assert_range_header() {
        let builder = Builder::new(TABLE_URL, None, HeaderMap::new()).range(10, 20);
        assert_eq!(
            builder.headers.get("Range").unwrap(),
            HeaderValue::from_static("10-20")
        );
    }

    #[test]
    fn single_assert_accept_header() {
        let builder = Builder::new(TABLE_URL, None, HeaderMap::new()).single();
        assert_eq!(
            builder.headers.get("Accept").unwrap(),
            HeaderValue::from_static("application/vnd.pgrst.object+json")
        );
    }

    #[test]
    fn upsert_assert_prefer_header() {
        let builder = Builder::new(TABLE_URL, None, HeaderMap::new()).upsert("ignored");
        assert_eq!(
            builder.headers.get("Prefer").unwrap(),
            HeaderValue::from_static("return=representation,resolution=merge-duplicates")
        );
    }

    #[test]
    fn not_rpc_should_not_have_flag() {
        let builder = Builder::new(TABLE_URL, None, HeaderMap::new()).select("ignored");
        assert_eq!(builder.is_rpc, false);
    }

    #[test]
    fn rpc_should_have_body_and_flag() {
        let builder = Builder::new(RPC_URL, None, HeaderMap::new()).rpc("{\"a\": 1, \"b\": 2}");
        assert_eq!(builder.body.unwrap(), "{\"a\": 1, \"b\": 2}");
        assert_eq!(builder.is_rpc, true);
    }

    #[test]
    fn chain_filters() -> Result<(), Box<dyn std::error::Error>> {
        let builder = Builder::new(TABLE_URL, None, HeaderMap::new())
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
