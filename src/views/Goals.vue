<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import Icon from "../components/Icon.vue";
import Modal from "../components/Modal.vue";
import CurrencySelect from "../components/CurrencySelect.vue";
import MoneyInput from "../components/MoneyInput.vue";
import { api, Goal } from "../lib/api";
import { bump, version } from "../lib/store";
import { formatMoney, intlLocale, toISODateLocal, fromISODate } from "../lib/format";
import { settings } from "../lib/store";

const { t } = useI18n();
const goals = ref<Goal[]>([]);
const open = ref(false);
const editingId = ref<number | null>(null);
const form = ref({ name: "", targetMinor: 0, currency: settings.value.baseCurrency, targetDate: "", color: "", note: "", icon: "🎯" });

async function load() {
  goals.value = await api.listGoals();
}
onMounted(load);
watch(version, load);

function openAdd() {
  editingId.value = null;
  form.value = { name: "", targetMinor: 0, currency: settings.value.baseCurrency, targetDate: "", color: "", note: "", icon: "🎯" };
  open.value = true;
}
function openEdit(g: Goal) {
  editingId.value = g.id;
  form.value = {
    name: g.name, targetMinor: g.targetMinor, currency: g.currency,
    targetDate: g.targetDate ? toISODateLocal(g.targetDate) : "", color: g.color, note: g.note, icon: g.icon || "🎯",
  };
  open.value = true;
}
async function save() {
  await api.saveGoal({
    id: editingId.value ?? undefined,
    ...form.value,
    targetDate: form.value.targetDate ? fromISODate(form.value.targetDate) : null,
    accountId: null,
  });
  open.value = false;
  bump();
}
async function remove(g: Goal) {
  if (!confirm(t("confirm.deleteBody"))) return;
  await api.deleteGoal(g.id);
  bump();
}
function pct(g: Goal) {
  if (g.targetMinor <= 0) return 0;
  return Math.min((g.savedMinor / g.targetMinor) * 100, 100);
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("goals.title") }}</h1>
      </div>
      <button class="btn btn-primary" @click="openAdd"><Icon name="add" /> {{ t("goals.new") }}</button>
    </div>

    <div v-if="!goals.length" class="card empty-state">
      <div class="emoji">🎯</div>
      {{ t("goals.noGoals") }}
    </div>

    <div class="grid" style="grid-template-columns: repeat(auto-fill, minmax(280px, 1fr))">
      <div v-for="g in goals" :key="g.id" class="card">
        <div style="display: flex; align-items: center; gap: 12px">
          <span class="avatar">{{ g.icon || "🎯" }}</span>
          <div class="grow">
            <div class="bold">{{ g.name }}</div>
            <div class="faint">{{ formatMoney(g.savedMinor, g.currency, intlLocale(settings.language)) }} / {{ formatMoney(g.targetMinor, g.currency, intlLocale(settings.language)) }}</div>
          </div>
          <button class="icon-btn" @click="openEdit(g)"><Icon name="edit" :size="15" /></button>
          <button class="icon-btn danger" @click="remove(g)"><Icon name="trash" :size="15" /></button>
        </div>
        <div style="margin-top: 12px">
          <div class="progress"><span :style="{ width: pct(g) + '%' }"></span></div>
          <div class="faint" style="margin-top: 6px">{{ Math.round(pct(g)) }}% · {{ t("goals.progress") }}</div>
        </div>
      </div>
    </div>

    <Modal v-if="open" :title="editingId === null ? t('goals.new') : t('common.edit')" @close="open = false">
      <div class="field">
        <label>{{ t("common.name") }}</label>
        <input class="input" v-model="form.name" />
      </div>
      <div class="field-row">
        <div class="field">
          <label>{{ t("goals.target") }}</label>
          <MoneyInput v-model="form.targetMinor" :currency="form.currency" />
        </div>
        <div class="field">
          <label>{{ t("common.currency") }}</label>
          <CurrencySelect v-model="form.currency" />
        </div>
      </div>
      <div class="field">
        <label>{{ t("goals.targetDate") }}</label>
        <input class="input" type="date" v-model="form.targetDate" />
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
  </div>
</template>
