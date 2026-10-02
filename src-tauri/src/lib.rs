mod backup;
mod commands;
mod db;
mod export;
mod fx;
mod models;
mod money;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let database = db::init().expect("failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            get_settings_cmd,
            save_settings_cmd,
            list_accounts_cmd,
            save_account_cmd,
            delete_account_cmd,
            reorder_accounts_cmd,
            list_categories_cmd,
            save_category_cmd,
            delete_category_cmd,
            reorder_categories_cmd,
            list_transactions_cmd,
            save_transaction_cmd,
            delete_transaction_cmd,
            delete_transactions_cmd,
            count_transactions_cmd,
            list_tags_cmd,
            list_goals_cmd,
            save_goal_cmd,
            delete_goal_cmd,
            list_debts_cmd,
            save_debt_cmd,
            delete_debt_cmd,
            add_debt_payment_cmd,
            list_budgets_cmd,
            save_budget_cmd,
            delete_budget_cmd,
            list_recurring_cmd,
            save_recurring_cmd,
            delete_recurring_cmd,
            toggle_recurring_cmd,
            run_recurring_cmd,
            run_recurring_one_cmd,
            list_crypto_assets_cmd,
            save_crypto_asset_cmd,
            delete_crypto_asset_cmd,
            fetch_crypto_price_cmd,
            fetch_crypto_chart_cmd,
            list_cards_cmd,
            save_card_cmd,
            delete_card_cmd,
            list_installments_cmd,
            save_installment_cmd,
            delete_installment_cmd,
            pay_installment_cmd,
            report_summary_cmd,
            report_series_cmd,
            report_by_category_cmd,
            report_largest_expense_cmd,
            fetch_fx_rate_cmd,
            export_xlsx_cmd,
            export_csv_cmd,
            backup_db_cmd,
            restore_db_cmd,
            parse_amount_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
