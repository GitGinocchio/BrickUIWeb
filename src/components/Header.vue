<template>
    <header :style="{ backgroundColor: backgroundColor }">
        <div class="container">
            <div class="brand">
                <img src="../assets/logo.svg"></img>
                <div>
                    <h2>BrickUI</h2>
                    <p>Desktop Customization</p>
                </div>
            </div>
            <nav>
                <NButton quaternary medium>Home</NButton>
                <NButton quaternary medium>Chi siamo</NButton>
                <NButton quaternary medium>Contatti</NButton>
            </nav>
            <div>
                <NButton secondary medium @click="redirectToDownload">
                    <template #icon>
                        <DownloadIcon />
                    </template>
                    Download
                </NButton>
                <!--
                <NButton secondary medium>
                    <template #icon>
                        <DownloadIcon />
                    </template>
                    Login
                </NButton>
                -->
            </div>
        </div>
    </header>
</template>

<script setup lang="ts">
import { computed, h, onMounted, onUnmounted, ref } from "vue";
import { NButton } from "naive-ui";
import { Download as DownloadIcon } from "lucide-vue-next";

function redirectToDownload() {
    window.location.href = "/api/download/latest";
}

const scrollY = ref(0);

const backgroundColor = computed(() => {
    return `rgba(0,0,0,${Math.max(1 - scrollY.value / 250, 0.015)})`;
});

function handleScroll() {
  scrollY.value = window.scrollY
}

onMounted(() => {
  window.addEventListener('scroll', handleScroll)
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
})
</script>

<style scoped> 
header {
    position: fixed; /* rimane in cima */
    top: 0;
    left: 0;
    width: 100%;
    align-items: center;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(8px); /* sfocatura del background */
    -webkit-backdrop-filter: blur(8px); /* compatibilità Safari */
    background: rgba(0, 0, 0, 1); /* trasparente leggermente bianco */
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    z-index: 1000;
    height: 64px;
}

header div, nav {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.container {
    display: flex;
    justify-content: space-between;
    margin-left: 16.25rem;
    margin-right: 16.25rem;
    height: 64px;
}

.brand {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: .75rem;
    color: white;
}

.brand div {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0;
}

.brand h1 {
    font-weight: 700;
}

.brand p {
    color: #737373;
    font-size: .75rem;
    line-height: 1rem;
}

.brand img { 
    width: 40px;
    aspect-ratio: 1;
}
</style>