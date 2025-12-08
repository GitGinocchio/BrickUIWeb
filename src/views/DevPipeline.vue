<template>
    <iframe
        src="https://trello.com/b/0H5Yn2mK.html"
        color="black"
        style="filter:  invert(10)
                        hue-rotate(180deg)
                        brightness(0.3)
                        contrast(3)
                        grayscale(1)
                        saturate(1);"
        :width="pageWidth" 
        :height="pageHeight"
    />
</template>

<style>
iframe {
    margin-top: 64px;
    border: none;
}
</style>

<script setup lang="ts">
import { computed } from 'vue';

const pageHeight = computed(() => {
    return Math.max(
        document.body.scrollHeight,
        document.documentElement.scrollHeight,
        document.body.offsetHeight,
        document.documentElement.offsetHeight,
        document.body.clientHeight,
        document.documentElement.clientHeight
    );
});

const pageWidth = computed(() => {
    return Math.max(
        document.body.scrollWidth,
        document.documentElement.scrollWidth,
        document.body.offsetWidth,
        document.documentElement.offsetWidth,
        document.body.clientWidth,
        document.documentElement.clientWidth
    );
});

</script>