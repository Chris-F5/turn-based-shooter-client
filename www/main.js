import init, { run } from './turn_based_shooter_client.js';

async function run_wasm() {
    await init();
    run();

}
run_wasm();