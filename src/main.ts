import './assets/main.css'

import { computed, createApp, ref } from 'vue'
import { darkTheme, lightTheme } from "naive-ui";
import App from './App.vue'
import router from './router'
import i18n from "./i18n";

const media = window.matchMedia('(prefers-color-scheme: dark)')
media.addEventListener('change', updateSystemTheme)

const systemIsDark = ref(false);

function updateSystemTheme() {
  systemIsDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches
}

const THEME: String = "light";

const theme = computed(() => {
  switch (THEME) {
    case "light":
      return lightTheme
    case "dark":
      return darkTheme
    case "system":
      updateSystemTheme();
      return systemIsDark.value ? darkTheme : lightTheme
  }
});

const app = createApp(App)

app.use(router)
app.use(i18n);

app.provide("theme", theme);

app.mount('#app')
