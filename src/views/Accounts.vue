<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import Icon from "../components/Icon.vue";
import Modal from "../components/Modal.vue";
import CurrencySelect from "../components/CurrencySelect.vue";
import MoneyInput from "../components/MoneyInput.vue";
import { api, Account } from "../lib/api";
import { settings, bump, version } from "../lib/store";
import { formatMoney, intlLocale } from "../lib/format";

const { t } = useI18n();
const accounts = ref<Account[]>([]);
const open = ref(false);
const editingId = ref<number | null>(null);
const form = ref({ name: "", kind: "cash", currency: "TRY", openingBalanceMinor: 0, icon: "💵", color: "" });

const kinds = [
  { key: "cash", icon: "💵" },
  { key: "bank", icon: "🏦" },
  { key: "ewallet", icon: "📱" },
  { key: "credit_card", icon: "💳" },
  { key: "savings", icon: "🏆" },
];

async function load() {
  accounts.value = await api.listAccounts();
}
onMounted(load);
watch(version, load);

function kindIcon(k: string) {
  return kinds.find((x) => x.key === k)?.icon || "💵";
}

function openAdd() {
  editingId.value = null;
  form.value = { name: "", kind: "cash", currency: settings.value.baseCurrency, openingBalanceMinor: 0, icon: "💵", color: "" };
  open.value = true;
}
function openEdit(a: Account) {
  editingId.value = a.id;
  form.value = { name: a.name, kind: a.kind, currency: a.currency, openingBalanceMinor: a.openingBalanceMinor, icon: a.icon, color: a.color };
  open.value = true;
}
async function save() {
  await api.saveAccount({ id: editingId.value ?? undefined, ...form.value });
  open.value = false;
  bump();
}
async function remove(a: Account) {
  if (!confirm(t("confirm.deleteBody"))) return;
  await api.deleteAccount(a.id);
  bump();
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("accounts.title") }}</h1>
      </div>
      <button class="btn btn-primary" @click="openAdd"><Icon name="add" /> {{ t("accounts.new") }}</button>
    </div>

    <div v-if="!accounts.length" class="card empty-state">
      <div class="emoji">💳</div>
      {{ t("accounts.noAccounts") }}
    </div>

    <div class="grid" style="grid-template-columns: repeat(auto-fill, minmax(280px, 1fr))">
      <div v-for="a in accounts" :key="a.id" class="card">
        <div style="display: flex; align-items: center; gap: 12px">
          <span class="avatar" :style="a.color ? { background: a.color + '22', color: a.color } : {}">{{ a.icon }}</span>
          <div class="grow">
            <div class="bold">{{ a.name }}</div>
            <div class="faint">{{ t("accounts." + a.kind) }}</div>
          </div>
          <button class="icon-btn" @click="openEdit(a)"><Icon name="edit" :size="15" /></button>
          <button class="icon-btn danger" @click="remove(a)"><Icon name="trash" :size="15" /></button>
        </div>
        <div style="margin-top: 14px">
          <div class="faint">{{ t("accounts.balance") }}</div>
          <div class="bold" style="font-size: 20px">{{ formatMoney(a.balanceMinor, a.currency, intlLocale(settings.language)) }}</div>
        </div>
      </div>
    </div>

    <Modal v-if="open" :title="editingId === null ? t('accounts.new') : t('common.edit')" @close="open = false">
      <div class="field">
        <label>{{ t("accounts.name") }}</label>
        <input class="input" v-model="form.name" />
      </div>
      <div class="field">
        <label>{{ t("accounts.kind") }}</label>
        <div class="segmented" style="flex-wrap: wrap">
          <button v-for="k in kinds" :key="k.key" :class="{ active: form.kind === k.key }" @click="form.kind = k.key; form.icon = k.icon">
            {{ k.icon }} {{ t("accounts." + k.key) }}
          </button>
        </div>
      </div>
      <div class="field-row">
        <div class="field">
          <label>{{ t("common.currency") }}</label>
          <CurrencySelect v-model="form.currency" />
        </div>
        <div class="field">
          <label>{{ t("accounts.openingBalance") }}</label>
          <MoneyInput v-model="form.openingBalanceMinor" :currency="form.currency" />
        </div>
      </div>
      <div class="field-row">
        <div class="field">
          <label>{{ t("common.icon") }}</label>
          <input class="input" v-model="form.icon" />
        </div>
        <div class="field">
          <label>{{ t("common.color") }}</label>
          <input class="input" type="color" v-model="form.color" style="padding: 3px; height: 38px" />
        </div>
      </div>
      <div class="modal-actions">
        <button class="btn" @click="open = false">{{ t("common.cancel") }}</button>
        <button class="btn btn-primary" @click="save">{{ t("common.save") }}</button>
      </div>
    </Modal>
  </div>
</template>
