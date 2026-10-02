use crate::backup;
use crate::db::{now_secs, Db};
use crate::export;
use crate::fx;
use crate::models::*;
use crate::money::{self, RATE_SCALE};
use rusqlite::{params, Connection, OptionalExtension};
use std::sync::MutexGuard;
use tauri::State;

type Res<T> = Result<T, String>;

fn conn<'a>(state: &'a State<Db>) -> MutexGuard<'a, Connection> {
    state.0.lock().unwrap()
}

fn get_settings(c: &Connection) -> Res<Settings> {
    let row = c
        .query_row(
            "SELECT base_currency, language, theme, month_start_day, week_start, date_format FROM settings WHERE id=1",
            [],
            |r| {
                Ok(Settings {
                    base_currency: r.get(0)?,
                    language: r.get(1)?,
                    theme: r.get(2)?,
                    month_start_day: r.get(3)?,
                    week_start: r.get(4)?,
                    date_format: r.get(5)?,
                })
            },
        )
        .optional()
        .map_err(|e| e.to_string())?;
    Ok(row.unwrap_or(Settings {
        base_currency: "TRY".into(),
        language: "tr".into(),
        theme: "system".into(),
        month_start_day: 1,
        week_start: 1,
        date_format: "yyyy-MM-dd".into(),
    }))
}

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn get_settings_cmd(state: State<Db>) -> Res<Settings> {
    let c = conn(&state);
    get_settings(&c)
}

#[tauri::command]
pub fn save_settings_cmd(state: State<Db>, s: Settings) -> Res<Settings> {
    let c = conn(&state);
    c.execute(
        "UPDATE settings SET base_currency=?1, language=?2, theme=?3, month_start_day=?4, week_start=?5, date_format=?6, updated_at=?7 WHERE id=1",
        params![s.base_currency, s.language, s.theme, s.month_start_day, s.week_start, s.date_format, now_secs()],
    )
    .map_err(|e| e.to_string())?;
    get_settings(&c)
}

