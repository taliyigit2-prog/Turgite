<script setup lang="ts">
import { ref, onMounted, watch, computed } from "vue";
import { useI18n } from "vue-i18n";
import Icon from "../components/Icon.vue";
import Modal from "../components/Modal.vue";
import { api, Category } from "../lib/api";
import { bump, version } from "../lib/store";

const { t } = useI18n();
const categories = ref<Category[]>([]);
const open = ref(false);
const editingId = ref<number | null>(null);
const tab = ref("expense");
const form = ref({ name: "", kind: "expense", parentId: null as number | null, icon: "🏷️", color: "", keywords: "" });

const emojis = ["🏷️", "🍔", "☕", "🚗", "🏠", "💊", "🎬", "✈️", "🛒", "🎁", "📚", "💼", "🧾", "🎮", "👶", "💡"];

async function load() {
  categories.value = await api.listCategories();
}
onMounted(load);
watch(version, load);

const list = computed(() => categories.value.filter((c) => c.kind === tab.value));

function openAdd() {
  editingId.value = null;
  form.value = { name: "", kind: tab.value, parentId: null, icon: "🏷️", color: "", keywords: "" };
  open.value = true;
}
function openEdit(c: Category) {
  editingId.value = c.id;
  form.value = { name: c.name, kind: c.kind, parentId: c.parentId, icon: c.icon, color: c.color, keywords: c.keywords };
  open.value = true;
}
async function save() {
  await api.saveCategory({ id: editingId.value ?? undefined, ...form.value });
  open.value = false;
  bump();
}
async function remove(c: Category) {
  if (!confirm(t("confirm.deleteBody"))) return;
  await api.deleteCategory(c.id);
  bump();
}
</script>

<template>
  <div>
    <div class="page-head">
      <div>
        <h1 class="page-title">{{ t("categories.title") }}</h1>
      </div>
      <button class="btn btn-primary" @click="openAdd"><Icon name="add" /> {{ t("categories.new") }}</button>
    </div>

    <div class="segmented" style="margin-bottom: 16px">
      <button :class="{ active: tab === 'expense' }" @click="tab = 'expense'">{{ t("categories.expense") }}</button>
      <button :class="{ active: tab === 'income' }" @click="tab = 'income'">{{ t("categories.income") }}</button>
    </div>

    <div v-if="!list.length" class="card empty-state">
      <div class="emoji">🏷️</div>
      {{ t("categories.noCategories") }}
    </div>

    <div class="grid" style="grid-template-columns: repeat(auto-fill, minmax(240px, 1fr))">
      <div v-for="c in list" :key="c.id" class="card" style="padding: 14px">
        <div style="display: flex; align-items: center; gap: 12px">
          <span class="avatar" :style="c.color ? { background: c.color + '22', color: c.color } : {}">{{ c.icon || "🏷️" }}</span>
          <div class="grow">
            <div class="bold">{{ c.name }}</div>
            <div v-if="c.keywords" class="faint">{{ c.keywords }}</div>
          </div>
          <button class="icon-btn" @click="openEdit(c)"><Icon name="edit" :size="15" /></button>
          <button class="icon-btn danger" @click="remove(c)"><Icon name="trash" :size="15" /></button>
        </div>
      </div>
    </div>

    <Modal v-if="open" :title="editingId === null ? t('categories.new') : t('common.edit')" @close="open = false">
      <div class="field">
        <label>{{ t("common.name") }}</label>
        <input class="input" v-model="form.name" />
      </div>
      <div class="field-row">
        <div class="field">
          <label>{{ t("common.kind") }}</label>
          <select class="select" v-model="form.kind">
            <option value="expense">{{ t("categories.expense") }}</option>
            <option value="income">{{ t("categories.income") }}</option>
          </select>
        </div>
        <div class="field">
          <label>{{ t("categories.parent") }}</label>
          <select class="select" v-model="form.parentId">
            <option :value="null">{{ t("common.none") }}</option>
            <option v-for="c in list" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </div>
      </div>
      <div class="field">
        <label>{{ t("common.icon") }}</label>
        <div style="display: flex; gap: 4px; flex-wrap: wrap">
          <button
            v-for="e in emojis"
            :key="e"
            class="icon-btn"
            :style="form.icon === e ? { background: 'var(--accent-soft)' } : {}"
            @click="form.icon = e"
          >
            {{ e }}
          </button>
        </div>
      </div>
      <div class="field">
        <label>{{ t("categories.keywords") }} <span class="faint">({{ t("categories.keywordsHint") }})</span></label>
        <input class="input" v-model="form.keywords" />
      </div>
      <div class="field">
        <label>{{ t("common.color") }}</label>
        <input class="input" type="color" v-model="form.color" style="padding: 3px; height: 38px" />
      </div>
      <div class="modal-actions">
        <button class="btn" @click="open = false">{{ t("common.cancel") }}</button>
        <button class="btn btn-primary" @click="save">{{ t("common.save") }}</button>
      </div>
    </Modal>
  </div>
</template>
