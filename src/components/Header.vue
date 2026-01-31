<template>
  <header :style="{ backgroundColor: backgroundColor }">
    <div class="container">
      <!-- Logo + brand -->
      <div class="brand">
        <img src="../assets/logo.svg" alt="Logo" />
        <div>
          <h2>BrickUI</h2>
          <p class="short-description">Desktop Customization</p>
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

      <!-- Download button -->
      <div class="download-btn">
        <NButton secondary medium @click="redirectToDownload">
          <template #icon>
              <DownloadIcon />
          </template>
          Download
        </NButton>
      </div>

      <div class="burger-menu-btn">
        <NButton secondary medium @click.stop="drawerVisible = !drawerVisible">
            <template #icon>
                <MenuIcon v-if="!drawerVisible" />
                <XIcon v-else></XIcon>
            </template>
            Menu
        </NButton>
      </div>
    </div>
  </header>
  <Teleport to="body">
    <Transition name="drawer">
      <div
        v-if="drawerVisible"
        ref="drawerRef"
        class="nav-mobile"
        :style="{ backgroundColor: backgroundColor }"
      >
        <TransitionGroup name="stagger" tag="div" class="nav-mobile-inner">
          <RouterLink to="/" custom v-slot="{navigate, isActive}" :key="'home'">
            <NButton @click="(e) => { navigate(e); drawerVisible=false }" :class="{'is-active': isActive}" quaternary block>
              Home
            </NButton>
          </RouterLink>

          <RouterLink to="/about" custom v-slot="{navigate, isActive}" :key="'about'">
            <NButton @click="(e) => { navigate(e); drawerVisible=false }" :class="{'is-active': isActive}" quaternary block>
              About
            </NButton>
          </RouterLink>

          <RouterLink to="/contact" custom v-slot="{navigate, isActive}" :key="'contact'">
            <NButton @click="(e) => { navigate(e); drawerVisible=false }" :class="{'is-active': isActive}" quaternary block>
              Contatti
            </NButton>
          </RouterLink>

          <RouterLink to="/download" custom v-slot="{navigate, isActive}" :key="'download'">
            <NButton @click="(e) => { navigate(e); drawerVisible=false; redirectToDownload() }" :class="{'is-active': isActive}" primary block>
              <template #icon>
                <DownloadIcon />
              </template>
              Download
            </NButton>
          </RouterLink>
        </TransitionGroup>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from "vue";
import { NButton } from "naive-ui";
import { Download as DownloadIcon, Menu as MenuIcon, XIcon } from "lucide-vue-next";
import { RouterLink } from "vue-router";

function redirectToDownload() {
  window.location.href = "/api/download/latest";
}

const scrollY = ref(0);
const drawerVisible = ref(false);
const drawerRef = ref<HTMLElement | null>(null);

function handleClickOutside(event: Event) {
  if (!drawerVisible.value) return;

  const drawerEl = drawerRef.value;
  const target = event.target as Node;

  if (drawerEl && !drawerEl.contains(target)) {
    drawerVisible.value = false;
  }
}

const backgroundColor = computed(() => {
  return `rgba(0,0,0,${Math.max(1 - scrollY.value / 250, 0.015)})`;
});

function handleScroll() {
  scrollY.value = window.scrollY;
}

onMounted(() => {
  window.addEventListener("scroll", handleScroll);
  window.addEventListener("scroll", handleClickOutside);
  window.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
  window.removeEventListener("scroll", handleScroll);
  window.removeEventListener("scroll", handleClickOutside);
  window.removeEventListener("click", handleClickOutside);
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
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
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

.burger-menu-btn {
  display: none;
}

/* Drawer mobile */
.nav-mobile {
  display: none;
  flex-direction: column;
  position: fixed;
  margin-top: 0.5rem;
  margin-right: 1rem;
  top: 64px;
  right: 0;
  width: 200px;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  border-radius: 1rem;
  padding: 1rem;
  gap: 0.5rem;
  z-index: 1001;
  overflow: visible;
}

/* === DRAWER SLIDE DOWN === */
.drawer-enter-from {
  opacity: 0;
  transform: translateY(-20px) scale(0.98);
}

.drawer-enter-active {
  transition: all 0.25s cubic-bezier(.22,1,.36,1);
}

.drawer-enter-to {
  opacity: 1;
  transform: translateY(0) scale(1);
}

.drawer-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}

.drawer-leave-active {
  transition: all 0.15s ease-in;
}

.drawer-leave-to {
  opacity: 0;
  transform: translateY(-10px) scale(0.98);
}

.nav-mobile-inner {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

/* stato iniziale */
.stagger-enter-from {
  opacity: 0;
  transform: translateY(-10px);
}

/* animazione */
.stagger-enter-active {
  transition: all 0.35s ease;
}

/* stato finale */
.stagger-enter-to {
  opacity: 1;
  transform: translateY(0);
}

.stagger-enter-active:nth-child(1) { transition-delay: 0.10s; }
.stagger-enter-active:nth-child(2) { transition-delay: 0.20s; }
.stagger-enter-active:nth-child(3) { transition-delay: 0.30s; }
.stagger-enter-active:nth-child(4) { transition-delay: 0.40s; }


@media (max-width: 1400px) {
    .container {
      margin-left: 2rem;
      margin-right: 2rem;
    }

    .nav-desktop {
      display: none;
    }

    .short-description {
      display: none;
    }

    .download-btn {
      display: none;
    }

    .burger-menu-btn {
      display: flex;
    }

    .nav-mobile {
      display: flex;
    }
}
</style>
