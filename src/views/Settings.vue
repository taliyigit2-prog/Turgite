<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { save as saveDialog, open as openDialog } from "@tauri-apps/plugin-dialog";
import Icon from "../components/Icon.vue";
import CurrencySelect from "../components/CurrencySelect.vue";
import { api } from "../lib/api";
import { settings, saveSettings, toast } from "../lib/store";
import { formatMoney, formatDate, intlLocale, minorToString } from "../lib/format";

const { t, locale } = useI18n();

const languages = ["tr", "en", "ru", "de", "fr", "es"];
const themes = ["system", "light", "dark"];
const dateFormats = ["yyyy-MM-dd", "dd/MM/yyyy", "MM/dd/yyyy"];
const weekDays = [0, 1, 2, 3, 4, 5, 6];

const password = ref("");

onMounted(() => {
  locale.value = settings.value.language;
});

async function update(field: string, value: unknown) {
  await saveSettings({ ...settings.value, [field]: value });
}

async function buildExportRows() {
  const txns = await api.listTransactions({ limit: 100000 });
  const base = settings.value.baseCurrency;
  const headers = [
    t("export.date"),
    t("export.kind"),
    t("export.amount"),
    t("export.currency"),
    t("export.baseAmount"),
    t("export.category"),
    t("export.account"),
    t("export.payee"),
    t("export.note"),
    t("export.tags"),
  ];
  const rows = txns.map((tx) => [
    formatDate(tx.occurredAt, settings.value.language),
    t("txn." + tx.kind),
    minorToString(tx.amountMinor, tx.currency),
    tx.currency,
    minorToString(tx.amountBaseMinor, base),
    tx.categoryName || "",
    tx.accountName,
    tx.payee,
    tx.note,
    tx.tags.join(", "),
  ]);
  return { headers, rows };
}

async function doExportXlsx() {
  try {
    const path = await saveDialog({
      defaultPath: "turgite-transactions.xlsx",
      filters: [{ name: "Excel", extensions: ["xlsx"] }],
    });
    if (!path) return;
    const { headers, rows } = await buildExportRows();
    await api.exportXlsx(path, headers, rows);
    toast(t("settings.backupSuccess"));
  } catch {
    toast(t("error.exportFailed"));
  }
}

async function doExportCsv() {
  try {
    const path = await saveDialog({
      defaultPath: "turgite-transactions.csv",
      filters: [{ name: "CSV", extensions: ["csv"] }],
    });
    if (!path) return;
    const { headers, rows } = await buildExportRows();
    await api.exportCsv(path, headers, rows);
    toast(t("settings.backupSuccess"));
  } catch {
    toast(t("error.exportFailed"));
  }
}

async function doBackup() {
  try {
    const path = await saveDialog({
      defaultPath: "turgite-backup.db",
      filters: [{ name: t("settings.backup"), extensions: ["db"] }],
    });
    if (!path) return;
    await api.backupDb(path, password.value || null);
    toast(t("settings.backupSuccess"));
  } catch {
    toast(t("error.backupFailed"));
  }
}

async function doRestore() {
  try {
    const path = await openDialog({ multiple: false, filters: [{ name: t("settings.backup"), extensions: ["db"] }] });
    if (!path || Array.isArray(path)) return;
    await api.restoreDb(path as string, password.value || null);
    toast(t("settings.restoreSuccess"));
    location.reload();
  } catch {
    toast(t("error.restoreFailed"));
  }
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("settings.title") }}</h1>
      </div>
    </div>

    <div class="grid" style="max-width: 720px">
      <div class="card">
        <h3 style="margin-top: 0">{{ t("common.name") }}</h3>
        <div class="field">
          <label>{{ t("settings.baseCurrency") }}</label>
          <CurrencySelect :model-value="settings.baseCurrency" @update:model-value="update('baseCurrency', $event)" />
        </div>
        <div class="field">
          <label>{{ t("settings.language") }}</label>
          <select class="select" :value="settings.language" @change="update('language', ($event.target as HTMLSelectElement).value)">
            <option v-for="l in languages" :key="l" :value="l">{{ t("lang." + l) }}</option>
          </select>
        </div>
        <div class="field">
          <label>{{ t("settings.theme") }}</label>
          <div class="segmented">
            <button v-for="th in themes" :key="th" :class="{ active: settings.theme === th }" @click="update('theme', th)">
              {{ t("settings.theme" + th.charAt(0).toUpperCase() + th.slice(1)) }}
            </button>
          </div>
        </div>
      </div>

      <div class="card">
        <h3 style="margin-top: 0">{{ t("settings.dateFormat") }}</h3>
        <div class="field-row">
          <div class="field">
            <label>{{ t("settings.monthStartDay") }}</label>
            <input class="input" type="number" min="1" max="28" :value="settings.monthStartDay" @change="update('monthStartDay', parseInt(($event.target as HTMLInputElement).value, 10))" />
          </div>
          <div class="field">
            <label>{{ t("settings.weekStart") }}</label>
            <select class="select" :value="settings.weekStart" @change="update('weekStart', parseInt(($event.target as HTMLSelectElement).value, 10))">
              <option v-for="d in weekDays" :key="d" :value="d">{{ t("day." + ["sun", "mon", "tue", "wed", "thu", "fri", "sat"][d]) }}</option>
            </select>
          </div>
        </div>
        <div class="field">
          <label>{{ t("settings.dateFormat") }}</label>
          <select class="select" :value="settings.dateFormat" @change="update('dateFormat', ($event.target as HTMLSelectElement).value)">
            <option v-for="f in dateFormats" :key="f" :value="f">{{ f }}</option>
          </select>
        </div>
      </div>

      <div class="card">
        <h3 style="margin-top: 0">{{ t("settings.export") }}</h3>
        <p class="muted" style="margin-top: 0">{{ t("settings.exportAllHint") }}</p>
        <div style="display: flex; gap: 8px; flex-wrap: wrap">
          <button class="btn btn-primary" @click="doExportXlsx"><Icon name="download" /> {{ t("settings.exportXlsx") }}</button>
          <button class="btn" @click="doExportCsv"><Icon name="download" /> {{ t("settings.exportCsv") }}</button>
        </div>
      </div>

      <div class="card">
        <h3 style="margin-top: 0">{{ t("settings.backup") }} / {{ t("settings.restore") }}</h3>
        <div class="field">
          <label>{{ t("settings.password") }} <span class="faint">({{ t("settings.passwordHint") }})</span></label>
          <input class="input" type="password" v-model="password" />
        </div>
        <div style="display: flex; gap: 8px; flex-wrap: wrap">
          <button class="btn btn-primary" @click="doBackup"><Icon name="download" /> {{ t("settings.backupNow") }}</button>
          <button class="btn" @click="doRestore"><Icon name="upload" /> {{ t("settings.restoreNow") }}</button>
        </div>
      </div>

      <div class="card">
        <h3 style="margin-top: 0">{{ t("settings.about") }}</h3>
        <p class="muted">Turgite v0.1.0 — {{ t("app.tagline") }}</p>
      </div>
    </div>
  </div>
</template>
