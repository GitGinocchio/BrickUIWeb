<template>
    <div class="container">
        <h2>Everything you need to <span>customize your desktop</span></h2>
        <p class="description">Powerful features designed to make desktop customization simple and enjoyable</p>
        <div class="card-container">
            <div class="card" v-for="feature in features">
                <div class="icon">
                    <component :is="feature.icon" :size="24" />
                </div>
                <h3>{{ feature.title }}</h3>
                <p>{{ feature.description }}</p>
            </div>
        </div>
    </div>
</template>

<style scoped>
.container {
    display: flex;
    flex-direction: column;
    align-items: center;
    color: white;
    margin-bottom: 25rem;
}

.container h2 {
    line-height: 1;
    font-size: 3rem;
    margin-bottom: 1rem;
}

.container p {
    font-size: 1.125rem;
    line-height: 1.75rem;
    color: rgba(255, 255, 255, 0.7);
}

.card-container {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    margin-top: 4rem;
    max-width: 1400px;
    gap: 1rem;
}

.card {
    padding: 1.5rem;
    border-radius: .75rem;
    border: 1px solid #e6e6e6;
    border-width: 1px;
    cursor:default;
    
    /* Transizione fluida */
    transition: all 0.3s cubic-bezier(.4, 0, .2, 1);
}

.card:hover {
    border-color: #cb4153;
    background-color: #e6e6e6;
    transform: translateY(-0.5rem);
}

.card:hover h3 {
    color: #cb4153;
}
.card:hover p {
    color: black;
}

.card h3 {
    font-weight: 600;
    font-size: 1.25rem;
    line-height: 1.75rem;
    margin-top: 1rem;
}

.card p {
    margin-top: 1rem;
}

.icon {
    background-color: #cb4153;
    border-radius: .75rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    width: 3rem;
    aspect-ratio: 1;

    /* Applica transizione fluida alle trasformazioni */
    transition-property: transform;
    transition-timing-function: cubic-bezier(.4, 0, .2, 1);
    transition-duration: 0.15s;
}

/* Esempio di hover per far muovere o scalare l'icona */
.card:hover .icon {
    transform: scale(1.1);
}

.container span {
    display: block;
    color: #cb4153;
}

@media (max-width: 1400px) {
    .container {
        margin-bottom: 5rem;
    }

    .card-container {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        margin-left: auto;
        margin-right: auto;
    }

    .card p {
        margin-top: 0;
    }

    .icon {
        width: 3.5rem;
    }

    .icon :deep(svg) {
        scale: 1.25 !important;
    }
}

@media (max-width: 1000px) {
    .container h2 {
        align-self: first baseline;
        margin-left: 2rem;
        font-size: 1.7rem;
    }

    .container .description {
        align-self: first baseline;
        font-size: 1rem;
        margin-left: 2rem;
        margin-right: 2rem;
    }

    li, p, a {
        font-size: 5rem;
    }

    .card-container {
        margin-left: 2rem;
        margin-right: 2rem;
    }
}
</style>


<script lang="ts" setup>
import { Code, Palette, PanelsTopLeft, Zap } from 'lucide-vue-next';
import { h } from 'vue';

const features = [
    {
        icon: h(Palette),
        title: 'Fully Customizable',
        description: "Design widgets that match your style. Control colors, sizes, transparency, and more with an intuitive interface."
    },
    {
        icon: h(PanelsTopLeft),
        title: 'Smart Layout',
        description: "Drag and drop widgets anywhere on your desktop. Create the perfect workspace layout that fits your workflow."
    },
    {
        icon: h(Zap),
        title: 'Lightweight & Fast',
        description: "Minimal resource usage means your desktop stays responsive. BrickUI runs efficiently in the background."
    },
    {
        icon: h(Code),
        title: 'Developer Friendly',
        description: "Create your own custom bricks with simple APIs. Extend functionality with plugins and scripts."
    }
]
</script>