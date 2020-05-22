use crate::Builder;

fn clean_param<S>(param: S) -> String
where
    S: Into<String>,
{
    let param = param.into();
    if ",.:()".chars().any(|c| param.contains(c)) {
        format!("\"{}\"", param)
    } else {
        param
    }
}

impl Builder {
    /// Finds all rows whose value on the stated `column` exactly matches the
    /// specified `filter`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .eq("name", "New Zealand");
    /// ```
    pub fn eq<S, T>(mut self, column: S, filter: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((clean_param(column), format!("eq.{}", clean_param(filter))));
        self
    }

    /// Finds all rows whose value on the stated `column` doesn't exactly match
    /// the specified `filter`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .neq("name", "China");
    /// ```
    pub fn neq<S, T>(mut self, column: S, filter: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((clean_param(column), format!("neq.{}", clean_param(filter))));
        self
    }

    /// Finds all rows whose value on the stated `column` is greater than the
    /// specified `filter`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .gt("id", "20");
    /// ```
    pub fn gt<S, T>(mut self, column: S, filter: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((clean_param(column), format!("gt.{}", clean_param(filter))));
        self
    }

    /// Finds all rows whose value on the stated `column` is greater than or
    /// equal to the specified `filter`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .gte("id", "20");
    /// ```
    pub fn gte<S, T>(mut self, column: S, filter: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((clean_param(column), format!("gte.{}", clean_param(filter))));
        self
    }

    /// Finds all rows whose value on the stated `column` is less than the
    /// specified `filter`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .lt("id", "20");
    /// ```
    pub fn lt<S, T>(mut self, column: S, filter: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((clean_param(column), format!("lt.{}", clean_param(filter))));
        self
    }

    /// Finds all rows whose value on the stated `column` is less than or equal
    /// to the specified `filter`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .lte("id", "20");
    /// ```
    pub fn lte<S, T>(mut self, column: S, filter: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((clean_param(column), format!("lte.{}", clean_param(filter))));
        self
    }

    /// Finds all rows whose value in the stated `column` matches the supplied
    /// `pattern` (case sensitive).
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .like("name", "%United%");
    ///
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .like("name", "%United States%");
    /// ```
    pub fn like<S, T>(mut self, column: S, pattern: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        let pattern = pattern.into().replace('%', "*");
        self.queries
            .push((clean_param(column), format!("like.{}", pattern)));
        self
    }

    /// Finds all rows whose value in the stated `column` matches the supplied
    /// `pattern` (case insensitive).
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .ilike("name", "%United%");
    ///
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .ilike("name", "%united states%");
    /// ```
    pub fn ilike<S, T>(mut self, column: S, pattern: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        let pattern = pattern.into().replace('%', "*");
        self.queries
            .push((clean_param(column), format!("ilike.{}", pattern)));
        self
    }

    /// A check for exact equality (null, true, false), finds all rows whose
    /// value on the stated `column` exactly match the specified `filter`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .is("name", "null");
    /// ```
    pub fn is<S, T>(mut self, column: S, filter: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((clean_param(column), format!("is.{}", clean_param(filter))));
        self
    }

    /// Finds all rows whose value on the stated `column` is found on the
    /// specified `values`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .in_("name", vec!["China", "France"]);
    ///
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .in_("capitals", vec!["Beijing,China", "Paris,France"]);
    ///
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .in_("food_supplies", vec!["carrot (big)", "carrot (small)"]);
    /// ```
    pub fn in_<S, T, U>(mut self, column: S, values: T) -> Self
    where
        S: Into<String>,
        T: IntoIterator<Item = U>,
        U: Into<String>,
    {
        let values: Vec<_> = values.into_iter().map(clean_param).collect();
        self.queries
            .push((clean_param(column), format!("in.({})", values.join(","))));
        self
    }

    // TODO: Sanitize input
    /// Finds all rows whose json || array || range value on the stated `column`
    /// contains the values specified in `filter`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .cs("age_range", "(10,20)");
    /// ```
    pub fn cs<S, T>(mut self, column: S, filter: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((clean_param(column), format!("cs.{}", filter.into())));
        self
    }

