FROM jo3mccain/rusty as builder-base

ADD . /project
WORKDIR /project

COPY . .
RUN cargo test --all-features && \
    cargo package --all-features --allow-dirty