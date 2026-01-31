<template>
    <div class="container">
        <Hero class="hero"/>
        <Features />
        <FlickeringBackground 
            :color="colors.dark.accentForeground" 
            :square-size="15"
            :flicker-chance="0.5"
            :max-opacity="0.2"
        />
    </div>
</template>

<script setup lang="ts">
import FlickeringBackground from "@/components/FlickeringBackground.vue";
import Hero from "@/components/Hero.vue";

import { computed, onMounted } from "vue";
import { colors } from "@/themes/colors.ts";
import Features from "@/components/Features.vue";

</script>

<style scoped>
.container {
    flex-grow:1;
}

.hero {
    flex-grow: 1;
}
</style>
