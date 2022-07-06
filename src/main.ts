import debounce from "lodash/debounce";
import { main as workerMain, render } from "./worker";

const $focalLength = document.querySelector<HTMLInputElement>("#focal-length")!;
const $width = document.querySelector<HTMLInputElement>("#width")!;
const $canvas = document.querySelector<HTMLCanvasElement>("#canvas")!;

const aspectRatio = 16.0 / 9.0;
let width = Number($width.value);
let height = width / aspectRatio;

let x = 0;
let y = 0;
let z = 2;

function updateKeys() {
  if (Object.values(keys).some((a) => a)) {
    paint();
  }
  if (keys.w) {
    z -= 0.1;
  } else if (keys.s) {
    z += 0.1;
  }

  if (keys.a) {
    x += 0.1;
  } else if (keys.d) {
    x -= 0.1;
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
  console.log(`Size: ${width}x${height}\nPixels: ${width * height}`);
  paint();
}

const changeWidthDebounced = debounce(changeWidth, 200);

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

export async function paint() {
  await workerMain();

  const start = Date.now();
  const array = render({
    aspectRatio,
    width,
    focalLength: Number($focalLength.value),
    viewportHeight: 2,
    x,
    y,
    z,
  });

  const imageData = new ImageData(array.slice(), width, height);
  let end = Date.now();

  let time = end - start;
  const p = document.querySelector<HTMLDivElement>(".indicator")!;
  ctx.putImageData(imageData, 0, 0);
  p.innerHTML = `Rendered in ${time}ms (${Math.round(1000 / time)} FPS)`;
}

updateKeys();
paint();
