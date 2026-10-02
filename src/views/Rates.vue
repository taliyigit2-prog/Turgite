<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import Icon from "../components/Icon.vue";
import CurrencySelect from "../components/CurrencySelect.vue";
import MoneyInput from "../components/MoneyInput.vue";
import { api, FxRate } from "../lib/api";
import { settings, toast } from "../lib/store";
import { convertAmount, formatMoney, intlLocale } from "../lib/format";

const { t } = useI18n();

const from = ref(settings.value.baseCurrency);
const to = ref("USD");
const rate = ref<FxRate | null>(null);
const loading = ref(false);
const amountMinor = ref(0);

async function fetchRate() {
  if (from.value === to.value) {
    rate.value = { base: from.value, quote: to.value, rate: 1, rateScaled: 1e8, date: "", source: "" };
    return;
  }
  loading.value = true;
  try {
    rate.value = await api.fetchFxRate(from.value, to.value);
  } catch {
    toast(t("error.rate"));
  } finally {
    loading.value = false;
  }
}

function swap() {
  const tmp = from.value;
  from.value = to.value;
  to.value = tmp;
  fetchRate();
}

function compute() {
  if (!rate.value) return 0;
  return convertAmount(amountMinor.value, from.value, to.value, rate.value.rateScaled);
}

const rateText = () => {
  if (!rate.value) return "—";
  return rate.value.rate.toLocaleString(intlLocale(settings.value.language), {
    maximumFractionDigits: 6,
    minimumFractionDigits: 2,
  });
};

onMounted(fetchRate);
watch([from, to], fetchRate);
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("rates.title") }}</h1>
      </div>
    </div>

    <div class="grid" style="max-width: 720px">
      <div class="card">
        <div class="field-row" style="align-items: flex-end">
          <div class="field" style="margin: 0">
            <label>{{ t("rates.from") }}</label>
            <CurrencySelect v-model="from" />
          </div>
          <button class="icon-btn" style="margin-bottom: 2px" @click="swap"><Icon name="rates" :size="20" /></button>
          <div class="field" style="margin: 0">
            <label>{{ t("rates.to") }}</label>
            <CurrencySelect v-model="to" />
          </div>
        </div>

        <div style="text-align: center; padding: 24px 0">
          <div class="muted">{{ t("rates.rate") }}</div>
          <div style="font-size: 28px; font-weight: 700; letter-spacing: -0.02em">
            1 {{ from }} = {{ rateText() }} {{ to }}
          </div>
          <div v-if="rate?.source" class="faint" style="margin-top: 6px">
            {{ t("rates.source") }}: {{ rate.source }}<span v-if="rate.date"> · {{ rate.date }}</span>
          </div>
        </div>
      </div>

      <div class="card">
        <h3 style="margin-top: 0">{{ t("rates.convert") }}</h3>
        <div class="field">
          <label>{{ t("rates.amount") }} ({{ from }})</label>
          <MoneyInput v-model="amountMinor" :currency="from" />
        </div>
        <div class="field">
          <label>{{ t("rates.result") }} ({{ to }})</label>
          <div class="input" style="font-weight: 700">
            {{ rate ? formatMoney(compute(), to, intlLocale(settings.language)) : "—" }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
