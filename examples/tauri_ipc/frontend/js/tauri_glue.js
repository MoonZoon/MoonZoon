const invoke = window.__TAURI__.core.invoke;
const listen = window.__TAURI__.event.listen;

export async function show_window() {
    return await invoke("show_window");
}

export async function greet(name) {
    return await invoke("greet", { name: name });
}

export async function send_ipc_channel(on_message) {
    const ipc_channel = new window.__TAURI__.core.Channel();
    ipc_channel.onmessage = on_message;
    return await invoke("send_ipc_channel", { channel: ipc_channel });
}

export async function greet_through_channel(name) {
    return await invoke("greet_through_channel", { name: name });
}

export async function listen_greet_events(on_event) {
    return await listen("greet", (event) => on_event(event.payload));
}
