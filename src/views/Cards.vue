<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import Icon from "../components/Icon.vue";
import Modal from "../components/Modal.vue";
import CurrencySelect from "../components/CurrencySelect.vue";
import MoneyInput from "../components/MoneyInput.vue";
import { api, CreditCard, Installment, Account } from "../lib/api";
import { settings, bump, version, toast } from "../lib/store";
import { formatMoney, intlLocale, toISODateLocal, fromISODate, nowSecs } from "../lib/format";

const { t } = useI18n();
const cards = ref<CreditCard[]>([]);
const installments = ref<Installment[]>([]);
const accounts = ref<Account[]>([]);
const open = ref(false);
const instOpen = ref(false);
const payOpen = ref(false);
const editingId = ref<number | null>(null);
const payInst = ref<Installment | null>(null);
const payDate = ref(toISODateLocal(nowSecs()));
const form = ref({ accountId: null as number | null, name: "", creditLimitMinor: 0, statementDay: 1, dueDay: 10 });
const instForm = ref({ accountId: null as number | null, name: "", totalMinor: 0, currency: settings.value.baseCurrency, months: 3, startedAt: toISODateLocal(nowSecs()), note: "" });

async function load() {
  const [c, i, a] = await Promise.all([api.listCards(), api.listInstallments(), api.listAccounts()]);
  cards.value = c;
  installments.value = i;
  accounts.value = a;
}
onMounted(load);
watch(version, load);

function openAdd() {
  editingId.value = null;
  form.value = { accountId: accounts.value[0]?.id ?? null, name: "", creditLimitMinor: 0, statementDay: 1, dueDay: 10 };
  open.value = true;
}
function openEdit(c: CreditCard) {
  editingId.value = c.id;
  form.value = { accountId: c.accountId, name: c.name, creditLimitMinor: c.creditLimitMinor, statementDay: c.statementDay, dueDay: c.dueDay };
  open.value = true;
}
async function save() {
  await api.saveCard({ id: editingId.value ?? undefined, ...form.value });
  open.value = false;
  bump();
}
async function remove(c: CreditCard) {
  if (!confirm(t("confirm.deleteBody"))) return;
  await api.deleteCard(c.id);
  bump();
}