    // TODO: Sanitize input
    /// Finds all rows whose json || array || range value on the stated `column`
    /// is contained by the specified `filter`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .cd("age_range", "(10,20)");
    /// ```
    pub fn cd<S, T>(mut self, column: S, filter: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("cd.{}", filter.into())));
        self
    }

    /// Finds all rows whose range value on the stated `column` is strictly to
    /// the left of the specified `range`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .sl("age_range", (10, 20));
    /// ```
    pub fn sl<S>(mut self, column: S, range: (i64, i64)) -> Self
    where
        S: Into<String>,
    {
        self.queries
            .push((column.into(), format!("sl.({},{})", range.0, range.1)));
        self
    }

    /// Finds all rows whose range value on the stated `column` is strictly to
    /// the right of the specified `range`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .sr("age_range", (10, 20));
    /// ```
    pub fn sr<S>(mut self, column: S, range: (i64, i64)) -> Self
    where
        S: Into<String>,
    {
        self.queries
            .push((column.into(), format!("sr.({},{})", range.0, range.1)));
        self
    }

    /// Finds all rows whose range value on the stated `column` does not extend
    /// to the left of the specified `range`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .nxl("age_range", (10, 20));
    /// ```
    pub fn nxl<S>(mut self, column: S, range: (i64, i64)) -> Self
    where
        S: Into<String>,
    {
        self.queries
            .push((column.into(), format!("nxl.({},{})", range.0, range.1)));
        self
    }

    /// Finds all rows whose range value on the stated `column` does not extend
    /// to the right of the specified `range`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .nxr("age_range", (10, 20));
    /// ```
    pub fn nxr<S>(mut self, column: S, range: (i64, i64)) -> Self
    where
        S: Into<String>,
    {
        self.queries
            .push((column.into(), format!("nxr.({},{})", range.0, range.1)));
        self
    }

    /// Finds all rows whose range value on the stated `column` is adjacent to
    /// the specified `range`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .adj("age_range", (10, 20));
    /// ```
    pub fn adj<S>(mut self, column: S, range: (i64, i64)) -> Self
    where
        S: Into<String>,
    {
        self.queries
            .push((column.into(), format!("adj.({},{})", range.0, range.1)));
        self
    }

    // TODO: Sanitize input
    /// Finds all rows whose array || range value on the stated `column` is
    /// contained by the specified `filter`.
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .ov("age_range", "(10,20)");
    /// ```
    pub fn ov<S, T>(mut self, column: S, filter: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.queries
            .push((column.into(), format!("cd.{}", filter.into())));
        self
    }

    /// Finds all rows whose tsvector value on the stated `column` matches
    /// to_tsquery(`tsquery`).
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .fts("phrase", "The Fat Cats", Some("english"));
    /// ```
    pub fn fts<S, T>(mut self, column: S, tsquery: T, config: Option<&str>) -> Self
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

    /// Finds all rows whose tsvector value on the stated `column` matches
    /// plainto_tsquery(`tsquery`).
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .plfts("phrase", "The Fat Cats", None);
    /// ```
    pub fn plfts<S, T>(mut self, column: S, tsquery: T, config: Option<&str>) -> Self
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

    /// Finds all rows whose tsvector value on the stated `column` matches
    /// phraseto_tsquery(`tsquery`).
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .phfts("phrase", "The Fat Cats", Some("english"));
    /// ```
    pub fn phfts<S, T>(mut self, column: S, tsquery: T, config: Option<&str>) -> Self
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

    /// Finds all rows whose tsvector value on the stated `column` matches
    /// websearch_to_tsquery(`tsquery`).
    /// ```
    /// # use postgrest::Postgrest;
    /// let req = Postgrest::new("http://localhost:3000")
    ///     .from("table")
    ///     .select("*")
    ///     .wfts("phrase", "The Fat Cats", None);
    /// ```
    pub fn wfts<S, T>(mut self, column: S, tsquery: T, config: Option<&str>) -> Self
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
