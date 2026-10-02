<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import Icon from "../components/Icon.vue";
import TransactionEditor from "../components/TransactionEditor.vue";
import { api, Transaction, Account, Category } from "../lib/api";
import { settings, bump, version } from "../lib/store";
import { formatMoney, formatDate, intlLocale } from "../lib/format";

const { t } = useI18n();

const txns = ref<Transaction[]>([]);
const accounts = ref<Account[]>([]);
const categories = ref<Category[]>([]);
const search = ref("");
const kindFilter = ref("");
const accountFilter = ref<number | null>(null);

const editorOpen = ref(false);
const editing = ref<Transaction | null>(null);
const editorKind = ref("expense");

async function load() {
  const filter: Record<string, unknown> = { limit: 1000 };
  if (kindFilter.value) filter.kind = kindFilter.value;
  if (accountFilter.value) filter.accountId = accountFilter.value;
  if (search.value) filter.search = search.value;
  const [tlist, alist, clist] = await Promise.all([
    api.listTransactions(filter),
    api.listAccounts(),
    api.listCategories(),
  ]);
  txns.value = tlist;
  accounts.value = alist;
  categories.value = clist;
}

onMounted(load);
watch(version, load);

function openAdd(kind: string) {
  editing.value = null;
  editorKind.value = kind;
  editorOpen.value = true;
}
function openEdit(tx: Transaction) {
  editing.value = tx;
  editorOpen.value = true;
}
async function remove(tx: Transaction) {
  if (!confirm(t("confirm.deleteBody"))) return;
  await api.deleteTransaction(tx.id);
  bump();
}

function kindLabel(k: string) {
  return t("txn." + k);
}

function catOf(id: number | null) {
  return categories.value.find((c) => c.id === id);
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("nav.transactions") }}</h1>
        <p class="page-sub">{{ txns.length }} {{ t("common.total") }}</p>
      </div>
      <div style="display: flex; gap: 8px">
        <button class="btn btn-primary" @click="openAdd('expense')">
          <Icon name="add" /> {{ t("txn.addExpense") }}
        </button>
        <button class="btn" @click="openAdd('income')">
          <Icon name="add" /> {{ t("txn.addIncome") }}
        </button>
        <button class="btn" @click="openAdd('transfer')">
          <Icon name="add" /> {{ t("txn.addTransfer") }}
        </button>
      </div>
    </div>

    <div class="toolbar">
      <div style="position: relative; flex: 1; min-width: 180px">
        <input class="input" v-model="search" :placeholder="t('common.search')" @input="load" />
      </div>
      <select class="select" style="width: auto" v-model="kindFilter" @change="load">
        <option value="">{{ t("common.all") }}</option>
        <option value="expense">{{ t("txn.expense") }}</option>
        <option value="income">{{ t("txn.income") }}</option>
        <option value="transfer">{{ t("txn.transfer") }}</option>
      </select>
      <select class="select" style="width: auto" v-model="accountFilter" @change="load">
        <option :value="null">{{ t("common.all") }}</option>
        <option v-for="a in accounts" :key="a.id" :value="a.id">{{ a.name }}</option>
      </select>
    </div>

    <div class="card" style="padding: 6px 12px">
      <div v-if="!txns.length" class="empty-state">
        <div class="emoji">🧾</div>
        {{ t("txn.noTransactions") }}
      </div>
      <table v-else class="table">
        <thead>
          <tr>
            <th>{{ t("common.date") }}</th>
            <th>{{ t("common.category") }}</th>
            <th>{{ t("txn.payee") }}</th>
            <th>{{ t("common.account") }}</th>
            <th class="right">{{ t("common.amount") }}</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="tx in txns" :key="tx.id" @dblclick="openEdit(tx)">
            <td class="muted">{{ formatDate(tx.occurredAt, settings.language) }}</td>
            <td>
              <span class="badge" :class="tx.kind === 'income' ? 'income' : tx.kind === 'expense' ? 'expense' : ''">
                {{ catOf(tx.categoryId)?.name || kindLabel(tx.kind) }}
              </span>
            </td>
            <td>{{ tx.payee }}</td>
            <td class="muted">{{ tx.accountName }}</td>
            <td class="right bold" :class="tx.kind === 'income' ? 'amount-pos' : 'amount-neg'">
              {{ tx.kind === "income" ? "+" : tx.kind === "transfer" ? "↔ " : "-"
              }}{{ formatMoney(tx.amountMinor, tx.currency, intlLocale(settings.language)) }}
              <div v-if="tx.currency !== settings.baseCurrency" class="faint" style="font-weight: 400">
                {{ formatMoney(tx.amountBaseMinor, settings.baseCurrency, intlLocale(settings.language)) }}
              </div>
            </td>
            <td style="width: 70px">
              <button class="icon-btn" @click="openEdit(tx)"><Icon name="edit" :size="15" /></button>
              <button class="icon-btn danger" @click="remove(tx)"><Icon name="trash" :size="15" /></button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <TransactionEditor
      :open="editorOpen"
      :initial="editing"
      :kind="editorKind"
      @close="editorOpen = false"
      @saved="bump"
    />
  </div>
</template>
