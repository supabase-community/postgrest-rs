use crate::Builder;

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

impl Builder {
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
}
