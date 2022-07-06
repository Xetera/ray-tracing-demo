import * as wasm from "../ray-tracing/pkg";

type RenderProps = {
  aspectRatio: number;
  width: number;
  focalLength: number;
  viewportHeight: number;
  x: number;
  y: number;
  z: number;
};

export async function main() {
  // console.log(await WebAssembly.instantiateStreaming(fetch("/ray_tracing_bg.wasm")))

  // const wasm = await instantiateStreaming(fetch("/ray_tracing_bg.wasm"))
  // const wasm = await import("/ray_tracing_bg.wasm");
  // const memory = new WebAssembly.Memory({ initial: 256, maximum: 256 });
  // const importObj = {
  //   // wbg: 1,
  //   env: {
  //     abortStackOverflow: () => {
  //       throw new Error("overflow");
  //     },
  //     table: new WebAssembly.Table({
  //       initial: 0,
  //       maximum: 0,
  //       element: "anyfunc",
  //     }),
  //     tableBase: 0,
  //     memory: memory,
  //     memoryBase: 1024,
  //     STACKTOP: 0,
  //     STACK_MAX: memory.buffer.byteLength,
  //   },
  // };
  // console.log(wasm.default({}))
  // console.log(importObj);
  await wasm.default();
  console.log("aa");
  // await wasm.initThreadPool(navigator.hardwareConcurrency);
  console.log("worker ready");
  // self.onmessage = (e) => {
  //   const { x, y, z, aspectRatio, focalLength, width, buffer } = e.data;
  //   if (x !== undefined) {
  //     let start = Date.now();
  //     const result = render({
  //       aspectRatio,
  //       width,
  //       focalLength,
  //       viewportHeight: 2,
  //       x,
  //       y,
  //       z,
  //       buffer,
  //     });
  //     let time = Date.now() - start;
  //     console.log(`Rendered in ${time}ms`);
  //     self.postMessage({ result, time });
  //   }
  // };

  // self.postMessage({ event: "ready" });
}
export function render({
  aspectRatio,
  width,
  focalLength,
  viewportHeight,
  x,
  y,
  z,
}: // buffer: sharedArrayBuffer,
RenderProps) {
  // const backingArray = new Uint8ClampedArray(sharedArrayBuffer);

  const pointer = wasm.paint(
    width,
    viewportHeight,
    aspectRatio,
    focalLength,
    new Float32Array([x, y, z])
  );

  return new Uint8ClampedArray(
    wasm.sharedMemory().buffer,
    pointer.offset(),
    pointer.size()
  );

  // for (let i = 0; i < raw.length; i++) {
  //   backingArray[i] = raw[i];
  // }

  // return sharedArrayBuffer;
}

//
// main();
