# Capture The IP — [ipv4.quest]

This is an incredibly over-engineered Rust version of _ipv4.games_ <sup>[site][ipv4.games], [code][ipv4.games-src]</sup>.

The objective is to send requests to the site **[ipv4.quest]** from as many different IP addresses as possible.

If you claim and hold the majority of an `#.0.0.0/8` address block, you get a point.

## Technology

- language: [rust](https://www.rust-lang.org/)
- web framework: [axum] <sup>(build on [tower] and [hyper], runs on [tokio])</sup>
- datastore interface/orm: [diesel] - including some async flavours
- datastore: [postgres] - powerful and versatile database
- web hosting: [fly.io] - quirky but awesome app hosting
- database hosting: [neon.tech] - free tech preview of serverless postgres

## Design and architecture

### Project structure

This project uses a cargo workspace and is divided into several crates for different purposes.

There are crates for the binaries/executables and library crates for the business logic of the project.

`cti_server`, `cti_refresher`, and `cti_migrate` are the binaries. The first one is the most important, it's the "game" server itself. The refresher is currently not used separately, the server does this job itself as a background thread for now. The last one as the name indicates is to help with database migrations; since the project uses diesel as its database interface and ORM, its up to the administrating person to decide with tool to use, the cti_migrate can run on a remote server though without any Rust tooling present.

The actual business logic for the server and refresher lives in `cti_core`, which itself also consumes some helper crates like `cti_constants`, `cti_types`, `cti_env`, `cti_schema`, and `cti_assets`. The helper crates mostly came to exist as the migration tool's logic is a bit different, but still needed some common definitions and functions.

```
$ tree -d -L 1
.
├── cti_assets
├── cti_constants
├── cti_core
├── cti_env
├── cti_migrate
├── cti_refresher
├── cti_schema
├── cti_server
├── cti_types
├── frontend
├── migrations
└── tmp
```

### The Server

Since [axum] is a pretty slim web application framework, the code is neither exciting nor controversial.

Early on—due to some data model decisions—the service includes an HTTP app as well as a background worker thread.

To provide a nice graceful shutdown functionality the crate `tokio-graceful-shutdown` is used to manage the different subsystems (HTTP server, background worker, a nice shutdown timer).

The background thread communicates via channels, so that the shutdown process is also graceful for itself; [tokio's select!] is a pretty useful tool here.

All the background thread does is to continuously update some materialized views in an interval and set a timestamp when the last run was.

### Database

In total there are 3 tables, where one of them is due to diesel (keeping track of migrations). The other two are `captures` and `timestamps`. The latter is only to store a single timestamp for the refresh cycle, as I didn't see a need to involve another datastore like Redis or implement some distributed messaging system (which is probably required to really over-engineer this solution I guess).

The main table `captures` stores each claimed IP address match. To keep the storage needs in check only the last capture of an IP gets stored, so no history per IP is kept. Meaning: if you lost an IP to someone else, you disappear from the database (unless you have more IPs claimed, of course).

For various purposes there are a bunch of [materialized views], which are views, but persisted like tables. They can be refreshed to get the most recent version of the query they represent. This approach was used as a caching layer on the database side. The data does not need to be realtime and the mentioned timestamp informs users about when the last refresh/update happened.

The queries are not too slow, but even a few hundred milliseconds are already too slow for me. The materialized views help to keep that low enough for now.

Last but not least these views keep some nasty SQL away from the app itself.

### Frontend

It's very simple setup here. Almost all views are static and compiled into the final binary of the server.

The only dynamic view is the `/claim` endpoint, which sends a tiny HTML response with your IP and name included. That should make it usable outside of browsers, enabling you to verify everything worked without extra API calls.

The JavaScript code is vanilla, no fancy library or framework used. The index page makes a few API calls to retrieve some JSON data and renders it into the right places.

The only reason to leave it like that instead of over-engineering it is to provide a decent user experience. Any framework will ultimately add overhead/bloat which I don't want here.

One day I might add a secondary main page where I test fancy stuff like Wasm based views (maybe with Yew or whatever is the latest and greatest for such task).

-----

_Don't forget to visit **[ipv4.quest]** and claim your IP and block!_

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

<!-- links -->

[ipv4.quest]: https://ipv4.quest/
[ipv4.games]: https://ipv4.games/
[ipv4.games-src]: https://github.com/jart/cosmopolitan/blob/master/net/turfwar/turfwar.c
[rust]: https://www.rust-lang.org/
[axum]: https://crates.io/crates/axum
[tower]: https://crates.io/crates/tower
[hyper]: https://crates.io/crates/hyper
[tokio]: https://crates.io/crates/tokio
[diesel]: https://diesel.rs/
[postgres]: https://www.postgresql.org/
[fly.io]: https://fly.io/
[neon.tech]: https://neon.tech/
[tokio's select!]: https://docs.rs/tokio/latest/tokio/macro.select.html
[materialized views]: https://www.postgresql.org/docs/current/rules-materializedviews.html
