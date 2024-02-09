# linker
Quick link service using [Rocket]()

## Setup

Install Rust

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Install sqlx

    cargo install sqlx-cli

Create local database

    cargo sqlx database create

    cargo sqlx migrate run

    cargo sqlx prepare

### API

    GET /<id>
        Redirect to link with id, if not found use default link

    POST /<id>, body = { link: String }
        Create new link with id and link from body

    DELETE /<id>
        Delete link with id

    GET /debug/<id>
        Get link without redirection

    GET /debug/
        Get all links
