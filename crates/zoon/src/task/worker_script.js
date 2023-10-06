importScripts("{{js_url}}");

let should_create_wasm_module_instance = true;

let worker_entry_point_resolve;
const worker_entry_point = new Promise(resolve => worker_entry_point_resolve = resolve);

self.onmessage = async event => {
    if(should_create_wasm_module_instance) {
        should_create_wasm_module_instance = false;
        const [wasm_module, wasm_memory] = event.data;
        const instance_creator = await wasm_bindgen(wasm_module, wasm_memory);
        const { worker_entry_point } = await instance_creator;
        worker_entry_point_resolve(worker_entry_point);
    } else {
        (await worker_entry_point)(event.data);
    }
}
