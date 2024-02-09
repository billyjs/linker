# linker
Quick link service using axum, sqlite and sqlx

## Setup

Install Rust

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Install sqlx cli

    cargo install sqlx-cli

Create local database

    cargo sqlx database create

    cargo sqlx migrate run

    cargo sqlx prepare

### API

    GET /<id>
        Redirect to link with id

    PUT /<id>, body = { href: String }
        Create new link with id and link from body

    DELETE /<id>
        Delete link with id

    GET /debug/<id>
        Get link without redirection

    GET /debug/
        Get all links

### TODO

ci
* cargo install sqlx-cli && cargo sqlx prepare --check

docker

tests

openapi spec
* not really priority
