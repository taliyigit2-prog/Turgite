<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import Icon from "../components/Icon.vue";
import Modal from "../components/Modal.vue";
import MoneyInput from "../components/MoneyInput.vue";
import { api, Budget, Category } from "../lib/api";
import { settings, bump, version } from "../lib/store";
import { formatMoney, intlLocale, nowSecs } from "../lib/format";

const { t } = useI18n();
const budgets = ref<Budget[]>([]);
const categories = ref<Category[]>([]);
const open = ref(false);
const editingId = ref<number | null>(null);
const form = ref({ categoryId: null as number | null, period: "monthly", amountBaseMinor: 0, rollover: false, active: true });

async function load() {
  const [b, c] = await Promise.all([api.listBudgets(), api.listCategories()]);
  budgets.value = b;
  categories.value = c;
}
onMounted(load);
watch(version, load);

function openAdd() {
  editingId.value = null;
  form.value = { categoryId: null, period: "monthly", amountBaseMinor: 0, rollover: false, active: true };
  open.value = true;
}
function openEdit(b: Budget) {
  editingId.value = b.id;
  form.value = { categoryId: b.categoryId, period: b.period, amountBaseMinor: b.amountBaseMinor, rollover: b.rollover, active: b.active };
  open.value = true;
}
async function save() {
  await api.saveBudget({ id: editingId.value ?? undefined, ...form.value, startAt: nowSecs() });
  open.value = false;
  bump();
}
async function remove(b: Budget) {
  if (!confirm(t("confirm.deleteBody"))) return;
  await api.deleteBudget(b.id);
  bump();
}
function pct(b: Budget) {
  return Math.min(b.percent, 100);
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("budgets.title") }}</h1>
      </div>
      <button class="btn btn-primary" @click="openAdd"><Icon name="add" /> {{ t("budgets.new") }}</button>
    </div>

    <div v-if="!budgets.length" class="card empty-state">
      <div class="emoji">📊</div>
      {{ t("budgets.noBudgets") }}
    </div>

    <div class="grid" style="grid-template-columns: repeat(auto-fill, minmax(300px, 1fr))">
      <div v-for="b in budgets" :key="b.id" class="card">
        <div style="display: flex; align-items: center; gap: 12px">
          <span class="avatar" :style="b.categoryColor ? { background: b.categoryColor + '22', color: b.categoryColor } : {}">
            {{ b.categoryIcon || "📊" }}
          </span>
          <div class="grow">
            <div class="bold">{{ b.categoryName || t("common.all") }}</div>
            <div class="faint">{{ t("budgets." + b.period) }}</div>
          </div>
          <button class="icon-btn" @click="openEdit(b)"><Icon name="edit" :size="15" /></button>
          <button class="icon-btn danger" @click="remove(b)"><Icon name="trash" :size="15" /></button>
        </div>
        <div style="margin: 14px 0 8px">
          <div style="display: flex; justify-content: space-between; margin-bottom: 6px">
            <span class="muted">{{ formatMoney(b.spentMinor, settings.baseCurrency, intlLocale(settings.language)) }}</span>
            <span class="bold">{{ formatMoney(b.amountBaseMinor, settings.baseCurrency, intlLocale(settings.language)) }}</span>
          </div>
          <div class="progress">
            <span :style="{ width: pct(b) + '%', background: b.remainingMinor < 0 ? 'var(--danger)' : 'var(--accent)' }"></span>
          </div>
          <div class="faint" style="margin-top: 6px">
            {{ t("budgets.remaining") }}:
            <b :class="b.remainingMinor < 0 ? 'amount-neg' : ''">{{
              formatMoney(b.remainingMinor, settings.baseCurrency, intlLocale(settings.language))
            }}</b>
          </div>
        </div>
      </div>
    </div>

    <Modal v-if="open" :title="editingId === null ? t('budgets.new') : t('common.edit')" @close="open = false">
      <div class="field">
        <label>{{ t("common.category") }}</label>
        <select class="select" v-model="form.categoryId">
          <option :value="null">{{ t("common.all") }}</option>
          <option v-for="c in categories.filter((x) => x.kind === 'expense')" :key="c.id" :value="c.id">{{ c.name }}</option>
        </select>
      </div>
      <div class="field">
        <label>{{ t("budgets.period") }}</label>
        <select class="select" v-model="form.period">
          <option value="weekly">{{ t("budgets.weekly") }}</option>
          <option value="monthly">{{ t("budgets.monthly") }}</option>
          <option value="yearly">{{ t("budgets.yearly") }}</option>
        </select>
      </div>
      <div class="field">
        <label>{{ t("budgets.amount") }}</label>
        <MoneyInput v-model="form.amountBaseMinor" :currency="settings.baseCurrency" />
      </div>
      <div class="field" style="display: flex; align-items: center; gap: 8px">
        <input type="checkbox" id="roll" v-model="form.rollover" />
        <label for="roll" style="margin: 0">{{ t("budgets.rollover") }}</label>
      </div>
      <div class="modal-actions">
        <button class="btn" @click="open = false">{{ t("common.cancel") }}</button>
        <button class="btn btn-primary" @click="save">{{ t("common.save") }}</button>
      </div>
    </Modal>
  </div>
</template>
