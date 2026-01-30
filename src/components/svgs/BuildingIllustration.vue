<template>
  <div class="svg-wrapper" v-html="svgContent" :data-theme="theme"></div>
</template>


<script setup lang="ts">
import { ref } from 'vue'
import svgContentRaw from '@/assets/images/Building.svg?raw';

const props = defineProps({ theme: String });
const svgContent = ref(svgContentRaw);
</script>

<style scoped>
.svg-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 700px;
}
</style>