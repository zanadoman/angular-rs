[![Frontend](https://img.shields.io/badge/frontend-Angular%20+%20Ng--Bootstrap-red)]()
[![Rust](https://img.shields.io/badge/backend-Rust%20+%20Axum-orange)]()
[![Database](https://img.shields.io/badge/database-SQLx%20+%20PostgreSQL-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()

# 🚀 [Angular x Rust + Axum Webstack](https://github.com/zanadoman/angular-rs)

A modern full-stack web application template built with **Angular**, **Rust (Axum)**, and **SQLx** — combining the performance of Rust with the flexibility of Angular.

---

## 🧩 Tech Stack

- **Frontend:** [🅰️ Angular](https://angular.dev/) + [Ng-Bootstrap](https://ng-bootstrap.github.io/#/home)
- **Backend:** [🦀 Rust](https://rust-lang.org/) + [Axum](https://github.com/tokio-rs/axum)
- **Database Layer:** [🚀 SQLx](https://github.com/launchbadge/sqlx)
- **Database:** [🐘 PostgreSQL](https://www.postgresql.org/)
- **Documentation:** [📚 Swagger](https://swagger.io/)
- **Build Tools:** `npm`, `cargo`, `sqlx-cli`, `watchexec-cli`

---

## ⚙️ Prerequisites

Before you begin, ensure you have the following installed:

- [🐘 PostgreSQL](https://www.postgresql.org/download/)
- [🦀 Rust & Cargo](https://rust-lang.org/tools/install/)
- [📦 Node.js & npm](https://nodejs.org/)

Then install required Rust tools:

```sh
rustup toolchain install stable --profile minimal
cargo install sqlx-cli -F postgres,rustls --no-default-features
cargo install watchexec-cli --no-default-features
```

---

## 🛠️ Project Setup

Clone and configure the project:

```sh
git clone https://github.com/zanadoman/angular-rs.git
cd angular-rs

npm ci                                               # Install frontend dependencies
cp .env.example .env                                 # Copy environment configuration
cp .cargo/config.toml.example .cargo/config.toml     # Copy rust-analyzer configuration
sqlx database setup                                  # Create database and run migrations
```

Edit your `.env` and `.cargo/config.toml` files to set up database credentials and connection info.

---

## 🧱 Development Commands

### 🗃️ Database (SQLx)

```sh
sqlx database setup                              # Set up database
sqlx database reset                              # Drop & recreate database (fresh setup)
sqlx database drop                               # Drop database
sqlx migrate add -r -s create_examples_table     # Create a new migration
sqlx migrate run                                 # Apply migrations
sqlx migrate revert                              # Roll back last migration
```

### ⚙️ Backend (Rust)

```sh
cargo run --bin argon2     # Generate Argon2 password hashes
```

### 🧩 Frontend (Angular)

```sh
npx ng g interceptor interceptors/example                     # Generate a new HTTP interceptor
npx ng g enum models/example-model                            # Generate a new data model
npx ng g interface models/example-model                       # Generate a new data model
npx ng g service services/example-service                     # Generate a new injectable service
npx ng g guard guards/example                                 # Generate a new route guard
npx ng g component pages/example-page --ng-html               # Generate a new page
npx ng g component components/example-component --ng-html     # Generate a new component
```

### 🧹 Project-wide (Tooling)

```sh
npm run format     # Format source code
```

---

## 🔁 Build & Run

### 🔧 Development Mode (Live Reloading)

```sh
npm run watch
```

### 🏗️ Production Build

```sh
npm run build
```

### 🌐 Serve Production

```sh
npm run serve
```

### 🧪 Linting & Testing

```sh
npm test
```

## 📁 Project Structure (Overview)

### 🦀 Backend (Rust + Axum)

```
api/api.rs            # API entry point
api/lib.rs            # Library root
api/models/*.rs       # Data models and database abstractions
api/handlers/*.rs     # HTTP handlers (controllers)
api/router.rs         # Primary router configuration
api/router/*.rs       # Sub-routers grouped by feature or module
api/argon2.rs         # Utility binary for generating Argon2 password hashes (cargo run --bin argon2)
```

### 🅰️ Frontend (Angular)

```
src/app/app.config.ts      # Configuration
src/app/app.routes.ts      # Routing definitions
src/app/interceptors/*     # HTTP interceptors
src/app/models/*           # Data models
src/app/services/*         # Injectable services
src/app/guards/*           # Route guards
src/app/pages/*            # Pages
src/app/components/*       # Components
```

---

## 📚 API Documentation (Swagger / OpenAPI)

This project exposes an auto-generated Swagger UI in development mode for exploring and testing the backend API.

### 🌐 Swagger UI

Once the backend is running, you can access the interactive API documentation at:

```
http://localhost:8080/api/docs/swagger-ui/
```

> The Swagger UI is generated automatically from the backend’s OpenAPI definitions and always reflects the current API state.

### 📄 OpenAPI Spec

The raw OpenAPI specification is also available (useful for codegen or external tools):

```
http://localhost:8080/api/docs/openapi.json
```

---

## 🐳 Docker Support

### 🚀 Start DB Container

```sh
docker compose up -d db
```

### 🔁 Run a One-Off Command Inside the App Container

```sh
docker compose run --rm app bash -c 'sqlx migrate run'
```

### 🚀 Serve the App Inside the App Container

```sh
docker compose run --rm -P app bash -c 'npm run build && npm run serve'
```

### 🧹 Cleanup

```sh
docker compose down --remove-orphans
```

> ⚠️ If you are running the app inside Docker:
>
> - Set `APP_ADDRESS=0.0.0.0` in `.env` so the app is accessible from outside the container.
> - If PostgreSQL is also running in Docker, set `DATABASE_URL` to use the DB service name as the host (usually `db`) instead of `127.0.0.1`.

---

**_🚀 Enjoy!_**
