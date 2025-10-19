<template>
    <canvas ref="canvasRef" class="background"> </canvas>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";

const canvasRef = ref<HTMLCanvasElement | null>(null);

onMounted(() => {
    const canvas = canvasRef.value;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Funzione per aggiornare le dimensioni del canvas
    const setCanvasSize = () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
    };
    setCanvasSize();
    window.addEventListener("resize", setCanvasSize);

    // Configurazione mattoni
    const brickWidth = 80;
    const brickHeight = 40;
    const gap = 10;
    const bricks: Array<{
        x: number;
        y: number;
        glow: number;
        targetGlow: number;
    }> = [];

    // Creazione griglia
    const cols = Math.ceil(canvas.width / (brickWidth + gap)) + 2;
    const rows = Math.ceil(canvas.height / (brickHeight + gap)) + 2;

    for (let row = 0; row < rows; row++) {
        for (let col = 0; col < cols; col++) {
            const offsetX = row % 2 === 0 ? 0 : (brickWidth + gap) / 2;
            bricks.push({
                x: col * (brickWidth + gap) + offsetX - brickWidth,
                y: row * (brickHeight + gap) - brickHeight,
                glow: 0,
                targetGlow: 0,
            });
        }
    }

    // Tracciamento del mouse
    let mouseX = -1000;
    let mouseY = -1000;

    const handleMouseMove = (e: MouseEvent) => {
        mouseX = e.clientX;
        mouseY = e.clientY;
    };
    window.addEventListener("mousemove", handleMouseMove);

    // Loop di animazione
    const animate = () => {
        ctx.fillStyle = "rgba(5, 5, 5, 0.05)";
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        bricks.forEach((brick) => {
            const dx = mouseX - (brick.x + brickWidth / 2);
            const dy = mouseY - (brick.y + brickHeight / 2);
            const distance = Math.sqrt(dx * dx + dy * dy);

            // Glow in base alla distanza
            brick.targetGlow = distance < 150 ? 1 - distance / 150 : 0;
            brick.glow += (brick.targetGlow - brick.glow) * 0.1;

            // Colori
            const baseColor = `rgba(220, 38, 38, ${0.15 + brick.glow * 0.3})`;
            const glowColor = `rgba(239, 68, 68, ${brick.glow * 0.8})`;

            // Corpo del mattone
            ctx.fillStyle = baseColor;
            ctx.fillRect(brick.x, brick.y, brickWidth, brickHeight);

            // Effetto glow SOLO tra le fessure
            if (brick.glow > 0.05) {
                ctx.shadowBlur = 20 * brick.glow;
                ctx.shadowColor = "#ef4444";
                ctx.strokeStyle = `rgba(239, 68, 68, ${brick.glow * 0.8})`;
                ctx.lineWidth = gap; // spessore delle fughe
                ctx.strokeRect(brick.x, brick.y, brickWidth, brickHeight);
                ctx.shadowBlur = 0;
            }

            // Bordo del mattone
            ctx.strokeStyle = `rgba(185, 28, 28, ${0.3 + brick.glow * 0.5})`;
            ctx.lineWidth = 1;
            ctx.strokeRect(brick.x, brick.y, brickWidth, brickHeight);

            // Linea di dettaglio
            /*
            ctx.strokeStyle = `rgba(127, 29, 29, ${0.2 + brick.glow * 0.3})`;
            ctx.beginPath();
            ctx.moveTo(brick.x, brick.y + brickHeight / 2);
            ctx.lineTo(brick.x + brickWidth, brick.y + brickHeight / 2);
            ctx.stroke();
            */
        });

        requestAnimationFrame(animate);
    };

    animate();

    // Cleanup
    onUnmounted(() => {
        window.removeEventListener("resize", setCanvasSize);
        window.removeEventListener("mousemove", handleMouseMove);
    });
});
</script>

<style scoped>
body {
    background-color: black;
}
.background {
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
}
</style>
