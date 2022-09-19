# stage 1 - cache skeleton
FROM rust as planner

WORKDIR /app

RUN cargo install cargo-chef

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


# stage 2 - use cached deps
from rust as cacher

WORKDIR /app

RUN cargo install cargo-chef

COPY --from=planner app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json


# stage 3 - build
FROM rust as builder

# copy app into the docker
COPY . /app

# set work dir
WORKDIR /app

# copy deps
COPY --from=cacher /app/target target
COPY --from=cacher usr/local/cargo /usr/local/cargo

# build app
RUN cargo build --release

# use google distroless as runtime image
FROM gcr.io/distroless/cc-debian11

# copy app from builder
COPY --from=builder ./app/target/release/back_end /app/back_end

# set work dir in second image
WORKDIR /app

# start app
CMD ["./back_end"]
