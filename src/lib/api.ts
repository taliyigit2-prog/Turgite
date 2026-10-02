import { invoke } from "@tauri-apps/api/core";

// ---- Type definitions mirroring the Rust backend (camelCase) ----

export interface Settings {
  baseCurrency: string;
  language: string;
  theme: string;
  monthStartDay: number;
  weekStart: number;
  dateFormat: string;
}

export interface Account {
  id: number;
  name: string;
  kind: string;
  currency: string;
  openingBalanceMinor: number;
  icon: string;
  color: string;
  sortOrder: number;
  balanceMinor: number;
}

export interface AccountInput {
  id?: number;
  name: string;
  kind: string;
  currency: string;
  openingBalanceMinor: number;
  icon: string;
  color: string;
}

export interface Category {
  id: number;
  name: string;
  kind: string;
  parentId: number | null;
  icon: string;
  color: string;
  keywords: string;
  sortOrder: number;
}

export interface CategoryInput {
  id?: number;
  name: string;
  kind: string;
  parentId: number | null;
  icon: string;
  color: string;
  keywords: string;
}

export interface Transaction {
  id: number;
  kind: string;
  accountId: number;
  accountName: string;
  accountCurrency: string;
  destAccountId: number | null;
  destAccountName: string | null;
  categoryId: number | null;
  categoryName: string | null;
  categoryIcon: string | null;
  categoryColor: string | null;
  goalId: number | null;
  amountMinor: number;
  currency: string;
  amountBaseMinor: number;
  rateScaled: number;
  occurredAt: number;
  payee: string;
  note: string;
  source: string;
  tags: string[];
}

export interface TxnInput {
  id?: number;
  kind: string;
  accountId: number;
  destAccountId: number | null;
  categoryId: number | null;
  goalId?: number | null;
  amountMinor: number;
  currency: string;
  rateScaled?: number;
  occurredAt: number;
  payee: string;
  note: string;
  tags: string[];
}

export interface TxnFilter {
  kind?: string;
  accountId?: number;
  categoryId?: number;
  from?: number;
  to?: number;
  search?: string;
  limit?: number;
  offset?: number;
}

export interface Goal {
  id: number;
  name: string;
  targetMinor: number;
  currency: string;
  targetDate: number | null;
  accountId: number | null;
  color: string;
  note: string;
  icon: string;
  savedMinor: number;
}

export interface Debt {
  id: number;
  counterpart: string;
  direction: string;
  principalMinor: number;
  currency: string;
  accountId: number | null;
  dueAt: number | null;
  settledAt: number | null;
  note: string;
  paidMinor: number;
  outstandingMinor: number;
}

export interface Budget {
  id: number;
  categoryId: number | null;
  categoryName: string | null;
  categoryIcon: string | null;
  categoryColor: string | null;
  period: string;
  amountBaseMinor: number;
  startAt: number;
  rollover: boolean;
  active: boolean;
  spentMinor: number;
  remainingMinor: number;
  percent: number;
}

export interface Recurring {
  id: number;
  name: string;
  kind: string;
  accountId: number;
  accountName: string;
  destAccountId: number | null;
  categoryId: number | null;
  categoryName: string | null;
  amountMinor: number;
  currency: string;
  payee: string;
  note: string;
  freq: string;
  intervalN: number;
  nextRunAt: number;
  endAt: number | null;
  autoPost: boolean;
  active: boolean;
}

export interface CryptoAsset {
  id: number;
  coinId: string;
  symbol: string;
  name: string;
  amount: number;
  costBasisBaseMinor: number;
}

export interface CreditCard {
  id: number;
  accountId: number;
  accountName: string;
  name: string;
  creditLimitMinor: number;
  statementDay: number;
  dueDay: number;
  balanceMinor: number;
}

export interface Installment {
  id: number;
  accountId: number;
  name: string;
  totalMinor: number;
  currency: string;
  months: number;
  paidCount: number;
  startedAt: number;
  note: string;
  monthlyMinor: number;
  remainingMonths: number;
  remainingMinor: number;
}

export interface ReportSummary {
  incomeMinor: number;
  expenseMinor: number;
  netMinor: number;
  txCount: number;
}

export interface SeriesPoint {
  date: number;
  income: number;
  expense: number;
  net: number;
}

export interface CategoryTotal {
  categoryId: number | null;
  name: string;
  icon: string;
  color: string;
  total: number;
}

export interface LargestExpense {
  id: number;
  payee: string;
  amountMinor: number;
  currency: string;
  amountBaseMinor: number;
  occurredAt: number;
}

export interface FxRate {
  base: string;
  quote: string;
  rate: number;
  rateScaled: number;
  date: string;
  source: string;
}

export interface CryptoPrice {
  coinId: string;
  price: number;
  priceBaseMinor: number;
}

// ---- API wrappers ----

