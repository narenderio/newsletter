# Newsletter Service (Rust Backend)

A production-ready backend service built in **Rust** for managing newsletter subscriptions, designed for reliability, scalability, and maintainability.

## 🚀 Tech Stack

- Rust
- Actix Web
- PostgreSQL
- Docker
- SQLx

## 📦 Features

- REST API for managing subscriptions
- Health check endpoint
- Database integration with PostgreSQL
- Dockerized setup for local development
- SQL migrations using SQLx

⚙️ Getting Started

1. Start Postgres
   ./scripts/init_db.sh
2. Run migrations
   sqlx migrate run
3. Run the application
   cargo run
