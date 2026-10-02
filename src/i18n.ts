import { createI18n } from "vue-i18n";
import { messages, LocaleCode } from "./locales";

export const i18n = createI18n({
  legacy: false,
  locale: "tr",
  fallbackLocale: "en",
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  messages: messages as any,
});
