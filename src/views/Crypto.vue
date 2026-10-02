<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import VChart from "vue-echarts";
import Icon from "../components/Icon.vue";
import Modal from "../components/Modal.vue";
import MoneyInput from "../components/MoneyInput.vue";
import { api, CryptoAsset } from "../lib/api";
import { settings, bump, version, toast } from "../lib/store";
import { formatMoney, intlLocale } from "../lib/format";

const { t } = useI18n();

const POPULAR = [
  { coinId: "bitcoin", symbol: "btc", name: "Bitcoin" },
  { coinId: "ethereum", symbol: "eth", name: "Ethereum" },
  { coinId: "solana", symbol: "sol", name: "Solana" },
  { coinId: "cardano", symbol: "ada", name: "Cardano" },
  { coinId: "ripple", symbol: "xrp", name: "XRP" },
  { coinId: "dogecoin", symbol: "doge", name: "Dogecoin" },
  { coinId: "polkadot", symbol: "dot", name: "Polkadot" },
  { coinId: "tron", symbol: "trx", name: "TRON" },
];

const assets = ref<CryptoAsset[]>([]);
const prices = ref<Record<number, number>>({}); // assetId -> priceBaseMinor
const loading = ref(false);
const open = ref(false);
const editingId = ref<number | null>(null);
const form = ref({ coinId: "bitcoin", symbol: "btc", name: "Bitcoin", amount: 0, amountText: "", costBasisMinor: 0 });
const selectedId = ref<number | null>(null);
const chartData = ref<[number, number][]>([]);
const chartCache = new Map<string, [number, number][]>();
let chartSeq = 0;

async function load() {
  assets.value = await api.listCryptoAssets();
  if (assets.value.length && selectedId.value === null) selectedId.value = assets.value[0].id;
}

async function refreshPrices() {
  loading.value = true;
  const base = settings.value.baseCurrency;
  for (const a of assets.value) {
    try {
      const p = await api.fetchCryptoPrice(a.coinId, base);
      prices.value[a.id] = p.priceBaseMinor;
    } catch {
      // offline — keep previous value
    }
  }
  loading.value = false;
  await loadChart();
  toast(t("crypto.refreshed"));
}

async function loadChart() {
  const coinId = assets.value.find((a) => a.id === selectedId.value)?.coinId;
  if (!coinId) {
    chartData.value = [];
    return;
  }
  const cached = chartCache.get(coinId);
  if (cached) {
    chartData.value = cached;
    return;
  }
  const mySeq = ++chartSeq;
  chartData.value = [];
  try {
    const data = await api.fetchCryptoChart(coinId, settings.value.baseCurrency, 30);
    if (mySeq === chartSeq) {
      chartCache.set(coinId, data);
      chartData.value = data;
    }
  } catch {
    if (mySeq === chartSeq) toast(t("error.rate"));
  }
}

function selectAsset(id: number) {
  selectedId.value = id;
  loadChart();
}

function totalValue() {
  let total = 0;
  for (const a of assets.value) {
    const p = prices.value[a.id];
    if (p) total += a.amount * p;
  }
  return Math.round(total);
}

function valueOf(a: CryptoAsset) {
  const p = prices.value[a.id];
  if (!p) return null;
  return Math.round(a.amount * p);
}

function pnlOf(a: CryptoAsset) {
  const v = valueOf(a);
  if (v === null) return null;
  return v - a.costBasisBaseMinor;
}

function openAdd() {
  editingId.value = null;
  form.value = { coinId: "bitcoin", symbol: "btc", name: "Bitcoin", amount: 0, amountText: "", costBasisMinor: 0 };
  open.value = true;
}
function openEdit(a: CryptoAsset) {
  editingId.value = a.id;
  form.value = { coinId: a.coinId, symbol: a.symbol, name: a.name, amount: a.amount, amountText: String(a.amount), costBasisMinor: a.costBasisBaseMinor };
  open.value = true;
}
function pickCoin(p: { coinId: string; symbol: string; name: string }) {
  form.value.coinId = p.coinId;
  form.value.symbol = p.symbol;
  form.value.name = p.name;
}
async function save() {
  const amount = parseFloat(form.value.amountText);
  if (isNaN(amount) || amount <= 0) {
    toast(t("error.required"));
    return;
  }
  await api.saveCryptoAsset({
    id: editingId.value ?? undefined,
    coinId: form.value.coinId,
    symbol: form.value.symbol,
    name: form.value.name,
    amount,
    costBasisBaseMinor: form.value.costBasisMinor,
  });
  open.value = false;
  bump();
  refreshPrices();
}
async function remove(a: CryptoAsset) {
  if (!confirm(t("confirm.deleteBody"))) return;
  await api.deleteCryptoAsset(a.id);
  bump();
}

onMounted(async () => {
  await load();
  if (assets.value.length) await refreshPrices();
});
watch(version, load);

