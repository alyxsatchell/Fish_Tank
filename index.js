import wasmInit, { FishTank, Pos } from "./pkg/fish_tank.js";

const runWasm = async () => {

  const rustWasm = await wasmInit("./pkg/fish_tank_bg.wasm");

  const canvasElement = document.querySelector("canvas");

  const canvasContext = canvasElement.getContext("2d");
  const canvasImageData = canvasContext.createImageData(
    canvasElement.width,
    canvasElement.height
  );
  canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);
  let fish_count = prompt("Input Number Of Fishys!")  
  const fish_tank = FishTank.new(fish_count, Pos.new(255,255));
  const tick = () => {
    rustWasm.my_init_function()
    fish_tank.tick();
    const canvasPtr = fish_tank.get_canvas();
    const wasmByteMemoryArray = new Uint8Array(rustWasm.memory.buffer, canvasPtr, 255 * 255 * 4);
    canvasImageData.data.set(wasmByteMemoryArray);
    canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);
    canvasContext.putImageData(canvasImageData, 0, 0);
  };
  setInterval(() =>{
    tick();
  }, 1000/24)
  // tick();
};
runWasm();
