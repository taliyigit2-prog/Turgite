import { currencyByCode, currencySymbol } from "./currencies";

export function exponentOf(code: string): number {
  return currencyByCode(code)?.exponent ?? 2;
}

/** Convert minor units to a plain number string ("12.34"). */
export function minorToString(minor: number, code: string): string {
  const exp = exponentOf(code);
  const factor = Math.pow(10, exp);
  const neg = minor < 0;
  const abs = Math.abs(minor);
  const whole = Math.floor(abs / factor);
  const frac = Math.round(abs % factor);
  if (exp === 0) return (neg ? "-" : "") + whole.toString();
  return (neg ? "-" : "") + whole.toString() + "." + frac.toString().padStart(exp, "0");
}

/** Parse a user-entered amount string into minor units. */
export function parseAmount(s: string, code: string): number | null {
  const exp = exponentOf(code);
  const t = s.trim().replace(",", ".");
  if (t === "") return null;
  const parts = t.split(".");
  if (parts.length > 2) return null;
  const whole = parseInt(parts[0], 10);
  if (isNaN(whole)) return null;
  let frac = 0;
  if (parts.length === 2) {
    const fs = parts[1];
    if (fs.length > exp) return null;
    if (fs !== "") {
      frac = parseInt(fs.padEnd(exp, "0"), 10);
      if (isNaN(frac)) return null;
    }
  }
  const factor = Math.pow(10, exp);
  return whole * factor + frac;
}

export function formatMoney(minor: number, code: string, locale: string): string {
  const sym = currencySymbol(code);
  const num = minor / Math.pow(10, exponentOf(code));
  const formatted = new Intl.NumberFormat(locale, {
    minimumFractionDigits: exponentOf(code),
    maximumFractionDigits: exponentOf(code),
  }).format(num);
  return `${formatted} ${sym}`.trim();
}

export function formatMoneyWithCode(minor: number, code: string, locale: string): string {
  const num = minor / Math.pow(10, exponentOf(code));
  const formatted = new Intl.NumberFormat(locale, {
    minimumFractionDigits: exponentOf(code),
    maximumFractionDigits: exponentOf(code),
  }).format(num);
  return `${formatted} ${code.toUpperCase()}`.trim();
}

const localeMap: Record<string, string> = {
  tr: "tr-TR",
  en: "en-US",
  ru: "ru-RU",
  de: "de-DE",
  fr: "fr-FR",
  es: "es-ES",
};

export function intlLocale(lang: string): string {
  return localeMap[lang] || "en-US";
}

export function formatDate(ts: number, lang: string): string {
  return new Intl.DateTimeFormat(intlLocale(lang), {
    year: "numeric",
    month: "short",
    day: "numeric",
  }).format(new Date(ts * 1000));
}

export function formatDateTime(ts: number, lang: string): string {
  return new Intl.DateTimeFormat(intlLocale(lang), {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(new Date(ts * 1000));
}

export function toISODate(ts: number): string {
  return new Date(ts * 1000).toISOString().slice(0, 10);
}

export function toISODateLocal(ts: number): string {
  const d = new Date(ts * 1000);
  const y = d.getFullYear();
  const m = String(d.getMonth() + 1).padStart(2, "0");
  const day = String(d.getDate()).padStart(2, "0");
  return `${y}-${m}-${day}`;
}

export function fromISODate(iso: string): number {
  const [y, m, d] = iso.split("-").map((x) => parseInt(x, 10));
  return Math.floor(new Date(y, (m || 1) - 1, d || 1).getTime() / 1000);
}

export function convertAmount(amountMinor: number, from: string, to: string, rateScaled: number): number {
  const rate = rateScaled / 1e8;
  const fe = exponentOf(from);
  const te = exponentOf(to);
  return Math.round((amountMinor * rate * Math.pow(10, te)) / Math.pow(10, fe));
}

export function nowSecs(): number {
  return Math.floor(Date.now() / 1000);
}

export function startOfMonth(): number {
  const d = new Date();
  return Math.floor(new Date(d.getFullYear(), d.getMonth(), 1).getTime() / 1000);
}

export function startOfYear(): number {
  const d = new Date();
  return Math.floor(new Date(d.getFullYear(), 0, 1).getTime() / 1000);
}

export function endOfMonth(): number {
  const d = new Date();
  return Math.floor(new Date(d.getFullYear(), d.getMonth() + 1, 1).getTime() / 1000) - 1;
}

/** Returns { from, to } for a month offset (0 = current, -1 = previous, ...). */
export function monthBounds(offset = 0): { from: number; to: number } {
  const d = new Date();
  const y = d.getFullYear();
  const m = d.getMonth() + offset;
  const from = new Date(y, m, 1).getTime() / 1000;
  const to = new Date(y, m + 1, 1).getTime() / 1000 - 1;
  return { from: Math.floor(from), to: Math.floor(to) };
}

export function daysAgo(days: number): number {
  const d = new Date();
  d.setDate(d.getDate() - days);
  d.setHours(0, 0, 0, 0);
  return Math.floor(d.getTime() / 1000);
}
