# Oracle Service

Oracle database service for RemitRoot application.

## Overview

This service provides a Node.js/TypeScript wrapper for Oracle database interactions with connection pooling, migrations, and logging.

## Setup

### Prerequisites

- Node.js 16+
- Oracle Database 11g+
- npm or yarn

### Installation

```bash
npm install
```

### Environment Configuration

Copy `.env.example` to `.env` and configure your Oracle database connection:

```bash
cp .env.example .env
```

Configure the following variables:

- `ORACLE_USER`: Your Oracle database user
- `ORACLE_PASSWORD`: Your Oracle database password
- `ORACLE_CONNECTION_STRING`: Connection string (e.g., `localhost:1521/XEPDB1`)
- `ORACLE_POOL_MIN`: Minimum connections in pool (default: 2)
- `ORACLE_POOL_MAX`: Maximum connections in pool (default: 10)

## Development

### Build

```bash
npm run build
```

### Run

```bash
npm run dev
```

### Testing

```bash
npm test
npm run test:watch
npm run test:coverage
```

## Migrations

### Run Migrations

```bash
npm run migrate
```

### Create New Migration

```bash
npm run migrate:create your_migration_name
```

This will create a new migration file in `src/migrations/` with the timestamp prefix.

## Project Structure

```
src/
├── index.ts                 # Main entry point
├── database/
│   └── connection.ts        # Oracle connection pool management
├── models/
│   ├── User.ts             # User model
│   └── Transaction.ts      # Transaction model
├── migrations/
│   ├── Migration.ts        # Migration base class
│   ├── 001_initial_schema.ts
│   ├── runner.ts          # Migration runner
│   └── create.ts          # Migration generator
├── utils/
│   └── logger.ts          # Logging utility
└── __tests__/
    └── database.test.ts   # Database tests
```

## API

### OracleDatabase

Singleton class for managing Oracle database connections.

#### Methods

- `getInstance()`: Get singleton instance
- `initialize()`: Initialize connection pool
- `getConnection()`: Get a connection from the pool
- `execute<T>(sql, binds?, options?)`: Execute SQL and return result
- `query<T>(sql, binds?)`: Execute query and return rows
- `getPoolInfo()`: Get pool statistics
- `closePool()`: Close all connections

### Logger

Utility for logging to console and files.

#### Methods

- `getInstance()`: Get singleton instance
- `error(message, error?)`: Log error
- `warn(message, data?)`: Log warning
- `info(message, data?)`: Log info
- `debug(message, data?)`: Log debug

## License

See LICENSE file in the root directory.
