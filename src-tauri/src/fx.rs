use crate::models::{CryptoPrice, FxRate};
use crate::money::RATE_SCALE;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

type Cache = Mutex<HashMap<String, (i64, f64)>>;

fn cache() -> &'static Cache {
    static C: OnceLock<Cache> = OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}

fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}

fn cached(key: &str, ttl: i64) -> Option<f64> {
    let m = cache().lock().unwrap();
    m.get(key).and_then(|(t, v)| if now() - *t < ttl { Some(*v) } else { None })
}

fn set_cache(key: &str, v: f64) {
    let mut m = cache().lock().unwrap();
    m.insert(key.to_string(), (now(), v));
}

async fn get_json(url: &str) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client
        .get(url)
        .header("User-Agent", "Turgite/0.1 (+https://github.com/taliyigit2-prog/Turgite)")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }
    resp.json::<serde_json::Value>().await.map_err(|e| e.to_string())
}

const FX_CDN_1: &str = "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1/currencies";
const FX_CDN_2: &str = "https://latest.currency-api.pages.dev/v1/currencies";

/// Fetch the exchange rate: 1 unit of `base` = `rate` units of `quote`.
pub async fn fetch_fx_rate(base: &str, quote: &str) -> Result<FxRate, String> {
    let b = base.to_ascii_lowercase();
    let q = quote.to_ascii_lowercase();
    if b == q {
        return Ok(FxRate {
            base: base.to_uppercase(),
            quote: quote.to_uppercase(),
            rate: 1.0,
            rate_scaled: RATE_SCALE,
            date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            source: "identity".into(),
        });
    }
    let key = format!("{}|{}", b, q);
    if let Some(v) = cached(&key, 3600) {
        return Ok(make_fx(base, quote, v, "cache"));
    }
    let mut last_err = String::new();
    for cdn in [FX_CDN_1, FX_CDN_2] {
        let url = format!("{}/{}.json", cdn, b);
        match get_json(&url).await {
            Ok(v) => {
                if let Some(rate) = v.get(&b).and_then(|m| m.get(&q)).and_then(|x| x.as_f64()) {
                    set_cache(&key, rate);
                    let date = v
                        .get("date")
                        .and_then(|x| x.as_str())
                        .unwrap_or("")
                        .to_string();
                    return Ok(FxRate {
                        base: base.to_uppercase(),
                        quote: quote.to_uppercase(),
                        rate,
                        rate_scaled: (rate * RATE_SCALE as f64).round() as i64,
                        date,
                        source: "currency-api".into(),
                    });
                }
                last_err = format!("{} not found", quote);
            }
            Err(e) => last_err = e,
        }
    }
    Err(last_err)
}

fn make_fx(base: &str, quote: &str, rate: f64, source: &str) -> FxRate {
    FxRate {
        base: base.to_uppercase(),
        quote: quote.to_uppercase(),
        rate,
        rate_scaled: (rate * RATE_SCALE as f64).round() as i64,
        date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        source: source.into(),
    }
}

const CG_SIMPLE: &str = "https://api.coingecko.com/api/v3/simple/price";
const CG_CHART: &str = "https://api.coingecko.com/api/v3/coins";

/// Fetch current crypto price in base currency. Returns minor units.
pub async fn fetch_crypto_price(coin_id: &str, base: &str) -> Result<CryptoPrice, String> {
    let url = format!(
        "{}?ids={}&vs_currencies={}",
        CG_SIMPLE,
        coin_id.to_ascii_lowercase(),
        base.to_ascii_lowercase()
    );
    let v = get_json(&url).await?;
    let price = v
        .get(coin_id.to_ascii_lowercase())
        .and_then(|m| m.get(base.to_ascii_lowercase()))
        .and_then(|x| x.as_f64())
        .ok_or_else(|| "price not found".to_string())?;
    let exp = crate::money::exponent(base);
    let minor = (price * 10f64.powi(exp as i32)).round() as i64;
    Ok(CryptoPrice {
        coin_id: coin_id.to_string(),
        price,
        price_base_minor: minor,
    })
}

/// Fetch a price history series (timestamp, price) for a coin.
pub async fn fetch_crypto_chart(coin_id: &str, base: &str, days: u32) -> Result<Vec<(i64, f64)>, String> {
    let url = format!(
        "{}/{}/market_chart?vs_currency={}&days={}",
        CG_CHART,
        coin_id.to_ascii_lowercase(),
        base.to_ascii_lowercase(),
        days
    );
    let v = get_json(&url).await?;
    let prices = v
        .get("prices")
        .and_then(|x| x.as_array())
        .ok_or_else(|| "no price history".to_string())?;
    let out: Vec<(i64, f64)> = prices
        .iter()
        .filter_map(|p| {
            let ts = p.get(0)?.as_f64()? as i64 / 1000;
            let px = p.get(1)?.as_f64()?;
            Some((ts, px))
        })
        .collect();
    Ok(out)
}
