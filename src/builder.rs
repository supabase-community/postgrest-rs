use reqwest::{Client, Error, Method, Response};

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
    // TODO: Maybe change to HeaderMap in the future
    headers: Vec<(String, String)>,
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
            ..Default::default()
        }
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

    // TODO: Open-ended range
    pub fn limit(mut self, count: usize) -> Self {
        self.headers
            .push(("Content-Range".to_string(), format!("0-{}", count - 1)));
        self
    }

    pub fn single(mut self) -> Self {
        self.headers.push((
            "Accept".to_string(),
            "application/vnd.pgrst.object+json".to_string(),
        ));
        self
    }

    // TODO: Write-only tables
    // TODO: URL-encoded payload
    // TODO: Allow specifying columns
    pub fn insert(mut self, body: &str) -> Self {
        self.method = Method::POST;
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
        self.method = Method::POST;
        self.headers.push((
            "Prefer".to_string(),
            // Maybe check if this works as intended...
            "return=representation; resolution=merge-duplicates".to_string(),
        ));
        self.body = Some(body.to_string());
        self
    }

    pub fn single_upsert(mut self, primary_column: &str, key: &str, body: &str) -> Self {
        self.method = Method::PUT;
        self.headers
            .push(("Prefer".to_string(), "return=representation".to_string()));
        self.queries
            .push((primary_column.to_string(), format!("eq.{}", key)));
        self.body = Some(body.to_string());
        self
    }

    pub fn update(mut self, body: &str) -> Self {
        self.method = Method::PATCH;
        self.headers
            .push(("Prefer".to_string(), "return=representation".to_string()));
        self.body = Some(body.to_string());
        self
    }

    pub fn delete(mut self) -> Self {
        self.method = Method::DELETE;
        self.headers
            .push(("Prefer".to_string(), "return=representation".to_string()));
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

    pub async fn execute(self) -> Result<Response, Error> {
        let mut req = Client::new().request(self.method.clone(), &self.url);
        if let Some(schema) = self.schema {
            // NOTE: Upstream bug: RPC only works with Accept-Profile
            // Will change when upstream is fixed
            let key = if !self.is_rpc || self.method == Method::GET || self.method == Method::HEAD {
                "Accept-Profile"
            } else {
                "Content-Profile"
            };
            req = req.header(key, schema);
        }
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
