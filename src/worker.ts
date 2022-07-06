import {
  initThreadPool,
  paint as rayPaint,
  default as _default,
} from "../ray-tracing/pkg/ray_tracing";

type RenderProps = {
  buffer: SharedArrayBuffer;
  aspectRatio: number;
  width: number;
  focalLength: number;
  viewportHeight: number;
  x: number;
  y: number;
  z: number;
};

function render({
  aspectRatio,
  width,
  focalLength,
  viewportHeight,
  x,
  y,
  z,
  buffer,
}: RenderProps) {
  const backing = new Uint8ClampedArray(buffer);
  const data = rayPaint(
    width,
    viewportHeight,
    aspectRatio,
    focalLength,
    new Float32Array([x, y, z])
  );

  for (let i = 0; i < data.length; i++) {
    backing[i] = data[i];
  }

  return buffer;
}

async function main() {
  await _default();
  await initThreadPool(navigator.hardwareConcurrency);
  console.log("worker ready");
  self.postMessage(JSON.stringify({ event: "ready" }));
}

self.onmessage = (e) => {
  const { x, y, z, aspectRatio, focalLength, width, buffer } = e.data;
  if (x !== undefined) {
    let start = Date.now();
    const result = render({
      aspectRatio,
      width,
      focalLength,
      viewportHeight: 2,
      x,
      y,
      z,
      buffer,
    });
    let time = Date.now() - start;
    console.log(`Rendered in ${time}ms`);
    self.postMessage({ result, time });
  }
};

main();
