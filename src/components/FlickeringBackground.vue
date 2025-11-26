<template>
  <canvas ref="canvasRef" />
</template>

<style scoped>
canvas {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: -1;
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
  if (!context.value) return "rgba(0,0,0,";
  const hex = color.replace(/^#/, "");
  const bigint = parseInt(hex, 16);
  const r = (bigint >> 16) & 255;
  const g = (bigint >> 8) & 255;
  const b = bigint & 255;
  return `rgba(${r}, ${g}, ${b},`;
});

let gridParams: ReturnType<typeof setupCanvas>;
let animationFrameId: number;
let lastTime = 0;

function setupCanvas(canvas: HTMLCanvasElement, width: number, height: number) {
  const dpr = window.devicePixelRatio || 1;
  canvas.width = width * dpr;
  canvas.height = height * dpr;
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;

  const cols = Math.floor(width / (squareSize + gridGap));
  const rows = Math.floor(height / (squareSize + gridGap));

  const squares = new Float32Array(cols * rows);
  for (let i = 0; i < squares.length; i++) {
    squares[i] = Math.random() * maxOpacity;
  }

  return { cols, rows, squares, dpr };
}

function updateSquares(squares: Float32Array, deltaTime: number) {
  for (let i = 0; i < squares.length; i++) {
    if (Math.random() < flickerChance * deltaTime) {
      squares[i] = Math.random() * maxOpacity;
    }
  }
}

function drawGrid(ctx: CanvasRenderingContext2D, width: number, height: number, cols: number, rows: number, squares: Float32Array, dpr: number) {
  ctx.clearRect(0, 0, width, height);
  for (let i = 0; i < cols; i++) {
    for (let j = 0; j < rows; j++) {
      const opacity = squares[i * rows + j];
      ctx.fillStyle = `${computedColor.value}${opacity})`;
      ctx.fillRect(
        i * (squareSize + gridGap) * dpr,
        j * (squareSize + gridGap) * dpr,
        squareSize * dpr,
        squareSize * dpr
      );
    }
  }
}

function updateCanvasSize() {
  if (!canvasRef.value) return;
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

  gridParams = setupCanvas(canvasRef.value, pageWidth + 5, pageHeight); //window.outerHeight + 10
}

function animate(time: number) {
  const deltaTime = (time - lastTime) / 1000;
  lastTime = time;

  updateSquares(gridParams.squares, deltaTime);
  drawGrid(context.value!, canvasRef.value!.width, canvasRef.value!.height, gridParams.cols, gridParams.rows, gridParams.squares, gridParams.dpr);

  animationFrameId = requestAnimationFrame(animate);
}

onMounted(() => {
  if (!canvasRef.value) return;
  context.value = canvasRef.value.getContext("2d")!;
  updateCanvasSize();

  window.addEventListener("resize", updateCanvasSize);

  animationFrameId = requestAnimationFrame(animate);
});

onBeforeUnmount(() => {
  cancelAnimationFrame(animationFrameId);
  window.removeEventListener("resize", updateCanvasSize);
});
</script>
