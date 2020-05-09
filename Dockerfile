FROM rustlang/rust:nightly AS base

# Get necessary bins
RUN cargo install diesel_cli --no-default-features --features postgres \
 && cargo install cargo-watch

FROM rustlang/rust:nightly AS app

# Cache the bins and the package index
COPY --from=base $CARGO_HOME/bin $CARGO_HOME/bin
COPY --from=base $CARGO_HOME/registry/index $CARGO_HOME/registry/index
COPY --from=base $CARGO_HOME/registry/cache $CARGO_HOME/registry/cache

WORKDIR /app

COPY entrypoint.sh entrypoint.sh
RUN chmod +x entrypoint.sh
