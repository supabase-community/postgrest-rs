use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error, Method, Response,
};

macro_rules! filter {
    ( $( $op:ident ),* ) => {
        $(
            pub fn $op(mut self, column: &str, param: &str) -> Self {
                self.queries.push((column.to_string(),
                                   format!("{}.{}", stringify!($op), param)));
                self
            }
        )*
    }
}

#[derive(Default)]
pub struct Builder {
    method: Method,
    url: String,
    schema: Option<String>,
    queries: Vec<(String, String)>,
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
        Builder {
            method: Method::GET,
            url: url.to_string(),
            schema,
            headers: HeaderMap::new(),
            ..Default::default()
        }
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
        self.headers.append(
            "Content-Range",
            HeaderValue::from_str(&format!("0-{}", count - 1)).unwrap(),
        );
        self
    }

    pub fn range(mut self, low: usize, high: usize) -> Self {
        self.headers.append(
            "Content-Range",
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
            .append("Prefer", HeaderValue::from_static("return=representation"));
        self.body = Some(body.to_string());
        self
    }

    pub fn insert_csv(mut self, body: &str) -> Self {
        self.headers
            .append("Content-Type", HeaderValue::from_static("text/csv"));
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

    // It's unfortunate that `in` is a keyword, otherwise it'd belong in the
    // collection of filters below
    filter!(
        eq, gt, gte, lt, lte, neq, like, ilike, is, fts, plfts, phfts, wfts, cs, cd, ov, sl, sr,
        nxr, nxl, adj, not
    );

    pub fn in_(mut self, column: &str, param: &str) -> Self {
        self.queries
            .push((column.to_string(), format!("in.{}", param)));
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
            // NOTE: Upstream bug: RPC only works with Accept-Profile
            // Will change when upstream is fixed
            let key = if !self.is_rpc || self.method == Method::GET || self.method == Method::HEAD {
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

        let resp = req.send().await?;

        Ok(resp)
    }
}
