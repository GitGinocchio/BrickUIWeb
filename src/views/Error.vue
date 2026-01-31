<template>
  <div class="container">
    <FlickeringBackground 
      :color="colors.dark.accentForeground" 
      :square-size="15"
      :flicker-chance="0.5"
      :max-opacity="0.15"
    />
    <NResult
      :status="(icon as any)"
      :title="title"
      :description="errorMessage"
      :theme-overrides="{ titleTextColor: titleTextColor }"
    >
      <template #footer>
        <NButton secondary @click="router.push('/')">Back to home</NButton>
        <NButton quaternary @click="router.go(-1)">Go back</NButton>
      </template>
    </NResult>
  </div>
</template>

<script setup lang="ts">
import { colors } from '@/themes/colors';
import FlickeringBackground from '@/components/FlickeringBackground.vue';
import { NResult, NButton } from 'naive-ui';
import {  useI18n } from "vue-i18n";
import { computed } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const { t } = useI18n();

const params = computed(() => {
  return new URLSearchParams(window.location.search);
});

const status_code = computed(() => {
  const code = params.value.get("status_code");
  return code ? code : "404";
});

const icon = computed(() => {
  const code = status_code.value;

  // Se il parametro è assente → default "404"
  if (!code) return "404";

  // Se è uno dei valori semanticamente validi
  if (["info", "warning", "success", "error"].includes(code)) {
    return code;
  }

  // Se è un numero, convertilo
  const num = parseInt(code, 10);
  if (!isNaN(num)) {
    if (num >= 500) return "500";
    if (num === 403) return "403";
    if (num === 418) return "418";
    if (num >= 400) return "404";
    if (num >= 200 && num <= 299) return "success";
  }

  // fallback generico
  return "error";
});

const titleTextColor = computed(() => {
  switch (icon.value) {
    case "success":
      return "#18a058";
    case "warning":
      return "#f0a020";
    case "info":
      return "#2080f0";
    default:
      return "#cb4153"
  }
});

const title = computed(() => {
  const title_param = params.value.get('title');
  if (title_param) return title_param

  return t(`title-${status_code.value}`, t("unhandled-title", String(status_code.value)));
});

const errorMessage = computed(() => {
  const message_param = params.value.get('message');
  if (message_param) return message_param

  return t(status_code.value, t("unhandled-error-message", "Unhandled error"));
});
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex-grow:1;
  overflow-y: hidden;
  justify-content: center;
}

:deep(.n-result-header__title) { 
  font-size: 5rem !important;
}

:deep(.n-result-header__description) {
  font-size: 2rem !important;
  color: #737373;
}

:deep(.n-result-footer) {
  display: flex;
  flex-direction: row;
  justify-content: center;
  gap: 1rem; 
}
</style>