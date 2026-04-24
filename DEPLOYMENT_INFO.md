# 📋 Deployment Bilgileri

## ⚠️ Windows Build Tools Gereksinimi

Windows'ta Soroban kontratlarını derlemek için Visual Studio Build Tools gereklidir.

### Build Tools Kurulumu

1. [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022) indirin
2. "Desktop development with C++" seçeneğini işaretleyin
3. Kurulumu tamamlayın
4. Bilgisayarı yeniden başlatın

### Alternatif: Linux/WSL Kullanımı

```bash
# WSL2 kurulumu (PowerShell Admin)
wsl --install

# WSL içinde
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --locked soroban-cli
rustup target add wasm32-unknown-unknown

# Kontratı derle
cd contract
cargo build --target wasm32-unknown-unknown --release
```

## 🚀 Manuel Deploy Adımları

### 1. Kontratı Derle

```bash
cd contract
cargo build --target wasm32-unknown-unknown --release
```

Çıktı: `target/wasm32-unknown-unknown/release/kumbara_contract.wasm`

### 2. Soroban CLI ile Deploy

```bash
# Identity oluştur
soroban keys generate kumbara --network testnet

# Public key al
soroban keys address kumbara

# Friendbot'tan XLM al
soroban keys fund kumbara --network testnet

# Deploy et
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/kumbara_contract.wasm \
  --source kumbara \
  --network testnet
```

### 3. Kontrat ID'yi Kaydet

Deploy sonrası aldığınız CONTRACT_ID'yi kaydedin.

## 🧪 Test Komutları

```bash
# Kumbara başlat (10 XLM hedef)
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source kumbara \
  --network testnet \
  -- initialize \
  --owner <YOUR_PUBLIC_KEY> \
  --goal 100000000

# Para yatır (5 XLM)
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source kumbara \
  --network testnet \
  -- deposit \
  --owner <YOUR_PUBLIC_KEY> \
  --amount 50000000

# Bakiye sorgula
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source kumbara \
  --network testnet \
  -- get_balance \
  --owner <YOUR_PUBLIC_KEY>
```

## 📊 Kontrat Fonksiyonları

| Fonksiyon | Parametreler | Açıklama |
|-----------|--------------|----------|
| `initialize` | owner, goal | Yeni kumbara oluştur |
| `deposit` | owner, amount | Para yatır |
| `withdraw` | owner, amount | Para çek |
| `get_balance` | owner | Bakiye sorgula |
| `get_goal` | owner | Hedef sorgula |
| `is_goal_reached` | owner | Hedefe ulaşıldı mı? |

**Not:** 1 XLM = 10,000,000 stroops

## 🌐 Explorer

- Testnet: `https://stellar.expert/explorer/testnet/contract/[CONTRACT_ID]`
- Mainnet: `https://stellar.expert/explorer/public/contract/[CONTRACT_ID]`

## 🔗 Kaynaklar

- [Soroban Docs](https://soroban.stellar.org/docs)
- [Stellar CLI](https://github.com/stellar/stellar-cli)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
