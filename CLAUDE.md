# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Flashy Card is a web application for studying languages using the flashcard method. It's built with Rust using Axum web framework, SQLx for database interactions, and Askama for HTML templating.

## Architecture

### Core Components

- **main.rs**: Application entry point that parses CLI arguments and starts the Axum server
- **server.rs**: Router setup and database connection pool initialization
- **server/views.rs**: Request handlers that render HTML templates
- **server/api.rs**: API endpoints (mounted at `/api`) that return JSON/non-HTML responses
- **repository.rs**: Database query functions that interact with PostgreSQL via SQLx
- **types.rs**: Data structures for database models and request/response types
- **templates.rs**: Askama template struct definitions

### Database Schema

The application uses PostgreSQL with five main tables:
- **languages**: Stores languages to study (with unique slugs for URLs)
- **card_decks**: Collections of cards for a given language (references `language_id`)
- **card_types**: Pre-populated lookup table with 11 types: Noun, Verb, Adjective, Adverb, Conjunction, Preposition, Root, Idiom, Pronoun, Phrase, Numeral
- **cards**: Individual flashcards with target word, translation, hints, examples, and additional info (references `type_id`)
- **card_to_deck**: Many-to-many junction table mapping cards to decks

Migrations are stored in `flashy-card/migrations/` and managed by SQLx CLI.

### Request Flow

**HTML Page Requests:**
1. Axum listener receives request
2. Router matches route and calls handler in `server/views.rs`
3. Handler extracts query parameters and PgPool from Axum extractors
4. Handler calls functions in `repository.rs` to query database
5. Askama template is rendered with data from database
6. HTML response returned to client

**API Requests:**
1. Request hits `/api/*` route
2. Router forwards to API subrouter defined in `server/api.rs`
3. Handler processes request and returns JSON or other non-HTML response

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

The DATABASE_URL environment variable must be set:
```
DATABASE_URL=postgres://dev:dev@localhost:5432/flashcards
```

For local development with Docker, this is automatically configured in `docker-compose.yml`.

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

Or in debug mode with logging:
```bash
cd flashy-card
RUST_LOG=debug cargo run
```

CLI options:
- `-m, --max-db-connections <N>`: Max database connections (default: 5)
- `-s, --static-dir <PATH>`: Static files directory (default: "static")
- `-b, --bind-address <IP>`: Bind address (default: "0.0.0.0")
- `-p, --port <PORT>`: Port to listen on (default: 3000)

### Logging

The application uses `env_logger`. Set the `RUST_LOG` environment variable to control log levels:
```bash
RUST_LOG=debug cargo run    # Debug level
RUST_LOG=info cargo run     # Info level
RUST_LOG=warn cargo run     # Warn level
```

In Docker Compose, logging is configured via the `RUST_LOG` environment variable in `docker-compose.yml`.

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

**For HTML views:**
1. Do not assume the CSS styles you need exist. Check the stylesheet to find what you need if it exists, or add it if it does not.
2. Add handler function in `server/views.rs`
3. Register route in `server.rs` `create_router()` function
4. Create template struct in `templates.rs` if needed
5. Create corresponding HTML template in `templates/` directory

**For API endpoints:**
1. Add handler function in `server/api.rs`
2. Register route in `server/api.rs` `make_api_router()` function
3. API routes are automatically nested under `/api` prefix

### Adding Database Queries

1. Add query function to `repository.rs`
2. Use `sqlx::query!` or `query_as!` macros for compile-time verification
3. Define return types in `types.rs` with `#[derive(FromRow)]` for SQLx mapping
4. Call repository functions from handlers in `views.rs` or `api.rs`