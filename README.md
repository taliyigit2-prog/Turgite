# Turgite

**Turgite** is a free, open-source, multi-currency and multi-language personal finance tracker for macOS, Windows and Linux. Track expenses, income, budgets, recurring bills, goals, debts, credit cards, installments and crypto holdings — all in a clean, fast desktop app.

## Features

- 🌍 **6 languages** — Turkish, English, Russian, German, French, Spanish
- 💱 **160+ currencies** (full ISO 4217 catalogue) with automatic exchange rates
- 🧮 **Multi-currency transactions** — enter any amount in any currency; Turgite stores the exchange rate at entry time and shows everything in your base currency
- 📊 **Overview & reports** — cash-flow charts, category breakdowns, largest expenses
- 🏦 Accounts, categories, tags, payees
- 🎯 Budgets (weekly / monthly / yearly, rollover)
- 🔁 Recurring transactions
- 🥅 Goals and debts with progress tracking
- 💳 **Credit cards** — limits, statement/due days, installments
- 🪙 **Crypto portfolio** — live prices and P/L (via CoinGecko)
- 📤 Export to **Excel (.xlsx)** and **CSV**
- 🔐 Encrypted backups (AES-256-GCM + Argon2)

## Tech stack

- [Tauri 2](https://tauri.app) (Rust) + [Vue 3](https://vuejs.org) + [Vite](https://vite.dev)
- SQLite (embedded) — data stored locally, never leaves your machine
- [ECharts](https://echarts.apache.org) for charts
- [vue-i18n](https://vue-i18n.intlify.dev) for localization

## Data sources

- Foreign exchange rates: [currency-api](https://github.com/fawazahmed0/currency-api) (no API key, no rate limits)
- Crypto prices: [CoinGecko](https://www.coingecko.com) public API

## Development

Prerequisites: [Rust](https://rustup.rs), [Node.js](https://nodejs.org) 18+, and platform build tools.

```bash
npm install
npm run tauri dev
```

To build a production bundle:

```bash
npm run tauri build
```

## License

[MIT](./LICENSE)
