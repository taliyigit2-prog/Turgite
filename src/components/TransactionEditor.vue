<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { useI18n } from "vue-i18n";
import Modal from "./Modal.vue";
import Icon from "./Icon.vue";
import CurrencySelect from "./CurrencySelect.vue";
import MoneyInput from "./MoneyInput.vue";
import { api, Transaction, Account, Category, FxRate } from "../lib/api";
import { settings, toast } from "../lib/store";
import { parseAmount, convertAmount, toISODateLocal, fromISODate, nowSecs, formatMoney, intlLocale } from "../lib/format";

const props = defineProps<{ open: boolean; initial: Transaction | null; kind: string }>();
const emit = defineEmits<{ (e: "close"): void; (e: "saved"): void }>();

const { t } = useI18n();

const accounts = ref<Account[]>([]);
const categories = ref<Category[]>([]);

const form = ref({
  kind: "expense",
  amountMinor: 0,
  amountText: "",
  currency: settings.value.baseCurrency,
  accountId: null as number | null,
  destAccountId: null as number | null,
  categoryId: null as number | null,
  payee: "",
  note: "",
  tags: "",
  dateIso: toISODateLocal(nowSecs()),
  rateScaled: 0,
  rateText: "",
});

const rate = ref<FxRate | null>(null);
const fetchingRate = ref(false);

const isTransfer = computed(() => form.value.kind === "transfer");
const needsRate = computed(() => form.value.currency !== settings.value.baseCurrency);

const baseEquivalent = computed(() => {
  if (!needsRate.value) return form.value.amountMinor;
  if (!form.value.rateScaled) return null;
  return convertAmount(form.value.amountMinor, form.value.currency, settings.value.baseCurrency, form.value.rateScaled);
});

watch(
  () => props.open,
  async (open) => {
    if (!open) return;
    const [a, c] = await Promise.all([api.listAccounts(), api.listCategories()]);
    accounts.value = a;
    categories.value = c;

    if (props.initial) {
      const x = props.initial;
      form.value = {
        kind: x.kind,
        amountMinor: x.amountMinor,
        amountText: "",
        currency: x.currency,
        accountId: x.accountId,
        destAccountId: x.destAccountId,
        categoryId: x.categoryId,
        payee: x.payee,
        note: x.note,
        tags: x.tags.join(", "),
        dateIso: toISODateLocal(x.occurredAt),
        rateScaled: x.rateScaled,
        rateText: x.rateScaled ? (x.rateScaled / 1e8).toFixed(6) : "",
      };
      if (x.rateScaled) {
        rate.value = {
          base: settings.value.baseCurrency,
          quote: x.currency,
          rate: x.rateScaled / 1e8,
          rateScaled: x.rateScaled,
          date: "",
          source: "",
        };
      }
    } else {
      form.value = {
        kind: props.kind,
        amountMinor: 0,
        amountText: "",
        currency: settings.value.baseCurrency,
        accountId: a[0]?.id ?? null,
        destAccountId: null,
        categoryId: null,
        payee: "",
        note: "",
        tags: "",
        dateIso: toISODateLocal(nowSecs()),
        rateScaled: 0,
        rateText: "",
      };
      rate.value = null;
    }
  }
);

watch(
  () => form.value.currency,
  async (cur) => {
    if (cur !== settings.value.baseCurrency) {
      await doFetchRate(cur);
    } else {
      rate.value = null;
      form.value.rateScaled = 0;
    }
  }
);

async function doFetchRate(cur: string) {
  fetchingRate.value = true;
  try {
    const r = await api.fetchFxRate(settings.value.baseCurrency, cur);
    rate.value = r;
    form.value.rateScaled = r.rateScaled;
    form.value.rateText = r.rate.toFixed(6);
  } catch {
    toast(t("error.rate"));
  } finally {
    fetchingRate.value = false;
  }
}

function onRateText() {
  const v = parseFloat(form.value.rateText);
  if (!isNaN(v) && v > 0) {
    form.value.rateScaled = Math.round(v * 1e8);
    rate.value = { base: settings.value.baseCurrency, quote: form.value.currency, rate: v, rateScaled: Math.round(v * 1e8), date: "", source: "" };
  }
}

function expenseCategories() {
  return categories.value.filter((c) => c.kind === "expense");
}
function incomeCategories() {
  return categories.value.filter((c) => c.kind === "income");
}

