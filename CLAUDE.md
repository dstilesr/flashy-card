# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Flashy Card is a web application for studying languages using the flashcard method. It's built with Rust using Axum web framework, SQLx for database interactions, and Askama for HTML templating.

## Architecture

### Core Components

- **main.rs**: Application entry point that parses CLI arguments and starts the Axum server
- **server.rs**: Router setup and database connection pool initialization
- **server/views.rs**: Request handlers that render templates
- **templates.rs**: Askama template struct definitions

### Database Schema

The application uses PostgreSQL with four main tables:
- **languages**: Stores languages to study (with unique slugs)
- **card_decks**: Collections of cards for a given language
- **card_types**: Pre-populated lookup table for parts of speech (Noun, Verb, Adjective, etc.) and other categories (Idiom, Phrase, Root, etc.)
- **cards**: Individual flashcards with target word, translation, hints, examples, and additional info
- **card_to_deck**: Many-to-many mapping between cards and decks

### Request Flow

1. Axum listener receives request
2. Router matches route and calls appropriate handler in `server/views.rs`
3. Handler uses database pool (PgPool) from router state to query data
4. Askama template is rendered with data
5. HTML response returned to client

### Static Files & Templates

- HTML templates: `flashy-card/templates/*.html` (Askama syntax)
- Static assets: `flashy-card/static/` (served at `/static` route)
- Templates use `base.html` as a base template with blocks for extending

## Development Commands

### Local Development (with Docker)

Start the full stack:
```bash
docker compose up --build -d
```

This starts two containers:
- PostgreSQL database on port 5432
- Flashy Card app on port 3000

Access the app at http://localhost:3000

Stop containers:
```bash
docker compose down
```

### Database Setup

The DATABASE_URL environment variable must be set (see `.env.example`):
```
DATABASE_URL=postgres://dev:dev@localhost:5432/flashcards
```

Run migrations using SQLx CLI:
```bash
cd flashy-card
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

### Building & Running Locally

Build the application:
```bash
cd flashy-card
cargo build --release
```

Run the application:
```bash
cd flashy-card
./target/release/flashy-card
```

CLI options:
- `-m, --max-db-connections <N>`: Max database connections (default: 5)
- `-s, --static-dir <PATH>`: Static files directory (default: "static")
- `-b, --bind-address <IP>`: Bind address (default: "0.0.0.0")
- `-p, --port <PORT>`: Port to listen on (default: 3000)

### Testing

Run tests:
```bash
cd flashy-card
cargo test
```

## Key Dependencies

- **axum**: Web framework
- **sqlx**: Async PostgreSQL driver with compile-time query verification
- **askama**: Type-safe template engine
- **tokio**: Async runtime
- **tower-http**: HTTP middleware (serving static files)
- **clap**: CLI argument parsing

## Important Notes

### SQLx Compile-Time Query Verification

SQLx checks SQL queries at compile time. When adding new queries:
1. Database must be running with up-to-date migrations
2. Set DATABASE_URL environment variable
3. Use SQLx macros: `query!`, `query_as!`, etc.

Alternatively, prepare queries for offline compilation:
```bash
cargo sqlx prepare
```

### Docker Multi-Stage Build

The Dockerfile uses a two-stage build:
1. Builder stage: Compiles Rust code in Alpine
2. Runner stage: Minimal Alpine image with just the binary

Templates, static files, and migrations must be copied to appropriate stages.

### Adding Routes

1. Add handler function in `server/views.rs`
2. Register route in `server.rs` `create_router()` function
3. Create template struct in `templates.rs` if needed
4. Create corresponding HTML template in `templates/` directory