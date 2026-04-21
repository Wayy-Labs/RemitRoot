# Backend 🛠️

> **Central API and services for RemitRoot** — handles farmer profiles, vendor registry, webhooks, and off-chain data management.

---

## Table of Contents

- [Overview](#overview)
- [Services](#services)
- [API Routes](#api-routes)
- [Tech Stack](#tech-stack)
- [Setup](#setup)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Environment Variables](#environment-variables)
  - [Running Locally](#running-locally)
- [Database Schema](#database-schema)
- [Webhook Integration](#webhook-integration)
- [Testing](#testing)
- [Deployment](#deployment)

---

## Overview

The backend is the central hub that connects all RemitRoot components. It manages off-chain data, processes mobile money webhooks, and provides REST APIs for the frontend applications.

**Key responsibilities:**
- Farmer and vendor profile management
- Escrow state mirroring from Stellar blockchain
- Mobile money webhook processing
- SMS/USSD communication via Africa's Talking
- KYC data handling (SEP-0012)

---

## Services

### 📦 `backend-api`

Central REST API handling off-chain data: farmer profiles, vendor registry, notification dispatch.

```
packages/backend-api/
├── src/
│   ├── routes/
│   │   ├── farmers.ts         # Farmer CRUD + KYC status
│   │   ├── vendors.ts         # Vendor registry
│   │   ├── escrows.ts         # Escrow state mirror (indexed from chain)
│   │   └── webhooks.ts        # Mobile money inbound hooks
│   ├── services/
│   │   ├── stellar.ts         # Horizon event indexer
│   │   ├── sms.ts             # Africa's Talking SMS gateway
│   │   └── kyc.ts             # SEP-0012 KYC integration
│   ├── db/
│   │   ├── schema.ts          # Drizzle ORM schema
│   │   └── migrations/
│   └── index.ts               # Express app entry
├── .env.example
└── package.json
```

### 📦 `oracle`

Node.js service that listens for mobile money payment webhooks and triggers on-chain repayment.

```
packages/oracle/
├── src/
│   ├── listeners/
│   │   ├── mpesa.ts           # M-Pesa Daraja API listener
│   │   ├── momo.ts            # MTN MoMo listener
│   │   └── stellar.ts         # Stellar Horizon stream watcher
│   ├── triggers/
│   │   └── repayment.ts       # Calls trigger_repay() on contract
│   └── index.ts
└── package.json
```

**Oracle flow:**
1. Mobile money provider calls webhook when farmer pays
2. Oracle verifies payment amount and farmer identity
3. Oracle calls `trigger_repay(escrow_id)` on the Soroban contract
4. Contract releases proportional USDC back to sender's account

---

## API Routes

| Method | Path | Description |
|---|---|---|
| `GET` | `/farmers` | List verified farmers |
| `GET` | `/farmers/:id` | Farmer profile + escrow history |
| `POST` | `/farmers/kyc` | Submit KYC (SEP-0012) |
| `GET` | `/vendors` | List verified agro vendors |
| `GET` | `/escrows/:id` | Escrow state (mirrors chain) |
| `POST` | `/webhooks/mpesa` | M-Pesa payment notification |
| `POST` | `/webhooks/momo` | MTN Mobile Money notification |

---

## Tech Stack

| Layer | Technology |
|---|---|
| Runtime | Node.js 20+ |
| Framework | Express.js |
| Database | PostgreSQL |
| ORM | Drizzle ORM |
| Blockchain | Stellar SDK (Horizon + Soroban) |
| Mobile Money | M-Pesa Daraja API, MTN MoMo |
| Communication | Africa's Talking (SMS + USSD) |
| Authentication | JWT + SEP-0010 |
| Testing | Jest + Supertest |

---

## Setup

### Prerequisites

- **Node.js** v20+
- **PostgreSQL** 14+
- **Redis** (for caching and job queues)
- **Stellar CLI** (for contract interaction)

### Installation

```bash
# Navigate to backend directory
cd Backend

# Install dependencies
npm install

# Set up database
npm run db:migrate

# Seed initial data (optional)
npm run db:seed
```

### Environment Variables

Copy and configure the environment file:

```bash
cp .env.example .env
```

**Required variables:**

```env
# Database
DATABASE_URL=postgresql://localhost:5432/remitroot
REDIS_URL=redis://localhost:6379

# Stellar
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org
ESCROW_CONTRACT_ID=

# Authentication
JWT_SECRET=your-super-secret-jwt-key
API_SECRET=your-api-secret

# Mobile Money (Oracle)
MPESA_CONSUMER_KEY=your-mpesa-key
MPESA_CONSUMER_SECRET=your-mpesa-secret
MPESA_SHORTCODE=your-mpesa-shortcode
MPESA_PASSKEY=your-mpesa-passkey

# Communication
AFRICAS_TALKING_API_KEY=your-africas-talking-key
AFRICAS_TALKING_USERNAME=your-africas-talking-username

# External Services
KYC_PROVIDER_URL=https://kyc-provider.com/api
ANCHOR_WEBHOOK_SECRET=your-webhook-secret
```

### Running Locally

```bash
# Start development server
npm run dev

# Start with hot reload
npm run dev:watch

# Build for production
npm run build

# Start production server
npm start

# Run database migrations
npm run db:migrate

# Create new migration
npm run db:migration:create migration_name

# Reset database
npm run db:reset
```

The backend API will be available at `http://localhost:4000`

---

## Database Schema

### Core Tables

```sql
-- Farmers table
CREATE TABLE farmers (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  stellar_address VARCHAR(56) UNIQUE NOT NULL,
  phone_number VARCHAR(20) UNIQUE NOT NULL,
  full_name VARCHAR(255) NOT NULL,
  country_code VARCHAR(2) NOT NULL,
  kyc_status VARCHAR(20) DEFAULT 'pending',
  credit_score INTEGER DEFAULT 0,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

-- Vendors table
CREATE TABLE vendors (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  stellar_address VARCHAR(56) UNIQUE NOT NULL,
  business_name VARCHAR(255) NOT NULL,
  location_lat DECIMAL(10, 8),
  location_lng DECIMAL(11, 8),
  is_verified BOOLEAN DEFAULT false,
  created_at TIMESTAMP DEFAULT NOW()
);

-- Escrows table (mirrors on-chain state)
CREATE TABLE escrows (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  contract_escrow_id VARCHAR(64) UNIQUE NOT NULL,
  sender_address VARCHAR(56) NOT NULL,
  farmer_id UUID REFERENCES farmers(id),
  vendor_id UUID REFERENCES vendors(id),
  amount DECIMAL(20, 8) NOT NULL,
  currency VARCHAR(3) DEFAULT 'USDC',
  status VARCHAR(20) NOT NULL,
  crop_season VARCHAR(50),
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

-- Repayments table
CREATE TABLE repayments (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  escrow_id UUID REFERENCES escrows(id),
  amount DECIMAL(20, 8) NOT NULL,
  mobile_money_provider VARCHAR(20) NOT NULL,
  transaction_id VARCHAR(100) UNIQUE NOT NULL,
  confirmed_at TIMESTAMP DEFAULT NOW()
);
```

---

## Webhook Integration

### Mobile Money Providers

**M-Pesa (Kenya/Tanzania):**
- Endpoint: `/webhooks/mpesa`
- Security: HMAC signature validation
- Events: Payment confirmation, reversal

**MTN MoMo (Multiple African countries):**
- Endpoint: `/webhooks/momo`
- Security: API key authentication
- Events: Deposit notification, status updates

### Stellar Webhooks

**Horizon Event Stream:**
- Listens for contract transactions
- Updates local escrow state
- Triggers notifications

**Contract Events:**
- `escrow_created`
- `voucher_minted`
- `voucher_redeemed`
- `repayment_triggered`

---

## Testing

```bash
# Run all tests
npm test

# Run with coverage
npm run test:coverage

# Run integration tests
npm run test:integration

# Run specific test file
npm test -- farmers.test.ts

# Run tests in watch mode
npm run test:watch
```

**Test categories:**
- Unit tests for services and utilities
- Integration tests for API endpoints
- Database migration tests
- Webhook processing tests

---

## Deployment

### Docker

```bash
# Build Docker image
docker build -t remitroot-backend .

# Run with Docker Compose
docker-compose up -d

# View logs
docker-compose logs -f backend
```

### Production

```bash
# Install production dependencies
npm ci --only=production

# Run database migrations
npm run db:migrate

# Start production server
NODE_ENV=production npm start
```

**Health checks:**
- `/health` - Basic health status
- `/health/ready` - Database and external services check
- `/metrics` - Prometheus metrics endpoint

---

## Contributing

1. Follow the existing code style (ESLint + Prettier)
2. Write tests for new features
3. Update documentation for API changes
4. Use conventional commit messages
5. Ensure all tests pass before submitting PR

---

## License

MIT © 2026 RemitRoot Contributors
