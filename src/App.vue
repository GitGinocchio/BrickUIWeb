<template>
    <NConfigProvider :theme-overrides="darkTheme">
      <div class="layout">
        <Header />
        <main class="content">
          <RouterView />
        </main>
        <Footer />
      </div>
    </NConfigProvider>
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import Header from "@/components/Header.vue";
import Footer from "./components/Footer.vue";
import { NConfigProvider } from "naive-ui"
import { dark as darkTheme } from "./themes/dark"
import { ref } from "vue";
</script>

<style scoped>
.container {
  display:flex;
  flex-direction: column;
  min-height: 100vh;
}

.layout {
  display: flex;
  flex-direction: column;
  height: 100vh; /* altezza totale della finestra */
}

.content {
  flex: 1;  /* OCCUPA tutto lo spazio rimanente */
  display: flex;
  flex-direction: column;
}

body {
  font-family: ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";
}
</style>
