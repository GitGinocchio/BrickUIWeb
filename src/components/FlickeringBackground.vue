<template>
  <canvas ref="canvasRef" />
</template>

<style scoped>
canvas {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: -1;
  overflow-x: hidden;
  overflow-y: hidden
}
</style>

<script lang="ts" setup>
import { ref, onMounted, onBeforeUnmount, computed } from "vue";

interface FlickeringGridProps {
  squareSize?: number;
  gridGap?: number;
  flickerChance?: number;
  color?: string;
  maxOpacity?: number;
}

const props = withDefaults(defineProps<FlickeringGridProps>(), {
  squareSize: 4,
  gridGap: 6,
  flickerChance: 0.3,
  color: "#FF0000",
  maxOpacity: 0.3,
});

const { squareSize, gridGap, flickerChance, color, maxOpacity } = props;

const canvasRef = ref<HTMLCanvasElement>();
const context = ref<CanvasRenderingContext2D>();

const computedColor = computed(() => {
  const hex = color.replace(/^#/, "");
  const bigint = parseInt(hex, 16);
  const r = (bigint >> 16) & 255;
  const g = (bigint >> 8) & 255;
  const b = bigint & 255;
  return `rgba(${r}, ${g}, ${b},`;
});

let gridParams: ReturnType<typeof setupCanvas> | null = null;
let animationFrameId: number;
let lastTime = 0;
let resizeTimeout: number;

function setupCanvas(canvas: HTMLCanvasElement, width: number, height: number) {
  const ctx = context.value!;
  const dpr = window.devicePixelRatio || 1;

  canvas.width = width * dpr;
  canvas.height = height * dpr;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  ctx.setTransform(dpr, 0, 0, dpr, 0, 0); // 🔥 SCALE CORRETTO

  const cols = Math.floor(width / (squareSize + gridGap));
  const rows = Math.floor(height / (squareSize + gridGap));

  const squares = new Float32Array(cols * rows);
  for (let i = 0; i < squares.length; i++) {
    squares[i] = Math.random() * maxOpacity;
  }

  return { cols, rows, squares, width, height };
}

function updateSquares(squares: Float32Array, deltaTime: number) {
  for (let i = 0; i < squares.length; i++) {
    if (Math.random() < flickerChance * deltaTime) {
      squares[i] = Math.random() * maxOpacity;
    }
  }
}

function drawGrid(ctx: CanvasRenderingContext2D) {
  if (!gridParams) return;

  const { width, height, cols, rows, squares } = gridParams;

  ctx.clearRect(0, 0, width, height);

  for (let i = 0; i < cols; i++) {
    for (let j = 0; j < rows; j++) {
      const opacity = squares[i * rows + j];
      ctx.fillStyle = `${computedColor.value}${opacity})`;
      ctx.fillRect(
        i * (squareSize + gridGap),
        j * (squareSize + gridGap),
        squareSize,
        squareSize
      );
    }
  }
}

function updateCanvasSize() {
  if (!canvasRef.value || !context.value) return;

  const pageHeight = Math.max(
    document.body.scrollHeight,
    document.documentElement.scrollHeight,
    document.body.offsetHeight,
    document.documentElement.offsetHeight,
    document.body.clientHeight,
    document.documentElement.clientHeight
  );

  const pageWidth = Math.max(
    document.body.scrollWidth,
    document.documentElement.scrollWidth,
    document.body.offsetWidth,
    document.documentElement.offsetWidth,
    document.body.clientWidth,
    document.documentElement.clientWidth
  );

  gridParams = setupCanvas(canvasRef.value, pageWidth + 10, pageHeight);
  lastTime = performance.now();
}

function animate(time: number) {
  if (!gridParams || !context.value) {
    animationFrameId = requestAnimationFrame(animate);
    return;
  }

  const deltaTime = (time - lastTime) / 1000;
  lastTime = time;

  updateSquares(gridParams.squares, deltaTime);
  drawGrid(context.value);

  animationFrameId = requestAnimationFrame(animate);
}

function handleResize() {
  clearTimeout(resizeTimeout);
  resizeTimeout = window.setTimeout(updateCanvasSize, 150); // debounce
}

onMounted(() => {
  if (!canvasRef.value) return;

  context.value = canvasRef.value.getContext("2d")!;
  updateCanvasSize();

  window.addEventListener("resize", handleResize);
  animationFrameId = requestAnimationFrame(animate);
});

onBeforeUnmount(() => {
  cancelAnimationFrame(animationFrameId);
  window.removeEventListener("resize", handleResize);
});
</script>
