# Platform Fee Structure & Token Distribution

## 🎯 **Platform Fee Sources**

The Solana Memes platform generates fees from multiple sources, and these fees are received in **different tokens** depending on the transaction type:

---

## 💰 **Fee Sources & Token Types**

### **1. Trading Fees (Primary Revenue)**
```rust
// Trading fee structure
pub struct TradingFees {
    pub fee_percentage: u8,          // 1.2% default (adjustable via governance)
    pub fee_token: FeeToken,         // Token used for trading
    pub fee_destination: FeeDestination, // Where fees are sent
}

pub enum FeeToken {
    SOL,    // SOL trading pairs
    USDC,   // USDC trading pairs
    SMEME,  // SMEME trading pairs
    Token,  // Individual meme token trading
}
```

**Fee Collection:**
- **SOL Trading**: Fees received in **SOL**
- **USDC Trading**: Fees received in **USDC**
- **SMEME Trading**: Fees received in **SMEME**
- **Token Trading**: Fees received in the **traded token**

### **2. Token Creation Fees**
```rust
// Token creation fee structure
pub struct CreationFees {
    pub creation_fee: u64,           // Fixed fee for token creation
    pub fee_token: FeeToken,         // Always USDC for creation
    pub vesting_fee: u64,           // Additional fee for vesting setup
}
```

**Fee Collection:**
- **Token Creation**: Fees received in **USDC**
- **Vesting Setup**: Additional fees in **USDC**

### **3. LBM Participation Fees**
```rust
// LBM fee structure
pub struct LBMFees {
    pub participation_fee: u64,      // Fee for LBM participation
    pub fee_token: FeeToken,         // USDC for LBM
    pub success_fee: u64,           // Success fee on token launch
}
```

**Fee Collection:**
- **LBM Participation**: Fees received in **USDC**
- **Success Fees**: Fees received in **launched token**

### **4. Staking Fees**
```rust
// Staking fee structure
pub struct StakingFees {
    pub staking_fee: u64,            // Fee for staking operations
    pub unstaking_fee: u64,          // Fee for unstaking
    pub fee_token: FeeToken,         // SOL for staking operations
}
```

**Fee Collection:**
- **Staking Operations**: Fees received in **SOL**

---

## 📊 **Fee Distribution by Token Type**

### **SOL Fees (From SOL Trading & Staking)**
```rust
pub struct SOLFeeDistribution {
    pub total_sol_fees: u64,         // Total SOL fees collected
    pub distribution: FeeDistribution,
}

pub struct FeeDistribution {
    pub smeme_stakers: u64,          // 55% to SMEME stakers
    pub treasury: u64,               // 25% to treasury
    pub creators: u64,               // 10% to creators
    pub buyback_fund: u64,           // 5% to buyback fund
    pub development: u64,            // 5% to development
}
```

### **USDC Fees (From Token Creation & LBM)**
```rust
pub struct USDCFeeDistribution {
    pub total_usdc_fees: u64,        // Total USDC fees collected
    pub distribution: FeeDistribution,
}
```

### **SMEME Fees (From SMEME Trading)**
```rust
pub struct SMEMEFeeDistribution {
    pub total_smeme_fees: u64,       // Total SMEME fees collected
    pub distribution: FeeDistribution,
}
```

### **Token Fees (From Individual Token Trading)**
```rust
pub struct TokenFeeDistribution {
    pub token_mint: Pubkey,          // Specific token mint
    pub total_token_fees: u64,       // Total fees in that token
    pub distribution: TokenSpecificDistribution,
}

pub struct TokenSpecificDistribution {
    pub smeme_stakers: u64,          // 55% to SMEME stakers (converted to SOL)
    pub treasury: u64,               // 25% to treasury (converted to USDC)
    pub creators: u64,               // 10% to creators (in original token)
    pub buyback_fund: u64,           // 5% to buyback fund (converted to USDC)
    pub development: u64,            // 5% to development (converted to SOL)
}
```

---

## 🔄 **Fee Conversion & Distribution**

### **Multi-Token Fee Handling**
```rust
// Fee conversion and distribution system
pub struct FeeConversionSystem {
    pub jupiter_integration: bool,   // Use Jupiter for token swaps
    pub conversion_thresholds: u64,  // Minimum amounts for conversion
    pub conversion_fees: u64,        // Fees for token conversion
}

// Fee distribution logic
pub fn distribute_fees(
    fee_token: FeeToken,
    fee_amount: u64,
) -> Result<()> {
    match fee_token {
        FeeToken::SOL => {
            // Distribute SOL fees directly
            distribute_sol_fees(fee_amount)?;
        },
        FeeToken::USDC => {
            // Distribute USDC fees directly
            distribute_usdc_fees(fee_amount)?;
        },
        FeeToken::SMEME => {
            // Distribute SMEME fees directly
            distribute_smeme_fees(fee_amount)?;
        },
        FeeToken::Token => {
            // Convert token fees to SOL/USDC for distribution
            convert_and_distribute_token_fees(fee_amount)?;
        },
    }
    Ok(())
}
```

