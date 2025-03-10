# The HASH Graph Query Layer

To run the HASH Graph Query Layer make sure `cargo-make` is installed:

```shell
cargo install cargo-make
```

## Run the Graph

1.  In order to set up the database, first the database has to be started:

> **CAUTION:** At the moment, the graph starts the services it depends on differently to the rest of the codebase.
>
> **Before running the following command, ensure you tear down any existing `external-services` that were started as outlined in the [README for the workspace](/packages/hash/README.md).** Similarly, **ensure you call `deployment-down` before trying to run the `external-services`.**
>
> It is planned to address this by revisiting the way the services are orchestrated, while still allowing for local non-container-based development.

```shell
cargo make deployment-up
```

_It is possible to teardown the database with the equivalent `deployment-down` task_

Then, the Graph Query Layer can be started:

```shell
cargo run
```

### Logging configuration

Some of the libraries used are very talkative in `trace` logging configurations, especially `mio`, `hyper`, and `tokio_util`.
If you're interested in just increasing the logs for the Graph, we recommend specifically targeting the crates with `RUST_LOG=graph=trace,hash_graph=trace`.

## Development

In order to build run the following command:

```shell
cargo make build
```

In order to create an optimized build, run:

```shell
cargo make --profile production build
```

Please see the list of all possible `cargo make` commands:

```shell
cargo make --list-all-steps
```

Every command line argument passed will also be forwarded to the subcommand, e.g. this will not only build the documentation but also open it in the browser:

```shell
cargo make doc --open
```

---

It's possible to recreate the database by using

```shell
cargo make recreate-db
```

## Test the code

The code base has two test suites: The unit test suite and the integration tests. To run the unit-test suite, simply run the `test` command:

```shell
cargo make test
```

For the integration tests, the database needs to be deployed [as specified here](../README.md#running-the-database). Next, the integration test suite can be started:

```shell
cargo make test-integration
```

The REST API can be tested as well. Note, that this requires the Graph to run and does not clean up the database after running:

```shell
cargo make test-rest-api
```

## Generate OpenAPI client

The HASH Graph produces an OpenAPI Spec while running, which can be used to generate the `@hashintel/hash-graph-client` typescript client. In the `hash_graph` directory run:

```shell
cargo make generate-openapi-client
```

Make sure to run this command whenever changes are made to the specification. CI will not pass otherwise.

## Benchmark the code

The benchmark suite can be ran with:

```shell
cargo make bench
```

### Note:

The benchmarks currently have a fairly costly (in time) setup cost per suite on initialization.
As such, the benchmark databases **are not cleaned up** between or after runs.

This also means that if breaking changes are made to the seeding logic, **you must manually delete the benchmark tables to have them reseed**.
