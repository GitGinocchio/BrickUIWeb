<template>
    <section class="introduzione">
        <container>
            <Fadecontent>
                <img src="../assets/logo.svg" class="animation">
            </Fadecontent>

            <Animatedcontent
                :distance="30"
                direction="vertical"
                :duration="0.8"
                :initial-opacity="0"
                :animate-opacity="true"
                :scale="1"
                :threshold="0.1"
            >
                <h1>Who we are</h1>
                <p>Discover our story behind BrickUI and the people who make customization possible for your desktop.</p>
            </Animatedcontent>
        </container>
    </section>

    <section class="missione">
        <div class="container">
            <div class="titolo">
                <img src="../assets/logo.svg">

                <h2>Our Mission</h2>
            </div>

            <div class="boxes">
                <div class="box">
                    <img src="../assets/logo.svg">

                    <h3>Straightforwardness</h3>
                    <p>BrickUI is designed to be intuitive and easy to use, making customization accessible to everyone.</p>
                </div>

                <div class="box">
                    <img src="../assets/logo.svg">

                    <h3>Customization</h3>
                    <p>BrickUI allows users to personalize their desktop environment with a wide range of customization options.</p>
                </div>

                <div class="box">
                    <img src="../assets/logo.svg">

                    <h3>Community</h3>
                    <p>BrickUI fosters a vibrant community where users can share themes, plugins, and ideas to enhance their desktop experience.</p>
                </div>
            </div>
        </div>
    </section>

    <section class="storia">
        <container>
            <div class="titolo">
                <img src="../assets/logo.svg">

                <h2>Our Story</h2>
                <p>An Idea born while an hackathon from a project used to milions of users.</p>
            </div>

            <div class="main-cointainer">
                <div class="line"/>

                <div class="container">
                    <div class="timeline" v-for="(item, i) in timeline" :class="{'reverse': i % 2 != 0}">
                        <div class="hiden_box"/>

                        <div class="dot"/>

                        <div class="timeline-box">
                            <h4>{{ item.year }}</h4>
                            <h3>{{ item.title }}</h3>
                            <p>{{ item.paragraph }}</p>
                        </div>
                    </div>
                </div>
            </div>
        </container>
    </section>

    <section class="squadra">
        <container>
            <div class="titolo">
                <img src="../assets/logo.svg">

                <h2>Our Team</h2>
                <p>The people behind BrickUI whom work everyday to improve your experience.</p>
            </div>

            <div class="container">
                <div class="team-box" v-for="item in team">
                    <img src="../assets/logo.svg">

                    <h3>{{ item.name }} {{ item.cognome }}</h3>
                    <p class="role">{{ item.role }}</p>
                    <p>{{ item.about }}</p>
                </div>
            </div>
        </container>
    </section>
</template>

<style scoped>
    .introduzione {
        color: white;
        max-width: 100%;
        padding: 8rem 0 60px 0;
        font-size: large;
    }

    .missione {
        color: white;
        background-color: rgb(5, 5, 5);
        padding: 4rem 0 4rem 0;
        font-size: medium;
    }

    .storia {
        color: white;
        padding: 4rem 0 4rem 0;
        font-size: medium;
    }

    .squadra {
        color: white;
        padding: 4rem 0 4rem 0;
        background-color: rgb(5, 5, 5);
        font-size: medium;
    }

    .introduzione container {
        text-align: center;
    }

    .missione .container {
        padding: 0 2rem 0 2rem;
        margin: auto;
        max-width: 56rem;
    }

    .storia .container {
        display: flex;
        flex-direction: column;
        gap: 40px;
    }

    .storia .main-cointainer {
        display:flex;
        flex-direction: row;
        justify-content: center;
        position: relative;
    }

    .squadra .container {
        display: flex;
        flex-direction: row;
        justify-content: center;
        gap: 2rem;
    }
    
    .missione .box img, .squadra .team-box img, h3, h4, .squadra .role {
        margin-bottom: 10px;
    }

    .introduzione img {
        width: 90px;
    }

    .missione img {
        width: 35px;
    }

    .squadra .team-box img {
        width: 40px;
    }

    /* Missione */
    .titolo {
        text-align: center;
        margin-bottom: 30px;
    }

    .titolo img {
        width: 60px;
    }

    /* Boxes */
    .boxes {
        display: flex;
        gap: 1.5rem;
    }

    .box {
        border: solid 1px #535353;
        border-radius: 12px;
        text-align: left;
        padding: 1.5rem;
        box-shadow: 1px 1px 5px 3px #a0323f66;
        max-width: 17rem;
    }

    .timeline-box {
        border: solid 1px #535353;
        border-radius: 12px;
        text-align: left;
        padding: 1.5rem;
        box-shadow: 1px 1px 5px 3px #a0323f66;
        max-width: 19rem;
    }

    .timeline-box h4, .squadra .role {
        color: #a0323f;
    }

    .squadra .team-box {
        border: solid 1px #535353;
        border-radius: 12px;
        text-align: center;
        padding: 1.5rem;
        box-shadow: 1px 1px 5px 3px #a0323f66;
        max-width: 19rem;
    }

    /* Timeline */
    .hiden_box {
        visibility: hidden;
        display: flex;
        width: 19rem;
    }

    .storia .timeline .dot {
        border: solid black 4px;
        background-color: #a0323f;
        height: 17px;
        width: 17px;
        margin: auto 0 auto 0;
        z-index: 10;
        border-radius: 12px;  
    }

    .storia .timeline {
        display: flex;
        gap: 20px;
        justify-content: center;
    }

    /* Line */
    .storia .main-cointainer .line {
        border: solid 1px;
        height: 100%;
        position: absolute;
    }

    .reverse {
        display: flex;
        flex-direction: row-reverse;
    }

    /* Animation */
    @keyframes upAndDown {
        0% {
            transform: translateY(0);
        }

        100% {
            transform: translateY(-20px);
        }
    }

    .animation {
        animation: upAndDown 2.5s ease-in-out 0s infinite alternate;
    }
</style>

<script setup lang="ts">
    import Animatedcontent from "@/components/Animatedcontent.vue";
    import Fadecontent from "@/components/Fadecontent.vue";
    import { colors } from "@/themes/colors";
    import { onMounted, ref } from "vue";

    const timeline = ref([
        { year : 2023, title : "The Idea", paragraph : "Born the idea of BrickUI while an hackthon" },
        { year : 2024, title : "First Release", paragraph : "Released the first version of BrickUI to the public" },
        { year : 2024, title : "Community Growth", paragraph : "Reached over 10,000 users and a thriving community" }
    ]);

    const team = ref([
        { name : "Giulio", cognome : "Tognetto", role : "Founder & Backend Developer", about : "Type something after this is just some test" },
        { name : "Deniel", cognome : "Bertolini", role : "Frontend Developer", about : "Type something after this is just some test" },
        { name : "Keon Ymerson", cognome : "Lacbay", role : "Frontend Developer", about : "Type something after this is just some test" }
    ]);
</script>