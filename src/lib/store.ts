import { reactive, ref, computed } from "vue";
import { api, Settings } from "./api";

export const settings = ref<Settings>({
  baseCurrency: "TRY",
  language: "tr",
  theme: "system",
  monthStartDay: 1,
  weekStart: 1,
  dateFormat: "yyyy-MM-dd",
});

export const loaded = ref(false);

export async function loadSettings() {
  try {
    settings.value = await api.getSettings();
  } catch {
    // defaults already set
  }
  applyTheme(settings.value.theme);
  loaded.value = true;
}

export async function saveSettings(s: Settings) {
  settings.value = await api.saveSettings(s);
  applyTheme(settings.value.theme);
}

export const language = computed(() => settings.value.language);
export const baseCurrency = computed(() => settings.value.baseCurrency);
export const locale = computed(() => {
  const m: Record<string, string> = {
    tr: "tr-TR",
    en: "en-US",
    ru: "ru-RU",
    de: "de-DE",
    fr: "fr-FR",
    es: "es-ES",
  };
  return m[settings.value.language] || "en-US";
});

export function applyTheme(theme: string) {
  const prefersDark = window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;
  const dark = theme === "dark" || (theme === "system" && prefersDark);
  document.documentElement.setAttribute("data-theme", dark ? "dark" : "light");
}

export function watchSystemTheme() {
  if (window.matchMedia) {
    window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
      applyTheme(settings.value.theme);
    });
  }
}

export const toastMsg = ref("");
export function toast(msg: string) {
  toastMsg.value = msg;
  setTimeout(() => {
    if (toastMsg.value === msg) toastMsg.value = "";
  }, 2500);
}

// Helper to produce a reactive copy of nothing (for re-renders after edits)
export const version = reactive({ n: 0 });
export function bump() {
  version.n++;
}
