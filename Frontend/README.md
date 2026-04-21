# Frontend 📱

> **User interfaces for RemitRoot ecosystem** — web apps, PWAs, and mobile interfaces for senders, farmers, and vendors.

---

## Table of Contents

- [Overview](#overview)
- [Applications](#applications)
- [Tech Stack](#tech-stack)
- [Setup](#setup)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Environment Variables](#environment-variables)
  - [Running Locally](#running-locally)
- [Application Details](#application-details)
- [Stellar Integration](#stellar-integration)
- [Testing](#testing)
- [Deployment](#deployment)
- [Mobile Optimization](#mobile-optimization)

---

## Overview

The frontend consists of three specialized applications, each tailored for different user groups in the RemitRoot ecosystem:

- **app-sender**: Web application for diaspora senders
- **app-farmer**: Progressive Web App for farmers (mobile-first)
- **app-vendor**: QR scanner interface for agro-input vendors

All applications share common components and utilities while maintaining distinct user experiences optimized for their target audiences.

---

## Applications

### 📦 `app-sender`

Web application used by diaspora senders (built with Next.js + Freighter wallet integration).

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

## Tech Stack

| Layer | Technology |
|---|---|
| Framework | Next.js 14 (App Router) |
| Language | TypeScript |
| Styling | Tailwind CSS |
| State Management | Zustand + React Query |
| Blockchain | Stellar SDK (Freighter, Horizon, Soroban) |
| Authentication | SEP-0010 (Stellar Web Auth) |
| PWA | Service Workers + Web App Manifest |
| QR Code | react-qr-code + @zxing/library |
| Testing | Jest + React Testing Library |
| Build | Vite (for vendor app) / Next.js (for others) |

---

## Setup

### Prerequisites

- **Node.js** v20+
- **npm** v10+
- **Freighter** browser wallet (for sender app testing)
- A **Stellar Testnet** account funded via [Friendbot](https://friendbot.stellar.org)

### Installation

```bash
# Navigate to frontend directory
cd Frontend

# Install all workspace dependencies
npm install

# Install dependencies for each app (optional)
npm install --workspace app-sender
npm install --workspace app-farmer
npm install --workspace app-vendor
```

### Environment Variables

Copy and configure environment files for each application:

```bash
# Sender app
cp apps/app-sender/.env.example apps/app-sender/.env.local

# Farmer app
cp apps/app-farmer/.env.example apps/app-farmer/.env.local

# Vendor app
cp apps/app-vendor/.env.example apps/app-vendor/.env.local
```

**Common variables:**

```env
# Stellar network
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org
STELLAR_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

# Contract IDs
ESCROW_CONTRACT_ID=your-contract-id-here

# Assets
USDC_ISSUER=GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5
USDC_CODE=USDC
RVCH_ISSUER=your-issuer-account
RVCH_CODE=RVCH

# Backend API
NEXT_PUBLIC_API_URL=http://localhost:4000

# Anchor (for deposits)
ANCHOR_HOME_DOMAIN=testanchor.stellar.org
```

### Running Locally

```bash
# Start sender + farmer apps in development mode
npm run dev

# Start individual apps
npm run dev --workspace app-sender      # http://localhost:3000
npm run dev --workspace app-farmer      # http://localhost:3001
npm run dev --workspace app-vendor      # http://localhost:3002

# Build for production
npm run build

# Start production servers
npm run start
```

---

## Application Details

### Sender App Features

**Dashboard:**
- Active escrows overview
- Total impact metrics
- Quick funding actions

**Funding Flow:**
- Farmer profile browsing with filters
- Vendor selection by location
- Crop season and amount configuration
- Real-time escrow tracking

**Wallet Integration:**
- Freighter wallet connection
- SEP-0010 authentication
- USDC balance display
- Transaction history

### Farmer App Features

**Mobile-First Design:**
- Optimized for 2G/3G networks
- Offline voucher display
- SMS notifications integration

**Voucher Management:**
- QR code generation and sharing
- Voucher status tracking
- Redemption history

**Repayment Interface:**
- Mobile money provider selection
- Amount calculation with fees
- Payment confirmation

### Vendor App Features

**QR Scanning:**
- Camera-based QR reader
- Voucher validation before redemption
- Batch processing support

**Transaction Management:**
- Real-time USDC receipt confirmation
- Digital receipt generation
- Sales analytics dashboard

**Offline Support:**
- Cached vendor information
- Queue processing for poor connectivity

---

## Stellar Integration

### Authentication (SEP-0010)

```typescript
// Connect wallet and authenticate
const { connect } = useStellar();
await connect();
```

### Contract Interactions

```typescript
// Fund an escrow
const { fund } = useEscrow();
const txId = await fund({
  vendorId: 'vendor-123',
  cropSeason: 'maize-2024',
  amount: '200'
});
```

### Asset Management

```typescript
// Check USDC balance
const balance = await getAssetBalance('USDC', usdcIssuer);

// Track voucher tokens
const vouchers = await getAccountAssets('RVCH', rvchIssuer);
```

---

## Testing

```bash
# Run all frontend tests
npm run test --workspaces --if-present

# Run tests for specific app
npm run test --workspace app-sender
npm run test --workspace app-farmer
npm run test --workspace app-vendor

# Run with coverage
npm run test:coverage --workspaces --if-present

# Run E2E tests
npm run test:e2e --workspaces --if-present
```

**Test categories:**
- Component unit tests
- Integration tests with mock APIs
- E2E tests for critical user flows
- Wallet integration tests

---

## Deployment

### Vercel (Recommended)

```bash
# Deploy sender app
vercel --prod --scope app-sender

# Deploy farmer app
vercel --prod --scope app-farmer

# Deploy vendor app
vercel --prod --scope app-vendor
```

### Docker

```bash
# Build Docker images
docker build -t remitroot-sender -f apps/app-sender/Dockerfile .
docker build -t remitroot-farmer -f apps/app-farmer/Dockerfile .
docker build -t remitroot-vendor -f apps/app-vendor/Dockerfile .

# Run with Docker Compose
docker-compose up -d
```

### Environment-Specific Configs

**Development:**
- Testnet Stellar network
- Local backend API
- Hot reload enabled

**Staging:**
- Testnet Stellar network
- Staging backend API
- Analytics and monitoring

**Production:**
- Mainnet Stellar network
- Production backend API
- Full monitoring and alerting

---

## Mobile Optimization

### PWA Features

**Service Worker:**
- Offline voucher display
- Cached transaction history
- Background sync for payments

**App Manifest:**
- Installable on mobile devices
- Custom splash screens
- Appropriate icons and themes

### Performance Optimizations

**Bundle Size:**
- Code splitting by route
- Lazy loading components
- Image optimization

**Network Considerations:**
- Progressive loading
- Fallback content for slow connections
- Efficient API caching

### Accessibility

**Screen Reader Support:**
- Semantic HTML structure
- ARIA labels and descriptions
- Keyboard navigation

**Visual Accessibility:**
- High contrast mode support
- Scalable fonts
- Color-blind friendly design

---

## Contributing

1. Follow the existing component structure
2. Use TypeScript strictly
3. Test on mobile devices
4. Consider offline functionality
5. Follow accessibility best practices
6. Update documentation for UI changes

---

## License

MIT © 2026 RemitRoot Contributors
