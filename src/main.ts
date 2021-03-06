import debounce from "lodash/debounce";
import throttle from "lodash/throttle";
import {
  default as init,
  PixelData,
  RelativeDirection,
  Scene,
  sharedMemory,
} from "../ray-tracing/pkg";

const $focalLength = document.querySelector<HTMLInputElement>("#focal-length")!;
const $width = document.querySelector<HTMLInputElement>("#width")!;
const $canvas = document.querySelector<HTMLCanvasElement>("#canvas")!;
const $antiAliasing =
  document.querySelector<HTMLInputElement>("#anti-aliasing")!;

const aspectRatio = 16.0 / 9.0;
let width = Number($width.value);
let height = width / aspectRatio;

let viewportHeight = 2;

let scene: Scene;

function changeRotation(e: MouseEvent) {
  const percentageX = e.offsetX / width;
  const percentageY = e.offsetY / height;
  const yExtrema = Math.PI / 2;
  const xExtrema = Math.PI * 2;
  let y: number = yExtrema * (0.5 - percentageY);

  let x: number = -xExtrema * (0.5 - percentageX);
  const array = new Float32Array([y, x, 0]);
  scene.rotateToPointer(array);
  if (!isMoving()) {
    paint();
  }
}

$canvas.addEventListener("mouseleave", () => {
  scene.rotateToPointer(new Float32Array([0, 0, 0]));
  paint();
});

$canvas.addEventListener("mousemove", throttle(changeRotation, 20));

function updateKeys() {
  if (keys.w) {
    scene.move_along(RelativeDirection.Up);
  } else if (keys.s) {
    scene.move_along(RelativeDirection.Down);
  }

  if (keys.a) {
    scene.move_along(RelativeDirection.Left);
  } else if (keys.d) {
    scene.move_along(RelativeDirection.Right);
  }

  if (keys[" "]) {
    scene.up();
  } else if (keys.Shift) {
    scene.down();
  }

  if (isMoving()) {
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
  " ": false,
  Shift: false,
};

const isMoving = () => Object.values(keys).some((val) => Boolean(val));

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

const changeWidthDebounced = debounce(changeWidth, 200);

$width.addEventListener("input", () => {
  width = Number($width.value);
  height = Math.floor(width / aspectRatio);
  $canvas.width = width;
  $canvas.height = height;
  changeWidthDebounced(width, height);
});

$antiAliasing.addEventListener("input", () => {
  const aa = Number($antiAliasing.value);
  console.log(aa);
  scene.set_aa(aa);
  paint();
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
  let fps = 1000 / time;
  p.innerHTML = `Rendered in ${time}ms (${
    fps > 1 ? Math.round(fps) : fps.toFixed(2)
  } FPS)`;
}

async function main() {
  await init();
  scene = Scene.new(
    width,
    viewportHeight,
    aspectRatio,
    Number($focalLength.value),
    new Float32Array([0, 0, 3]),
    new Float32Array([0, 0, 0]),
    Number($antiAliasing.value)
  );
  console.log("inited wasm");
  // setInterval(() => {
  //   scene.turn(RelativeDirection.Right);
  //   paint();
  // }, 20);
  updateKeys();
  paint();
}

main();
