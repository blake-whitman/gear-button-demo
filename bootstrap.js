import init, { start } from "./your_crate_name.js";

async function run() {
  await init();
  start("the_canvas");
}

run();