function openInst() {
  instForm.value = { accountId: accounts.value[0]?.id ?? null, name: "", totalMinor: 0, currency: settings.value.baseCurrency, months: 3, startedAt: toISODateLocal(nowSecs()), note: "" };
  instOpen.value = true;
}
async function saveInst() {
  await api.saveInstallment({ ...instForm.value, startedAt: fromISODate(instForm.value.startedAt) });
  instOpen.value = false;
  bump();
}
async function removeInst(i: Installment) {
  if (!confirm(t("confirm.deleteBody"))) return;
  await api.deleteInstallment(i.id);
  bump();
}
function openPay(i: Installment) {
  payInst.value = i;
  payDate.value = toISODateLocal(nowSecs());
  payOpen.value = true;
}
async function doPay() {
  if (!payInst.value) return;
  await api.payInstallment(payInst.value.id, fromISODate(payDate.value));
  payOpen.value = false;
  toast(t("cards.paid"));
  bump();
}
function limitPct(c: CreditCard) {
  if (c.creditLimitMinor <= 0) return 0;
  return Math.min((c.balanceMinor / c.creditLimitMinor) * 100, 100);
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("cards.title") }}</h1>
      </div>
      <div style="display: flex; gap: 8px">
        <button class="btn" @click="openInst"><Icon name="add" /> {{ t("cards.newInstallment") }}</button>
        <button class="btn btn-primary" @click="openAdd"><Icon name="add" /> {{ t("cards.new") }}</button>
      </div>
    </div>

    <div v-if="!cards.length" class="card empty-state">
      <div class="emoji">💳</div>
      {{ t("cards.noCards") }}
    </div>

    <div class="grid" style="grid-template-columns: repeat(auto-fill, minmax(300px, 1fr))">
      <div v-for="c in cards" :key="c.id" class="card">
        <div style="display: flex; align-items: center; gap: 12px">
          <span class="avatar">💳</span>
          <div class="grow">
            <div class="bold">{{ c.name }}</div>
            <div class="faint">{{ c.accountName }}</div>
          </div>
          <button class="icon-btn" @click="openEdit(c)"><Icon name="edit" :size="15" /></button>
          <button class="icon-btn danger" @click="remove(c)"><Icon name="trash" :size="15" /></button>
        </div>
        <div style="margin-top: 14px">
          <div style="display: flex; justify-content: space-between; margin-bottom: 6px">
            <span class="muted">{{ t("cards.balance") }}: {{ formatMoney(c.balanceMinor, settings.baseCurrency, intlLocale(settings.language)) }}</span>
            <span class="faint">{{ t("cards.creditLimit") }}: {{ formatMoney(c.creditLimitMinor, settings.baseCurrency, intlLocale(settings.language)) }}</span>
          </div>
          <div class="progress"><span :style="{ width: limitPct(c) + '%', background: 'var(--accent)' }"></span></div>
          <div class="faint" style="margin-top: 6px">
            {{ t("cards.statementDay") }}: {{ c.statementDay }} · {{ t("cards.dueDay") }}: {{ c.dueDay }}
          </div>
        </div>
      </div>
    </div>

    <h2 style="margin: 24px 0 12px; font-size: 18px">{{ t("cards.installments") }}</h2>
    <div v-if="!installments.length" class="card empty-state" style="padding: 28px">
      <div class="emoji">🗓️</div>
      {{ t("cards.noInstallments") }}
    </div>
    <div class="card" style="padding: 6px 12px" v-else>
      <table class="table">
        <thead>
          <tr>
            <th>{{ t("common.name") }}</th>
            <th>{{ t("cards.monthly") }}</th>
            <th>{{ t("cards.paid") }}</th>
            <th class="right">{{ t("cards.remaining") }}</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="i in installments" :key="i.id">
            <td>{{ i.name }}</td>
            <td class="muted">{{ formatMoney(i.monthlyMinor, i.currency, intlLocale(settings.language)) }}</td>
            <td class="muted">{{ i.paidCount }} / {{ i.months }}</td>
            <td class="right bold">{{ formatMoney(i.remainingMinor, i.currency, intlLocale(settings.language)) }}</td>
            <td style="width: 110px; white-space: nowrap">
              <button class="btn btn-sm btn-primary" :disabled="i.remainingMonths <= 0" @click="openPay(i)">
                <Icon name="check" :size="14" /> {{ t("cards.pay") }}
              </button>
              <button class="icon-btn danger" @click="removeInst(i)"><Icon name="trash" :size="15" /></button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <Modal v-if="open" :title="editingId === null ? t('cards.new') : t('common.edit')" @close="open = false">
      <div class="field">
        <label>{{ t("common.name") }}</label>
        <input class="input" v-model="form.name" />
      </div>
      <div class="field">
        <label>{{ t("cards.linkAccount") }}</label>
        <select class="select" v-model="form.accountId">
          <option v-for="a in accounts" :key="a.id" :value="a.id">{{ a.name }}</option>
        </select>
      </div>
      <div class="field">
        <label>{{ t("cards.creditLimit") }}</label>
        <MoneyInput v-model="form.creditLimitMinor" :currency="settings.baseCurrency" />
      </div>
      <div class="field-row">
        <div class="field">
          <label>{{ t("cards.statementDay") }}</label>
          <input class="input" type="number" min="1" max="31" v-model="form.statementDay" />
        </div>
        <div class="field">
          <label>{{ t("cards.dueDay") }}</label>
          <input class="input" type="number" min="1" max="31" v-model="form.dueDay" />
        </div>
      </div>
      <div class="modal-actions">
        <button class="btn" @click="open = false">{{ t("common.cancel") }}</button>
        <button class="btn btn-primary" @click="save">{{ t("common.save") }}</button>
      </div>
    </Modal>

    <Modal v-if="instOpen" :title="t('cards.newInstallment')" @close="instOpen = false">
      <div class="field">
        <label>{{ t("common.name") }}</label>
        <input class="input" v-model="instForm.name" />
      </div>
      <div class="field">
        <label>{{ t("cards.linkAccount") }}</label>
        <select class="select" v-model="instForm.accountId">
          <option v-for="a in accounts" :key="a.id" :value="a.id">{{ a.name }}</option>
        </select>
      </div>
      <div class="field-row">
        <div class="field">
          <label>{{ t("cards.total") }}</label>
          <MoneyInput v-model="instForm.totalMinor" :currency="instForm.currency" />
        </div>
        <div class="field">
          <label>{{ t("common.currency") }}</label>
          <CurrencySelect v-model="instForm.currency" />
        </div>
      </div>
      <div class="field-row">
        <div class="field">
          <label>{{ t("cards.months") }}</label>
          <input class="input" type="number" min="1" v-model="instForm.months" />
        </div>
        <div class="field">
          <label>{{ t("cards.startedAt") }}</label>
          <input class="input" type="date" v-model="instForm.startedAt" />
        </div>
      </div>
      <div class="modal-actions">
        <button class="btn" @click="instOpen = false">{{ t("common.cancel") }}</button>
        <button class="btn btn-primary" @click="saveInst">{{ t("common.save") }}</button>
      </div>
    </Modal>

    <Modal v-if="payOpen" :title="t('cards.pay')" @close="payOpen = false">
      <p class="muted">{{ payInst?.name }}</p>
      <div class="field">
        <label>{{ t("common.date") }}</label>
        <input class="input" type="date" v-model="payDate" />
      </div>
      <div class="field">
        <label>{{ t("cards.monthly") }}</label>
        <div class="bold">{{ payInst ? formatMoney(payInst.monthlyMinor, payInst.currency, intlLocale(settings.language)) : "" }}</div>
      </div>
      <div class="modal-actions">
        <button class="btn" @click="payOpen = false">{{ t("common.cancel") }}</button>
        <button class="btn btn-primary" @click="doPay">{{ t("cards.pay") }}</button>
      </div>
    </Modal>
  </div>
</template>
