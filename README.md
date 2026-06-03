# 📧 Newsletter Service (Rust Backend)

A production-ready, secure newsletter management and delivery service built with **Rust** and **Actix Web**. This project is designed for high reliability, scalability, and maintainability, featuring session management, idempotent delivery, and background workers.

---

## ✨ Features

- **Subscription Management**: Securely handle user subscriptions with email confirmation loops.
- **Newsletter Publication**: Multi-channel publication (API & Admin dashboard) with delivery status tracking.
- **Idempotent Delivery**: Ensures subscribers receive each newsletter exactly once, even in case of retries.
- **Background Worker**: Purpose-built worker for asynchronous newsletter delivery, decoupling API response time from email sending.
- **Secure Authentication**: Password-based authentication with Argon2 hashing and session management.
- **Rich Telemetry**: Comprehensive logging and tracing with Bunyan-formatted JSON output for production observability.
- **Robust Configuration**: Environment-specific configurations (local, production, test) using YAML.

---

## 🛠️ Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/) (2024 Edition)
- **Web Framework**: [Actix Web](https://actix.rs/)
- **Database**: [PostgreSQL](https://www.postgresql.org/) with [SQLx](https://github.com/launchbadge/sqlx) (Typed SQL)
- **Sessions**: [Redis](https://redis.io/) via `actix-session`
- **Security**: Argon2 (Hashing), HMAC (Secrets), Flash Messages
- **Testing**: `wiremock`, `fake`, `claims`, `quickcheck`
- **Deployment**: Docker, DigitalOcean App Platform

---

## 🧱 Architecture

The project is split into two primary execution units:
1. **API Server**: Handles incoming HTTP requests for user interactions, management, and newsletter submissions.
2. **Background Worker**: A dedicated task that consumes the newsletter queue and manages the delivery to subscribers asynchronously.

---

## 🚀 Getting Started

### Prerequisites

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Docker**: [Install Docker](https://docs.docker.com/get-docker/)
- **SQLx CLI**: 
  ```bash
  cargo install sqlx-cli --no-default-features --features rustls,postgres
  ```

### Local Setup

1. **Initialize Infrastructure**:
   Launch PostgreSQL and Redis containers, create the database, and run migrations:
   ```bash
   ./scripts/init_db.sh
   ./scripts/init_redis.sh
   ```

2. **Environment Variables**:
   Copy `.env.example` to `.env` (if applicable) or ensure variables in `configuration/local.yaml` are correct.

3. **Run the Application**:
   ```bash
   cargo run
   ```

---

## 📡 API Endpoints

### Public Endpoints
- `GET /health_check` - Service status & health monitoring.
- `GET /` - Public home page.
- `POST /subscriptions` - Subscribe a new email to the newsletter.
- `GET /subscriptions/confirm` - Confirm a subscription via token.
- `GET /login` / `POST /login` - Authentication management.

### Admin Endpoints (Protected)
- `GET /admin/dashboard` - Admin control panel.
- `GET /admin/newsletters` - Newsletter composition portal.
- `POST /admin/newsletters` - Submit a newsletter for delivery.
- `POST /admin/logout` - Terminate session.

---

## 🧪 Testing

The project uses extensive integration testing to ensure reliability.

```bash
# Run all tests
cargo test

# Run tests in offline mode (using cached SQLx data)
SQLX_OFFLINE=true cargo test
```

---

## 📦 Deployment

The project is containerized and ready for deployment via **Docker**. A `spec.yaml` is provided for seamless integration with **DigitalOcean App Platform**.

```bash
# Build the Docker image
docker build --tag newsletter --file Dockerfile .
```

---

## 📝 Logging & Monitoring

Logs are formatted in JSON using the **Bunyan formatter**, making them easy to ingest into ELK stacks or managed logging services.

To view human-readable logs locally:
```bash
cargo run | bunyan
```
*(Requires `bunyan` CLI tool installed via `npm install -g bunyan`)*
