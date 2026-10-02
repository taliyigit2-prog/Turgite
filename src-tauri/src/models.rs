use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub base_currency: String,
    pub language: String,
    pub theme: String,
    pub month_start_day: i64,
    pub week_start: i64,
    pub date_format: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub currency: String,
    pub opening_balance_minor: i64,
    pub icon: String,
    pub color: String,
    pub sort_order: i64,
    pub balance_minor: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInput {
    pub id: Option<i64>,
    pub name: String,
    pub kind: String,
    pub currency: String,
    pub opening_balance_minor: i64,
    pub icon: String,
    pub color: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub parent_id: Option<i64>,
    pub icon: String,
    pub color: String,
    pub keywords: String,
    pub sort_order: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryInput {
    pub id: Option<i64>,
    pub name: String,
    pub kind: String,
    pub parent_id: Option<i64>,
    pub icon: String,
    pub color: String,
    pub keywords: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: i64,
    pub kind: String,
    pub account_id: i64,
    pub account_name: String,
    pub account_currency: String,
    pub dest_account_id: Option<i64>,
    pub dest_account_name: Option<String>,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub category_icon: Option<String>,
    pub category_color: Option<String>,
    pub amount_minor: i64,
    pub currency: String,
    pub amount_base_minor: i64,
    pub rate_scaled: i64,
    pub occurred_at: i64,
    pub payee: String,
    pub note: String,
    pub source: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxnInput {
    pub id: Option<i64>,
    pub kind: String,
    pub account_id: i64,
    pub dest_account_id: Option<i64>,
    pub category_id: Option<i64>,
    pub amount_minor: i64,
    pub currency: String,
    pub rate_scaled: Option<i64>,
    pub occurred_at: i64,
    pub payee: String,
    pub note: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxnFilter {
    pub kind: Option<String>,
    pub account_id: Option<i64>,
    pub category_id: Option<i64>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub id: i64,
    pub name: String,
    pub target_minor: i64,
    pub currency: String,
    pub target_date: Option<i64>,
    pub account_id: Option<i64>,
    pub color: String,
    pub note: String,
    pub icon: String,
    pub saved_minor: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalInput {
    pub id: Option<i64>,
    pub name: String,
    pub target_minor: i64,
    pub currency: String,
    pub target_date: Option<i64>,
    pub account_id: Option<i64>,
    pub color: String,
    pub note: String,
    pub icon: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Debt {
    pub id: i64,
    pub counterpart: String,
    pub direction: String,
    pub principal_minor: i64,
    pub currency: String,
    pub account_id: Option<i64>,
    pub due_at: Option<i64>,
    pub settled_at: Option<i64>,
    pub note: String,
    pub paid_minor: i64,
    pub outstanding_minor: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebtInput {
    pub id: Option<i64>,
    pub counterpart: String,
    pub direction: String,
    pub principal_minor: i64,
    pub currency: String,
    pub account_id: Option<i64>,
    pub due_at: Option<i64>,
    pub settled_at: Option<i64>,
    pub note: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub id: i64,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub category_icon: Option<String>,
    pub category_color: Option<String>,
    pub period: String,
    pub amount_base_minor: i64,
    pub start_at: i64,
    pub rollover: bool,
    pub active: bool,
    pub spent_minor: i64,
    pub remaining_minor: i64,
    pub percent: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BudgetInput {
    pub id: Option<i64>,
    pub category_id: Option<i64>,
    pub period: String,
    pub amount_base_minor: i64,
    pub start_at: i64,
    pub rollover: bool,
    pub active: bool,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recurring {
    pub id: i64,
    pub name: String,
    pub kind: String,
    pub account_id: i64,
    pub account_name: String,
    pub dest_account_id: Option<i64>,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub amount_minor: i64,
    pub currency: String,
    pub payee: String,
    pub note: String,
    pub freq: String,
    pub interval_n: i64,
    pub next_run_at: i64,
    pub end_at: Option<i64>,
    pub auto_post: bool,
    pub active: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringInput {
    pub id: Option<i64>,
    pub name: String,
    pub kind: String,
    pub account_id: i64,
    pub dest_account_id: Option<i64>,
    pub category_id: Option<i64>,
    pub amount_minor: i64,
    pub currency: String,
    pub payee: String,
    pub note: String,
    pub freq: String,
    pub interval_n: i64,
    pub next_run_at: i64,
    pub end_at: Option<i64>,
    pub auto_post: bool,
    pub active: bool,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CryptoAsset {
    pub id: i64,
    pub coin_id: String,
    pub symbol: String,
    pub name: String,
    pub amount: f64,
    pub cost_basis_base_minor: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoAssetInput {
    pub id: Option<i64>,
    pub coin_id: String,
    pub symbol: String,
    pub name: String,
    pub amount: f64,
    pub cost_basis_base_minor: i64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreditCard {
    pub id: i64,
    pub account_id: i64,
    pub account_name: String,
    pub name: String,
    pub credit_limit_minor: i64,
    pub statement_day: i64,
    pub due_day: i64,
    pub balance_minor: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreditCardInput {
    pub id: Option<i64>,
    pub account_id: i64,
    pub name: String,
    pub credit_limit_minor: i64,
    pub statement_day: i64,
    pub due_day: i64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Installment {
    pub id: i64,
    pub account_id: i64,
    pub name: String,
    pub total_minor: i64,
    pub currency: String,
    pub months: i64,
    pub started_at: i64,
    pub note: String,
    pub monthly_minor: i64,
    pub remaining_months: i64,
    pub remaining_minor: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallmentInput {
    pub id: Option<i64>,
    pub account_id: i64,
    pub name: String,
    pub total_minor: i64,
    pub currency: String,
    pub months: i64,
    pub started_at: i64,
    pub note: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportSummary {
    pub income_minor: i64,
    pub expense_minor: i64,
    pub net_minor: i64,
    pub tx_count: i64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SeriesPoint {
    pub date: i64,
    pub income: i64,
    pub expense: i64,
    pub net: i64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CategoryTotal {
    pub category_id: Option<i64>,
    pub name: String,
    pub icon: String,
    pub color: String,
    pub total: i64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LargestExpense {
    pub id: i64,
    pub payee: String,
    pub amount_minor: i64,
    pub currency: String,
    pub amount_base_minor: i64,
    pub occurred_at: i64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FxRate {
    pub base: String,
    pub quote: String,
    pub rate: f64,
    pub rate_scaled: i64,
    pub date: String,
    pub source: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CryptoPrice {
    pub coin_id: String,
    pub price: f64,
    pub price_base_minor: i64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct CsvPreview {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub total_rows: usize,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TagCount {
    pub name: String,
    pub count: i64,
}