async function save() {
  const amountMinor = parseAmount(form.value.amountText, form.value.currency);
  if (amountMinor === null || amountMinor <= 0 || form.value.accountId === null) {
    toast(t("error.required"));
    return;
  }
  const tags = form.value.tags
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);

  await api.saveTransaction({
    id: props.initial?.id,
    kind: form.value.kind,
    accountId: form.value.accountId,
    destAccountId: isTransfer.value ? form.value.destAccountId : null,
    categoryId: form.value.kind === "transfer" ? null : form.value.categoryId,
    amountMinor,
    currency: form.value.currency,
    rateScaled: needsRate.value ? form.value.rateScaled : undefined,
    occurredAt: fromISODate(form.value.dateIso),
    payee: form.value.payee,
    note: form.value.note,
    tags,
  });
  emit("saved");
  emit("close");
}

const title = computed(() => {
  if (props.initial) return t("common.edit");
  return t("txn.add" + form.value.kind.charAt(0).toUpperCase() + form.value.kind.slice(1));
});
</script>

<template>
  <Modal v-if="open" :title="title" wide @close="emit('close')">
    <div class="segmented" style="margin-bottom: 16px">
      <button :class="{ active: form.kind === 'expense' }" @click="form.kind = 'expense'">
        {{ t("txn.expense") }}
      </button>
      <button :class="{ active: form.kind === 'income' }" @click="form.kind = 'income'">
        {{ t("txn.income") }}
      </button>
      <button :class="{ active: form.kind === 'transfer' }" @click="form.kind = 'transfer'">
        {{ t("txn.transfer") }}
      </button>
    </div>

    <div class="field">
      <label>{{ t("common.amount") }}</label>
      <div style="display: flex; gap: 8px">
        <input
          class="input"
          type="text"
          inputmode="decimal"
          v-model="form.amountText"
          placeholder="0.00"
          style="flex: 1"
        />
        <div style="width: 130px">
          <CurrencySelect v-model="form.currency" />
        </div>
      </div>
    </div>

    <div v-if="needsRate" class="field">
      <label>{{ t("txn.rate") }} ({{ t("txn.rateHint", { base: settings.baseCurrency }) }})</label>
      <div style="display: flex; gap: 8px">
        <input class="input" type="text" v-model="form.rateText" @change="onRateText" placeholder="0.000000" style="flex: 1" />
        <button class="btn" :disabled="fetchingRate" @click="doFetchRate(form.currency)">
          <Icon name="refresh" /> {{ t("txn.fetchRate") }}
        </button>
      </div>
      <div v-if="baseEquivalent !== null" class="muted" style="margin-top: 6px">
        {{ t("txn.baseEquivalent") }}:
        <b>{{ formatMoney(baseEquivalent, settings.baseCurrency, intlLocale(settings.language)) }}</b>
      </div>
    </div>

    <div class="field-row">
      <div class="field">
        <label>{{ t("common.account") }}</label>
        <select class="select" v-model="form.accountId">
          <option v-for="a in accounts" :key="a.id" :value="a.id">{{ a.name }}</option>
        </select>
      </div>
      <div v-if="isTransfer" class="field">
        <label>{{ t("txn.toAccount") }}</label>
        <select class="select" v-model="form.destAccountId">
          <option v-for="a in accounts" :key="a.id" :value="a.id">{{ a.name }}</option>
        </select>
      </div>
    </div>

    <div class="field" v-if="form.kind !== 'transfer'">
      <label>{{ t("common.category") }}</label>
      <select class="select" v-model="form.categoryId">
        <option :value="null">{{ t("common.none") }}</option>
        <option
          v-for="c in form.kind === 'expense' ? expenseCategories() : incomeCategories()"
          :key="c.id"
          :value="c.id"
        >
          {{ c.name }}
        </option>
      </select>
    </div>

    <div class="field-row">
      <div class="field">
        <label>{{ t("txn.payee") }}</label>
        <input class="input" v-model="form.payee" />
      </div>
      <div class="field">
        <label>{{ t("common.date") }}</label>
        <input class="input" type="date" v-model="form.dateIso" />
      </div>
    </div>

    <div class="field">
      <label>{{ t("txn.tags") }} <span class="faint">({{ t("txn.tagsHint") }})</span></label>
      <input class="input" v-model="form.tags" />
    </div>

    <div class="field">
      <label>{{ t("common.note") }} <span class="faint">({{ t("common.optional") }})</span></label>
      <textarea class="textarea" v-model="form.note" rows="2"></textarea>
    </div>

    <div class="modal-actions">
      <button class="btn" @click="emit('close')">{{ t("common.cancel") }}</button>
      <button class="btn btn-primary" @click="save">{{ t("common.save") }}</button>
    </div>
  </Modal>
</template>
