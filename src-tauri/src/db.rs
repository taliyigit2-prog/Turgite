use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Db(pub Mutex<Connection>);

pub fn app_data_dir() -> PathBuf {
    let base = dirs_like();
    let dir = base.join("com.turgite.app");
    std::fs::create_dir_all(&dir).ok();
    dir
}

// Minimal portable home/app-data resolution without pulling extra crates.
fn dirs_like() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        let p = PathBuf::from(home);
        #[cfg(target_os = "macos")]
        return p.join("Library").join("Application Support");
        #[cfg(target_os = "windows")]
        if let Ok(ad) = std::env::var("APPDATA") {
            return PathBuf::from(ad);
        }
        #[allow(unreachable_code)]
        p.join(".local").join("share")
    } else {
        PathBuf::from(".")
    }
}

pub fn init() -> Result<Db, String> {
    let dir = app_data_dir();
    let path = dir.join("turgite.db");
    let conn = Connection::open(&path).map_err(|e| e.to_string())?;
    conn.execute_batch("PRAGMA journal_mode=WAL;").map_err(|e| e.to_string())?;
    conn.execute_batch("PRAGMA foreign_keys=ON;").map_err(|e| e.to_string())?;
    migrate(&conn)?;
    Ok(Db(Mutex::new(conn)))
}

fn migrate(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
          id INTEGER PRIMARY KEY CHECK (id = 1),
          base_currency TEXT NOT NULL DEFAULT 'TRY',
          language TEXT NOT NULL DEFAULT 'tr',
          theme TEXT NOT NULL DEFAULT 'system',
          month_start_day INTEGER NOT NULL DEFAULT 1,
          week_start INTEGER NOT NULL DEFAULT 1,
          date_format TEXT NOT NULL DEFAULT 'yyyy-MM-dd',
          updated_at INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS accounts (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT NOT NULL,
          kind TEXT NOT NULL DEFAULT 'cash'
            CHECK (kind IN ('cash','bank','ewallet','credit_card','savings')),
          currency TEXT NOT NULL,
          opening_balance_minor INTEGER NOT NULL DEFAULT 0,
          icon TEXT NOT NULL DEFAULT 'wallet',
          color TEXT NOT NULL DEFAULT '',
          sort_order INTEGER NOT NULL DEFAULT 0,
          created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS categories (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT NOT NULL,
          kind TEXT NOT NULL CHECK (kind IN ('expense','income')),
          parent_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
          icon TEXT NOT NULL DEFAULT 'dot',
          color TEXT NOT NULL DEFAULT '',
          keywords TEXT NOT NULL DEFAULT '',
          sort_order INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS goals (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT NOT NULL,
          target_minor INTEGER NOT NULL,
          currency TEXT NOT NULL,
          target_date INTEGER,
          account_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
          color TEXT NOT NULL DEFAULT '',
          note TEXT NOT NULL DEFAULT '',
          icon TEXT NOT NULL DEFAULT '',
          created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS recurring_rules (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT NOT NULL,
          kind TEXT NOT NULL CHECK (kind IN ('expense','income','transfer')),
          account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
          dest_account_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
          category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
          amount_minor INTEGER NOT NULL,
          currency TEXT NOT NULL,
          payee TEXT NOT NULL DEFAULT '',
          note TEXT NOT NULL DEFAULT '',
          freq TEXT NOT NULL CHECK (freq IN ('daily','weekly','monthly','yearly')),
          interval_n INTEGER NOT NULL DEFAULT 1,
          next_run_at INTEGER NOT NULL,
          end_at INTEGER,
          auto_post INTEGER NOT NULL DEFAULT 0,
          active INTEGER NOT NULL DEFAULT 1,
          created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS transactions (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          kind TEXT NOT NULL CHECK (kind IN ('expense','income','transfer')),
          account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
          dest_account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE,
          category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
          amount_minor INTEGER NOT NULL CHECK (amount_minor >= 0),
          currency TEXT NOT NULL,
          amount_base_minor INTEGER NOT NULL DEFAULT 0,
          rate_scaled INTEGER NOT NULL DEFAULT 0,
          occurred_at INTEGER NOT NULL,
          payee TEXT NOT NULL DEFAULT '',
          note TEXT NOT NULL DEFAULT '',
          goal_id INTEGER REFERENCES goals(id) ON DELETE SET NULL,
          recurring_id INTEGER REFERENCES recurring_rules(id) ON DELETE SET NULL,
          source TEXT NOT NULL DEFAULT 'manual'
            CHECK (source IN ('manual','csv','text','recurring')),
          receipt_path TEXT NOT NULL DEFAULT '',
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS tags (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS transaction_tags (
          transaction_id INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
          tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
          PRIMARY KEY (transaction_id, tag_id)
        );

        CREATE TABLE IF NOT EXISTS budgets (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          category_id INTEGER REFERENCES categories(id) ON DELETE CASCADE,
          period TEXT NOT NULL DEFAULT 'monthly'
            CHECK (period IN ('weekly','monthly','yearly')),
          amount_base_minor INTEGER NOT NULL,
          start_at INTEGER NOT NULL,
          rollover INTEGER NOT NULL DEFAULT 0,
          active INTEGER NOT NULL DEFAULT 1,
          created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS debts (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          counterpart TEXT NOT NULL,
          direction TEXT NOT NULL CHECK (direction IN ('lent','owed')),
          principal_minor INTEGER NOT NULL,
          currency TEXT NOT NULL,
          account_id INTEGER REFERENCES accounts(id) ON DELETE SET NULL,
          due_at INTEGER,
          settled_at INTEGER,
          note TEXT NOT NULL DEFAULT '',
          created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS debt_payments (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          debt_id INTEGER NOT NULL REFERENCES debts(id) ON DELETE CASCADE,
          amount_minor INTEGER NOT NULL,
          paid_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS crypto_assets (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          coin_id TEXT NOT NULL,
          symbol TEXT NOT NULL,
          name TEXT NOT NULL,
          amount REAL NOT NULL DEFAULT 0,
          cost_basis_base_minor INTEGER NOT NULL DEFAULT 0,
          created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS credit_cards (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
          name TEXT NOT NULL,
          credit_limit_minor INTEGER NOT NULL DEFAULT 0,
          statement_day INTEGER NOT NULL DEFAULT 1,
          due_day INTEGER NOT NULL DEFAULT 10,
          created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS installments (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
          name TEXT NOT NULL,
          total_minor INTEGER NOT NULL,
          currency TEXT NOT NULL,
          months INTEGER NOT NULL DEFAULT 1,
          started_at INTEGER NOT NULL,
          note TEXT NOT NULL DEFAULT '',
          created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS fx_cache (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          base TEXT NOT NULL,
          quote TEXT NOT NULL,
          rate REAL NOT NULL,
          date TEXT NOT NULL,
          updated_at INTEGER NOT NULL,
          UNIQUE(base, quote, date)
        );

        INSERT INTO settings (id) SELECT 1 WHERE NOT EXISTS (SELECT 1 FROM settings);
        "#,
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn now_secs() -> i64 {
    chrono::Utc::now().timestamp()
}
