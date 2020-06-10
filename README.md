# postgrest-rs

[![Build](https://github.com/supabase/postgrest-rs/workflows/CI/badge.svg)](https://github.com/supabase/postgrest-rs/actions?query=branch%3Amaster)
[![Crate](https://img.shields.io/crates/v/postgrest.svg)](https://crates.io/crates/postgrest)
[![API](https://docs.rs/postgrest/badge.svg)](https://docs.rs/postgrest)
[![License: Apache-2.0 OR MIT](https://img.shields.io/crates/l/postgrest.svg)](#license)

[PostgREST](https://postgrest.org/) client-side library 🦀. This library provides an ORM interface to PostgREST.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
postgrest = "1.0"
```

Simple example:

```rust
use postgrest::Postgrest;

let client = Postgrest::new("https://your.postgrest.endpoint");
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
    .eq("name", "New Zealand")                        // You can filter for equality...
    .gt("id", "20")
    .lt("id", "20")
    .gte("id", "20")
    .lte("id", "20")
    .like("name", "%United%")                         // ...do pattern matching...
    .ilike("name", "%United%")
    .is("name", "null")
    .in_("name", vec!["China", "France"])
    .neq("name", "China")
    .fts("phrase", "The Fat Cats", Some("english"))   // ...do full text search...
    .plfts("phrase", "The Fat Cats", None)
    .phfts("phrase", "The Fat Cats", Some("english"))
    .wfts("phrase", "The Fat Cats", None)
    .cs("countries", "(10,20)")
    .cd("countries", "(10,20)")
    .ov("population_range", (100, 500))
    .sl("population_range", (100, 500))               // ...and range operations!
    .sr("population_range", (100, 500))               // Find out more about the filters at:
    .nxl("population_range", (100, 500))              // https://postgrest.org/en/stable/api.html#operators
    .nxr("population_range", (100, 500))
    .adj("population_range", (100, 500))
    .select("*")
    .execute()
    .await?;
```

Check out the [API docs](https://docs.rs/postgrest) for more info!

## Contributing

Contributions are welcome! There might be some features you want in, or some
unclear documentation, or a bug—either way, feel free to create an issue, and
we'll work it out!

Boring stuff below.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

-   Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
    https://www.apache.org/licenses/LICENSE-2.0)
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

---

![](https://camo.githubusercontent.com/f266fbf746ee25b75480696176e356b84688f1e9/68747470733a2f2f67697463646e2e78797a2f7265706f2f73757061626173652f6d6f6e6f7265706f2f6d61737465722f7765622f7374617469632f77617463682d7265706f2e676966)
