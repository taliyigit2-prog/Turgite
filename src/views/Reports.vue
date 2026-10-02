<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import VChart from "vue-echarts";
import { api } from "../lib/api";
import { settings, version } from "../lib/store";
import { formatMoney, formatDate, intlLocale, daysAgo, nowSecs, startOfMonth, startOfYear, monthBounds, toISODateLocal, fromISODate } from "../lib/format";

const { t } = useI18n();

const range = ref("thisMonth");
const customFrom = ref(toISODateLocal(daysAgo(29)));
const customTo = ref(toISODateLocal(nowSecs()));
const summary = ref({ incomeMinor: 0, expenseMinor: 0, netMinor: 0, txCount: 0 });
const series = ref<{ date: number; income: number; expense: number; net: number }[]>([]);
const byCategory = ref<{ name: string; color: string; total: number }[]>([]);

function rangeBounds() {
  const to = nowSecs();
  if (range.value === "thisMonth") return { from: startOfMonth(), to };
  if (range.value === "lastMonth") return monthBounds(-1);
  if (range.value === "thisYear") return { from: startOfYear(), to };
  if (range.value === "last30") return { from: daysAgo(29), to };
  return { from: fromISODate(customFrom.value), to: fromISODate(customTo.value) };
}

async function load() {
  const { from, to } = rangeBounds();
  const [s, sr, cat] = await Promise.all([
    api.reportSummary(from, to),
    api.reportSeries(from, to),
    api.reportByCategory(from, to, "expense"),
  ]);
  summary.value = s;
  series.value = sr;
  byCategory.value = cat;
}
onMounted(load);
watch(version, load);
watch(range, load);
watch([customFrom, customTo], () => {
  if (range.value === "custom") load();
});

const barOption = computed(() => ({
  backgroundColor: "transparent",
  grid: { left: 8, right: 8, top: 20, bottom: 8, containLabel: true },
  tooltip: { trigger: "axis" },
  legend: { data: [t("reports.income"), t("reports.expense")], textStyle: { color: "var(--text-muted)" } },
  xAxis: { type: "category", data: series.value.map((p) => formatDate(p.date, settings.value.language).slice(0, 6)) },
  yAxis: { type: "value", splitLine: { lineStyle: { color: "rgba(128,128,128,0.12)" } } },
  series: [
    { name: t("reports.income"), type: "bar", data: series.value.map((p) => p.income), itemStyle: { color: "#16a34a" } },
    { name: t("reports.expense"), type: "bar", data: series.value.map((p) => p.expense), itemStyle: { color: "#e11d48" } },
  ],
}));

const pieOption = computed(() => ({
  backgroundColor: "transparent",
  tooltip: { trigger: "item" },
  series: [
    {
      type: "pie",
      radius: ["40%", "68%"],
      label: { show: false },
      data: byCategory.value.map((c) => ({ name: c.name, value: c.total, itemStyle: { color: c.color || "#0d9488" } })),
    },
  ],
}));
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("reports.title") }}</h1>
      </div>
      <div class="segmented">
        <button :class="{ active: range === 'thisMonth' }" @click="range = 'thisMonth'">{{ t("reports.thisMonth") }}</button>
        <button :class="{ active: range === 'lastMonth' }" @click="range = 'lastMonth'">{{ t("reports.lastMonth") }}</button>
        <button :class="{ active: range === 'last30' }" @click="range = 'last30'">{{ t("reports.last30") }}</button>
        <button :class="{ active: range === 'thisYear' }" @click="range = 'thisYear'">{{ t("reports.thisYear") }}</button>
        <button :class="{ active: range === 'custom' }" @click="range = 'custom'">{{ t("reports.custom") }}</button>
      </div>
    </div>

    <div v-if="range === 'custom'" class="toolbar">
      <span class="muted">{{ t("reports.from") }}</span>
      <input class="input" type="date" v-model="customFrom" style="width: auto" />
      <span class="muted">{{ t("reports.to") }}</span>
      <input class="input" type="date" v-model="customTo" style="width: auto" />
    </div>

    <div class="stats" style="margin-bottom: 16px">
      <div class="stat">
        <div class="label">{{ t("reports.income") }}</div>
        <div class="value income">{{ formatMoney(summary.incomeMinor, settings.baseCurrency, intlLocale(settings.language)) }}</div>
      </div>
      <div class="stat">
        <div class="label">{{ t("reports.expense") }}</div>
        <div class="value expense">{{ formatMoney(summary.expenseMinor, settings.baseCurrency, intlLocale(settings.language)) }}</div>
      </div>
      <div class="stat">
        <div class="label">{{ t("overview.net") }}</div>
        <div class="value" :class="summary.netMinor >= 0 ? 'income' : 'expense'">
          {{ formatMoney(summary.netMinor, settings.baseCurrency, intlLocale(settings.language)) }}
        </div>
      </div>
    </div>

    <div class="grid" style="grid-template-columns: 1.5fr 1fr">
      <div class="card">
        <h3 style="margin-top: 0">{{ t("reports.income") }} / {{ t("reports.expense") }}</h3>
        <VChart :option="barOption" autoresize style="height: 280px" />
      </div>
      <div class="card">
        <h3 style="margin-top: 0">{{ t("reports.byCategory") }}</h3>
        <VChart v-if="byCategory.length" :option="pieOption" autoresize style="height: 280px" />
        <div v-else class="empty-state">{{ t("reports.noData") }}</div>
      </div>
    </div>
  </div>
</template>
