<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import Icon from "../components/Icon.vue";
import Modal from "../components/Modal.vue";
import CurrencySelect from "../components/CurrencySelect.vue";
import MoneyInput from "../components/MoneyInput.vue";
import { api, Recurring, Account, Category } from "../lib/api";
import { settings, bump, version, toast } from "../lib/store";
import { formatMoney, formatDate, intlLocale, fromISODate, toISODateLocal, nowSecs } from "../lib/format";

const { t } = useI18n();
const items = ref<Recurring[]>([]);
const accounts = ref<Account[]>([]);
const categories = ref<Category[]>([]);
const open = ref(false);
const editingId = ref<number | null>(null);
const form = ref({
  name: "",
  kind: "expense",
  accountId: null as number | null,
  destAccountId: null as number | null,
  categoryId: null as number | null,
  amountMinor: 0,
  currency: settings.value.baseCurrency,
  payee: "",
  note: "",
  freq: "monthly",
  intervalN: 1,
  nextRunAt: toISODateLocal(nowSecs()),
  autoPost: false,
  active: true,
});

async function load() {
  const [r, a, c] = await Promise.all([api.listRecurring(), api.listAccounts(), api.listCategories()]);
  items.value = r;
  accounts.value = a;
  categories.value = c;
}
onMounted(async () => {
  await load();
  try {
    const posted = await api.runRecurring();
    if (posted > 0) {
      bump();
      toast(t("recurring.runNow"));
    }
  } catch {
    // ignore auto-post failures (offline etc.)
  }
});
watch(version, load);

function openAdd() {
  editingId.value = null;
  form.value = {
    name: "", kind: "expense", accountId: accounts.value[0]?.id ?? null, destAccountId: null, categoryId: null,
    amountMinor: 0, currency: settings.value.baseCurrency, payee: "", note: "", freq: "monthly",
    intervalN: 1, nextRunAt: toISODateLocal(nowSecs()), autoPost: false, active: true,
  };
  open.value = true;
}
function openEdit(r: Recurring) {
  editingId.value = r.id;
  form.value = {
    name: r.name, kind: r.kind, accountId: r.accountId, destAccountId: r.destAccountId, categoryId: r.categoryId,
    amountMinor: r.amountMinor, currency: r.currency, payee: r.payee, note: r.note, freq: r.freq,
    intervalN: r.intervalN, nextRunAt: toISODateLocal(r.nextRunAt), autoPost: r.autoPost, active: r.active,
  };
  open.value = true;
}
async function save() {
  await api.saveRecurring({
    id: editingId.value ?? undefined,
    ...form.value,
    nextRunAt: fromISODate(form.value.nextRunAt),
    endAt: null,
  });
  open.value = false;
  bump();
}
async function remove(r: Recurring) {
  if (!confirm(t("confirm.deleteBody"))) return;
  await api.deleteRecurring(r.id);
  bump();
}
async function toggle(r: Recurring) {
  await api.toggleRecurring(r.id, !r.active);
  bump();
}
async function runNow(r: Recurring) {
  await api.runRecurringOne(r.id);
  toast(t("recurring.runNow"));
  bump();
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("recurring.title") }}</h1>
      </div>
      <button class="btn btn-primary" @click="openAdd"><Icon name="add" /> {{ t("recurring.new") }}</button>
    </div>

    <div v-if="!items.length" class="card empty-state">
      <div class="emoji">🔁</div>
      {{ t("recurring.noRecurring") }}
    </div>

    <div class="grid" style="grid-template-columns: repeat(auto-fill, minmax(300px, 1fr))">
      <div v-for="r in items" :key="r.id" class="card" :style="{ opacity: r.active ? 1 : 0.55 }">
        <div style="display: flex; align-items: center; gap: 12px">
          <div class="grow">
            <div class="bold">{{ r.name }}</div>
            <div class="faint">
              {{ t("txn." + r.kind) }} · {{ t("recurring." + r.freq) }} · {{ r.accountName }}
            </div>
          </div>
          <span class="badge" :class="r.active ? '' : 'expense'">{{ r.active ? t("recurring.active") : t("recurring.inactive") }}</span>
        </div>
        <div style="margin-top: 10px; display: flex; justify-content: space-between; align-items: center">
          <div>
            <div class="bold" style="font-size: 18px">{{ formatMoney(r.amountMinor, r.currency, intlLocale(settings.language)) }}</div>
            <div class="faint">{{ t("recurring.nextRun") }}: {{ formatDate(r.nextRunAt, settings.language) }}</div>
          </div>
          <div style="display: flex">
            <button class="icon-btn" :title="t('recurring.runNow')" @click="runNow(r)"><Icon name="add" :size="15" /></button>
            <button class="icon-btn" :title="t('common.edit')" @click="openEdit(r)"><Icon name="edit" :size="15" /></button>
            <button class="icon-btn" :title="r.active ? t('recurring.inactive') : t('recurring.active')" @click="toggle(r)">
              <Icon name="check" :size="15" />
            </button>
            <button class="icon-btn danger" @click="remove(r)"><Icon name="trash" :size="15" /></button>
          </div>
        </div>
      </div>
    </div>

    <Modal v-if="open" :title="editingId === null ? t('recurring.new') : t('common.edit')" wide @close="open = false">
      <div class="field">
        <label>{{ t("common.name") }}</label>
        <input class="input" v-model="form.name" />
      </div>
      <div class="field-row">
        <div class="field">
          <label>{{ t("common.kind") }}</label>
          <select class="select" v-model="form.kind">
            <option value="expense">{{ t("txn.expense") }}</option>
            <option value="income">{{ t("txn.income") }}</option>
            <option value="transfer">{{ t("txn.transfer") }}</option>
          </select>
        </div>
        <div class="field">
          <label>{{ t("recurring.freq") }}</label>
          <select class="select" v-model="form.freq">
            <option value="daily">{{ t("recurring.daily") }}</option>
            <option value="weekly">{{ t("recurring.weekly") }}</option>
            <option value="monthly">{{ t("recurring.monthly") }}</option>
            <option value="yearly">{{ t("recurring.yearly") }}</option>
          </select>
        </div>
      </div>
      <div class="field-row">
        <div class="field">
          <label>{{ t("common.amount") }}</label>
          <div style="display: flex; gap: 8px">
            <MoneyInput v-model="form.amountMinor" :currency="form.currency" />
            <div style="width: 110px">
              <CurrencySelect v-model="form.currency" />
            </div>
          </div>
        </div>
        <div class="field">
          <label>{{ t("recurring.nextRun") }}</label>
          <input class="input" type="date" v-model="form.nextRunAt" />
        </div>
      </div>
      <div class="field-row">
        <div class="field">
          <label>{{ t("common.account") }}</label>
          <select class="select" v-model="form.accountId">
            <option v-for="a in accounts" :key="a.id" :value="a.id">{{ a.name }}</option>
          </select>
        </div>
        <div class="field" v-if="form.kind !== 'transfer'">
          <label>{{ t("common.category") }}</label>
          <select class="select" v-model="form.categoryId">
            <option :value="null">{{ t("common.none") }}</option>
            <option v-for="c in categories" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </div>
      </div>
      <div class="field" style="display: flex; align-items: center; gap: 8px">
        <input type="checkbox" id="auto" v-model="form.autoPost" />
        <label for="auto" style="margin: 0">{{ t("recurring.autoPost") }}</label>
      </div>
      <div class="modal-actions">
        <button class="btn" @click="open = false">{{ t("common.cancel") }}</button>
        <button class="btn btn-primary" @click="save">{{ t("common.save") }}</button>
      </div>
    </Modal>
  </div>
</template>
