# 🔗 ChainX - Ethereum Blockchain Explorer

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue" alt="Version">
  <img src="https://img.shields.io/badge/Rust-2024-edition-yellow" alt="Rust Edition">
  <img src="https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-green" alt="Platform">
</p>

ChainX is a powerful command-line blockchain explorer for Ethereum and EVM-compatible networks. It allows you to查询 transactions, blocks, addresses, and smart contracts directly from your terminal.

---

## ✨ Features

- **🔍 Multi-Chain Support** - Query 11+ blockchain networks
- **📦 Block Details** - View block information, transactions, and rewards
- **💳 Address Queries** - Check wallet balances and token holdings
- **📄 Transaction Details** - Full transaction information with fees
- **📜 Smart Contract Info** - Contract metadata, creation details, and transactions
- **💰 ETH Price** - Real-time ETH price display
- **🛡️ Security** - Scam/spoofed token filtering
- **♾️ Interactive Loop** - Continuous querying without restarting

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

### Running ChainX

```bash
cargo run
```

### Input Types

ChainX automatically detects what you're querying based on the input format:

| Input Format | Example | Query Type |
|-------------|---------|------------|
| `0x...` (42 chars) | `0x742d35Cc6634C0532925a3b844Bc9e7595f...` | Address or Contract |
| `0x...` (66 chars) | `0x1234567890abcdef1234567890abcdef12345678...` | Transaction Hash |
| Number | `18547293` or `0x11a3b2e` | Block Number |

### Exit Commands

To exit the program, type:
- `quit`
- `exit`
- `q`
- `x`

---

## 📊 Output Examples

### Transaction Query

When you enter a **transaction hash** (0x...):

```
╔═══════════════════════════════════════════════════════════╗
║                 TRANSACTION DETAILS                       ║
╠═══════════════════════════════════════════════════════════╣
║  Transaction Hash:   0xabc123...def456                     ║
║  Status:            ✅ Success                             ║
║  Block:            18547293                               ║
║  Timestamp:        2024-01-15 14:32:15                   ║
║  From:             0x742d35Cc6634C0532925a...             ║
║  To:               0xdAC17F958D2ee523a2206206994597C13D... ║
║  Value:            1.50000000 ETH                         ║
║  Transaction Fee:  0.00250000 ETH                         ║
║  Gas Price:        25.000000 Gwei                         ║
╚═══════════════════════════════════════════════════════════╝
```

**Fields Displayed:**
- Transaction Hash
- Status (Success/Failure)
- Block Number
- Timestamp
- From Address
- To Address
- Value Transferred
- Transaction Fee
- Gas Price

---

### Block Query

When you enter a **block number**:

```
╔═══════════════════════════════════════════════════════════╗
║                    BLOCK DETAILS                          ║
╠═══════════════════════════════════════════════════════════╣
║  Block Height:      18547293                              ║
║  Status:            Confirmed                              ║
║  Timestamp:         2024-01-15 14:32:15                   ║
║  Transactions:      142                                    ║
║  Withdrawals:      32                                     ║
║  Block Reward:     0.0000 ETH                            ║
║  Gas Used:         14,521,876 (87.5%)                     ║
╠═══════════════════════════════════════════════════════════╣
║  Block Hash:       0xdef789...abc123                      ║
║  Parent Hash:      0x456abc...789def                      ║
║  Miner:           0x952B9003CE6CA4B5C...                  ║
║  Gas Limit:       30,000,000                              ║
║  Difficulty:      52,456,789,123,456                       ║
╚═══════════════════════════════════════════════════════════╝
```

**Fields Displayed:**
- Block Height/Number
- Status
- Timestamp
- Transaction Count
- Withdrawal Count
- Block Reward
- Gas Used (amount and percentage)
- Block Hash
- Parent Hash
- Miner Address
- Gas Limit
- Difficulty

---

### Address Query (EOA/Wallet)

When you enter an **Ethereum address** (0x...):

```
╔═══════════════════════════════════════════════════════════╗
║                  ADDRESS DETAILS                          ║
╠═══════════════════════════════════════════════════════════╣
║  Address:           0x742d35Cc6634C0532925a3b...          ║
║  Type:             EOA (Wallet)                           ║
║  Balance:          2.54320000 ETH                         ║
║  USD Value:        $4,832.00 USD                          ║
║  Nonce (TX Count): 45                                     ║
╠═══════════════════════════════════════════════════════════╣
║  Contract Code:    None (EOA)                            ║
╚═══════════════════════════════════════════════════════════╝
```

**Fields Displayed:**
- Address
- Type (EOA or Smart Contract)
- ETH Balance
- USD Value (calculated from ETH price)
- Nonce (Transaction Count)
- Contract Code Status

---

### Contract Query

When you enter a **smart contract address** (0x...):

```
╔════════════════════════════════════════════════════════════════════════════════════╗
║                              CONTRACT DETAILS                                      ║
╠════════════════════════════════════════════════════════════════════════════════════╣
║  Contract:          0xdAC17F958D2ee523a2206206994597C13D831ec7                     ║
║  ETH Balance:      5.23400000 ETH                                                 ║
║  USD Value:        $9,944.60 USD                                                  ║
╠════════════════════════════════════════════════════════════════════════════════════╣
║  CONTRACT INFORMATION                                                              ║
╠════════════════════════════════════════════════════════════════════════════════════╣
║  Name:             Tether USD (USDT)                                               ║
║  Creator:         0x5E4c3a55eE1a4C22B4d53C0903B3A5C2...                           ║
║  Compiler:         v0.8.19+commit.7dd6d404                                        ║
║  Creation Tx:     0x789abc...def123                                              ║
╠════════════════════════════════════════════════════════════════════════════════════╣
║  LAST 5 TRANSACTIONS                                                               ║
╠════════════════════════════════════════════════════════════════════════════════════╣
║  Transaction #1                                                                  ║
║  Hash:    0x456def789...abc123                                                    ║
║  Value:   0.050000 ETH                                                           ║
║  Status:  ✅                                                                      ║
║  Function: transfer                                                               ║
...
╚════════════════════════════════════════════════════════════════════════════════════╝
```

**Fields Displayed:**
- Contract Address
- ETH Balance
- USD Value
- Contract Name
- Contract Creator
- Compiler Version
- Creation Transaction Hash
- Contract Type (Proxy if applicable)
- Implementation Address (for proxies)
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
├── main.rs              # Entry point
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
├── config/              # Chain configuration
├── formatting/         # Output formatting
├── models/              # Data models
├── utils/               # Utility functions
├── validation/         # Input validation
└── cache/              # Price caching
```

---

## 📦 Dependencies

- **reqwest** - HTTP client
- **serde** - Serialization
- **tokio** - Async runtime
- **clap** - CLI parsing
- **chrono** - Date/time
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

---

<p align="center">
  Made with ❤️ for the Ethereum Community
</p>
