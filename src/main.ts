import debounce from "lodash/debounce";
import throttle from "lodash/throttle";
import {
  default as init,
  PixelData,
  RelativeMovement,
  Scene,
  sharedMemory,
} from "../ray-tracing/pkg";

const $focalLength = document.querySelector<HTMLInputElement>("#focal-length")!;
const $width = document.querySelector<HTMLInputElement>("#width")!;
const $canvas = document.querySelector<HTMLCanvasElement>("#canvas")!;
const $movement =
  document.querySelector<HTMLButtonElement>("#movement-toggle")!;

const aspectRatio = 16.0 / 9.0;
let width = Number($width.value);
let height = width / aspectRatio;

type Rotation = [number, number, number];
let defaultRotation: Rotation = [0, 0, 0];
let rotation: Rotation = defaultRotation;
let isRotating = false;
let ready = false;

let viewportHeight = 2;

let scene: Scene;

$movement.addEventListener("click", () => {
  if (isRotating) {
    rotation = defaultRotation;
  }
  isRotating = !isRotating;
  paint();
});
let x = 0;
let y = 0;
let z = 0;

function changeRotation(e: MouseEvent) {
  if (!isRotating) {
    return;
  }
  const percentageX = e.clientX / window.innerWidth;
  const percentageY = e.clientY / window.innerHeight;
  rotation[1] = Math.PI - percentageX * Math.PI;
  // rotation[1] = percentageY * Math.PI * 2;
  paint();
}

document.addEventListener("mousemove", throttle(changeRotation, 50));

function updateKeys() {
  if (keys.w) {
    scene.move_along(RelativeMovement.Forward);
  } else if (keys.s) {
    scene.move_along(RelativeMovement.Back);
  }

  if (keys.a) {
    scene.move_along(RelativeMovement.Left);
  } else if (keys.d) {
    scene.move_along(RelativeMovement.Right);
  }
  if (Object.values(keys).some((a) => a)) {
    paint();
  }
  requestAnimationFrame(updateKeys);
}

$canvas.width = width;
$canvas.height = height;

const ctx = $canvas.getContext("2d")!;

const keys = {
  w: false,
  a: false,
  s: false,
  d: false,
};

document.addEventListener("keydown", (event) => {
  if (event.key in keys) {
    keys[event.key as keyof typeof keys] = true;
  }
});

document.addEventListener("keyup", (event) => {
  if (event.key in keys) {
    keys[event.key as keyof typeof keys] = false;
  }
});

function changeWidth(width: number, height: number) {
  // buffer = new SharedArrayBuffer(width * height * 4);
  // array = new Uint8ClampedArray(buffer);
  scene.changeWidth(width);
  console.log(`Size: ${width}x${height}\nPixels: ${width * height}`);
  paint();
}

// setInterval(() => {
//   rotation = (rotation + Math.PI / 64) % (Math.PI * 2);
//   paint();
// }, 16);

const changeWidthDebounced = debounce(changeWidth, 200);

let dragging;

$canvas.addEventListener("touchstart", (e) => {
  console.log(e);
});

$width.addEventListener("input", () => {
  width = Number($width.value);
  height = Math.floor(width / aspectRatio);
  $canvas.width = width;
  $canvas.height = height;
  changeWidthDebounced(width, height);
});

$focalLength.addEventListener("input", () => {
  paint();
});

function fromPixelData(pointer: PixelData): Uint8ClampedArray {
  return new Uint8ClampedArray(
    sharedMemory().buffer,
    pointer.offset(),
    pointer.size()
  );
}

export async function paint() {
  const start = Date.now();

  const array = fromPixelData(scene.render());

  const imageData = new ImageData(array.slice(), width, height);
  let end = Date.now();

  let time = end - start;
  const p = document.querySelector<HTMLDivElement>(".indicator")!;
  ctx.putImageData(imageData, 0, 0);
  p.innerHTML = `Rendered in ${time}ms (${Math.round(1000 / time)} FPS)`;
}

async function main() {
  await init();
  ready = true;
  scene = Scene.new(
    width,
    viewportHeight,
    aspectRatio,
    Number($focalLength.value),
    new Float32Array([0, 0, 3]),
    new Float32Array([0, 0, 0])
  );
  console.log("inited wasm");
  updateKeys();
  paint();
}

main();