// ---------------------------------------------------------------------------
// Accounts
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn list_accounts_cmd(state: State<Db>) -> Res<Vec<Account>> {
    let c = conn(&state);
    let mut stmt = c
        .prepare(
            "SELECT a.id, a.name, a.kind, a.currency, a.opening_balance_minor, a.icon, a.color, a.sort_order,
             COALESCE((
               SELECT SUM(CASE
                 WHEN t.kind='income' THEN t.amount_minor
                 WHEN t.kind='expense' THEN -t.amount_minor
                 WHEN t.kind='transfer' AND t.dest_account_id=a.id THEN t.amount_minor
                 WHEN t.kind='transfer' AND t.account_id=a.id THEN -t.amount_minor
                 ELSE 0 END)
               FROM transactions t
               WHERE (t.account_id=a.id OR t.dest_account_id=a.id) AND t.currency=a.currency
             ), 0) + a.opening_balance_minor
             FROM accounts a ORDER BY a.sort_order, a.id",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            Ok(Account {
                id: r.get(0)?,
                name: r.get(1)?,
                kind: r.get(2)?,
                currency: r.get(3)?,
                opening_balance_minor: r.get(4)?,
                icon: r.get(5)?,
                color: r.get(6)?,
                sort_order: r.get(7)?,
                balance_minor: r.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for a in rows {
        out.push(a.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

#[tauri::command]
pub fn save_account_cmd(state: State<Db>, input: AccountInput) -> Res<()> {
    let c = conn(&state);
    match input.id {
        Some(id) => {
            c.execute(
                "UPDATE accounts SET name=?1, kind=?2, currency=?3, opening_balance_minor=?4, icon=?5, color=?6 WHERE id=?7",
                params![input.name, input.kind, input.currency, input.opening_balance_minor, input.icon, input.color, id],
            )
            .map_err(|e| e.to_string())?;
        }
        None => {
            c.execute(
                "INSERT INTO accounts (name, kind, currency, opening_balance_minor, icon, color, sort_order, created_at) VALUES (?1,?2,?3,?4,?5,?6,(SELECT COALESCE(MAX(sort_order),0)+1 FROM accounts),?7)",
                params![input.name, input.kind, input.currency, input.opening_balance_minor, input.icon, input.color, now_secs()],
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_account_cmd(state: State<Db>, id: i64) -> Res<()> {
    let c = conn(&state);
    c.execute("DELETE FROM accounts WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn reorder_accounts_cmd(state: State<Db>, ids: Vec<i64>) -> Res<()> {
    let c = conn(&state);
    for (i, id) in ids.iter().enumerate() {
        c.execute("UPDATE accounts SET sort_order=?1 WHERE id=?2", params![i as i64, id])
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Categories
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn list_categories_cmd(state: State<Db>) -> Res<Vec<Category>> {
    let c = conn(&state);
    let mut stmt = c
        .prepare(
            "SELECT id, name, kind, parent_id, icon, color, keywords, sort_order FROM categories ORDER BY kind, sort_order, id",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            Ok(Category {
                id: r.get(0)?,
                name: r.get(1)?,
                kind: r.get(2)?,
                parent_id: r.get(3)?,
                icon: r.get(4)?,
                color: r.get(5)?,
                keywords: r.get(6)?,
                sort_order: r.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for a in rows {
        out.push(a.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

#[tauri::command]
pub fn save_category_cmd(state: State<Db>, input: CategoryInput) -> Res<()> {
    let c = conn(&state);
    match input.id {
        Some(id) => {
            c.execute(
                "UPDATE categories SET name=?1, kind=?2, parent_id=?3, icon=?4, color=?5, keywords=?6 WHERE id=?7",
                params![input.name, input.kind, input.parent_id, input.icon, input.color, input.keywords, id],
            )
            .map_err(|e| e.to_string())?;
        }
        None => {
            c.execute(
                "INSERT INTO categories (name, kind, parent_id, icon, color, keywords, sort_order) VALUES (?1,?2,?3,?4,?5,?6,(SELECT COALESCE(MAX(sort_order),0)+1 FROM categories WHERE kind=?2))",
                params![input.name, input.kind, input.parent_id, input.icon, input.color, input.keywords],
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_category_cmd(state: State<Db>, id: i64) -> Res<()> {
    let c = conn(&state);
    c.execute("DELETE FROM categories WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn reorder_categories_cmd(state: State<Db>, ids: Vec<i64>) -> Res<()> {
    let c = conn(&state);
    for (i, id) in ids.iter().enumerate() {
        c.execute("UPDATE categories SET sort_order=?1 WHERE id=?2", params![i as i64, id])
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Transactions
// ---------------------------------------------------------------------------
fn row_to_tx(r: &rusqlite::Row) -> rusqlite::Result<Transaction> {
    let tags_str: Option<String> = r.get(16)?;
    let tags = tags_str
        .map(|s| s.split(',').map(|x| x.to_string()).collect())
        .unwrap_or_default();
    Ok(Transaction {
        id: r.get(0)?,
        kind: r.get(1)?,
        account_id: r.get(2)?,
        account_name: r.get(3)?,
        account_currency: r.get(4)?,
        dest_account_id: r.get(5)?,
        dest_account_name: r.get(6)?,
        category_id: r.get(7)?,
        category_name: r.get(8)?,
        category_icon: r.get(9)?,
        category_color: r.get(10)?,
        amount_minor: r.get(11)?,
        currency: r.get(12)?,
        amount_base_minor: r.get(13)?,
        rate_scaled: r.get(14)?,
        occurred_at: r.get(15)?,
        payee: r.get(17)?,
        note: r.get(18)?,
        source: r.get(19)?,
        tags,
    })
}

const TX_SELECT: &str = "SELECT t.id, t.kind, t.account_id, a.name, a.currency, t.dest_account_id, da.name, t.category_id, c.name, c.icon, c.color, t.amount_minor, t.currency, t.amount_base_minor, t.rate_scaled, t.occurred_at, (SELECT group_concat(tg.name, ',') FROM tags tg JOIN transaction_tags tt ON tt.tag_id=tg.id WHERE tt.transaction_id=t.id), t.payee, t.note, t.source FROM transactions t LEFT JOIN accounts a ON a.id=t.account_id LEFT JOIN accounts da ON da.id=t.dest_account_id LEFT JOIN categories c ON c.id=t.category_id";

fn get_tx(c: &Connection, id: i64) -> Res<Transaction> {
    c.query_row(&format!("{} WHERE t.id=?1", TX_SELECT), params![id], row_to_tx)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_transactions_cmd(state: State<Db>, filter: TxnFilter) -> Res<Vec<Transaction>> {
    let c = conn(&state);
    let mut sql = String::from(TX_SELECT);
    let mut conds: Vec<String> = Vec::new();
    let mut args: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(k) = &filter.kind {
        conds.push("t.kind = ?".into());
        args.push(Box::new(k.clone()));
    }
    if let Some(a) = filter.account_id {
        conds.push("(t.account_id = ? OR t.dest_account_id = ?)".into());
        args.push(Box::new(a));
        args.push(Box::new(a));
    }
    if let Some(cat) = filter.category_id {
        conds.push("t.category_id = ?".into());
        args.push(Box::new(cat));
    }
    if let Some(f) = filter.from {
        conds.push("t.occurred_at >= ?".into());
        args.push(Box::new(f));
    }
    if let Some(t) = filter.to {
        conds.push("t.occurred_at <= ?".into());
        args.push(Box::new(t));
    }
    if let Some(s) = &filter.search {
        if !s.is_empty() {
            conds.push("(t.payee LIKE ? OR t.note LIKE ? OR c.name LIKE ?)".into());
            let pat = format!("%{}%", s);
            args.push(Box::new(pat.clone()));
            args.push(Box::new(pat.clone()));
            args.push(Box::new(pat));
        }
    }
    if !conds.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conds.join(" AND "));
    }
    sql.push_str(" ORDER BY t.occurred_at DESC, t.id DESC");
    let limit = filter.limit.unwrap_or(500);
    let offset = filter.offset.unwrap_or(0);
    sql.push_str(" LIMIT ? OFFSET ?");
    args.push(Box::new(limit));
    args.push(Box::new(offset));

    let mut stmt = c.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(rusqlite::params_from_iter(args.iter()), row_to_tx)
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

fn apply_tags(c: &Connection, tx_id: i64, tags: &[String]) -> Res<()> {
    c.execute("DELETE FROM transaction_tags WHERE transaction_id=?1", params![tx_id])
        .map_err(|e| e.to_string())?;
    for t in tags {
        let t = t.trim();
        if t.is_empty() {
            continue;
        }
        c.execute("INSERT OR IGNORE INTO tags (name) VALUES (?1)", params![t])
            .map_err(|e| e.to_string())?;
        c.execute(
            "INSERT OR IGNORE INTO transaction_tags (transaction_id, tag_id) VALUES (?1, (SELECT id FROM tags WHERE name=?2))",
            params![tx_id, t],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn save_transaction_cmd(state: State<Db>, input: TxnInput) -> Res<Transaction> {
    let c = conn(&state);
    let s = get_settings(&c)?;
    let base = s.base_currency.clone();
    let rate_scaled = input.rate_scaled.unwrap_or(RATE_SCALE);
    let amount_base_minor = if input.currency == base {
        input.amount_minor
    } else {
        money::to_base_minor(input.amount_minor, &input.currency, &base, rate_scaled)
    };
    let now = now_secs();
    let id = match input.id {
        Some(id) => {
            c.execute(
                "UPDATE transactions SET kind=?1, account_id=?2, dest_account_id=?3, category_id=?4, amount_minor=?5, currency=?6, amount_base_minor=?7, rate_scaled=?8, occurred_at=?9, payee=?10, note=?11, updated_at=?12 WHERE id=?13",
                params![input.kind, input.account_id, input.dest_account_id, input.category_id, input.amount_minor, input.currency, amount_base_minor, rate_scaled, input.occurred_at, input.payee, input.note, now, id],
            )
            .map_err(|e| e.to_string())?;
            id
        }
        None => {
            c.execute(
                "INSERT INTO transactions (kind, account_id, dest_account_id, category_id, amount_minor, currency, amount_base_minor, rate_scaled, occurred_at, payee, note, source, created_at, updated_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,'manual',?12,?12)",
                params![input.kind, input.account_id, input.dest_account_id, input.category_id, input.amount_minor, input.currency, amount_base_minor, rate_scaled, input.occurred_at, input.payee, input.note, now],
            )
            .map_err(|e| e.to_string())?;
            c.last_insert_rowid()
        }
    };
    apply_tags(&c, id, &input.tags)?;
    get_tx(&c, id)
}

#[tauri::command]
pub fn delete_transaction_cmd(state: State<Db>, id: i64) -> Res<()> {
    let c = conn(&state);
    c.execute("DELETE FROM transactions WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_transactions_cmd(state: State<Db>, ids: Vec<i64>) -> Res<()> {
    let c = conn(&state);
    for id in ids {
        c.execute("DELETE FROM transactions WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn count_transactions_cmd(state: State<Db>) -> Res<i64> {
    let c = conn(&state);
    c.query_row("SELECT COUNT(*) FROM transactions", [], |r| r.get(0))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_tags_cmd(state: State<Db>) -> Res<Vec<String>> {
    let c = conn(&state);
    let mut stmt = c.prepare("SELECT name FROM tags ORDER BY name").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |r| r.get::<_, String>(0)).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Goals
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn list_goals_cmd(state: State<Db>) -> Res<Vec<Goal>> {
    let c = conn(&state);
    let mut stmt = c
        .prepare(
            "SELECT g.id, g.name, g.target_minor, g.currency, g.target_date, g.account_id, g.color, g.note, g.icon,
             COALESCE((SELECT SUM(t.amount_minor) FROM transactions t WHERE t.goal_id=g.id AND t.currency=g.currency AND t.kind='expense'),0)
             FROM goals g ORDER BY g.id",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            Ok(Goal {
                id: r.get(0)?,
                name: r.get(1)?,
                target_minor: r.get(2)?,
                currency: r.get(3)?,
                target_date: r.get(4)?,
                account_id: r.get(5)?,
                color: r.get(6)?,
                note: r.get(7)?,
                icon: r.get(8)?,
                saved_minor: r.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

#[tauri::command]
pub fn save_goal_cmd(state: State<Db>, input: GoalInput) -> Res<()> {
    let c = conn(&state);
    match input.id {
        Some(id) => {
            c.execute(
                "UPDATE goals SET name=?1, target_minor=?2, currency=?3, target_date=?4, account_id=?5, color=?6, note=?7, icon=?8 WHERE id=?9",
                params![input.name, input.target_minor, input.currency, input.target_date, input.account_id, input.color, input.note, input.icon, id],
            ).map_err(|e| e.to_string())?;
        }
        None => {
            c.execute(
                "INSERT INTO goals (name, target_minor, currency, target_date, account_id, color, note, icon, created_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                params![input.name, input.target_minor, input.currency, input.target_date, input.account_id, input.color, input.note, input.icon, now_secs()],
            ).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_goal_cmd(state: State<Db>, id: i64) -> Res<()> {
    let c = conn(&state);
    c.execute("DELETE FROM goals WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Debts
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn list_debts_cmd(state: State<Db>) -> Res<Vec<Debt>> {
    let c = conn(&state);
    let mut stmt = c
        .prepare(
            "SELECT d.id, d.counterpart, d.direction, d.principal_minor, d.currency, d.account_id, d.due_at, d.settled_at, d.note,
             COALESCE((SELECT SUM(p.amount_minor) FROM debt_payments p WHERE p.debt_id=d.id),0)
             FROM debts d ORDER BY d.settled_at IS NULL DESC, d.id",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            let principal: i64 = r.get(3)?;
            let paid: i64 = r.get(9)?;
            Ok(Debt {
                id: r.get(0)?,
                counterpart: r.get(1)?,
                direction: r.get(2)?,
                principal_minor: principal,
                currency: r.get(4)?,
                account_id: r.get(5)?,
                due_at: r.get(6)?,
                settled_at: r.get(7)?,
                note: r.get(8)?,
                paid_minor: paid,
                outstanding_minor: (principal - paid).max(0),
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

#[tauri::command]
pub fn save_debt_cmd(state: State<Db>, input: DebtInput) -> Res<()> {
    let c = conn(&state);
    match input.id {
        Some(id) => {
            c.execute(
                "UPDATE debts SET counterpart=?1, direction=?2, principal_minor=?3, currency=?4, account_id=?5, due_at=?6, settled_at=?7, note=?8 WHERE id=?9",
                params![input.counterpart, input.direction, input.principal_minor, input.currency, input.account_id, input.due_at, input.settled_at, input.note, id],
            ).map_err(|e| e.to_string())?;
        }
        None => {
            c.execute(
                "INSERT INTO debts (counterpart, direction, principal_minor, currency, account_id, due_at, settled_at, note, created_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                params![input.counterpart, input.direction, input.principal_minor, input.currency, input.account_id, input.due_at, input.settled_at, input.note, now_secs()],
            ).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_debt_cmd(state: State<Db>, id: i64) -> Res<()> {
    let c = conn(&state);
    c.execute("DELETE FROM debts WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn add_debt_payment_cmd(state: State<Db>, debt_id: i64, amount_minor: i64, paid_at: i64) -> Res<()> {
    let c = conn(&state);
    c.execute(
        "INSERT INTO debt_payments (debt_id, amount_minor, paid_at) VALUES (?1,?2,?3)",
        params![debt_id, amount_minor, paid_at],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Budgets
// ---------------------------------------------------------------------------
fn period_start(start_at: i64, period: &str, now: i64) -> i64 {
    use chrono::{Datelike, NaiveDateTime, TimeZone, Utc};
    let dt = Utc.timestamp_opt(start_at, 0).single().unwrap_or_else(|| Utc::now());
    let n = Utc.timestamp_opt(now, 0).single().unwrap_or_else(|| Utc::now());
    match period {
        "weekly" => {
            let days = (n - dt).num_days();
            let weeks = days / 7;
            (dt + chrono::Duration::days(weeks * 7)).timestamp()
        }
        "yearly" => {
            let years = n.year() - dt.year();
            let candidate = dt.with_year(dt.year() + years).unwrap_or(dt);
            if candidate.timestamp() > now { candidate.timestamp() - 31536000 } else { candidate.timestamp() }
        }
        _ => {
            let months = (n.year() - dt.year()) * 12 + (n.month() as i32 - dt.month() as i32);
            let candidate = dt + chrono::Duration::days(0);
            // approximate by adding months via NaiveDate
            let nd = NaiveDateTime::from_timestamp_opt(dt.timestamp(), 0).unwrap().date();
            let mut y = nd.year();
            let mut m = nd.month() as i32 + months;
            while m > 12 { m -= 12; y += 1; }
            while m < 1 { m += 12; y -= 1; }
            let d = chrono::NaiveDate::from_ymd_opt(y, m as u32, nd.day()).unwrap_or(nd);
            let ts = d.and_hms_opt(0,0,0).unwrap().and_utc().timestamp();
            let _ = candidate;
            ts
        }
    }
}

#[tauri::command]
pub fn list_budgets_cmd(state: State<Db>) -> Res<Vec<Budget>> {
    let c = conn(&state);
    let s = get_settings(&c)?;
    let base = s.base_currency.clone();
    let now = now_secs();
    let mut stmt = c
        .prepare(
            "SELECT b.id, b.category_id, c.name, c.icon, c.color, b.period, b.amount_base_minor, b.start_at, b.rollover, b.active FROM budgets b LEFT JOIN categories c ON c.id=b.category_id ORDER BY b.id",
        )
        .map_err(|e| e.to_string())?;
    let budgets = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, Option<i64>>(1)?,
                r.get::<_, Option<String>>(2)?,
                r.get::<_, Option<String>>(3)?,
                r.get::<_, Option<String>>(4)?,
                r.get::<_, String>(5)?,
                r.get::<_, i64>(6)?,
                r.get::<_, i64>(7)?,
                r.get::<_, i64>(8)?,
                r.get::<_, i64>(9)?,
            ))
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for b in budgets {
        let (id, cat_id, cat_name, cat_icon, cat_color, period, amount, start_at, rollover, active) =
            b.map_err(|e| e.to_string())?;
        let ps = period_start(start_at, &period, now);
        let spent: i64 = match cat_id {
            Some(cid) => c
                .query_row(
                    "SELECT COALESCE(SUM(amount_base_minor),0) FROM transactions WHERE kind='expense' AND category_id=?1 AND occurred_at>=?2",
                    params![cid, ps],
                    |r| r.get(0),
                )
                .unwrap_or(0),
            None => c
                .query_row(
                    "SELECT COALESCE(SUM(amount_base_minor),0) FROM transactions WHERE kind='expense' AND occurred_at>=?1",
                    params![ps],
                    |r| r.get(0),
                )
                .unwrap_or(0),
        };
        let _ = base; // amount_base_minor is already in base units for every row
        let remaining = amount - spent;
        let percent = if amount > 0 { spent as f64 / amount as f64 * 100.0 } else { 0.0 };
        out.push(Budget {
            id,
            category_id: cat_id,
            category_name: cat_name,
            category_icon: cat_icon,
            category_color: cat_color,
            period,
            amount_base_minor: amount,
            start_at,
            rollover: rollover != 0,
            active: active != 0,
            spent_minor: spent,
            remaining_minor: remaining,
            percent,
        });
    }
    Ok(out)
}

#[tauri::command]
pub fn save_budget_cmd(state: State<Db>, input: BudgetInput) -> Res<()> {
    let c = conn(&state);
    let rollover = if input.rollover { 1 } else { 0 };
    let active = if input.active { 1 } else { 0 };
    match input.id {
        Some(id) => {
            c.execute(
                "UPDATE budgets SET category_id=?1, period=?2, amount_base_minor=?3, start_at=?4, rollover=?5, active=?6 WHERE id=?7",
                params![input.category_id, input.period, input.amount_base_minor, input.start_at, rollover, active, id],
            ).map_err(|e| e.to_string())?;
        }
        None => {
            c.execute(
                "INSERT INTO budgets (category_id, period, amount_base_minor, start_at, rollover, active, created_at) VALUES (?1,?2,?3,?4,?5,?6,?7)",
                params![input.category_id, input.period, input.amount_base_minor, input.start_at, rollover, active, now_secs()],
            ).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_budget_cmd(state: State<Db>, id: i64) -> Res<()> {
    let c = conn(&state);
    c.execute("DELETE FROM budgets WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Recurring
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn list_recurring_cmd(state: State<Db>) -> Res<Vec<Recurring>> {
    let c = conn(&state);
    let mut stmt = c
        .prepare(
            "SELECT r.id, r.name, r.kind, r.account_id, a.name, r.dest_account_id, r.category_id, c.name, r.amount_minor, r.currency, r.payee, r.note, r.freq, r.interval_n, r.next_run_at, r.end_at, r.auto_post, r.active FROM recurring_rules r LEFT JOIN accounts a ON a.id=r.account_id LEFT JOIN categories c ON c.id=r.category_id ORDER BY r.active DESC, r.next_run_at",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            Ok(Recurring {
                id: r.get(0)?,
                name: r.get(1)?,
                kind: r.get(2)?,
                account_id: r.get(3)?,
                account_name: r.get(4)?,
                dest_account_id: r.get(5)?,
                category_id: r.get(6)?,
                category_name: r.get(7)?,
                amount_minor: r.get(8)?,
                currency: r.get(9)?,
                payee: r.get(10)?,
                note: r.get(11)?,
                freq: r.get(12)?,
                interval_n: r.get(13)?,
                next_run_at: r.get(14)?,
                end_at: r.get(15)?,
                auto_post: r.get::<_, i64>(16)? != 0,
                active: r.get::<_, i64>(17)? != 0,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

#[tauri::command]
pub fn save_recurring_cmd(state: State<Db>, input: RecurringInput) -> Res<()> {
    let c = conn(&state);
    let auto = if input.auto_post { 1 } else { 0 };
    let active = if input.active { 1 } else { 0 };
    match input.id {
        Some(id) => {
            c.execute(
                "UPDATE recurring_rules SET name=?1, kind=?2, account_id=?3, dest_account_id=?4, category_id=?5, amount_minor=?6, currency=?7, payee=?8, note=?9, freq=?10, interval_n=?11, next_run_at=?12, end_at=?13, auto_post=?14, active=?15 WHERE id=?16",
                params![input.name, input.kind, input.account_id, input.dest_account_id, input.category_id, input.amount_minor, input.currency, input.payee, input.note, input.freq, input.interval_n, input.next_run_at, input.end_at, auto, active, id],
            ).map_err(|e| e.to_string())?;
        }
        None => {
            c.execute(
                "INSERT INTO recurring_rules (name, kind, account_id, dest_account_id, category_id, amount_minor, currency, payee, note, freq, interval_n, next_run_at, end_at, auto_post, active, created_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16)",
                params![input.name, input.kind, input.account_id, input.dest_account_id, input.category_id, input.amount_minor, input.currency, input.payee, input.note, input.freq, input.interval_n, input.next_run_at, input.end_at, auto, active, now_secs()],
            ).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_recurring_cmd(state: State<Db>, id: i64) -> Res<()> {
    let c = conn(&state);
    c.execute("DELETE FROM recurring_rules WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn toggle_recurring_cmd(state: State<Db>, id: i64, active: bool) -> Res<()> {
    let c = conn(&state);
    c.execute(
        "UPDATE recurring_rules SET active=?1 WHERE id=?2",
        params![if active { 1 } else { 0 }, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Crypto
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn list_crypto_assets_cmd(state: State<Db>) -> Res<Vec<CryptoAsset>> {
    let c = conn(&state);
    let mut stmt = c
        .prepare("SELECT id, coin_id, symbol, name, amount, cost_basis_base_minor FROM crypto_assets ORDER BY id")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            Ok(CryptoAsset {
                id: r.get(0)?,
                coin_id: r.get(1)?,
                symbol: r.get(2)?,
                name: r.get(3)?,
                amount: r.get(4)?,
                cost_basis_base_minor: r.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

#[tauri::command]
pub fn save_crypto_asset_cmd(state: State<Db>, input: CryptoAssetInput) -> Res<()> {
    let c = conn(&state);
    match input.id {
        Some(id) => {
            c.execute(
                "UPDATE crypto_assets SET coin_id=?1, symbol=?2, name=?3, amount=?4, cost_basis_base_minor=?5 WHERE id=?6",
                params![input.coin_id, input.symbol, input.name, input.amount, input.cost_basis_base_minor, id],
            ).map_err(|e| e.to_string())?;
        }
        None => {
            c.execute(
                "INSERT INTO crypto_assets (coin_id, symbol, name, amount, cost_basis_base_minor, created_at) VALUES (?1,?2,?3,?4,?5,?6)",
                params![input.coin_id, input.symbol, input.name, input.amount, input.cost_basis_base_minor, now_secs()],
            ).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_crypto_asset_cmd(state: State<Db>, id: i64) -> Res<()> {
    let c = conn(&state);
    c.execute("DELETE FROM crypto_assets WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn fetch_crypto_price_cmd(coin_id: String, base: String) -> Res<CryptoPrice> {
    fx::fetch_crypto_price(&coin_id, &base).await
}

#[tauri::command]
pub async fn fetch_crypto_chart_cmd(coin_id: String, base: String, days: u32) -> Res<Vec<(i64, f64)>> {
    fx::fetch_crypto_chart(&coin_id, &base, days).await
}

// ---------------------------------------------------------------------------
// Credit cards + installments
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn list_cards_cmd(state: State<Db>) -> Res<Vec<CreditCard>> {
    let c = conn(&state);
    let mut stmt = c
        .prepare(
            "SELECT cc.id, cc.account_id, a.name, cc.name, cc.credit_limit_minor, cc.statement_day, cc.due_day,
             COALESCE((SELECT SUM(t.amount_minor) FROM transactions t WHERE t.account_id=cc.account_id AND t.kind='expense'),0)
             FROM credit_cards cc LEFT JOIN accounts a ON a.id=cc.account_id ORDER BY cc.id",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            Ok(CreditCard {
                id: r.get(0)?,
                account_id: r.get(1)?,
                account_name: r.get(2)?,
                name: r.get(3)?,
                credit_limit_minor: r.get(4)?,
                statement_day: r.get(5)?,
                due_day: r.get(6)?,
                balance_minor: r.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

#[tauri::command]
pub fn save_card_cmd(state: State<Db>, input: CreditCardInput) -> Res<()> {
    let c = conn(&state);
    match input.id {
        Some(id) => {
            c.execute(
                "UPDATE credit_cards SET account_id=?1, name=?2, credit_limit_minor=?3, statement_day=?4, due_day=?5 WHERE id=?6",
                params![input.account_id, input.name, input.credit_limit_minor, input.statement_day, input.due_day, id],
            ).map_err(|e| e.to_string())?;
        }
        None => {
            c.execute(
                "INSERT INTO credit_cards (account_id, name, credit_limit_minor, statement_day, due_day, created_at) VALUES (?1,?2,?3,?4,?5,?6)",
                params![input.account_id, input.name, input.credit_limit_minor, input.statement_day, input.due_day, now_secs()],
            ).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_card_cmd(state: State<Db>, id: i64) -> Res<()> {
    let c = conn(&state);
    c.execute("DELETE FROM credit_cards WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn list_installments_cmd(state: State<Db>) -> Res<Vec<Installment>> {
    let c = conn(&state);
    let now = now_secs();
    let mut stmt = c
        .prepare("SELECT id, account_id, name, total_minor, currency, months, started_at, note FROM installments ORDER BY id")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, i64>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, i64>(5)?,
                r.get::<_, i64>(6)?,
                r.get::<_, String>(7)?,
            ))
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        let (id, account_id, name, total, currency, months, started_at, note) = r.map_err(|e| e.to_string())?;
        let monthly = if months > 0 { total / months } else { total };
        let elapsed_months = (now - started_at) / (30 * 86400);
        let remaining_months = (months - elapsed_months).max(0);
        let remaining = remaining_months * monthly;
        out.push(Installment {
            id,
            account_id,
            name,
            total_minor: total,
            currency,
            months,
            started_at,
            note,
            monthly_minor: monthly,
            remaining_months,
            remaining_minor: remaining,
        });
    }
    Ok(out)
}

#[tauri::command]
pub fn save_installment_cmd(state: State<Db>, input: InstallmentInput) -> Res<()> {
    let c = conn(&state);
    match input.id {
        Some(id) => {
            c.execute(
                "UPDATE installments SET account_id=?1, name=?2, total_minor=?3, currency=?4, months=?5, started_at=?6, note=?7 WHERE id=?8",
                params![input.account_id, input.name, input.total_minor, input.currency, input.months, input.started_at, input.note, id],
            ).map_err(|e| e.to_string())?;
        }
        None => {
            c.execute(
                "INSERT INTO installments (account_id, name, total_minor, currency, months, started_at, note, created_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
                params![input.account_id, input.name, input.total_minor, input.currency, input.months, input.started_at, input.note, now_secs()],
            ).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn delete_installment_cmd(state: State<Db>, id: i64) -> Res<()> {
    let c = conn(&state);
    c.execute("DELETE FROM installments WHERE id=?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Reports
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn report_summary_cmd(state: State<Db>, from: i64, to: i64) -> Res<ReportSummary> {
    let c = conn(&state);
    let row = c
        .query_row(
            "SELECT COALESCE(SUM(CASE WHEN kind='income' THEN amount_base_minor ELSE 0 END),0),
                    COALESCE(SUM(CASE WHEN kind='expense' THEN amount_base_minor ELSE 0 END),0),
                    COUNT(*)
             FROM transactions WHERE occurred_at>=?1 AND occurred_at<=?2",
            params![from, to],
            |r| {
                Ok(ReportSummary {
                    income_minor: r.get(0)?,
                    expense_minor: r.get(1)?,
                    net_minor: 0,
                    tx_count: r.get(2)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;
    Ok(ReportSummary {
        net_minor: row.income_minor - row.expense_minor,
        ..row
    })
}

#[tauri::command]
pub fn report_series_cmd(state: State<Db>, from: i64, to: i64) -> Res<Vec<SeriesPoint>> {
    let c = conn(&state);
    let mut stmt = c
        .prepare(
            "SELECT occurred_at, kind, amount_base_minor FROM transactions WHERE occurred_at>=?1 AND occurred_at<=?2 ORDER BY occurred_at",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(params![from, to], |r| {
            Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?, r.get::<_, i64>(2)?))
        })
        .map_err(|e| e.to_string())?;
    let mut map: std::collections::BTreeMap<i64, (i64, i64)> = std::collections::BTreeMap::new();
    for r in rows {
        let (ts, kind, amt) = r.map_err(|e| e.to_string())?;
        let day = local_day(ts);
        let e = map.entry(day).or_insert((0, 0));
        match kind.as_str() {
            "income" => e.0 += amt,
            _ => e.1 += amt,
        }
    }
    Ok(map
        .into_iter()
        .map(|(d, (inc, exp))| SeriesPoint {
            date: d,
            income: inc,
            expense: exp,
            net: inc - exp,
        })
        .collect())
}

fn local_day(ts: i64) -> i64 {
    use chrono::TimeZone;
    let local = chrono::Local.timestamp_opt(ts, 0).single();
    match local {
        Some(dt) => {
            let midnight = dt
                .date_naive()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(chrono::Local)
                .unwrap()
                .timestamp();
            midnight
        }
        None => ts - (ts % 86400),
    }
}

#[tauri::command]
pub fn report_by_category_cmd(state: State<Db>, from: i64, to: i64, kind: String) -> Res<Vec<CategoryTotal>> {
    let c = conn(&state);
    let mut stmt = c
        .prepare(
            "SELECT t.category_id, COALESCE(c.name,'?'), COALESCE(c.icon,'dot'), COALESCE(c.color,''), SUM(t.amount_base_minor)
             FROM transactions t LEFT JOIN categories c ON c.id=t.category_id
             WHERE t.occurred_at>=?1 AND t.occurred_at<=?2 AND t.kind=?3
             GROUP BY t.category_id, c.name, c.icon, c.color ORDER BY SUM(t.amount_base_minor) DESC",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(params![from, to, kind], |r| {
            Ok(CategoryTotal {
                category_id: r.get(0)?,
                name: r.get(1)?,
                icon: r.get(2)?,
                color: r.get(3)?,
                total: r.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

#[tauri::command]
pub fn report_largest_expense_cmd(state: State<Db>, from: i64, to: i64, limit: i64) -> Res<Vec<LargestExpense>> {
    let c = conn(&state);
    let mut stmt = c
        .prepare(
            "SELECT id, payee, amount_minor, currency, amount_base_minor, occurred_at FROM transactions WHERE kind='expense' AND occurred_at>=?1 AND occurred_at<=?2 ORDER BY amount_base_minor DESC LIMIT ?3",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(params![from, to, limit], |r| {
            Ok(LargestExpense {
                id: r.get(0)?,
                payee: r.get(1)?,
                amount_minor: r.get(2)?,
                currency: r.get(3)?,
                amount_base_minor: r.get(4)?,
                occurred_at: r.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// FX
// ---------------------------------------------------------------------------
#[tauri::command]
pub async fn fetch_fx_rate_cmd(base: String, quote: String) -> Res<FxRate> {
    fx::fetch_fx_rate(&base, &quote).await
}

// ---------------------------------------------------------------------------
// Export (frontend supplies localized headers + rows)
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn export_xlsx_cmd(path: String, headers: Vec<String>, rows: Vec<Vec<String>>) -> Res<()> {
    export::write_xlsx(&path, &headers, &rows)
}

#[tauri::command]
pub fn export_csv_cmd(path: String, headers: Vec<String>, rows: Vec<Vec<String>>) -> Res<()> {
    export::write_csv(&path, &headers, &rows)
}

// ---------------------------------------------------------------------------
// Backup / restore
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn backup_db_cmd(state: State<Db>, path: String, password: Option<String>) -> Res<()> {
    let c = conn(&state);
    let bytes = backup::snapshot_bytes(&c)?;
    let data = match password {
        Some(p) if !p.is_empty() => backup::encrypt(&bytes, &p)?,
        _ => bytes,
    };
    std::fs::write(&path, data).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn restore_db_cmd(state: State<Db>, path: String, password: Option<String>) -> Res<()> {
    let data = std::fs::read(&path).map_err(|e| e.to_string())?;
    let bytes = match password {
        Some(p) if !p.is_empty() => backup::decrypt(&data, &p)?,
        _ => data,
    };
    let mut c = conn(&state);
    backup::restore_into(&mut c, &bytes)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Misc
// ---------------------------------------------------------------------------
#[tauri::command]
pub fn parse_amount_cmd(s: String, code: String) -> Res<i64> {
    money::parse_amount_to_minor(&s, &code).ok_or_else(|| "invalid amount".to_string())
}
