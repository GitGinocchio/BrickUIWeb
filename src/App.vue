<template>
    <NConfigProvider :theme-overrides="darkTheme">
        <RouterView />
    </NConfigProvider>
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import { NConfigProvider } from "naive-ui"
import { dark as darkTheme } from "./themes/dark"
import { ref } from "vue";
</script>

<style scoped>

</style>
