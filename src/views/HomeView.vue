<template>
    <FlickeringBackground 
        :color="colors.dark.accentForeground" 
        :square-size="15"
        :flicker-chance="0.25"
        :max-opacity="0.2"
    />
    <Header />
    <Hero />
</template>

<script setup lang="ts">
import FlickeringBackground from "@/components/FlickeringBackgroung.vue";
import Header from "@/components/Header.vue";
import Hero from "@/components/Hero.vue";

import { computed, onMounted } from "vue";
import { colors } from "@/themes/colors.ts";

</script>

<style>
body {
    background-color: black;
}
</style>
