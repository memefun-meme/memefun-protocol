# 🚀 Quick Start Guide - Solana Memes

Get up and running with the Solana Memes project in minutes!

## 📋 Prerequisites

Before you begin, make sure you have the following installed:

- **Node.js** 18+ ([Download](https://nodejs.org/))
- **Rust** 1.70+ ([Install](https://rustup.rs/))
- **Solana CLI** 1.16+ ([Install](https://docs.solana.com/cli/install-solana-cli-tools))
- **Anchor Framework** 0.28+ ([Install](https://www.anchor-lang.com/docs/installation))
- **PostgreSQL** 14+ ([Install](https://www.postgresql.org/download/))

## ⚡ Quick Setup

### 1. Clone and Navigate

```bash
git clone <repository-url>
cd solana-memes
```

### 2. Run Setup Script

```bash
./scripts/setup.sh
```

This script will:
- ✅ Check all prerequisites
- ✅ Install dependencies
- ✅ Set up environment files
- ✅ Build the project
- ✅ Run initial tests

### 3. Configure Environment

```bash
cp env.example .env
# Edit .env with your configuration
```

### 4. Start Development Server

```bash
npm run dev
```

Visit `http://localhost:3000` to see the application!

## 🎯 First Steps

### Create Your First Memecoin

1. **Connect Wallet**: Click "Connect Wallet" in the top right
2. **Navigate to Creator**: Go to "Create Token" page
3. **Fill Details**:
   - Name: "MyFirstMeme"
   - Symbol: "MFM"
   - Supply: 1000000000
   - Description: "My first memecoin!"
4. **Deploy**: Click "Create Token"

### Explore Tokens

1. **Browse**: Visit "Token Explorer" to see all tokens
2. **Trade**: Click on any token to view details and trade
3. **Stake**: Go to "Staking" to earn rewards

## 🔧 Development Workflow

### Running Tests

```bash
# All tests
npm test

# Specific test file
npm test -- tests/token.test.ts

# With coverage
npm run test:coverage
```

### Building for Production

```bash
# Build everything
npm run build

# Deploy to localnet
npm run deploy:local

# Deploy to devnet
npm run deploy:devnet
```

### Database Operations

```bash
# Setup database
npm run setup-db

# Run migrations
npm run migrate

# Seed data
npm run seed
```

## 🐳 Docker Development

### Start All Services

```bash
docker-compose up -d
```

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f api
```

### Stop Services

```bash
docker-compose down
```

## 📊 Monitoring

Access monitoring dashboards:

- **Grafana**: http://localhost:3002 (admin/admin)
- **Prometheus**: http://localhost:9090

## 🚨 Troubleshooting

### Common Issues

**Node.js version too old**
```bash
# Install Node.js 18+
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs
```

**Solana CLI not found**
```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"
```

**Anchor not found**
```bash
# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

**PostgreSQL connection failed**
```bash
# Start PostgreSQL service
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

### Reset Everything

```bash
# Clean all
rm -rf node_modules
rm -rf target
rm -rf dist
rm .env

# Reinstall
npm install
cargo build
cp env.example .env
```

## 📚 Next Steps

- 📖 Read the [Full Documentation](docs/)
- 🎨 Customize the [UI Components](src/client/components/)
- 🔧 Modify [Smart Contracts](src/programs/)
- 📊 Set up [Analytics](docs/ANALYTICS.md)
- 🚀 Deploy to [Production](docs/DEPLOYMENT.md)

## 🆘 Need Help?

- 📖 [Documentation](docs/)
- 💬 [Discord Community](https://discord.gg/solana-memes)
- 🐛 [Report Issues](https://github.com/your-username/solana-memes/issues)
- 📧 [Email Support](mailto:support@solana-memes.com)

---

**Happy coding! 🚀**
