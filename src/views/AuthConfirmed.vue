<template>
  <div class="container">
    <NCard class="card" size="large" >
      <h1>🎉 Benvenuto su BrickUI!</h1>
      <p v-if="isDesktop">
        Il tuo account è stato confermato. Stiamo aprendo l'app desktop...
      </p>
      <p v-else>
        Il tuo account è stato confermato! Per continuare, apri l'app BrickUI sul desktop.
      </p>

      <div v-if="isDesktop" id="fallback">
        <p>Se l'app non si apre automaticamente, clicca qui:</p>
        <button @click="openApp" class="button">Apri BrickUI</button>
        <a href="/" class="button secondary">Continua sul web</a>
      </div>
    </NCard>
    <FlickeringBackground 
      :color="colors.dark.accentForeground" 
      :square-size="15"
      :flicker-chance="0.5"
      :max-opacity="0.15"
    />
  </div>
</template>

<script setup lang="ts">
import { NCard } from 'naive-ui';
import FlickeringBackground from '@/components/FlickeringBackground.vue';
import { ref, onMounted } from 'vue';
import { colors } from '@/themes/colors';

const isDesktop = ref(false);

function detectDesktop() {
  // Moderni browser: navigator.userAgentData
  if ((navigator as any).userAgentData) {
    return !(navigator as any).userAgentData.mobile;
  }

  // Fallback classico
  return !/Mobi|Android|iPhone|iPad|iPod/i.test(navigator.userAgent);
}

function openApp() {
  window.location.href = `brickui://auth/confirmed/${window.location.hash}`;
}

onMounted(() => {
  isDesktop.value = detectDesktop();
  if (!isDesktop.value) return;

  setTimeout(() => openApp(), 700);
});
</script>

<style scoped>
.container {
  font-family: sans-serif;
  text-align: center;
  padding: 2rem;
  margin-top: 8rem;
  flex-grow:1;
  align-content: center;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: calc(100vh - 8rem); /* Adjust based on header/footer height */
}

.card {
  max-width: 30rem;
  width: 100%;
}
</style>