const chartOption = computed(() => ({
  backgroundColor: "transparent",
  grid: { left: 8, right: 8, top: 16, bottom: 8, containLabel: true },
  tooltip: { trigger: "axis" },
  xAxis: {
    type: "category",
    data: chartData.value.map((p) => new Date(p[0] * 1000).toLocaleDateString(intlLocale(settings.value.language)).slice(0, 6)),
  },
  yAxis: { type: "value", scale: true, splitLine: { lineStyle: { color: "rgba(128,128,128,0.12)" } } },
  series: [{ type: "line", showSymbol: false, data: chartData.value.map((p) => p[1]), lineStyle: { width: 2, color: "#f59e0b" }, areaStyle: { opacity: 0.08, color: "#f59e0b" } }],
}));
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("crypto.title") }}</h1>
        <p class="page-sub">{{ t("crypto.totalValue") }}: {{ formatMoney(totalValue(), settings.baseCurrency, intlLocale(settings.language)) }}</p>
      </div>
      <div style="display: flex; gap: 8px">
        <button class="btn" :disabled="loading" @click="refreshPrices">
          <Icon name="refresh" /> {{ t("crypto.refresh") }}
        </button>
        <button class="btn btn-primary" @click="openAdd"><Icon name="add" /> {{ t("crypto.addAsset") }}</button>
      </div>
    </div>

    <div v-if="!assets.length" class="card empty-state">
      <div class="emoji">🪙</div>
      {{ t("crypto.noAssets") }}
    </div>

    <div class="card" style="padding: 6px 12px" v-else>
      <table class="table">
        <thead>
          <tr>
            <th>{{ t("crypto.coin") }}</th>
            <th class="right">{{ t("crypto.amount") }}</th>
            <th class="right">{{ t("crypto.currentPrice") }}</th>
            <th class="right">{{ t("crypto.value") }}</th>
            <th class="right">{{ t("crypto.pnl") }}</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="a in assets" :key="a.id" :class="{ '': true }" style="cursor: pointer" @click="selectAsset(a.id)">
            <td>
              <div class="bold">{{ a.name }}</div>
              <div class="faint">{{ a.symbol.toUpperCase() }}</div>
            </td>
            <td class="right">{{ a.amount }}</td>
            <td class="right">
              {{ prices[a.id] ? formatMoney(prices[a.id], settings.baseCurrency, intlLocale(settings.language)) : "—" }}
            </td>
            <td class="right bold">
              {{ valueOf(a) !== null ? formatMoney(valueOf(a)!, settings.baseCurrency, intlLocale(settings.language)) : "—" }}
            </td>
            <td class="right bold" :class="pnlOf(a) !== null && pnlOf(a)! >= 0 ? 'amount-pos' : 'amount-neg'">
              {{ pnlOf(a) !== null ? formatMoney(pnlOf(a)!, settings.baseCurrency, intlLocale(settings.language)) : "—" }}
            </td>
            <td style="width: 70px" @click.stop>
              <button class="icon-btn" @click="openEdit(a)"><Icon name="edit" :size="15" /></button>
              <button class="icon-btn danger" @click="remove(a)"><Icon name="trash" :size="15" /></button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="card" style="margin-top: 16px" v-if="chartData.length">
      <h3 style="margin-top: 0">{{ assets.find((a) => a.id === selectedId)?.name }} · {{ t("crypto.range30d") }}</h3>
      <VChart :option="chartOption" autoresize style="height: 220px" />
    </div>

    <Modal v-if="open" :title="editingId === null ? t('crypto.addAsset') : t('common.edit')" @close="open = false">
      <div class="field">
        <label>{{ t("crypto.coin") }}</label>
        <div style="display: flex; gap: 4px; flex-wrap: wrap; margin-bottom: 8px">
          <button v-for="p in POPULAR" :key="p.coinId" class="chip" style="cursor: pointer; border: none" :style="form.coinId === p.coinId ? { background: 'var(--accent-soft)', color: 'var(--accent-text)' } : {}" @click="pickCoin(p)">
            {{ p.symbol.toUpperCase() }}
          </button>
        </div>
        <input class="input" v-model="form.coinId" :placeholder="t('crypto.addCoinHint')" />
      </div>
      <div class="field">
        <label>{{ t("crypto.amount") }}</label>
        <input class="input" type="text" inputmode="decimal" v-model="form.amountText" placeholder="0.0" />
      </div>
      <div class="field">
        <label>{{ t("crypto.costBasis") }} ({{ settings.baseCurrency }})</label>
        <MoneyInput v-model="form.costBasisMinor" :currency="settings.baseCurrency" />
      </div>
      <div class="modal-actions">
        <button class="btn" @click="open = false">{{ t("common.cancel") }}</button>
        <button class="btn btn-primary" @click="save">{{ t("common.save") }}</button>
      </div>
    </Modal>
  </div>
</template>
