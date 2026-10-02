//! Currency helpers: ISO 4217 minor-unit exponents, integer money math and
//! formatting. All stored amounts are integers in the currency's minor unit
//! (kuruş/cents/kopek). Exchange rates are stored scaled by 1e8 to stay exact.

/// Minor-unit exponent (number of decimal digits) for a currency code.
pub fn exponent(code: &str) -> u32 {
    match code.to_ascii_uppercase().as_str() {
        "BIF" | "CLP" | "DJF" | "GNF" | "ISK" | "JPY" | "KMF" | "KRW" | "PYG"
        | "RWF" | "UGX" | "VND" | "VUV" | "XAF" | "XOF" | "XPF" => 0,
        "BHD" | "IQD" | "JOD" | "KWD" | "LYD" | "OMR" | "TND" => 3,
        _ => 2,
    }
}

/// 1e8 — the scale we store exchange rates at.
pub const RATE_SCALE: i64 = 100_000_000;

/// Convert an amount in `currency` minor units into base-currency minor units
/// using `rate_scaled` (units of base per unit of foreign, * 1e8).
/// `amount_base = round(amount_minor * rate / 10^exponent_foreign * 10^exponent_base)`.
pub fn to_base_minor(amount_minor: i64, currency: &str, base: &str, rate_scaled: i64) -> i64 {
    let fexp = exponent(currency) as u32;
    let bexp = exponent(base) as u32;
    // numerator = amount_minor * rate_scaled * 10^bexp
    // denominator = 10^fexp * 10^8
    let num = (amount_minor as i128) * (rate_scaled as i128) * 10i128.pow(bexp);
    let den = 10i128.pow(fexp) * (RATE_SCALE as i128);
    ((num + den / 2) / den) as i64
}

/// Round an amount string ("12.34") to minor units for a currency.
/// Returns None on parse failure. Accepts comma or dot decimal separators.
pub fn parse_amount_to_minor(s: &str, code: &str) -> Option<i64> {
    let exp = exponent(code) as u32;
    let t = s.trim().replace(',', ".");
    if t.is_empty() {
        return None;
    }
    let parts: Vec<&str> = t.split('.').collect();
    if parts.len() > 2 {
        return None;
    }
    let whole: i64 = parts[0].parse().ok()?;
    let mut frac: i64 = 0;
    if parts.len() == 2 {
        let fs = parts[1];
        if fs.len() > exp as usize {
            return None;
        }
        if !fs.is_empty() {
            frac = fs.parse().ok()?;
        }
    }
    let factor = 10i64.pow(exp);
    Some(whole * factor + frac)
}

/// Format minor units as a human string, without a currency symbol.
#[allow(dead_code)]
pub fn format_minor(minor: i64, code: &str) -> String {
    let exp = exponent(code) as u32;
    let factor = 10i64.pow(exp);
    let neg = minor < 0;
    let abs = minor.abs();
    let whole = abs / factor;
    let frac = abs % factor;
    let mut s = if neg { "-".to_string() } else { String::new() };
    if exp == 0 {
        s.push_str(&format!("{}", whole));
    } else {
        s.push_str(&format!("{}.{:0width$}", whole, frac, width = exp as usize));
    }
    s
}

/// Format minor units with thousands separators (no symbol).
#[allow(dead_code)]
pub fn format_minor_grouped(minor: i64, code: &str) -> String {
    let exp = exponent(code) as u32;
    let factor = 10i64.pow(exp);
    let neg = minor < 0;
    let abs = minor.abs();
    let whole = abs / factor;
    let frac = abs % factor;
    let grouped = whole
        .to_string()
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(",")
        .chars()
        .rev()
        .collect::<String>();
    let mut s = if neg { "-".to_string() } else { String::new() };
    if exp == 0 {
        s.push_str(&grouped);
    } else {
        s.push_str(&format!("{}.{:0width$}", grouped, frac, width = exp as usize));
    }
    s
}
