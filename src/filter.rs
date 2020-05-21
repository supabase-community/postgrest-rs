use crate::Builder;

impl Builder {
    pub fn eq<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("eq.{}", param.into())));
        self
    }

    pub fn neq<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("neq.{}", param.into())));
        self
    }

    pub fn gt<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("gt.{}", param.into())));
        self
    }

    pub fn gte<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("gte.{}", param.into())));
        self
    }

    pub fn lt<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("lt.{}", param.into())));
        self
    }

    pub fn lte<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("lte.{}", param.into())));
        self
    }

    pub fn like<S>(mut self, column: S, param: &str) -> Self
    where
        S: Into<String>,
    {
        let param = str::replace(param, '%', "*");
        self.queries
            .push((column.into(), format!("like.{}", param)));
        self
    }

    pub fn ilike<S>(mut self, column: S, param: &str) -> Self
    where
        S: Into<String>,
    {
        let param = str::replace(param, '%', "*");
        self.queries
            .push((column.into(), format!("ilike.{}", param)));
        self
    }

    pub fn is<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("is.{}", param.into())));
        self
    }

    pub fn in_<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<Vec<String>>,
    {
        // As per PostgREST docs, `in` should allow quoted commas
        let param: Vec<String> = param
            .into()
            .iter()
            .map(|s| {
                if s.contains(',') {
                    format!("\"{}\"", s)
                } else {
                    s.to_string()
                }
            })
            .collect();
        self.queries
            .push((column.into(), format!("in.({})", param.join(","))));
        self
    }

    pub fn cs<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<Vec<String>>,
    {
        self.queries
            .push((column.into(), format!("cs.{{{}}}", param.into().join(","))));
        self
    }

    pub fn cd<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<Vec<String>>,
    {
        self.queries
            .push((column.into(), format!("cd.{{{}}}", param.into().join(","))));
        self
    }

    pub fn sl<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("sl.{}", param.into())));
        self
    }

    pub fn sr<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("sr.{}", param.into())));
        self
    }

    pub fn nxl<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("nxl.{}", param.into())));
        self
    }

    pub fn nxr<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("nxr.{}", param.into())));
        self
    }

    pub fn adj<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("adj.{}", param.into())));
        self
    }

    pub fn ov<S, T>(mut self, column: S, param: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("ov.{}", param.into())));
        self
    }

    pub fn fts<S, T>(mut self, column: S, tsquery: T, config: Option<String>) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        let config = if let Some(conf) = config {
            format!("({})", conf)
        } else {
            String::new()
        };
        self.queries
            .push((column.into(), format!("fts{}.{}", config, tsquery.into())));
        self
    }

    pub fn plfts<S, T>(mut self, column: S, tsquery: T, config: Option<String>) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        let config = if let Some(conf) = config {
            format!("({})", conf)
        } else {
            String::new()
        };
        self.queries
            .push((column.into(), format!("plfts{}.{}", config, tsquery.into())));
        self
    }

    pub fn phfts<S, T>(mut self, column: S, tsquery: T, config: Option<String>) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        let config = if let Some(conf) = config {
            format!("({})", conf)
        } else {
            String::new()
        };
        self.queries
            .push((column.into(), format!("phfts{}.{}", config, tsquery.into())));
        self
    }

    pub fn wfts<S, T>(mut self, column: S, tsquery: T, config: Option<String>) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        let config = if let Some(conf) = config {
            format!("({})", conf)
        } else {
            String::new()
        };
        self.queries
            .push((column.into(), format!("wfts{}.{}", config, tsquery.into())));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Postgrest;

    const REST_URL: &str = "http://localhost:3000";

    #[test]
    fn simple_filters_assert_query() {
        let client = Postgrest::new(REST_URL);

        let req = client.from("users").select("ignored").eq("column", "key");
        assert_eq!(
            req.queries
                .contains(&("column".to_string(), "eq.key".to_string())),
            true
        );

        let req = client.from("users").select("ignored").neq("column", "key");
        assert_eq!(
            req.queries
                .contains(&("column".to_string(), "neq.key".to_string())),
            true
        );

        let req = client.from("users").select("ignored").gt("column", "key");
        assert_eq!(
            req.queries
                .contains(&("column".to_string(), "gt.key".to_string())),
            true
        );

        // ...
    }
}
