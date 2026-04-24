# 🚀 Stellar Kumbara Smart Contract Deployment

## Soroban Kurulumu

```bash
# Rust yükleyin (eğer yoksa)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Soroban CLI yükleyin
cargo install --locked soroban-cli

# Stellar target ekleyin
rustup target add wasm32-unknown-unknown
```

## Kontrat Derleme

```bash
cd contract
cargo build --target wasm32-unknown-unknown --release
```

Derlenmiş kontrat: `target/wasm32-unknown-unknown/release/kumbara_contract.wasm`

## Testnet'e Deploy

### 1. Cüzdan Oluştur/Yapılandır

```bash
# Yeni identity oluştur
soroban keys generate kumbara --network testnet

# Public key'i al
soroban keys address kumbara

# Friendbot'tan XLM al
soroban keys fund kumbara --network testnet
```

### 2. Kontratı Deploy Et

```bash
# Kontratı deploy et
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/kumbara_contract.wasm \
  --source kumbara \
  --network testnet

# Çıktı: CONTRACT_ID (kaydedin!)
```

### 3. Kontratı Test Et

```bash
# Kumbara başlat (hedef: 1000 stroops)
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source kumbara \
  --network testnet \
  -- initialize \
  --owner <YOUR_PUBLIC_KEY> \
  --goal 1000

# Para yatır
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source kumbara \
  --network testnet \
  -- deposit \
  --owner <YOUR_PUBLIC_KEY> \
  --amount 500

# Bakiye sorgula
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source kumbara \
  --network testnet \
  -- get_balance \
  --owner <YOUR_PUBLIC_KEY>

# Hedefe ulaşıldı mı?
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source kumbara \
  --network testnet \
  -- is_goal_reached \
  --owner <YOUR_PUBLIC_KEY>
```

## Mainnet'e Deploy

```bash
# Mainnet için identity oluştur
soroban keys generate kumbara-mainnet --network mainnet

# Gerçek XLM ile fonlayın (minimum 1 XLM)

# Deploy
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/kumbara_contract.wasm \
  --source kumbara-mainnet \
  --network mainnet
```

## Kontrat Fonksiyonları

| Fonksiyon | Açıklama | Parametreler |
|-----------|----------|--------------|
| `initialize` | Yeni kumbara oluştur | owner, goal |
| `deposit` | Para yatır | owner, amount |
| `withdraw` | Para çek | owner, amount |
| `get_balance` | Bakiye sorgula | owner |
| `get_goal` | Hedef sorgula | owner |
| `is_goal_reached` | Hedefe ulaşıldı mı? | owner |

## Deployment Bilgileri

**Network**: Testnet  
**Contract ID**: `[Deploy sonrası buraya eklenecek]`  
**Deployer**: `[Public key buraya eklenecek]`  
**Deploy Date**: `[Tarih buraya eklenecek]`

## Soroban Explorer

Kontratınızı görüntülemek için:
- Testnet: https://stellar.expert/explorer/testnet/contract/[CONTRACT_ID]
- Mainnet: https://stellar.expert/explorer/public/contract/[CONTRACT_ID]

## Notlar

- Testnet ücretsizdir ve test amaçlıdır
- Mainnet gerçek XLM kullanır
- Kontrat deploy edildikten sonra değiştirilemez
- Her kullanıcı kendi kumbarasını oluşturabilir
- Bakiyeler kontrat içinde tutulur (simüle edilir)

## Sorun Giderme

```bash
# Soroban versiyonunu kontrol et
soroban --version

# Network durumunu kontrol et
soroban network ls

# Kontrat testlerini çalıştır
cd contract
cargo test
```
