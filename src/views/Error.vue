<template>
  <n-result :status="(status_code as any)" />
</template>

<script setup lang="ts">
import { NResult } from 'naive-ui';
import { computed } from 'vue';


let status_code = computed(() => {
  const params = new URLSearchParams(window.location.search)
  const status_code = params.get("status_code");
  return ["500", "404", "403", "418"].includes(status_code as any) ? status_code : 'error'
});

</script>