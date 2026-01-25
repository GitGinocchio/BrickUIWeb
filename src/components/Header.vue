<template>
  <header :style="{ backgroundColor: backgroundColor }">
    <div class="container">
      <!-- Logo + brand -->
      <div class="brand">
        <img src="../assets/logo.svg" alt="Logo" />
        <div>
          <h2>BrickUI</h2>
          <p>Desktop Customization</p>
        </div>
      </div>

      <!-- Nav desktop -->
      <nav class="nav-desktop">
          <RouterLink to="/" custom v-slot="{navigate, isActive}">
              <NButton @click="navigate" :class="{'is-active': isActive}" quaternary medium>Home</NButton>
          </RouterLink>
          <RouterLink to="/about" custom v-slot="{navigate, isActive}">
              <NButton @click="navigate" :class="{'is-active': isActive}" quaternary medium>About</NButton>
          </RouterLink>
          <RouterLink to="/contact" custom v-slot="{navigate, isActive}">
              <NButton @click="navigate" :class="{'is-active': isActive}" quaternary medium>Contatti</NButton>
          </RouterLink>
      </nav>

      <!-- Burger mobile -->
      <div class="burger" @click="drawerVisible = !drawerVisible">
        <span :class="{ open: drawerVisible }"></span>
        <span :class="{ open: drawerVisible }"></span>
        <span :class="{ open: drawerVisible }"></span>
      </div>

      <!-- Download button -->
      <div class="download-btn">
        <NButton secondary medium @click="redirectToDownload">
          <template #icon>
              <DownloadIcon />
          </template>
          Download
        </NButton>
      </div>
    </div>

    <!-- Drawer mobile -->
    <div class="nav-mobile" v-show="drawerVisible">
      <RouterLink to="/" custom v-slot="{navigate, isActive}">
           <NButton @click="navigate; drawerVisible=false" :class="{'is-active': isActive}" quaternary block>Home</NButton>
      </RouterLink>
      <RouterLink to="/about" custom v-slot="{navigate, isActive}">
           <NButton @click="navigate; drawerVisible=false" :class="{'is-active': isActive}" quaternary block>About</NButton>
      </RouterLink>
      <RouterLink to="/contact" custom v-slot="{navigate, isActive}">
           <NButton @click="navigate; drawerVisible=false" :class="{'is-active': isActive}" quaternary block>Contatti</NButton>
      </RouterLink>
    </div>
  </header>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from "vue";
import { NButton } from "naive-ui";
import { Download as DownloadIcon } from "lucide-vue-next";
import { RouterLink } from "vue-router";

function redirectToDownload() {
  window.location.href = "/api/download/latest";
}

const scrollY = ref(0);
const drawerVisible = ref(false);

const backgroundColor = computed(() => {
  return `rgba(0,0,0,${Math.max(1 - scrollY.value / 250, 0.015)})`;
});

function handleScroll() {
  scrollY.value = window.scrollY;
}

onMounted(() => {
  window.addEventListener("scroll", handleScroll);
});

onUnmounted(() => {
  window.removeEventListener("scroll", handleScroll);
});
</script>

<style scoped>
header {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    align-items: center;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    background: rgba(0, 0, 0, 1);
    z-index: 1000;
    height: 64px;
}

nav {
    position: absolute;
    left: 50.30%;
    transform: translateX(-50%);
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

.is-active{
    background-color: #a0323f;
}

/* BURGER MOBILE */
.burger {
    display: none;
    flex-direction: column;
    gap: 5px;
    cursor: pointer;
}
.burger span {
    width: 25px;
    height: 3px;
    background: white;
    display: block;
    transition: 0.3s;
}
.burger span.open:nth-child(1) {
    transform: rotate(45deg) translate(5px, 5px);
}
.burger span.open:nth-child(2) {
    opacity: 0;
}
.burger span.open:nth-child(3) {
    transform: rotate(-45deg) translate(5px, -5px);
}

/* Drawer mobile */
.nav-mobile {
    display: none;
    flex-direction: column;
    position: absolute;
    top: 64px;
    right: 0;
    width: 200px;
    background: rgba(0,0,0,0.95);
    padding: 1rem;
    gap: 0.5rem;
    z-index: 1001;
}

/* MEDIA QUERY */
@media (max-width: 768px) {
    .nav-wrapper.nav-desktop {
        display: none;
    }
    .burger {
        display: flex;
    }
    .nav-mobile {
        display: flex;
    }
}
</style>
