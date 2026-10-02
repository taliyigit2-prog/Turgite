import { readFileSync, writeFileSync } from "node:fs";

const raw = JSON.parse(readFileSync("/tmp/currencies.json", "utf8"));

const ISO = [
  "AED","AFN","ALL","AMD","ANG","AOA","ARS","AUD","AWG","AZN","BAM","BBD","BDT","BGN","BHD","BIF","BMD","BND","BOB","BRL","BSD","BTN","BWP","BYN","BZD","CAD","CDF","CHF","CLP","CNY","COP","CRC","CUP","CVE","CZK","DJF","DKK","DOP","DZD","EGP","ERN","ETB","EUR","FJD","FKP","GBP","GEL","GGP","GHS","GIP","GMD","GNF","GTQ","GYD","HKD","HNL","HRK","HTG","HUF","IDR","ILS","IMP","INR","IQD","IRR","ISK","JEP","JMD","JOD","JPY","KES","KGS","KHR","KMF","KPW","KRW","KWD","KYD","KZT","LAK","LBP","LKR","LRD","LSL","LYD","MAD","MDL","MGA","MKD","MMK","MNT","MOP","MRU","MUR","MVR","MWK","MXN","MYR","MZN","NAD","NGN","NIO","NOK","NPR","NZD","OMR","PAB","PEN","PGK","PHP","PKR","PLN","PYG","QAR","RON","RSD","RUB","RWF","SAR","SBD","SCR","SDG","SEK","SGD","SHP","SLE","SLL","SOS","SRD","SSP","STN","SYP","SZL","THB","TJS","TMT","TND","TOP","TRY","TTD","TVD","TWD","TZS","UAH","UGX","USD","UYU","UZS","VES","VND","VUV","WST","XAF","XCD","XOF","XPF","YER","ZAR","ZMW","ZWL"
];

const SYM = {
  USD:"$", EUR:"€", TRY:"₺", RUB:"₽", GBP:"£", JPY:"¥", CNY:"¥", KRW:"₩", INR:"₹",
  AZN:"₼", BGN:"лв", BRL:"R$", CAD:"C$", CHF:"CHF", CZK:"Kč", DKK:"kr", GEL:"₾",
  HKD:"HK$", HUF:"Ft", IDR:"Rp", ILS:"₪", KZT:"₸", MXN:"Mex$", MYR:"RM", NOK:"kr",
  NZD:"NZ$", PHP:"₱", PLN:"zł", RON:"lei", RSD:"дин", SAR:"﷼", SEK:"kr", SGD:"S$",
  THB:"฿", TWD:"NT$", UAH:"₴", VND:"₫", ZAR:"R", AED:"د.إ", ARS:"AR$", CLP:"CLP$",
  COP:"CO$", EGP:"E£", NGN:"₦", PKR:"₨", QAR:"QR", TND:"DT", BDT:"৳", LKR:"Rs", NPR:"₨"
};

const ZERO = new Set(["BIF","CLP","DJF","GNF","ISK","JPY","KMF","KRW","PYG","RWF","UGX","VND","VUV","XAF","XOF","XPF"]);
const THREE = new Set(["BHD","IQD","JOD","KWD","LYD","OMR","TND"]);

const rows = ISO.map((code) => {
  const name = raw[code.toLowerCase()] || code;
  const symbol = SYM[code] || "";
  const exponent = ZERO.has(code) ? 0 : THREE.has(code) ? 3 : 2;
  return `  { code: "${code}", name: ${JSON.stringify(name)}, symbol: ${JSON.stringify(symbol)}, exponent: ${exponent} },`;
});

const out = `// ISO 4217 currency catalogue used across Turgite.
// Names sourced from the currency-api catalogue (public data).
export interface Currency {
  code: string;
  name: string;
  symbol: string;
  exponent: number; // number of minor-unit digits
}

export const CURRENCIES: Currency[] = [
${rows.join("\n")}
];

export function currencyByCode(code: string): Currency | undefined {
  const c = code.toUpperCase();
  return CURRENCIES.find((x) => x.code === c);
}

export function currencySymbol(code: string): string {
  return currencyByCode(code)?.symbol || code.toUpperCase();
}

export function currencyExponent(code: string): number {
  return currencyByCode(code)?.exponent ?? 2;
}
`;

writeFileSync("src/lib/currencies.ts", out);
console.log("Wrote", ISO.length, "currencies");
