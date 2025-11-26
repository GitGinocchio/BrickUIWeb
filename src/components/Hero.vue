<template>
    <section>
        <div class="main-container">
            <div class="slogan">
                <h1>Build your dream<span>Brick by Brick!</span></h1>
                <div class="slogan-description">
                    <p>
                        Transform your Windows desktop with beautiful, customizable widgets. <br>Create, arrange, 
                        and personalize your workspace exactly how you want it.
                    </p>
                </div>
                <div class="buttons">
                    <NButton class="download-nbutton" strong secondary size="large" @click="redirectToDownload" >
                    <template #icon>
                        <DownloadIcon />
                    </template>
                    Download on Windows
                    </NButton>

                    <NButton strong class="github-nbutton" tertiary type="primary" size="large" >
                        <template #icon>
                            <GithubIcon />
                        </template>
                        View On Github
                    </NButton>
                </div>
            </div>
            <div class="console-container">

            </div>
        </div>
    </section>
</template>

<style scoped>

section {
    padding-top: 13rem;
    padding-bottom: 8rem;
    align-self: center;
    width: 100%;
    display: flex;
}

.main-container{
    width: 100%;
    max-width: 1400px;
    margin-right: auto;
    margin-left: auto;
}


span{
    display: block;
    color: #cb4153;
}

h1{
    color: white;
    line-height: 1;
    font-size: 4.5rem;
    margin-bottom: 1rem;
}

.slogan .slogan-description p{
    font-size: 1.35rem;
    font-weight: 450;
    color: rgba(255, 255, 255, 0.7);
    margin-bottom: 1.5rem;
}

.slogan .buttons{
    display: flex;
    gap: 1rem;
}

.download-nbutton{
    --n-color-pressed: #7b2832 !important;
}

.github-nbutton{
    --n-color-focus: #fff !important;
}

.github-nbutton:deep(.n-button__content),
.github-nbutton:deep(.n-button__icon) {
    color: #1a1a1a;
    transition: color 0.3s ease;
}

.github-nbutton:hover:deep(.n-button__content),
.github-nbutton:hover:deep(.n-button__icon) {
    color: #ffffff;
}

.github-nbutton:hover:active:deep(.n-button__content),
.github-nbutton:hover:active:deep(.n-button__icon) {
    color: #fff;
}



</style>

<script setup lang="ts">
import { Download as DownloadIcon, Github as GithubIcon } from 'lucide-vue-next';
import { NButton } from 'naive-ui';

function redirectToDownload() {
    window.location.href = "/api/download/latest";
}
</script>