import { invoke } from "@tauri-apps/api/tauri";

export async function get_logs(): Promise<string[]> {
    return await invoke('get_logs', {})
}

export async function clear_logs() {
    await invoke('clear_logs', {})
}
