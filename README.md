[![Frontend](https://img.shields.io/badge/frontend-Angular%20+%20Ng--Bootstrap-red)]()
[![Rust](https://img.shields.io/badge/backend-Rust%20+%20Axum-orange)]()
[![Database](https://img.shields.io/badge/database-SQLx%20+%20MySQL-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()

# 🚀 Angular x Rust + Axum Webstack

A modern full-stack web application template built with **Angular**, **Rust (Axum)**,
and **SQLx** — combining the performance of Rust with the flexibility of Angular.

---

## 🧩 Tech Stack

* **Frontend:** [🅰️ Angular](https://angular.dev/) + [🎨 Ng-Bootstrap](https://ng-bootstrap.github.io/#/home)
* **Backend:** [🦀 Rust](https://rust-lang.org/) + [Axum](https://github.com/tokio-rs/axum)
* **Database Layer:** [SQLx](https://github.com/launchbadge/sqlx)
* **Database:** [🐬 MySQL](https://www.mysql.com/) (or compatible RDBMS)
* **Build Tools:** `sqlx-cli`, `cargo`, `npm`, `watchexec-cli`

---

## ⚙️ Prerequisites

Before you begin, ensure you have the following installed:

* [🐬 MySQL](https://www.mysql.com/) (or compatible RDBMS)
* [🦀 Rust & Cargo](https://rust-lang.org/tools/install/)
* [📦 Node.js & npm](https://nodejs.org/)

Then install global Rust tools:

```sh
rustup toolchain install nightly --profile minimal
rustup component add rust-docs rustfmt clippy --toolchain nightly
cargo install sqlx-cli --no-default-features -F mysql,rustls
cargo install watchexec-cli --no-default-features
```

---

## 🛠️ Project Setup

Clone and configure the project:

```sh
git clone https://github.com/zanadoman/angular-rs.git
cd angular-rs

npm ci                   # Install frontend dependencies
cp .env.example .env     # Copy environment configuration
sqlx database setup      # Create database and run migrations
```

Edit your `.env` file to set up database credentials and connection info.

---

## 🧱 Development Commands

### 🗃️ Database (SQLx)

```sh
sqlx database setup                      # Setup database
sqlx database reset                      # Drop & recreate database (fresh setup)
sqlx database drop                       # Drop database
sqlx migrate add -r -s new_migration     # Create a new migration
sqlx migrate run                         # Apply migrations
sqlx migrate revert                      # Roll back last migration
```

### ⚙️ Backend (Rust)

```sh
cargo fmt                  # Format source code
cargo clippy               # Run static analyzer
cargo run --bin argon2     # Generate Argon2 password hashes
```

### 🧩 Frontend (Angular)

```sh
npx ng g c new-component     # Generate a new Angular component
npx ng g s new-service       # Generate a new Angular service
npx ng g g new-guard         # Generate a new Angular guard
```

###

---

## 🔄 Build & Run

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

### 🧪 Testing

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
src/app.config.ts     # Frontend configuration
src/routes.ts         # Frontend routing definitions
src/pages/**/         # Standalone pages
src/pages/root/       # Main page
```

---

## 🐳 Docker Support

### 🔁 Run a One-Off Command Inside the App Container

```sh
docker compose run --rm app foobar
```

### 🧹 Cleanup

```sh
docker compose down --remove-orphans
```

---

***🚀 Enjoy!***