### **Token Fee Conversion Process**
```rust
// Convert individual token fees to standard tokens
pub fn convert_and_distribute_token_fees(
    token_fees: u64,
    token_mint: Pubkey,
) -> Result<()> {
    // 1. Convert 55% to SOL for SMEME stakers
    let smeme_share = token_fees * 55 / 100;
    let sol_converted = convert_token_to_sol(smeme_share, token_mint)?;
    distribute_to_smeme_stakers(sol_converted)?;
    
    // 2. Convert 25% to USDC for treasury
    let treasury_share = token_fees * 25 / 100;
    let usdc_converted = convert_token_to_usdc(treasury_share, token_mint)?;
    add_to_treasury(usdc_converted)?;
    
    // 3. Keep 10% in original token for creators
    let creator_share = token_fees * 10 / 100;
    distribute_to_creators(creator_share, token_mint)?;
    
    // 4. Convert 5% to USDC for buyback fund
    let buyback_share = token_fees * 5 / 100;
    let buyback_usdc = convert_token_to_usdc(buyback_share, token_mint)?;
    add_to_buyback_fund(buyback_usdc)?;
    
    // 5. Convert 5% to SOL for development
    let dev_share = token_fees * 5 / 100;
    let dev_sol = convert_token_to_sol(dev_share, token_mint)?;
    add_to_development_fund(dev_sol)?;
    
    Ok(())
}
```

---

## 📈 **Fee Collection Examples**

### **Example 1: SOL Trading**
```
User trades 1000 SOL worth of PEPE tokens
Trading fee: 1.2% = 12 SOL
Fee distribution:
- 6.6 SOL to SMEME stakers
- 3 SOL to treasury
- 1.2 SOL to creators
- 0.6 SOL to buyback fund
- 0.6 SOL to development
```

### **Example 2: USDC Token Creation**
```
Creator pays 1000 USDC to create token
Creation fee: 1000 USDC
Fee distribution:
- 550 USDC to SMEME stakers
- 250 USDC to treasury
- 100 USDC to creators
- 50 USDC to buyback fund
- 50 USDC to development
```

### **Example 3: Individual Token Trading**
```
User trades 1000 DOGE tokens
Trading fee: 1.2% = 12 DOGE tokens
Fee conversion and distribution:
- 6.6 DOGE → Convert to SOL → Distribute to SMEME stakers
- 3 DOGE → Convert to USDC → Add to treasury
- 1.2 DOGE → Keep as DOGE → Distribute to creators
- 0.6 DOGE → Convert to USDC → Add to buyback fund
- 0.6 DOGE → Convert to SOL → Add to development
```

---

## 🏦 **Treasury Management**

### **Multi-Token Treasury**
```rust
pub struct Treasury {
    pub sol_balance: u64,            // SOL holdings
    pub usdc_balance: u64,           // USDC holdings
    pub smeme_balance: u64,          // SMEME holdings
    pub token_balances: HashMap<Pubkey, u64>, // Individual token holdings
    pub buyback_fund_usdc: u64,      // USDC for buybacks
    pub development_fund_sol: u64,   // SOL for development
}
```

### **Treasury Operations**
```rust
// Treasury management functions
pub fn add_to_treasury(token: FeeToken, amount: u64) -> Result<()> {
    match token {
        FeeToken::SOL => treasury.sol_balance += amount,
        FeeToken::USDC => treasury.usdc_balance += amount,
        FeeToken::SMEME => treasury.smeme_balance += amount,
        FeeToken::Token => {
            // Add to specific token balance
            treasury.token_balances.insert(token_mint, amount);
        },
    }
    Ok(())
}
```

---

## 🎯 **Key Points Summary**

### **Fee Token Types:**
- ✅ **SOL**: From SOL trading and staking operations
- ✅ **USDC**: From token creation and LBM participation
- ✅ **SMEME**: From SMEME token trading
- ✅ **Individual Tokens**: From specific token trading

### **Distribution Strategy:**
- ✅ **SMEME Stakers**: Receive fees in SOL (converted if needed)
- ✅ **Treasury**: Receives fees in USDC (converted if needed)
- ✅ **Creators**: Receive fees in original tokens
- ✅ **Buyback Fund**: Receives fees in USDC (converted if needed)
- ✅ **Development**: Receives fees in SOL (converted if needed)

### **Conversion Process:**
- ✅ **Jupiter Integration**: Automatic token conversion for distribution
- ✅ **Conversion Thresholds**: Minimum amounts before conversion
- ✅ **Conversion Fees**: Small fees for token swaps
- ✅ **Real-time Conversion**: Immediate conversion for fee distribution

**This multi-token fee structure ensures the platform can handle any token while maintaining consistent distribution to all stakeholders!** 🚀
