<template>
    <NConfigProvider :theme-overrides="darkTheme">
      <div class="layout">
        <Header />
        <main class="content">
          <NSpin v-if="isLoading" size="large" class="page-loading" />
          <RouterView v-else />
        </main>
        <Footer />
      </div>
    </NConfigProvider>
</template>

<script setup lang="ts">
import { RouterLink, RouterView, useRouter } from "vue-router";
import Header from "@/components/Header.vue";
import Footer from "./components/Footer.vue";
import { NConfigProvider, NSpin } from "naive-ui"
import { dark as darkTheme } from "./themes/dark"
import { ref } from "vue";

const isLoading = ref(false);
const router = useRouter();

router.beforeEach((to, from, next) => {
  isLoading.value = true;
  next();
});

router.afterEach(() => {
  setTimeout(() => {
    isLoading.value = false;
  }, 5);
});
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
  min-height: 100vh; /* altezza totale della finestra */
}

.content {
  flex: 1;  /* OCCUPA tutto lo spazio rimanente */
  padding-top: 64px;
  display: flex;
  min-height: 0;
  flex-direction: column;
}

.page-loading {
  flex-grow: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
}

@media (max-width: 1400px) {
  .content {
    padding-top: 0;
  }
}

</style>
