# 💰 Turgite

**Turgite** is a free and open-source **personal finance tracker** for macOS, Windows and Linux. Track your expenses, income, budgets, recurring bills, goals, debts, credit cards, installments and even your crypto portfolio — in a fast, private, native desktop app.

<div align="center">

[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/taliyigit2-prog/Turgite?color=teal)](https://github.com/taliyigit2-prog/Turgite/releases)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue.svg)](#)

</div>

---

## ✨ Features

- 🌍 **6 languages** — Türkçe, English, Русский, Deutsch, Français, Español
- 💱 **160+ currencies** (full ISO 4217 catalogue) with automatic exchange rates
- 🧮 **Multi-currency transactions** — enter any amount in any currency. Turgite stores the exchange rate *at entry time* and shows everything in your base currency (default **TL ₺**)
- 📊 **Overview & reports** — cash-flow charts, category breakdowns, largest expenses
- 🏦 **Accounts** — cash, bank, e-wallet, credit card, savings
- 🏷️ **Categories** with icons, colors, keywords and sub-categories
- 🎯 **Budgets** — weekly / monthly / yearly, with rollover
- 🔁 **Recurring transactions**
- 🥅 **Goals** and **debts** (lent / owed) with progress tracking
- 💳 **Credit cards** — limits, statement & due days, **installments**
- 🪙 **Crypto portfolio** — live prices and profit/loss (via CoinGecko)
- 📤 Export to **Excel (.xlsx)** and **CSV**
- 🔐 **Encrypted backups** (AES-256-GCM + Argon2)
- 🌗 Light / dark / system theme
- 🔒 100% offline — your data stays in a local SQLite file on your machine

## ⬇️ Download

The easiest way to get Turgite is from the **[Releases page](https://github.com/taliyigit2-prog/Turgite/releases)**.

| Platform | File |
| --- | --- |
| 🍎 macOS (Apple Silicon) | [`Turgite-0.1.0-macos.dmg`](https://github.com/taliyigit2-prog/Turgite/releases/latest) |

> Windows and Linux builds are coming soon (the app is fully cross-platform — build it yourself with the steps below, or open an issue to request a build).

### Install on macOS

1. Download the `.dmg` from the Releases page and open it.
2. Drag **Turgite** into the **Applications** folder.
3. On first launch, macOS may warn that the app is from an "unidentified developer" (because it is not code-signed yet):

   - **Right-click** the app → **Open** → **Open** again, **or**
   - Run this in the terminal:
     ```bash
     xattr -dr com.apple.quarantine /Applications/Turgite.app
     ```

4. That's it — enjoy 🎉

## 📸 Screenshots

> Screenshots coming soon. Contributions welcome!

## 🛠️ Tech stack

- [Tauri 2](https://tauri.app) (Rust) + [Vue 3](https://vuejs.org) + [Vite](https://vite.dev)
- [SQLite](https://sqlite.org) (embedded, local)
- [ECharts](https://echarts.apache.org) for charts
- [vue-i18n](https://vue-i18n.intlify.dev) for localization

## 🔌 Data sources

| Data | Source | Notes |
| --- | --- | --- |
| Foreign exchange rates | [currency-api](https://github.com/fawazahmed0/currency-api) | No API key, no rate limits |
| Crypto prices | [CoinGecko](https://www.coingecko.com) | Public API |

Rates are fetched only when you enter a transaction or refresh prices, and cached locally.

## 🧑‍💻 Development

**Prerequisites:** [Rust](https://rustup.rs), [Node.js](https://nodejs.org) 18+.

```bash
git clone https://github.com/taliyigit2-prog/Turgite.git
cd Turgite
npm install
npm run tauri dev
```

Build a production bundle:

```bash
npm run tauri build
```

## 🌍 Contributing

Contributions are welcome! Translations, bug fixes, new features — feel free to open an issue or a pull request.

## 📄 License

[MIT](./LICENSE) — free for personal and commercial use.

---

## 🇹🇷 Türkçe

**Turgite**, macOS, Windows ve Linux için ücretsiz ve açık kaynaklı bir **kişisel finans takip** uygulamasıdır. Giderlerinizi, gelirlerinizi, bütçelerinizi, tekrarlayan ödemelerinizi, hedeflerinizi, borçlarınızı, kredi kartlarınızı, taksitlerinizi ve kripto portföyünüzü tek bir hızlı uygulamada takip edin.

**Öne çıkanlar:**

- 6 dil (Türkçe, İngilizce, Rusça, Almanca, Fransızca, İspanyolca)
- 160+ para birimi; işlem girerken döviz kuru otomatik çekilir ve **giriş anındaki kur** ile TL karşılığı saklanır
- Excel (.xlsx) ve CSV dışa aktarım
- Kredi kartı + taksit takibi, kripto portföyü, şifreli yedekleme

**İndirme:** [Releases](https://github.com/taliyigit2-prog/Turgite/releases) sayfasından `.dmg` dosyasını indirin, **Turgite**'yi **Uygulamalar** klasörüne sürükleyin. İlk açılışta macOS uyarı verirse uygulamaya **sağ tıklayın → Aç** seçin.

**Kurulum (geliştirme):**

```bash
git clone https://github.com/taliyigit2-prog/Turgite.git
cd Turgite
npm install
npm run tauri dev
```

---

## 🇷🇺 Русский

**Turgite** — бесплатный опенсорсный **менеджер личных финансов** для macOS, Windows и Linux. Расходы, доходы, бюджеты, повторяющиеся платежи, цели, долги, кредитные карты, рассрочки и криптопортфель — всё в одном быстром приложении.

**Ключевое:**

- 6 языков, 160+ валют; курс фиксируется **на момент ввода** транзакции
- Экспорт в Excel (.xlsx) и CSV
- Кредитные карты и рассрочки, криптопортфель, шифрованные резервные копии

**Скачать:** со страницы [Releases](https://github.com/taliyigit2-prog/Turgite/releases) (файл `.dmg`).

**Разработка:**

```bash
git clone https://github.com/taliyigit2-prog/Turgite.git
cd Turgite
npm install
npm run tauri dev
```
