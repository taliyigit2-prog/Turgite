<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import Icon from "../components/Icon.vue";
import Modal from "../components/Modal.vue";
import CurrencySelect from "../components/CurrencySelect.vue";
import MoneyInput from "../components/MoneyInput.vue";
import { api, Debt } from "../lib/api";
import { bump, version } from "../lib/store";
import { settings } from "../lib/store";
import { formatMoney, intlLocale, toISODateLocal, fromISODate, nowSecs } from "../lib/format";

const { t } = useI18n();
const debts = ref<Debt[]>([]);
const open = ref(false);
const payOpen = ref(false);
const editingId = ref<number | null>(null);
const payDebt = ref<Debt | null>(null);
const payAmount = ref(0);
const payDate = ref(toISODateLocal(nowSecs()));
const form = ref({ counterpart: "", direction: "owed", principalMinor: 0, currency: settings.value.baseCurrency, dueAt: "", note: "" });

async function load() {
  debts.value = await api.listDebts();
}
onMounted(load);
watch(version, load);

function openAdd() {
  editingId.value = null;
  form.value = { counterpart: "", direction: "owed", principalMinor: 0, currency: settings.value.baseCurrency, dueAt: "", note: "" };
  open.value = true;
}
function openEdit(d: Debt) {
  editingId.value = d.id;
  form.value = {
    counterpart: d.counterpart, direction: d.direction, principalMinor: d.principalMinor,
    currency: d.currency, dueAt: d.dueAt ? toISODateLocal(d.dueAt) : "", note: d.note,
  };
  open.value = true;
}
async function save() {
  await api.saveDebt({
    id: editingId.value ?? undefined,
    ...form.value,
    dueAt: form.value.dueAt ? fromISODate(form.value.dueAt) : null,
    accountId: null,
    settledAt: null,
  });
  open.value = false;
  bump();
}
async function remove(d: Debt) {
  if (!confirm(t("confirm.deleteBody"))) return;
  await api.deleteDebt(d.id);
  bump();
}
function openPay(d: Debt) {
  payDebt.value = d;
  payAmount.value = 0;
  payDate.value = toISODateLocal(nowSecs());
  payOpen.value = true;
}
async function doPay() {
  if (!payDebt.value || payAmount.value <= 0) return;
  await api.addDebtPayment(payDebt.value.id, payAmount.value, fromISODate(payDate.value));
  payOpen.value = false;
  bump();
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("debts.title") }}</h1>
      </div>
      <button class="btn btn-primary" @click="openAdd"><Icon name="add" /> {{ t("debts.new") }}</button>
    </div>

    <div v-if="!debts.length" class="card empty-state">
      <div class="emoji">🤝</div>
      {{ t("debts.noDebts") }}
    </div>

    <div class="grid" style="grid-template-columns: repeat(auto-fill, minmax(300px, 1fr))">
      <div v-for="d in debts" :key="d.id" class="card">
        <div style="display: flex; align-items: center; gap: 12px">
          <div class="grow">
            <div class="bold">{{ d.counterpart }}</div>
            <span class="badge" :class="d.direction === 'lent' ? 'income' : 'expense'">
              {{ d.direction === "lent" ? t("debts.lent") : t("debts.owed") }}
            </span>
          </div>
          <button class="icon-btn" @click="openEdit(d)"><Icon name="edit" :size="15" /></button>
          <button class="icon-btn danger" @click="remove(d)"><Icon name="trash" :size="15" /></button>
        </div>
        <div style="margin-top: 12px">
          <div style="display: flex; justify-content: space-between">
            <span class="faint">{{ t("debts.outstanding") }}</span>
            <span class="bold" style="font-size: 18px">{{ formatMoney(d.outstandingMinor, d.currency, intlLocale(settings.language)) }}</span>
          </div>
          <div class="faint" style="margin-top: 4px">
            {{ t("debts.principal") }}: {{ formatMoney(d.principalMinor, d.currency, intlLocale(settings.language)) }} ·
            {{ t("debts.paid") }}: {{ formatMoney(d.paidMinor, d.currency, intlLocale(settings.language)) }}
          </div>
        </div>
        <button class="btn btn-sm" style="margin-top: 12px" @click="openPay(d)">
          <Icon name="add" :size="14" /> {{ t("debts.payment") }}
        </button>
      </div>
    </div>

    <Modal v-if="open" :title="editingId === null ? t('debts.new') : t('common.edit')" @close="open = false">
      <div class="field">
        <label>{{ t("debts.counterpart") }}</label>
        <input class="input" v-model="form.counterpart" />
      </div>
      <div class="field">
        <label>{{ t("common.kind") }}</label>
        <select class="select" v-model="form.direction">
          <option value="owed">{{ t("debts.owed") }}</option>
          <option value="lent">{{ t("debts.lent") }}</option>
        </select>
      </div>
      <div class="field-row">
        <div class="field">
          <label>{{ t("debts.principal") }}</label>
          <MoneyInput v-model="form.principalMinor" :currency="form.currency" />
        </div>
        <div class="field">
          <label>{{ t("common.currency") }}</label>
          <CurrencySelect v-model="form.currency" />
        </div>
      </div>
      <div class="field">
        <label>{{ t("debts.dueAt") }}</label>
        <input class="input" type="date" v-model="form.dueAt" />
      </div>
      <div class="field">
        <label>{{ t("common.note") }}</label>
        <input class="input" v-model="form.note" />
      </div>
      <div class="modal-actions">
        <button class="btn" @click="open = false">{{ t("common.cancel") }}</button>
        <button class="btn btn-primary" @click="save">{{ t("common.save") }}</button>
      </div>
    </Modal>

    <Modal v-if="payOpen" :title="t('debts.payment')" @close="payOpen = false">
      <p class="muted">{{ payDebt?.counterpart }}</p>
      <div class="field">
        <label>{{ t("debts.amountPaid") }}</label>
        <MoneyInput v-model="payAmount" :currency="payDebt?.currency || settings.baseCurrency" />
      </div>
      <div class="field">
        <label>{{ t("common.date") }}</label>
        <input class="input" type="date" v-model="payDate" />
      </div>
      <div class="modal-actions">
        <button class="btn" @click="payOpen = false">{{ t("common.cancel") }}</button>
        <button class="btn btn-primary" @click="doPay">{{ t("common.save") }}</button>
      </div>
    </Modal>
  </div>
</template>
