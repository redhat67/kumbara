# 🐷 Stellar Kumbara

Stellar ağındaki XLM bakiyenizi takip etmek için şık ve minimalist bir kumbara uygulaması.

## ✨ Özellikler

- 🔐 **Freighter Cüzdan Entegrasyonu** - Güvenli cüzdan bağlantısı
- 🌐 **Testnet & Mainnet Desteği** - İstediğiniz ağı seçin
- 💰 **Gerçek Zamanlı Bakiye** - XLM bakiyenizi anında görün
- 🎁 **Friendbot Desteği** - Testnet'te ücretsiz XLM alın
- 📊 **İşlem Geçmişi** - Toplam işlem sayınızı görün
- 🎨 **Modern Tasarım** - Gri tonlarında şık banka arayüzü
- 📱 **Responsive** - Mobil ve masaüstünde mükemmel çalışır

## 🚀 Kurulum

### Gereksinimler

- Go 1.21 veya üzeri
- [Freighter Wallet](https://www.freighter.app/) tarayıcı uzantısı

### Çalıştırma

```bash
go run main.go
```

Tarayıcınızda `http://localhost:8080` adresine gidin.

## 📖 Kullanım

1. **Ağ Seçimi**: Testnet veya Mainnet seçin
2. **Cüzdan Bağlantısı**: "Freighter Bağlan" butonuna tıklayın
3. **İzin Verin**: Freighter popup'ında izin verin
4. **Bakiyenizi Görün**: XLM bakiyeniz ve işlem geçmişiniz görüntülenecek

### Testnet'te Test Etme

1. Testnet seçin
2. Cüzdanınızı bağlayın
3. "🎁 Testnet'ten Ücretsiz XLM Al" butonuna tıklayın
4. 10,000 test XLM otomatik olarak hesabınıza gelecek

## 🛠️ Teknolojiler

- **Backend**: Go
- **Frontend**: HTML, CSS, Vanilla JavaScript
- **Blockchain**: Stellar Network
- **Wallet**: Freighter API
- **SDK**: Stellar SDK

## 📁 Proje Yapısı

```
stellar-piggybank/
├── main.go           # Go web sunucusu
├── go.mod            # Go modül dosyası
├── static/
│   ├── index.html    # Ana sayfa
│   └── styles.css    # Stil dosyası
└── README.md         # Bu dosya
```

## 🎨 Tasarım

Gri tonlarında modern ve minimalist bir banka arayüzü. Gradient arka planlar, yumuşak gölgeler ve temiz tipografi ile profesyonel bir görünüm.

## 🔒 Güvenlik

- Özel anahtarlar hiçbir zaman sunucuya gönderilmez
- Tüm işlemler Freighter uzantısı üzerinden yapılır
- Sadece okuma izni kullanılır (bakiye görüntüleme)

## 📝 Notlar

- Mainnet'te hesap aktifleştirmek için minimum 1 XLM gerekir
- Testnet tamamen ücretsizdir ve test amaçlıdır
- Freighter uzantısı yüklü olmalıdır

## 🌟 Stellar Hakkında

Stellar, hızlı ve düşük maliyetli uluslararası para transferleri için tasarlanmış açık kaynaklı bir blockchain ağıdır.

---

**Yapımcı**: Stellar Kumbara Ekibi  
**Lisans**: MIT  
**Stellar**: [stellar.org](https://stellar.org)  
**Freighter**: [freighter.app](https://www.freighter.app)
