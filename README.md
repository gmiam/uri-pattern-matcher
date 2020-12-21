# Uri-pattern-matcher

A Rust library for parsing URI (path) patterns, matching against them and compare two patterns with each other.

This library is fast (1 to 10μs to parse common patterns) and don't use any additional allocation.

The library is considered immature (pre 1.0 release) because it hasn't been tested in production yet.
However, considering how few features it has, it should be pretty safe to use. 

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
uri-pattern-matcher = "0.1"
```

# Example

Here are examples for the common usages of this crate:

```rust
let pattern: UriPattern = "/api/{resource}/{id}/details".into();
assert!(pattern.is_match("/api/resource/id1/details"));
assert!(pattern.is_match("/api/customer/John/details"));
```

```rust
let pattern: UriPattern = "/api/{foo}/{bar}/zzz".into();
let pattern2: UriPattern = "/api/{foo}/bar/{zzz}".into();
assert_ne!(pattern, pattern2);
assert!(pattern > pattern2);
```

We are also able combine all of this using Iterators.
Here we'll retrieve the most specific pattern matching our candidate string:
```rust
// we use this because fold_first is behind this flag and on nightly only
#![feature(iterator_fold_self)]
let patterns: Vec<UriPattern> = vec![
    "/api/{foo}/{bar}/zzz".into(),
    "/api/{foo}/bar/{zzz}".into(),
    "/{api}/{foo}/foo/{zzz}".into()
    ];
let candidate = "/api/resource/bar/zzz";
let best_match = patterns.iter()
           .filter(|p| p.is_match(candidate))
           .fold_first(|a, b| {
               if a >= b { a } else { b }
           });
assert_eq!(best_match.unwrap(), &UriPattern::from("/api/{foo}/{bar}/zzz"));
```

# License

The library is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).