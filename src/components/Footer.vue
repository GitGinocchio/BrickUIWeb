<template>
    <footer>
        <div class="container">
            <div class="grid">
                <div class="brand">
                    <div class="logo">
                        <img src="../assets/logo.svg"></img>
                        <span>BrickUI</span>
                    </div>
                    <p>Build your dream desktop brick by brick with Powerful and customizable widgets for Windows.</p>
                </div>
                <div class="link-list">
                    <h4>Product</h4>
                    <ul>
                        <li><a href="/features">Features</a></li>
                        <li><a href="/api/download/latest">Download</a></li>
                        <li><a href="/documentation">Documentation</a></li>
                        <li><a href="/devpipeline">Pipeline</a></li>
                    </ul>
                </div>
                <div class="link-list">
                    <h4>Community</h4>
                    <ul>
                        <li><a href="https://github.com/BrickUIApp">Github</a></li>
                        <li><a href="#">Discord</a></li>
                        <li><a href="#">Contribute</a></li>
                    </ul>
                </div>
                <div class="link-list">
                    <h4>Connect</h4>
                    <div class="link-buttons">
                        <NButton secondary @click="">
                            <template #icon>
                                <Github></Github>
                            </template>
                        </NButton>
                        <NButton secondary @click="">
                            <template #icon>
                                <Twitter></Twitter>
                            </template>
                        </NButton>
                        <NButton secondary @click="">
                            <template #icon>
                                <Mail></Mail>
                            </template>
                        </NButton>
                    </div>
                </div>
            </div>
            <div class="copyright">
                <p>© 2025 BrickUI. All rights reserved. Made with ❤️</p>
            </div>
        </div>
    </footer>
</template>

<style scoped>

.container {
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(8px); /* sfocatura del background */
    -webkit-backdrop-filter: blur(8px); /* compatibilità Safari */
    background: rgba(255, 255, 255, 0.05); /* trasparente leggermente bianco */
    padding-top: 3rem;
    padding-bottom: 3rem;
    padding-left: 16.25rem;
    padding-right: 16.25rem;
}

.grid {
    display: grid;
    gap: 3rem;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    color: white;
    padding-bottom: 3rem;
}

.copyright {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 2rem;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    color: #737373;
}

/* Brand */

.brand {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.brand p {
    color: #737373;
}

.logo {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
}

.logo span {
    font-weight: 700;
    font-size: 18px;
}

.logo img {
    height: 32px;
    aspect-ratio: 1;
}

li {
    list-style: none;
    color: #737373;
}

a {
    text-decoration: none;
    color: #737373;
}

a:hover {
    color: #cb4153;
}

.link-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.link-buttons {
    display: flex;
    flex-direction: row;
    gap: .75rem;
}

.link-buttons button {
    width: 40px;
    height: 40px;
}
</style>

<script setup lang="ts">
import { Github, Mail, Twitter } from 'lucide-vue-next';
import { NButton } from 'naive-ui';
</script>