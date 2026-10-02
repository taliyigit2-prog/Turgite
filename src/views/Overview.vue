<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import VChart from "vue-echarts";
import { api, LargestExpense } from "../lib/api";
import { settings, bump, version } from "../lib/store";
import { formatMoney, formatDate, startOfMonth, daysAgo, nowSecs } from "../lib/format";

const { t } = useI18n();

const summary = ref({ incomeMinor: 0, expenseMinor: 0, netMinor: 0, txCount: 0 });
const series = ref<{ date: number; income: number; expense: number; net: number }[]>([]);
const byCategory = ref<{ name: string; color: string; total: number }[]>([]);
const largest = ref<LargestExpense[]>([]);

async function load() {
  const from = daysAgo(29);
  const to = nowSecs();
  const [s, sr, cat, large] = await Promise.all([
    api.reportSummary(startOfMonth(), to),
    api.reportSeries(from, to),
    api.reportByCategory(startOfMonth(), to, "expense"),
    api.reportLargestExpense(from, to, 5),
  ]);
  summary.value = s;
  series.value = sr;
  byCategory.value = cat;
  largest.value = large;
}

onMounted(load);
watch(version, load);

const base = computed(() => settings.value.baseCurrency);
const loc = computed(() => (settings.value.language === "tr" ? "tr-TR" : "en-US"));

const lineOption = computed(() => ({
  backgroundColor: "transparent",
  grid: { left: 8, right: 8, top: 16, bottom: 8, containLabel: true },
  tooltip: { trigger: "axis" },
  xAxis: {
    type: "category",
    data: series.value.map((p) => formatDate(p.date, settings.value.language).slice(0, 6)),
    axisLine: { show: false },
    axisTick: { show: false },
  },
  yAxis: { type: "value", splitLine: { lineStyle: { color: "rgba(128,128,128,0.12)" } } },
  series: [
    {
      name: t("overview.income"),
      type: "line",
      smooth: true,
      symbol: "none",
      data: series.value.map((p) => p.income),
      lineStyle: { width: 2 },
      itemStyle: { color: "#16a34a" },
      areaStyle: { opacity: 0.06, color: "#16a34a" },
    },
    {
      name: t("overview.expense"),
      type: "line",
      smooth: true,
      symbol: "none",
      data: series.value.map((p) => p.expense),
      lineStyle: { width: 2 },
      itemStyle: { color: "#e11d48" },
      areaStyle: { opacity: 0.06, color: "#e11d48" },
    },
  ],
}));

const pieOption = computed(() => ({
  backgroundColor: "transparent",
  tooltip: { trigger: "item" },
  legend: { orient: "vertical", right: 0, top: "middle", textStyle: { color: "var(--text-muted)" } },
  series: [
    {
      type: "pie",
      radius: ["45%", "72%"],
      center: ["35%", "50%"],
      avoidLabelOverlap: true,
      label: { show: false },
      data: byCategory.value.map((c) => ({
        name: c.name,
        value: c.total,
        itemStyle: { color: c.color || "#0d9488" },
      })),
    },
  ],
}));
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("nav.overview") }}</h1>
        <p class="page-sub">{{ t("overview.thisMonth") }}</p>
      </div>
    </div>

    <div class="stats">
      <div class="stat">
        <div class="label">{{ t("overview.income") }}</div>
        <div class="value income">{{ formatMoney(summary.incomeMinor, base, loc) }}</div>
      </div>
      <div class="stat">
        <div class="label">{{ t("overview.expense") }}</div>
        <div class="value expense">{{ formatMoney(summary.expenseMinor, base, loc) }}</div>
      </div>
      <div class="stat">
        <div class="label">{{ t("overview.net") }}</div>
        <div class="value" :class="summary.netMinor >= 0 ? 'income' : 'expense'">
          {{ formatMoney(summary.netMinor, base, loc) }}
        </div>
      </div>
      <div class="stat">
        <div class="label">{{ t("common.total") }}</div>
        <div class="value">{{ summary.txCount }}</div>
      </div>
    </div>

    <div class="grid" style="grid-template-columns: 1.6fr 1fr; margin-top: 16px">
      <div class="card">
        <h3 style="margin-top: 0">{{ t("overview.daily") }}</h3>
        <VChart :option="lineOption" autoresize style="height: 260px" />
      </div>
      <div class="card">
        <h3 style="margin-top: 0">{{ t("overview.byCategory") }}</h3>
        <VChart v-if="byCategory.length" :option="pieOption" autoresize style="height: 260px" />
        <div v-else class="empty-state">{{ t("overview.noData") }}</div>
      </div>
    </div>

    <div class="card" style="margin-top: 16px">
      <h3 style="margin-top: 0">{{ t("overview.largest") }}</h3>
      <div v-if="largest.length">
        <div v-for="e in largest" :key="e.id" class="list-row" style="cursor: default">
          <div class="grow">
            <div class="bold">{{ e.payee || t("txn.expense") }}</div>
            <div class="faint">{{ formatDate(e.occurredAt, settings.language) }}</div>
          </div>
          <div class="amount-neg bold">{{ formatMoney(e.amountBaseMinor, base, loc) }}</div>
        </div>
      </div>
      <div v-else class="empty-state">{{ t("overview.noData") }}</div>
    </div>
  </div>
</template>
