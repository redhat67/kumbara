# 🔒 DarkWeb3 - Data Breach Checker

A decentralized data breach monitoring system built on Stellar blockchain. Check if your personal information has been compromised in known data breaches.

## ✨ Features

- 🔐 **Privacy-First** - Only hashed data is stored and checked
- 🌐 **Blockchain-Based** - Immutable breach records on Stellar
- 🔍 **Instant Check** - Real-time breach detection
- 📊 **Breach History** - See which breaches affected you
- 🛡️ **Security Alerts** - Get notified about compromised data
- 🎨 **Dark Theme** - Cybersecurity-focused UI
- 📱 **Responsive** - Works on all devices

## 🚀 Installation

### Requirements

- Go 1.21 or higher
- Modern web browser

### Running

```bash
go run main.go
```

Open `http://localhost:8080` in your browser.

### Smart Contract Deploy

⚠️ **Windows Kullanıcıları**: Kontrat derlemek için Visual Studio Build Tools gereklidir.

Detaylı deploy rehberi için:
- [DEPLOY.md](DEPLOY.md) - Genel deploy adımları
- [DEPLOYMENT_INFO.md](DEPLOYMENT_INFO.md) - Windows kurulum ve detaylı bilgiler

**Hızlı Deploy (Linux/Mac/WSL):**
```bash
# Soroban CLI kur
cargo install --locked soroban-cli
rustup target add wasm32-unknown-unknown

# Kontratı derle
cd contract
cargo build --target wasm32-unknown-unknown --release

# Deploy et
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/kumbara_contract.wasm \
  --source kumbara \
  --network testnet
```

## 📖 Usage

1. **Enter Your Data**: Input email, username, or phone number
2. **Check Breach**: Click "Check for Breaches"
3. **View Results**: See if your data was compromised
4. **Get Details**: View breach dates and affected platforms

### How It Works

1. Your input is hashed using SHA-256 (never stored in plain text)
2. Hash is checked against blockchain breach records
3. Results show breach information without exposing actual data
4. You get actionable security recommendations

## 🛠️ Technologies

- **Backend**: Go
- **Frontend**: HTML, CSS, Vanilla JavaScript
- **Smart Contract**: Rust + Soroban
- **Blockchain**: Stellar Network
- **Hashing**: SHA-256
- **SDK**: Stellar SDK

## 📁 Project Structure

```
darkweb3/
├── main.go              # Go web server
├── go.mod               # Go module file
├── static/
│   ├── index.html       # Main page
│   └── styles.css       # Stylesheet
├── contract/            # Soroban smart contract
│   ├── src/
│   │   └── lib.rs       # Breach checker contract (Rust)
│   └── Cargo.toml       # Rust dependencies
├── DEPLOY.md            # Deployment guide
├── DEPLOYMENT_INFO.md   # Detailed deployment info
└── README.md            # This file
```

## 🎨 Tasarım

Gri tonlarında modern ve minimalist bir banka arayüzü. Gradient arka planlar, yumuşak gölgeler ve temiz tipografi ile profesyonel bir görünüm.

## 🔒 Güvenlik

- Özel anahtarlar hiçbir zaman sunucuya gönderilmez
- Tüm işlemler Freighter uzantısı üzerinden yapılır
- Sadece okuma izni kullanılır (bakiye görüntüleme)

## 📝 Important Notes

- **Privacy**: Only SHA-256 hashes are stored, never plain text data
- **Legal**: This tool helps users protect themselves, similar to haveibeenpwned.com
- **Ethical**: No actual leaked data is stored or displayed
- **Testnet**: Free for testing purposes
- **Security**: All checks are done client-side before hashing

## 📜 Smart Contract Features

Soroban-based breach checker contract:

- ✅ Store breach records (hashed data only)
- ✅ Check if data is compromised
- ✅ Record breach metadata (date, platform)
- ✅ Query breach history
- ✅ Privacy-preserving verification
- ✅ Immutable breach records

**Contract Functions:**
- `add_breach(hash, platform, date)` - Add breach record (admin only)
- `check_breach(hash)` - Check if data is compromised
- `get_breach_info(hash)` - Get breach details
- `get_breach_count()` - Total breaches recorded
- `is_compromised(hash)` - Quick breach check

For detailed deployment info, see [DEPLOYMENT_INFO.md](DEPLOYMENT_INFO.md).

## 🔐 Security & Privacy

- All data is hashed using SHA-256 before any checks
- No plain text data is ever transmitted or stored
- Blockchain ensures immutable breach records
- Client-side hashing protects user privacy
- Open source for transparency

## 🌟 About Stellar

Stellar is an open-source blockchain network designed for fast, low-cost transactions. Soroban is Stellar's smart contract platform, providing security and immutability for breach records.

---

**Creator**: DarkWeb3 Team  
**License**: MIT  
**Stellar**: [stellar.org](https://stellar.org)  
**Purpose**: Helping users protect their digital identity
