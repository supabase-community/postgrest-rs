# postgrest-rs

[![Build](https://github.com/supabase/postgrest-rs/workflows/CI/badge.svg)](https://github.com/supabase/postgrest-rs/actions?query=branch%3Amaster)
[![Crate](https://img.shields.io/crates/v/postgrest.svg)](https://crates.io/crates/postgrest)
[![API](https://docs.rs/postgrest/badge.svg)](https://docs.rs/postgrest)
[![License: Apache-2.0 OR MIT](https://img.shields.io/crates/l/postgrest.svg)](#license)

[PostgREST](https://postgrest.org/) client-side library for Rust. This library provides an ORM-like interface to PostgREST.

## Installation

```bash
cargo add postgrest
```

## Usage

### Create a Client

```rust
use postgrest::Postgrest;

let client = Postgrest::new("https://your.postgrest.endpoint")
    .insert_header("apikey", "your-api-key");
```

### Select

```rust
let resp = client
    .from("users")
    .select("id,name,email")
    .execute()
    .await?;

let body = resp.text().await?;
```

### Insert

```rust
let resp = client
    .from("users")
    .insert(r#"[{"name": "Alice", "email": "alice@example.com"}]"#)
    .execute()
    .await?;
```

### Update

```rust
let resp = client
    .from("users")
    .eq("id", "1")
    .update(r#"{"name": "Bob"}"#)
    .execute()
    .await?;
```

### Upsert

```rust
let resp = client
    .from("users")
    .upsert(r#"[{"id": 1, "name": "Bob", "email": "bob@example.com"}]"#)
    .execute()
    .await?;
```

### Delete

```rust
let resp = client
    .from("users")
    .eq("id", "1")
    .delete()
    .execute()
    .await?;
```

### Stored Procedures (RPC)

```rust
let resp = client
    .rpc("add", r#"{"a": 1, "b": 2}"#)
    .execute()
    .await?;
```

### Authentication

```rust
// Set a JWT on individual requests
let resp = client
    .from("users")
    .auth("your-jwt-token")
    .select("*")
    .execute()
    .await?;

// Or set default headers on the client
let client = Postgrest::new("https://your.postgrest.endpoint")
    .insert_header("apikey", "your-api-key")
    .insert_header("Authorization", "Bearer your-jwt-token");
```

### Filters

```rust
let resp = client
    .from("countries")
    .select("name,population")
    .eq("continent", "Europe")
    .gte("population", "1000000")
    .order("population.desc")
    .limit(10)
    .execute()
    .await?;
```

| Method | Operator | Description |
|--------|----------|-------------|
| `eq` | `=` | Equal |
| `neq` | `!=` | Not equal |
| `gt` | `>` | Greater than |
| `gte` | `>=` | Greater than or equal |
| `lt` | `<` | Less than |
| `lte` | `<=` | Less than or equal |
| `like` | `LIKE` | Pattern match (case-sensitive) |
| `ilike` | `ILIKE` | Pattern match (case-insensitive) |
| `is` | `IS` | Null / boolean check |
| `in_` | `IN` | One of a list of values |
| `fts` | `@@` | Full-text search |
| `plfts` | `@@` | Plain-to-tsquery |
| `phfts` | `@@` | Phrase-to-tsquery |
| `wfts` | `@@` | Web-search-to-tsquery |
| `cs` | `@>` | Contains |
| `cd` | `<@` | Contained by |
| `ov` | `&&` | Overlaps |
| `sl` | `<<` | Strictly left of |
| `sr` | `>>` | Strictly right of |
| `nxl` | `&<` | Does not extend left |
| `nxr` | `&>` | Does not extend right |
| `adj` | `-\|-` | Adjacent |

### CSV Response

Request results as CSV instead of JSON:

```rust
let resp = client
    .from("users")
    .select("*")
    .csv()
    .execute()
    .await?;

let csv_text = resp.text().await?;
```

### GeoJSON Response

Request results as GeoJSON (requires a PostGIS geometry/geography column):

```rust
let resp = client
    .from("locations")
    .select("*")
    .geojson()
    .execute()
    .await?;

let geojson = resp.text().await?;
```

### Explain (Query Plans)

Retrieve the execution plan for a query:

```rust
// Basic explain
let resp = client
    .from("users")
    .select("*")
    .explain()
    .execute()
    .await?;

// With options (analyze, verbose, format)
let resp = client
    .from("users")
    .select("*")
    .explain_with_options(true, true, "json")
    .execute()
    .await?;
```

### Typed Responses

Enable the `typed` feature to deserialize responses directly into Rust structs:

```toml
[dependencies]
postgrest = { version = "1.0", features = ["typed"] }
```

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: i64,
    name: String,
    email: String,
}

let users: Vec<User> = client
    .from("users")
    .select("id,name,email")
    .execute_and_parse::<User>()
    .await?;
```

## Features

- [x] Select, Insert, Update, Upsert, Delete
- [x] Stored procedures (RPC)
- [x] All PostgREST filter operators
- [x] Authentication (`auth()`, `insert_header()`)
- [x] CSV response format
- [x] GeoJSON response format
- [x] Query plan explain / explain with options
- [x] Typed deserialization (`typed` feature)
- [x] Ordering, limiting, pagination
- [x] Resource embedding (foreign table joins)

## Contributing

Contributions are welcome! Feel free to create an issue or submit a pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