export const api = {
  getSettings: () => invoke<Settings>("get_settings_cmd"),
  saveSettings: (s: Settings) => invoke<Settings>("save_settings_cmd", { s }),

  listAccounts: () => invoke<Account[]>("list_accounts_cmd"),
  saveAccount: (input: AccountInput) => invoke("save_account_cmd", { input }),
  deleteAccount: (id: number) => invoke("delete_account_cmd", { id }),
  reorderAccounts: (ids: number[]) => invoke("reorder_accounts_cmd", { ids }),

  listCategories: () => invoke<Category[]>("list_categories_cmd"),
  saveCategory: (input: CategoryInput) => invoke("save_category_cmd", { input }),
  deleteCategory: (id: number) => invoke("delete_category_cmd", { id }),
  reorderCategories: (ids: number[]) => invoke("reorder_categories_cmd", { ids }),

  listTransactions: (filter: TxnFilter = {}) => invoke<Transaction[]>("list_transactions_cmd", { filter }),
  saveTransaction: (input: TxnInput) => invoke<Transaction>("save_transaction_cmd", { input }),
  deleteTransaction: (id: number) => invoke("delete_transaction_cmd", { id }),
  deleteTransactions: (ids: number[]) => invoke("delete_transactions_cmd", { ids }),
  listTags: () => invoke<string[]>("list_tags_cmd"),

  listGoals: () => invoke<Goal[]>("list_goals_cmd"),
  saveGoal: (input: Record<string, unknown>) => invoke("save_goal_cmd", { input }),
  deleteGoal: (id: number) => invoke("delete_goal_cmd", { id }),

  listDebts: () => invoke<Debt[]>("list_debts_cmd"),
  saveDebt: (input: Record<string, unknown>) => invoke("save_debt_cmd", { input }),
  deleteDebt: (id: number) => invoke("delete_debt_cmd", { id }),
  addDebtPayment: (debtId: number, amountMinor: number, paidAt: number) =>
    invoke("add_debt_payment_cmd", { debtId, amountMinor, paidAt }),

  listBudgets: () => invoke<Budget[]>("list_budgets_cmd"),
  saveBudget: (input: Record<string, unknown>) => invoke("save_budget_cmd", { input }),
  deleteBudget: (id: number) => invoke("delete_budget_cmd", { id }),

  listRecurring: () => invoke<Recurring[]>("list_recurring_cmd"),
  saveRecurring: (input: Record<string, unknown>) => invoke("save_recurring_cmd", { input }),
  deleteRecurring: (id: number) => invoke("delete_recurring_cmd", { id }),
  toggleRecurring: (id: number, active: boolean) => invoke("toggle_recurring_cmd", { id, active }),
  runRecurring: () => invoke<number>("run_recurring_cmd"),
  runRecurringOne: (id: number) => invoke("run_recurring_one_cmd", { id }),

  listCryptoAssets: () => invoke<CryptoAsset[]>("list_crypto_assets_cmd"),
  saveCryptoAsset: (input: Record<string, unknown>) => invoke("save_crypto_asset_cmd", { input }),
  deleteCryptoAsset: (id: number) => invoke("delete_crypto_asset_cmd", { id }),
  fetchCryptoPrice: (coinId: string, base: string) => invoke<CryptoPrice>("fetch_crypto_price_cmd", { coinId, base }),
  fetchCryptoChart: (coinId: string, base: string, days: number) =>
    invoke<[number, number][]>("fetch_crypto_chart_cmd", { coinId, base, days }),

  listCards: () => invoke<CreditCard[]>("list_cards_cmd"),
  saveCard: (input: Record<string, unknown>) => invoke("save_card_cmd", { input }),
  deleteCard: (id: number) => invoke("delete_card_cmd", { id }),
  listInstallments: () => invoke<Installment[]>("list_installments_cmd"),
  saveInstallment: (input: Record<string, unknown>) => invoke("save_installment_cmd", { input }),
  deleteInstallment: (id: number) => invoke("delete_installment_cmd", { id }),
  payInstallment: (id: number, paidAt: number) => invoke("pay_installment_cmd", { id, paidAt }),

  reportSummary: (from: number, to: number) => invoke<ReportSummary>("report_summary_cmd", { from, to }),
  reportSeries: (from: number, to: number) => invoke<SeriesPoint[]>("report_series_cmd", { from, to }),
  reportByCategory: (from: number, to: number, kind: string) =>
    invoke<CategoryTotal[]>("report_by_category_cmd", { from, to, kind }),
  reportLargestExpense: (from: number, to: number, limit: number) =>
    invoke<LargestExpense[]>("report_largest_expense_cmd", { from, to, limit }),

  fetchFxRate: (base: string, quote: string) => invoke<FxRate>("fetch_fx_rate_cmd", { base, quote }),

  exportXlsx: (path: string, headers: string[], rows: string[][]) =>
    invoke("export_xlsx_cmd", { path, headers, rows }),
  exportCsv: (path: string, headers: string[], rows: string[][]) =>
    invoke("export_csv_cmd", { path, headers, rows }),

  backupDb: (path: string, password: string | null) => invoke("backup_db_cmd", { path, password }),
  restoreDb: (path: string, password: string | null) => invoke("restore_db_cmd", { path, password }),

  parseAmount: (s: string, code: string) => invoke<number>("parse_amount_cmd", { s, code }),
};
