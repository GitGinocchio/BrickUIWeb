<template>
    <header>
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
                <NButton secondary medium>
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

<style scoped> 
header {
    position: sticky; /* rimane in cima */
    top: 0;
    width: 100%;
    align-items: center;
    backdrop-filter: blur(10px); /* sfocatura del background */
    -webkit-backdrop-filter: blur(10px); /* compatibilità Safari */
    background: rgba(255, 255, 255, 0.05); /* trasparente leggermente bianco */
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
    justify-content: space-evenly;
    margin-left: 16rem;
    margin-right: 16rem;
    padding-left: 1.5rem;
    padding-right: 1.5rem;
    height: 100%;
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

<script setup lang="ts">
import { h } from "vue";
import { NButton } from "naive-ui";
import { Download as DownloadIcon } from "lucide-vue-next";
</script>