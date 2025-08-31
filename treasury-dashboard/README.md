# Solana Memes Treasury Dashboard MVP

A real-time treasury monitoring dashboard for the Solana Memes platform, providing transparency and trust-building features for users.

## 🏦 Features

### Core Data Display
- **Treasury Balance**: SOL and SPL token holdings
- **Buyback Activity**: SOL spent, tokens bought back, supply reduction
- **Token Metrics**: Total vs circulating supply, holder count
- **Transaction Log**: Last 10 treasury transactions with Solscan links

### Real-time Updates
- Live treasury balance monitoring
- Transaction feed with auto-refresh
- WebSocket subscriptions for instant updates
- 30-second auto-refresh intervals

### Data Sources
- **Solana RPC**: Direct blockchain queries
- **Helius API**: Enhanced token holder data (optional)
- **Indexer Integration**: Buyback event tracking
- **Transaction Monitoring**: Real-time log subscriptions

## 🚀 Quick Start

### Prerequisites
- Node.js 18+ 
- npm or yarn
- Solana CLI (optional)

### Installation

1. **Clone and navigate to the dashboard**
```bash
cd treasury-dashboard
```

2. **Install dependencies**
```bash
npm install
```

3. **Configure addresses**
Edit `src/pages/Dashboard.jsx` and update:
```javascript
const TREASURY_ADDRESS = 'YOUR_TREASURY_ADDRESS_HERE';
const TOKEN_MINT = 'YOUR_TOKEN_MINT_HERE';
```

4. **Optional: Add Helius API key**
Edit `src/lib/solana.js` and update:
```javascript
const HELIUS_RPC = 'https://rpc.helius.xyz/?api-key=YOUR_API_KEY';
```

5. **Start development server**
```bash
npm run dev
```

6. **Open in browser**
Navigate to `http://localhost:3001`

## 📊 Architecture

### Frontend (React + Vite)
```
treasury-dashboard/
├── src/
│   ├── components/
│   │   ├── TreasuryCard.jsx      # SOL/SPL balance display
│   │   ├── BuybackTable.jsx      # Buyback history table
│   │   ├── TokenStats.jsx        # Supply/holder metrics
│   │   └── TxFeed.jsx           # Live transaction feed
│   ├── lib/
│   │   ├── solana.js            # RPC helpers
│   │   └── format.js            # Data formatting utils
│   └── pages/
│       └── Dashboard.jsx        # Main dashboard page
```

### Data Flow
1. **Treasury Balance**: `getBalance()` + `getTokenAccountsByOwner()`
2. **Buyback Events**: Indexed from program logs
3. **Token Supply**: `getTokenSupply()` + burned token tracking
4. **Holders Count**: Helius API or RPC fallback
5. **Transactions**: `getSignaturesForAddress()` + `getTransaction()`

## 🔧 Configuration

### Environment Variables
Create `.env` file:
```env
VITE_TREASURY_ADDRESS=your_treasury_address
VITE_TOKEN_MINT=your_token_mint
VITE_HELIUS_API_KEY=your_helius_key
VITE_RPC_ENDPOINT=https://api.mainnet-beta.solana.com
```

### Treasury Address Setup
1. Deploy treasury wallet
2. Update address in configuration
3. Verify ownership and permissions
4. Test with small amounts first

### Token Mint Configuration
1. Deploy SPL token
2. Update mint address
3. Configure supply tracking
4. Set up burn tracking

## 📈 Data Sources

### Solana RPC Endpoints
- **Mainnet**: `https://api.mainnet-beta.solana.com`
- **Devnet**: `https://api.devnet.solana.com`
- **Custom**: Your own RPC endpoint

### Helius API (Optional)
- Enhanced token holder data
- Better performance for large token lists
- Webhook support for real-time updates

### Indexer Integration
- Buyback event indexing
- Historical data storage
- Analytics and reporting

## 🔄 Real-time Features

### WebSocket Subscriptions
```javascript
// Treasury balance changes
solanaService.subscribeToTreasury(address, callback)

// Transaction logs
solanaService.subscribeToLogs(address, callback)
```

### Auto-refresh
- 30-second intervals for data updates
- Manual refresh button
- Loading states and error handling

### Transaction Monitoring
- Real-time transaction feed
- Success/failure status
- Fee tracking
- Solscan integration

## 🛠️ Development

### Available Scripts
```bash
npm run dev          # Start development server
npm run build        # Build for production
npm run preview      # Preview production build
npm run lint         # Run ESLint
```

### Adding New Features
1. Create component in `src/components/`
2. Add data fetching in `src/lib/solana.js`
3. Integrate into `src/pages/Dashboard.jsx`
4. Update styling in `src/index.css`

### Testing
```bash
# Unit tests (when implemented)
npm run test

# Integration tests
npm run test:integration
```

## 🚀 Production Deployment

### Build
```bash
npm run build
```

### Deploy Options
1. **Vercel**: Connect GitHub repo
2. **Netlify**: Drag and drop `dist/` folder
3. **AWS S3**: Upload static files
4. **Custom Server**: Serve with nginx/Apache

### Environment Setup
1. Set production environment variables
2. Configure custom domain
3. Enable HTTPS
4. Set up monitoring

## 📊 Analytics & Monitoring

### Performance Metrics
- Page load times
- API response times
- Error rates
- User engagement

### Treasury Metrics
- Balance changes over time
- Buyback frequency
- Token holder growth
- Transaction volume

### Alerts
- Large balance changes
- Failed transactions
- API downtime
- Unusual activity

## 🔒 Security Considerations

### Data Validation
- Verify treasury address ownership
- Validate transaction signatures
- Check token mint authenticity
- Sanitize user inputs

### Access Control
- Read-only dashboard
- No private key storage
- Secure API key handling
- Rate limiting

### Privacy
- No user data collection
- Anonymous analytics
- GDPR compliance
- Data retention policies

## 🎯 Roadmap

### Phase 1 (MVP) ✅
- [x] Treasury balance display
- [x] Buyback activity tracking
- [x] Token supply metrics
- [x] Transaction feed
- [x] Real-time updates

### Phase 2 (Enhanced)
- [ ] DAO voting dashboard
- [ ] Buyback automation settings
- [ ] Yield farming integration
- [ ] Historical charts
- [ ] Mobile optimization

### Phase 3 (Advanced)
- [ ] Cross-chain treasury
- [ ] Advanced analytics
- [ ] API for third parties
- [ ] Multi-wallet support
- [ ] Governance integration

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Add tests
5. Submit pull request

## 📄 License

MIT License - see LICENSE file for details

## 🆘 Support

- **Documentation**: [Wiki](link-to-wiki)
- **Issues**: [GitHub Issues](link-to-issues)
- **Discord**: [Community Server](link-to-discord)
- **Email**: support@solana-memes.com

---

**Built with ❤️ for the Solana Memes community**
