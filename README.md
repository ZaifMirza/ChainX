# 🔗 ChainX - Terminal Blockchain Explorer

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue" alt="Version">
  <img src="https://img.shields.io/badge/Rust-2024-edition-yellow" alt="Rust Edition">
  <img src="https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-green" alt="Platform">
</p>

ChainX is a beautiful Terminal User Interface (TUI) blockchain explorer for Ethereum. Query transactions, blocks, addresses, and smart contracts directly from your terminal with a modern, interactive interface.

---

## ✨ Features

- **🖥️ Terminal UI** - Modern TUI built with ratatui for an immersive experience
- **💰 Live ETH Price** - Real-time ETH/USD price updates every 10 seconds
- **📦 Block Details** - View block information, transactions, and rewards
- **💳 Address Queries** - Check wallet balances and token holdings
- **📄 Transaction Details** - Full transaction information with fees
- **📜 Smart Contract Info** - Contract metadata, creation details, and transactions
- **🛡️ Security** - Scam/spoofed token filtering
- **⌨️ Keyboard Navigation** - Vim-style shortcuts for power users

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- Internet connection
- [Etherscan API Key](https://etherscan.io/apis) (free)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/chainx.git
cd chainx

# Create .env file
echo "ETHERSCAN_API_KEY=your_api_key_here" > .env

# Build the project
cargo build --release

# Run the explorer
cargo run --release
```

Or simply run with:
```bash
cargo run
```

---

## 📖 Usage

### Interface Layout

```
┌─────────────────────────────────────────────────────────┐
│ ETH: $3456.78          ChainX               ● Ethereum  │
├─────────────────────────────────────────────────────────┤
│                                                         │
│                    [Main Content Area]                  │
│                                                         │
├─────────────────────────────────────────────────────────┤
│ [Input Box]                                             │
├─────────────────────────────────────────────────────────┤
│ i: Input | h: Home | q: Quit | ↑↓: Scroll              │
└─────────────────────────────────────────────────────────┘
```

**Header:**
- **Left:** Live ETH/USD price (updates every 10 seconds)
- **Center:** ChainX logo
- **Right:** Chain indicator (● Ethereum)

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `i` or `/` | Enter input mode |
| `Enter` | Submit query |
| `Esc` | Cancel input / Exit input mode |
| `h` | Go to home screen |
| `q` | Quit application |
| `↑` / `↓` | Scroll content up/down |
| `j` / `k` | Scroll content up/down (vim-style) |
| `PgUp` / `PgDn` | Fast scroll |
| `Home` | Jump to top |
| `End` | Jump to bottom |

### Input Types

ChainX automatically detects what you're querying based on the input format:

| Input Format | Example | Query Type |
|-------------|---------|------------|
| `0x...` (42 chars) | `0x742d35Cc6634C0532925a3b844Bc9e7595f...` | Address or Contract |
| `0x...` (66 chars) | `0x1234567890abcdef1234567890abcdef12345678...` | Transaction Hash |
| Number | `18547293` or `0x11a3b2e` | Block Number |

---

## 📊 Query Examples

### Transaction Query

When you enter a **transaction hash** (0x...):

```
┌─────────────────────────────────────────────────────────┐
│              Transaction Details                        │
├─────────────────────────────────────────────────────────┤
│ Hash:            0xabc123...def456                      │
│ Status:          ✅ Success                             │
│ Block:           18547293                               │
│ Timestamp:       2024-01-15 14:32:15 UTC                │
│ From:            0x742d35Cc6634C0532925a3b...           │
│ To:              0xdAC17F958D2ee523a2206206994597C13D...│
│ Value:           1.50000000 ETH                         │
│                                                         │
│ Gas Information                                         │
│ Gas Used:        21000                                  │
│ Gas Price:       25.000000 Gwei                         │
│ Transaction Fee: 0.00250000 ETH                         │
│ Nonce:           45                                     │
└─────────────────────────────────────────────────────────┘
```

**Fields Displayed:**
- Transaction Hash
- Status (Success/Failure)
- Block Number & Confirmations
- Timestamp
- From/To Addresses
- Value Transferred
- Gas Used, Gas Price, Transaction Fee
- Nonce
- Input Data (if present)

---

### Block Query

When you enter a **block number**:

```
┌─────────────────────────────────────────────────────────┐
│                  Block Details                          │
├─────────────────────────────────────────────────────────┤
│ Block Number:    18547293                               │
│ Status:          Confirmed                              │
│ Timestamp:       2024-01-15 14:32:15 UTC                │
│ Age:             2h 15m ago                             │
│                                                         │
│ Block Statistics                                        │
│ Transactions:    142                                    │
│ Withdrawals:     32                                     │
│                                                         │
│ Gas Information                                         │
│ Gas Used:        14,521,876                             │
│ Gas %:           87.5%                                  │
│ Gas Limit:       30,000,000                             │
│ Block Reward:    Variable (EIP-1559)                    │
│                                                         │
│ Block Hashes                                            │
│ Block Hash:      0xdef789...abc123                      │
│ Parent Hash:     0x456abc...789def                      │
│ State Root:      0xabc123...def789                      │
│ Miner:           0x952B9003CE6CA4B5C...                 │
└─────────────────────────────────────────────────────────┘
```

**Fields Displayed:**
- Block Height/Number
- Confirmation Status
- Timestamp & Age
- Transaction & Withdrawal Count
- Gas Usage & Limit
- Block Reward
- Block, Parent, and State Hashes
- Miner Address
- Extra Data

---

### Address Query (EOA/Wallet)

When you enter an **Ethereum address** (0x...):

```
┌─────────────────────────────────────────────────────────┐
│                   Address Details                       │
├─────────────────────────────────────────────────────────┤
│ Address:         0x742d35Cc6634C0532925a3b...           │
│ Type:            EOA (Wallet)                           │
│                                                         │
│ Balance                                                 │
│ ETH Balance:     2.54320000 ETH                         │
│ USD Value:       $4,832.00 USD                          │
│ Transaction Count: 45                                   │
│                                                         │
│ Token Balances                                          │
│   • 1000.0000 USDT                                      │
│   • 50.5000 LINK                                        │
│   • 10000.0000 SHIB                                     │
└─────────────────────────────────────────────────────────┘
```

**Fields Displayed:**
- Address
- Type (EOA or Smart Contract)
- ETH Balance
- USD Value (calculated from live ETH price)
- Transaction Count
- Token Balances (scam tokens filtered)

---

### Contract Query

When you enter a **smart contract address** (0x...):

```
┌─────────────────────────────────────────────────────────┐
│                  Contract Details                       │
├─────────────────────────────────────────────────────────┤
│ Contract Address: 0xdAC17F958D2ee523a2206206994597C13D...│
│                                                         │
│ Balance                                                 │
│ ETH Balance:      5.23400000 ETH                        │
│ USD Value:        $9,944.60 USD                         │
│                                                         │
│ Contract Information                                    │
│ Name:             Tether USD (USDT)                     │
│ Symbol:           USDT                                  │
│ Creator:          0x5E4c3a55eE1a4C22B4d53C0903B3A5C2... │
│ Creation Tx:      0x789abc...def123                     │
│ Compiler:         v0.8.19+commit.7dd6d404               │
│ Type:             Proxy Contract                        │
│ Implementation:   0x1234...5678                         │
│ Total Transactions: 125043                              │
│                                                         │
│ Recent Transactions                                     │
│   #1 ──────────────────────────────────────────         │
│     Hash:    0x456def...abc123                          │
│     From:    0xabcd...1234                              │
│     To:      0xefgh...5678                              │
│     Value:   0.050000 ETH                               │
│     Status:  ✅ Success                                 │
└─────────────────────────────────────────────────────────┘
```

**Fields Displayed:**
- Contract Address
- ETH Balance & USD Value
- Contract Name & Symbol
- Creator Address
- Creation Transaction
- Compiler Version
- Contract Type (Proxy/Standard)
- Implementation Address (for proxies)
- Total Transaction Count
- Recent Transactions (last 5)

---

## 🌐 Supported Networks

| Network | Chain ID | Symbol | RPC URL |
|---------|----------|--------|---------|
| Ethereum | 1 | ETH | publicnode.com |
| Polygon | 137 | MATIC | polygon-rpc.com |
| BSC | 56 | BNB | bsc-dataseed1.binance.org |
| Avalanche | 43114 | AVAX | api.avax.network |
| Arbitrum | 42161 | ETH | arb1.arbitrum.io |
| Optimism | 10 | ETH | mainnet.optimism.io |
| Base | 8453 | ETH | mainnet.base.org |
| Celo | 42220 | CELO | forno.celo.org |
| Fantom | 250 | FTM | rpc.fantom.network |
| Goerli (Testnet) | 5 | ETH | publicnode.com |
| Sepolia (Testnet) | 11155111 | ETH | publicnode.com |

**Note:** Currently optimized for Ethereum mainnet. Multi-chain support coming soon.

---

## ⚙️ Configuration

### Environment Variables

Create a `.env` file in the project root:

```env
ETHERSCAN_API_KEY=your_api_key_here
```

### Getting an Etherscan API Key

1. Go to [Etherscan.io](https://etherscan.io)
2. Create an account
3. Navigate to API Keys
4. Generate a new API key

**Note:** The free tier includes 5 calls/second, which is sufficient for personal use.

---

## 🏗️ Architecture

```
src/
├── main.rs              # Entry point & TUI initialization
├── app/                 # Application logic
│   ├── config.rs        # Configuration
│   └── input.rs         # Input parsing
├── api/                 # External APIs
│   ├── rpc/             # Ethereum RPC client
│   └── etherscan/       # Etherscan API client
├── commands/            # Command handlers
│   ├── address.rs       # Address queries
│   ├── block.rs         # Block queries
│   ├── contract.rs      # Contract queries
│   └── transaction.rs   # Transaction queries
├── tui/                 # Terminal UI
│   ├── app.rs           # TUI app state
│   ├── ui.rs            # UI rendering
│   ├── events.rs        # Keyboard events
│   └── widgets/         # Display widgets
├── config/              # Chain configuration
├── models/              # Data models
├── utils/               # Utility functions
├── validation/          # Input validation
└── cache/               # Price caching
```

---

## 📦 Dependencies

- **ratatui** - Terminal UI framework
- **crossterm** - Cross-platform terminal manipulation
- **tokio** - Async runtime
- **reqwest** - HTTP client
- **serde** - Serialization
- **chrono** - Date/time handling
- **dotenv** - Environment variables

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

## 📝 License

MIT License - feel free to use this project for any purpose.

---

## 🙏 Acknowledgments

- [Ethereum Foundation](https://ethereum.org)
- [Etherscan](https://etherscan.io)
- [Public RPC Nodes](https://www.publicnode.com)
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library

---

<p align="center">
  Made with ❤️ for the Ethereum Community
</p>
