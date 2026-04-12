# RemitRoot 🌾

> **Cross-border farm input financing on Stellar** — diaspora families fund seeds, fertilizer, and equipment directly in their home country. No middlemen. No cash leakage. Repayment built in.

---

## Table of Contents

- [The Problem](#the-problem)
- [How RemitRoot Works](#how-remitroot-works)
- [Architecture Overview](#architecture-overview)
- [Monorepo Structure](#monorepo-structure)
- [Packages](#packages)
  - [contracts](#-contractsescrow)
  - [app-sender](#-app-sender)
  - [app-farmer](#-app-farmer)
  - [app-vendor](#-app-vendor)
  - [backend-api](#-backend-api)
  - [oracle](#-oracle)
  - [shared](#-shared)
- [Tech Stack](#tech-stack)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Environment Variables](#environment-variables)
  - [Running Locally](#running-locally)
- [Smart Contract Reference](#smart-contract-reference)
- [Money Flow](#money-flow)
- [Stellar Primitives Used](#stellar-primitives-used)
- [Testing](#testing)
- [Deployment](#deployment)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [License](#license)

---

## The Problem

Millions of migrants send remittances home every year, but the money rarely creates lasting economic mobility. Smallholder farmers in sub-Saharan Africa and Southeast Asia can't afford seeds or inputs at planting season. Microloans charge 30–60% annual interest. The sender has zero visibility into how money is used — and the farmer has no path to building credit.

**RemitRoot turns a remittance into a micro-investment:**

- The diaspora sender locks funds for a specific purpose (seed purchase, fertilizer, irrigation equipment)
- The farmer receives a tokenized voucher redeemable only at a verified agro-input vendor
- After harvest, repayment flows back automatically — no bank account required, just a phone
- Every transaction is auditable on Stellar's public ledger

---

## How RemitRoot Works

```
[Sender in US/EU]  →  Lock USDC via Anchor  →  Soroban Escrow Contract
                                                        ↓
                                              Mint Voucher Token (RVCH)
                                                        ↓
                              [Farmer scans QR at Agro Vendor]  →  Burn voucher, release goods
                                                        ↓
                              [Harvest season]  →  Farmer repays via M-Pesa / Mobile Money
                                                        ↓
                              Repayment streams back to Sender's Stellar account
```

**For the sender:** Connect wallet → choose a farmer profile → lock funds → track impact in real time.

**For the farmer:** Receive an SMS with a QR voucher → visit registered vendor → redeem for approved goods → repay after harvest via mobile money.

**For the vendor:** Scan the farmer's QR code → burn the voucher on-chain → receive USDC instantly.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                        STELLAR NETWORK                              │
│                                                                     │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────────────┐   │
│  │ Stellar       │   │ Path Payment  │   │ Soroban Escrow       │   │
│  │ Anchor        │──▶│ (FX / USDC)  │──▶│ Contract             │   │
│  │ (Fiat on-ramp)│   │              │   │ - Lock funds         │   │
│  └──────────────┘   └──────────────┘   │ - Mint voucher       │   │
│                                         │ - Release on redeem  │   │
│                                         │ - Accept repayments  │   │
│                                         └──────────────────────┘   │
│                                                    │                │
│                                         ┌──────────▼─────────┐    │
│                                         │ Voucher Token (RVCH)│    │
│                                         │ Stellar Custom Asset │    │
│                                         └──────────┬──────────┘    │
└────────────────────────────────────────────────────┼───────────────┘
                                                      │
               ┌──────────────────────┬───────────────┘
               │                      │
    ┌──────────▼───────┐   ┌──────────▼───────────┐
    │ app-farmer        │   │ app-vendor            │
    │ (PWA / USSD)      │   │ (QR Scanner + POS)   │
    └───────────────────┘   └──────────────────────┘
               │
    ┌──────────▼───────────┐
    │ oracle               │
    │ Mobile money webhook │
    │ Triggers repayment   │
    └──────────────────────┘
```

---

## Monorepo Structure

```
remitroot/
├── packages/
│   ├── contracts/            # Soroban smart contracts (Rust)
│   │   └── escrow/
│   ├── app-sender/           # Web app for diaspora senders (Next.js)
│   ├── app-farmer/           # PWA + USSD interface for farmers (Next.js)
│   ├── app-vendor/           # QR scanner POS for agro vendors (Next.js)
│   ├── backend-api/          # REST API + webhook handler (Node/Express)
│   ├── oracle/               # Harvest oracle & mobile money listener (Node)
│   └── shared/               # Types, utils, Stellar SDK helpers
├── docs/                     # Architecture docs, diagrams, ADRs
├── scripts/                  # Deployment & CI scripts
├── .github/
│   └── workflows/            # CI/CD pipelines
├── package.json              # Root workspace config (pnpm)
├── pnpm-workspace.yaml
├── turbo.json                # Turborepo task graph
└── README.md
```

---

## Packages

### 📦 `contracts/escrow`

The core Soroban smart contract written in Rust. This is the trustless heart of RemitRoot.

```
packages/contracts/escrow/
├── src/
│   ├── lib.rs               # Contract entry point
│   ├── escrow.rs            # Core escrow logic
│   ├── voucher.rs           # Voucher token mint/burn
│   ├── repayment.rs         # Repayment stream logic
│   ├── storage.rs           # Persistent state types
│   └── errors.rs            # Contract error codes
├── tests/
│   └── integration.rs       # Full flow integration tests
└── Cargo.toml
```

**Contract state machine:**

| State | Trigger | Description |
|---|---|---|
| `Created` | `fund()` | Sender deposits USDC, escrow created |
| `Funded` | `approve_farmer()` | Admin approves farmer, escrow locks |
| `VoucherMinted` | `mint_voucher()` | RVCH token sent to farmer's wallet |
| `Redeemed` | `redeem_voucher()` | Vendor burns token, goods delivered |
| `Repaying` | `trigger_repay()` | Oracle signals harvest season |
| `Closed` | `repay()` / `default()` | Escrow fully settled or timed out |

**Key contract functions:**

```rust
// Sender locks funds for a specific farmer + vendor + season
pub fn fund(env: Env, sender: Address, vendor_id: BytesN<32>,
            crop_season: Symbol, amount: i128) -> Result<BytesN<32>, Error>

// Admin/DAO approves farmer and mints voucher
pub fn approve_farmer(env: Env, escrow_id: BytesN<32>,
                      farmer: Address) -> Result<(), Error>

// Vendor calls this to burn voucher and release USDC to themselves
pub fn redeem_voucher(env: Env, escrow_id: BytesN<32>,
                      vendor: Address) -> Result<(), Error>

// Oracle triggers repayment window after harvest
pub fn trigger_repay(env: Env, escrow_id: BytesN<32>) -> Result<(), Error>

// Farmer calls to make a partial repayment
pub fn repay(env: Env, escrow_id: BytesN<32>,
             farmer: Address, amount: i128) -> Result<(), Error>

// Cancel and refund if no farmer approved within timeout
pub fn cancel(env: Env, escrow_id: BytesN<32>) -> Result<(), Error>
```

---

### 📦 `app-sender`

The web application used by diaspora senders (built with Next.js + Freighter wallet integration).

```
packages/app-sender/
├── src/
│   ├── app/                  # Next.js App Router pages
│   │   ├── page.tsx          # Landing / dashboard
│   │   ├── fund/             # Create a new financing round
│   │   ├── track/            # Track active escrows
│   │   └── history/          # Past transactions
│   ├── components/
│   │   ├── WalletConnect.tsx  # Freighter wallet button
│   │   ├── FarmerCard.tsx     # Farmer profile preview
│   │   ├── EscrowStatus.tsx   # Real-time escrow state
│   │   └── RepaymentTracker.tsx
│   ├── hooks/
│   │   ├── useStellar.ts      # Stellar SDK wrapper
│   │   └── useEscrow.ts       # Contract interaction hooks
│   └── lib/
│       ├── stellar.ts         # Horizon + Soroban RPC client
│       └── anchor.ts          # SEP-0010 auth + SEP-0024 deposit
├── public/
└── package.json
```

**Key user flows:**
1. Connect Freighter wallet (SEP-0010 auth)
2. Browse verified farmer profiles
3. Choose vendor, crop season, amount
4. Approve USDC deposit via Stellar Anchor
5. Monitor escrow state in real time

---

### 📦 `app-farmer`

Progressive Web App optimized for low-bandwidth mobile. Also exposes a USSD menu for feature phones.

```
packages/app-farmer/
├── src/
│   ├── app/
│   │   ├── page.tsx           # Dashboard: pending vouchers
│   │   ├── voucher/           # View + share QR voucher
│   │   └── repay/             # Submit repayment
│   ├── components/
│   │   ├── VoucherQR.tsx      # QR code display (offline capable)
│   │   └── RepayForm.tsx      # Mobile money repayment form
│   ├── ussd/
│   │   └── menu.ts            # Africa's Talking USSD gateway handler
│   └── lib/
│       └── stellar.ts         # Minimal Stellar SDK (wallet-less)
├── public/
│   └── manifest.json          # PWA manifest (offline support)
└── package.json
```

**USSD menu (feature phone support):**
```
Welcome to RemitRoot
1. Check my voucher
2. Repay loan
3. Check balance
0. Exit
```

---

### 📦 `app-vendor`

Simple QR scanner interface for agro-input vendors. Works on any Android device with a camera.

```
packages/app-vendor/
├── src/
│   ├── app/
│   │   ├── page.tsx           # Scan voucher QR
│   │   ├── confirm/           # Confirm redemption + item list
│   │   └── history/           # Transaction history
│   ├── components/
│   │   ├── QRScanner.tsx      # Camera QR reader
│   │   ├── RedemptionCard.tsx # Shows voucher details before burn
│   │   └── ReceiptPrint.tsx   # Printable paper receipt
│   └── lib/
│       └── stellar.ts
└── package.json
```

---

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

**API routes:**

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

### 📦 `shared`

Shared TypeScript types, constants, and Stellar SDK utilities used across all packages.

```
packages/shared/
├── src/
│   ├── types/
│   │   ├── escrow.ts          # EscrowState, EscrowDetails, etc.
│   │   ├── farmer.ts          # FarmerProfile, KYCStatus
│   │   └── vendor.ts          # VendorProfile
│   ├── constants/
│   │   ├── contracts.ts       # Deployed contract IDs per network
│   │   └── assets.ts          # USDC, RVCH asset definitions
│   └── stellar/
│       ├── client.ts          # Horizon + Soroban RPC factory
│       ├── sep10.ts           # SEP-0010 auth helper
│       └── sep24.ts           # SEP-0024 deposit/withdraw helper
└── package.json
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| Smart contracts | Rust + Soroban (Stellar smart contracts) |
| Blockchain | Stellar Mainnet / Testnet |
| FX / Payments | Stellar Path Payments, Stellar Anchors |
| Voucher token | Stellar Custom Asset (RVCH) |
| Auth | SEP-0010 (Stellar Web Auth), Freighter wallet |
| KYC | SEP-0012 |
| Fiat on/off-ramp | SEP-0024 (Anchor interactive deposit) |
| Frontend | Next.js 14, TypeScript, Tailwind CSS |
| Backend | Node.js, Express, Drizzle ORM, PostgreSQL |
| Mobile money | Africa's Talking (USSD + SMS), M-Pesa Daraja API, MTN MoMo |
| Monorepo | pnpm workspaces + Turborepo |
| CI/CD | GitHub Actions |
| Testnet | Stellar Testnet (Friendbot funded) |

---

## Getting Started

### Prerequisites

- **Node.js** v20+
- **pnpm** v9+
- **Rust** + `cargo` (for contract development)
- **Stellar CLI** (`stellar` — includes Soroban CLI)
- A **Freighter** browser wallet (for testing the sender app)
- A Stellar Testnet account funded via [Friendbot](https://friendbot.stellar.org)

### Installation

```bash
# Clone the repo
git clone https://github.com/your-org/remitroot.git
cd remitroot

# Install all workspace dependencies
pnpm install

# Install the Stellar CLI
cargo install --locked stellar-cli --features opt

# Install Rust Wasm target for Soroban contract compilation
rustup target add wasm32-unknown-unknown
```

### Environment Variables

Copy and fill in the environment files for each package:

```bash
cp packages/backend-api/.env.example packages/backend-api/.env
cp packages/oracle/.env.example packages/oracle/.env
cp packages/app-sender/.env.example packages/app-sender/.env.local
cp packages/app-farmer/.env.example packages/app-farmer/.env.local
cp packages/app-vendor/.env.example packages/app-vendor/.env.local
```

**Core variables (all packages share these via `.env` at the root):**

```env
# Stellar network
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org
STELLAR_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

# Deployed contract IDs (filled after deployment)
ESCROW_CONTRACT_ID=

# USDC asset on testnet
USDC_ISSUER=GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5
USDC_CODE=USDC

# Voucher token
RVCH_ISSUER=          # Your issuer account public key
RVCH_CODE=RVCH

# Anchor
ANCHOR_HOME_DOMAIN=testanchor.stellar.org

# Backend
DATABASE_URL=postgresql://localhost:5432/remitroot
API_SECRET=

# Mobile money (oracle)
MPESA_CONSUMER_KEY=
MPESA_CONSUMER_SECRET=
MPESA_SHORTCODE=
AFRICAS_TALKING_API_KEY=
AFRICAS_TALKING_USERNAME=
```

### Running Locally

**1. Build the Soroban contract:**

```bash
cd packages/contracts/escrow
stellar contract build
```

**2. Deploy to Stellar Testnet:**

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/remitroot_escrow.wasm \
  --source your-deployer-key \
  --network testnet
```

Copy the output contract ID into your `.env` as `ESCROW_CONTRACT_ID`.

**3. Start all services in development mode:**

```bash
# From the repo root — Turborepo runs everything in parallel
pnpm dev
```

This starts:
- `app-sender` on [http://localhost:3000](http://localhost:3000)
- `app-farmer` on [http://localhost:3001](http://localhost:3001)
- `app-vendor` on [http://localhost:3002](http://localhost:3002)
- `backend-api` on [http://localhost:4000](http://localhost:4000)
- `oracle` on [http://localhost:4001](http://localhost:4001)

**4. Run individual packages:**

```bash
pnpm --filter app-sender dev
pnpm --filter backend-api dev
pnpm --filter oracle dev
```

---

## Smart Contract Reference

### Escrow lifecycle

```
fund() ──▶ Funded ──▶ approve_farmer() ──▶ VoucherMinted
                                                 │
                                          redeem_voucher()
                                                 │
                                            Redeemed
                                                 │
                                          trigger_repay()  ◀── oracle
                                                 │
                                            Repaying
                                           ╱         ╲
                                     repay()       default()
                                        │               │
                                     Repaid         Defaulted
```

### Cancel / timeout

If no farmer is approved within `APPROVAL_TIMEOUT_LEDGERS` (≈ 7 days on testnet), anyone can call `cancel()` to refund the sender.

### Error codes

| Code | Name | Meaning |
|---|---|---|
| `1` | `AlreadyFunded` | `fund()` called on an existing escrow |
| `2` | `NotFunded` | Action requires `Funded` state |
| `3` | `Unauthorized` | Caller is not the expected party |
| `4` | `VoucherAlreadyMinted` | Cannot mint twice |
| `5` | `NotRedeemed` | Cannot trigger repay before redemption |
| `6` | `RepaymentComplete` | Surplus repayment rejected |
| `7` | `NotExpired` | Cannot cancel before timeout |

---

## Money Flow

```
1.  Sender deposits $200 USDC via Stellar Anchor (e.g. MoneyGram Ramps)
2.  Path payment converts USD → USDC on Stellar DEX (≈ 5 seconds, < $0.01 fee)
3.  Soroban escrow contract locks $200 USDC
4.  Contract mints 200 RVCH voucher tokens to farmer's Stellar account
5.  Farmer visits vendor, vendor scans QR, burns 200 RVCH
6.  Contract instantly releases $200 USDC to vendor's account
7.  Vendor gives farmer seeds/fertilizer worth $200
8.  [Post-harvest] Farmer sends $220 via M-Pesa (principal + 10% fee)
9.  Oracle detects M-Pesa payment, calls trigger_repay() on contract
10. Contract streams $220 USDC back to sender's Stellar account
```

**Fee structure:**

| Party | Fee |
|---|---|
| Stellar network fee | ~0.00001 XLM per tx (< $0.001) |
| Anchor on-ramp | 0.5–1% (Anchor-dependent) |
| RemitRoot protocol fee | 1% of amount (held in treasury account) |
| Farmer repayment fee | Configurable per escrow (default 10%) |

---

## Stellar Primitives Used

| Primitive | Purpose in RemitRoot |
|---|---|
| **Stellar Anchors** | Cash in/out for both sender (USD → USDC) and farmer (USDC → mobile money) |
| **Path Payments** | Automatic FX routing — sender pays in any currency, farmer receives local stablecoin |
| **Soroban Smart Contracts** | Escrow logic, voucher minting, repayment rules |
| **Stellar Custom Assets** | RVCH voucher token — transferable only to registered vendors |
| **SEP-0010** | Web authentication for sender app (wallet sign-in) |
| **SEP-0012** | KYC data collection for farmers and vendors |
| **SEP-0024** | Interactive deposit/withdrawal flow via Anchors |
| **Claimable Balances** | Unclaimed farmer vouchers held safely on-chain |

---

## Testing

```bash
# Run all tests across the monorepo
pnpm test

# Run only contract tests
cd packages/contracts/escrow
cargo test

# Run only backend tests
pnpm --filter backend-api test

# Run full integration test (requires local Stellar testnet)
pnpm test:integration
```

**Contract test coverage targets:**

- `fund()` — valid deposit, duplicate escrow rejection
- `approve_farmer()` — auth check, state transition
- `redeem_voucher()` — wrong vendor rejection, double-redeem protection
- `repay()` — partial repayments, overpayment rejection
- `cancel()` — before/after timeout, refund amount correctness
- `default()` — oracle-triggered, funds returned to sender

---

## Deployment

### Testnet

```bash
# Deploy contract
pnpm deploy:testnet

# Verify contract on Stellar Explorer
stellar contract info --id $ESCROW_CONTRACT_ID --network testnet
```

### Mainnet

```bash
# Requires a funded mainnet deployer account
pnpm deploy:mainnet
```

**GitHub Actions CI/CD** (`.github/workflows/`):

| Workflow | Trigger | Actions |
|---|---|---|
| `ci.yml` | Pull request | Lint, type-check, test all packages |
| `deploy-testnet.yml` | Push to `develop` | Deploy contracts + apps to testnet |
| `deploy-mainnet.yml` | Push to `main` | Deploy to mainnet (manual approval) |

---

## Contributing

This project was built for the **Stellar Wave Hackathon on Drips**. Contributions, issues, and pull requests are welcome.

**Good first issues** (labelled `good first issue` on GitHub):

- Add USSD language localisation (Swahili, Amharic, Hausa)
- Integrate a second mobile money provider (Airtel Money)
- Add farmer credit scoring based on on-chain repayment history
- Build a vendor onboarding flow with document upload
- Write additional Soroban contract edge-case tests
- Add push notifications for escrow state changes

**Contribution steps:**

```bash
# Fork the repo, then:
git checkout -b feature/your-feature-name
pnpm install
# Make your changes
pnpm lint && pnpm test
git commit -m "feat: describe your change"
git push origin feature/your-feature-name
# Open a pull request
```

Please follow the [Conventional Commits](https://www.conventionalcommits.org/) spec for commit messages.

---

## Roadmap

**v0.1 — Hackathon MVP**
- [x] Soroban escrow contract (full lifecycle)
- [x] Sender web app (fund + track)
- [x] Farmer PWA (view voucher QR)
- [x] Vendor QR scanner (redeem)
- [x] Oracle (M-Pesa webhook → trigger_repay)

**v0.2 — Post-Hackathon**
- [ ] USSD interface for feature phones (Africa's Talking)
- [ ] Second mobile money integration (MTN MoMo)
- [ ] Farmer credit score NFT (on-chain repayment history)
- [ ] Multi-currency support (local stablecoins via Anchors)
- [ ] DAO governance for vendor whitelisting

**v1.0 — Production**
- [ ] Full KYC/AML compliance (SEP-0012 + local regulation)
- [ ] Pilot with 3 agro-input vendors in Nigeria / Kenya
- [ ] Integration with agricultural NGO disbursement programs
- [ ] Mobile app (React Native) for farmer and vendor

---

## License

MIT © 2026 RemitRoot Contributors

---

<p align="center">
  Built with 💧 on <a href="https://stellar.org">Stellar</a> · Submitted to <a href="https://drips.network/wave">Drips Wave Hackathon</a>
</p>
