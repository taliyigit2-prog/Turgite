<script setup lang="ts">
import { ref, onMounted, watch, computed } from "vue";
import { useI18n } from "vue-i18n";
import Icon from "./components/Icon.vue";
import { loadSettings, settings, language, toastMsg, watchSystemTheme } from "./lib/store";
import { locale } from "./lib/store";

import Overview from "./views/Overview.vue";
import Transactions from "./views/Transactions.vue";
import Accounts from "./views/Accounts.vue";
import Categories from "./views/Categories.vue";
import Budgets from "./views/Budgets.vue";
import Recurring from "./views/Recurring.vue";
import Goals from "./views/Goals.vue";
import Debts from "./views/Debts.vue";
import Reports from "./views/Reports.vue";
import Cards from "./views/Cards.vue";
import Crypto from "./views/Crypto.vue";
import Rates from "./views/Rates.vue";
import SettingsView from "./views/Settings.vue";

const { t, locale: i18nLocale } = useI18n();

const view = ref("overview");

const navItems = [
  { key: "overview", icon: "overview" },
  { key: "transactions", icon: "transactions" },
  { key: "accounts", icon: "accounts" },
  { key: "categories", icon: "categories" },
  { key: "budgets", icon: "budgets" },
  { key: "recurring", icon: "recurring" },
  { key: "goals", icon: "goals" },
  { key: "debts", icon: "debts" },
  { key: "reports", icon: "reports" },
  { key: "cards", icon: "cards" },
  { key: "crypto", icon: "crypto" },
  { key: "rates", icon: "rates" },
  { key: "settings", icon: "settings" },
];

const components: Record<string, any> = {
  overview: Overview,
  transactions: Transactions,
  accounts: Accounts,
  categories: Categories,
  budgets: Budgets,
  recurring: Recurring,
  goals: Goals,
  debts: Debts,
  reports: Reports,
  cards: Cards,
  crypto: Crypto,
  rates: Rates,
  settings: SettingsView,
};

watch(language, (l) => {
  i18nLocale.value = l;
  document.documentElement.lang = l;
});

watch(locale, (l) => {
  document.documentElement.lang = l;
});

onMounted(async () => {
  await loadSettings();
  i18nLocale.value = language.value;
  watchSystemTheme();
});

const currentComponent = computed(() => components[view.value]);
</script>

<template>
  <div class="app-shell">
    <aside class="sidebar">
      <div class="brand">
        <span class="logo-dot">T</span>
        {{ t("app.name") }}
      </div>
      <nav class="nav">
        <button
          v-for="item in navItems"
          :key="item.key"
          class="nav-item"
          :class="{ active: view === item.key }"
          @click="view = item.key"
        >
          <Icon :name="item.icon" />
          <span>{{ t("nav." + item.key) }}</span>
        </button>
      </nav>
    </aside>

    <main class="main">
      <component :is="currentComponent" />
    </main>

    <transition name="fade">
      <div v-if="toastMsg" class="toast">{{ toastMsg }}</div>
    </transition>
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
