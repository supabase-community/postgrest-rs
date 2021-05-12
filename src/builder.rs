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
    /// Creates a new `Builder` with the specified `schema`.
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

    /// Add arbitrary headers to the request.  For instance when you may want to connect
    /// through an api gateway that needs an API key header in addition to the JWT.
    ///
    /// # Example
    ///
    /// ```
    /// use postgrest::Postgrest;
    ///
    /// static header_name: &'static str =  "foo";
    /// let header_value: String = "bar".to_string();
    ///
    /// let client = Postgrest::new("https://your.postgrest.endpoint/rest/v1/");
    /// client
    ///     .from("table")
    ///     .insert_header(header_name, header_value);
    /// ```
    pub fn insert_header(mut self, header_name: &'static str, header_value: String) -> Self {
        self.headers.insert(
            header_name,
            HeaderValue::from_str(&header_value)
                .expect("Couldn't convert supplied header value from String to str."),
        );
        self
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
    pub fn select<T>(mut self, columns: T) -> Self
    where
        T: Into<String>,
    {
        self.method = Method::GET;
        self.queries.push(("select".to_string(), columns.into()));
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
    pub fn delete(mut self) -> Self {
        self.method = Method::DELETE;
        self.headers
            .insert("Prefer", HeaderValue::from_static("return=representation"));
        self
    }

    /// Performs a stored procedure call. This should only be used through the
    /// `rpc()` method in `Postgrest`.
    pub fn rpc<T>(mut self, params: T) -> Self
    where
        T: Into<String>,
    {
        self.method = Method::POST;
        self.body = Some(params.into());
        self.is_rpc = true;
        self
    }

    /// Executes the PostgREST request.
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
    fn with_insert_header() {
        static A_HEADER_KEY: &str = "foo";
        let a_header_value: String = "bar".to_string();
        let builder = Builder::new(TABLE_URL, None).insert_header(A_HEADER_KEY, a_header_value);
        assert_eq!(
            builder.headers.get("foo").unwrap(),
            HeaderValue::from_static("bar")
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
