# E9TH - Utility Deflation Secure Token

E9TH is a Solana-based utility token designed to deliver immediate, tangible value to holders through discounts, early access, exclusive offerings, staking rewards, and curated airdrops.

## 🌐 Live Website
- **Website**: https://e9th.org
- **Dashboard**: https://e9th.org/app.html
- **Litepaper**: https://e9th.org/litepaper.html

## 🏗️ Project Structure

```
E9th/
├── program/                    # Solana program (Rust)
│   ├── src/                   # Program source code
│   ├── Cargo.toml            # Rust dependencies
│   └── build.sh              # Build script
├── static-site/              # Static website
│   ├── index.html            # Homepage
│   ├── app.html              # Dashboard
│   ├── litepaper.html        # Litepaper
│   ├── assets/               # Images and icons
│   └── deploy.sh             # Deployment script
└── E9th app/                 # React frontend (alternative)
    └── utility-deflation-secure/
```

## 🚀 Features

### Token Features
- **Deflationary Design**: Configurable burn rate on transfers
- **Utility-First**: Immediate value through discounts and access
- **Staking Rewards**: Earn rewards by staking E9TH tokens
- **Security-First**: Multisig ownership, pause/blacklist controls

### Website Features
- **Responsive Design**: Mobile-friendly interface
- **Wallet Integration**: Phantom wallet connectivity
- **Interactive Dashboard**: Token management and staking
- **Real-time Metrics**: Deflation tracking and statistics

## 🛠️ Development

### Prerequisites
- Node.js 18+
- Rust 1.70+
- Solana CLI
- Git

### Local Development

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/E9th.git
   cd E9th
   ```

2. **Run the static site locally**
   ```bash
   cd static-site
   python3 -m http.server 8000
   ```
   Visit: http://localhost:8000

3. **Build the Solana program**
   ```bash
   cd program
   cargo build-sbf
   ```

### Deployment

1. **Deploy to SiteGround** (current hosting)
   - Upload the `static-site/` folder contents to your web root
   - Ensure all assets are properly linked

2. **Deploy Solana program**
   ```bash
   cd program
   solana program deploy target/deploy/e9th_token_program.so
   ```

## 📊 Tokenomics

- **Ticker**: E9TH
- **Blockchain**: Solana (SPL / Token-2022 compatible)
- **Total Supply**: 10,000,000,000
- **Initial Price**: $0.00005
- **Model**: Deflationary

### Allocation
- Public Sale: 40% (4,000,000,000)
- Team & Founders: 20% (2,000,000,000)
- Marketing & Rewards: 15% (1,500,000,000)
- Liquidity: 15% (1,500,000,000)
- Treasury: 10% (1,000,000,000)

## 🔒 Security

- Multisig ownership for critical actions
- Emergency pause & blacklist protection
- On-chain logs for transparency
- Optional timelocks for major changes
- Independent audits planned

## 📱 Social Links

- **Twitter**: https://twitter.com/E9TH
- **Website**: https://e9th.org
- **Discord**: Coming soon

## 📄 License

This project is proprietary. All rights reserved.

## ⚠️ Disclaimer

E9TH is a utility token intended to access products and services. It is not an investment contract, security, or share. Participation involves risk. Do your own research and follow local laws.

---

**E9TH** - Where utility comes full circle.
