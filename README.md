# Hammer Assets v2

A robust architecture for institute-grade digital asset aggregation. This project periodically fetches crypto assets balances and prices, handling duplications and selection among data fetched from distinct data sources which may have overlapping coverage.

## Project Structure

This is a Rust workspace with multiple crates:

- `crates/migration/` - Database migrations using SeaORM
- `crates/entity/` - Database entity definitions (to be created)
- `crates/service/` - Database service layer (to be created)
- `crates/cam-api/` - CAM API client implementation (to be created)
- `crates/debank-api/` - DeBank API client implementation (to be created)
- `crates/worker/` - Periodic data fetching worker (to be created)

## Database Schema

The project uses SeaORM with PostgreSQL. The schema includes:
- Wallet and wallet metadata tables
- Currency and price tables with provider mapping
- Balance and balance entry tables with priority-based provider selection

## Database Operations

### Prerequisites

Install SeaORM CLI:
```bash
cargo install sea-orm-cli
```

### Running Migrations

```bash
# Run migrations from project root
sea-orm-cli migrate up --migration-dir ./crates/migration/src

# Or with database URL
sea-orm-cli migrate up

# Rollback migrations
sea-orm-cli migrate down --migration-dir ./crates/migration/src

# Check migration status
sea-orm-cli migrate status --migration-dir ./crates/migration/src
```

### Generating Entities

```bash
# Generate entities from migration files (recommended)
sea-orm-cli generate entity \
  --migration-dir ./crates/migration/src \
  --date-time-crate time \
  --with-copy-enums \
  --output-dir ./crates/entity/src

# Generate entities from database (if you have a live database)
sea-orm-cli generate entity \
  --database-url "postgres://username:password@localhost:5432/your_database" \
  --date-time-crate time \
  --with-copy-enums \
  --output-dir ./crates/entity/src
```

### Generate Command Options

- `--date-time-crate time`: Use `time` crate for datetime types (recommended)
- `--with-copy-enums`: Includes enum definitions directly in entity files (recommended)
- `--migration-dir`: Path to migration files (alternative to database-url)
- `--database-url`: PostgreSQL connection string for live database
- `--output-dir`: Directory to save generated entity files
- `--with-serde`: Include Serde derive macros for JSON serialization

### Database Connection

Set your database URL in environment:
```bash
export DATABASE_URL="postgres://username:password@localhost:5432/your_database"
```

## Code Quality

This project uses Rust 2024 edition with the latest stable versions of dependencies:

- SeaORM 1.1
- Tokio 1.37
- Rust Decimal 1.35

The `references/` directory is excluded from all cargo operations via `.cargo/config.toml`.

### Code Quality Checks

```bash
# Format check (using stable rustfmt)
cargo fmt --all -- --check

# Linting with clippy
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Type checking
cargo check --workspace --all-targets --all-features

# Run all quality checks
cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo check --workspace --all-targets --all-features
```

### Development Setup

```bash
# Install required tools
rustup component add rustfmt clippy

# Format code
cargo fmt --all

# Run clippy with suggestions
cargo clippy --workspace --all-targets --all-features
```