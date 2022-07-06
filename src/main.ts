import debounce from "lodash/debounce";

const $focalLength = document.querySelector<HTMLInputElement>("#focal-length")!;
const $width = document.querySelector<HTMLInputElement>("#width")!;
const $canvas = document.querySelector<HTMLCanvasElement>("#canvas")!;

const aspectRatio = 16.0 / 9.0;
let width = Number($width.value);
let height = width / aspectRatio;

const worker = new Worker(new URL("./worker.ts", import.meta.url), {
  type: "module",
});

let buffer = new SharedArrayBuffer(width * height * 4);
let array = new Uint8ClampedArray(buffer);

let ready = false;

let x = 0;
let y = 0;
let z = 2;

const frame = () =>
  worker.postMessage({
    focalLength: Number($focalLength.value),
    x,
    y,
    z,
    width,
    aspectRatio,
    buffer,
  });

function updateKeys() {
  if (ready && Object.values(keys).some((a) => a)) {
    frame();
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
  if (!ready) {
    return;
  }
  if (event.key in keys) {
    keys[event.key as keyof typeof keys] = true;
  }
  frame();
});

document.addEventListener("keyup", (event) => {
  if (!ready) {
    return;
  }
  if (event.key in keys) {
    keys[event.key as keyof typeof keys] = false;
  }
  frame();
});

function changeWidth(width: number, height: number) {
  buffer = new SharedArrayBuffer(width * height * 4);
  array = new Uint8ClampedArray(buffer);
  frame();
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
  frame();
});

function render() {}

export async function main() {
  updateKeys();
  worker.onmessage = (event) => {
    if (typeof event.data === "string") {
      const body = JSON.parse(event.data);
      if (body.event === "ready") {
        ready = true;
        frame();
      }
      return;
    }
    const { time } = event.data as { time: number };

    array.set(new Uint8ClampedArray(buffer), 0);
    const imageData = new ImageData(array.slice(), width, height);

    const p = document.querySelector<HTMLDivElement>(".indicator")!;
    ctx.putImageData(imageData, 0, 0);
    p.innerHTML = `Rendered in ${time}ms (${Math.round(1000 / time)} FPS)`;
  };
}

main();
