# postgrest-rs 🦀

[![build](https://github.com/supabase/postgrest-rs/workflows/CI/badge.svg)](https://github.com/supabase/postgrest-rs/actions?query=branch%3Amaster)
[![License: Apache-2.0 OR MIT](https://img.shields.io/crates/l/postgrest.svg)](#license)

[PostgREST](https://postgrest.org/) client-side library (Rust edition). This library aims to reach feature parity with [postgrest-js](https://github.com/supabase/postgrest-js) in providing an "ORM-like" RESTful interface.

## Usage

Generally, you want to instantiate a `Postgrest` struct, optionally switch
schema with `.schema()`, call `.from()` (or `.rpc()` for stored procedures), do
some filtering and stuff, and then call `.execute()`.

Simple example:

```rust
use postgrest::Postgrest;

let client = Postgrest::new("https://your-postgrest-endpoint");
let resp = client
    .from("your_table")
    .select("*")
    .execute()
    .await?;
let body = resp
    .text()
    .await?;
```

Using filters:

```rust
let resp = client
    .from("your_table")
    .eq("country", "Germany")
    .gte("id", "20")
    .select("*")
    .execute()
    .await?;
```

Updating a table:

```rust
let resp = client
    .from("your_table")
    .eq("username", "soedirgo")
    .update("{\"organization\": \"supabase\"}")
    .execute()
    .await?;
```

Executing stored procedures:

```rust
let resp = client
    .rpc("add", "{\"a\": 1, \"b\": 2}")
    .execute()
    .await?;
```

_Not enough filters_:

```rust
let resp = client
    .from("countries")
    .eq("name", "New Zealand")
    .gt("id", "20")
    .lt("id", "20")
    .gte("id", "20")
    .lte("id", "20")
    .like("name", "%United%")
    .ilike("name", "%United%")
    .is("name", "null")
    .in_("name", vec!["China", "France"])
    .neq("name", "China")
    .fts("phrase", "The Fat Cats", Some("english"))
    .plfts("phrase", "The Fat Cats", None)
    .phfts("phrase", "The Fat Cats", Some("english"))
    .wfts("phrase", "The Fat Cats", None)
    .cs("countries", "(10,20)")
    .cd("countries", "(10,20)")
    .ov("population_range", (100, 500))
    .sl("population_range", (100, 500))
    .sr("population_range", (100, 500))
    .nxl("population_range", (100, 500))
    .nxr("population_range", (100, 500))
    .adj("population_range", (100, 500))
    .select("*")
    .execute()
    .await?;
```

More examples incoming!

## Limitations

This library doesn't show the full extent of PostgREST, and definitely doesn't
replace the need to learn PostgREST. Some known limitations are:

-   Doesn't support `not`, `and`, and `or` in filtering
-   Many inputs are unsanitized (multi-column select, insert/update body, etc.)
-   Counting (with HEAD verb)
-   Resource embedding (embedded filters, etc.)

That said, if there are any features you want in, feel free to create an issue!

## Contributing

-   Fork the repo on GitHub
-   Clone the project to your own machine
-   Commit changes to your own branch
-   Push your work back up to your fork
-   Submit a Pull request so that we can review your changes and merge

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

-   Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
