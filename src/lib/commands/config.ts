import { invoke } from "@tauri-apps/api/tauri";

export async function set_proxy(proxy: String) {
    return await invoke('set_proxy', { proxy: proxy })
}

export async function get_proxy(): Promise<string | null> {
    return await invoke('get_proxy', {})
}

export async function test_proxy(proxy: String): Promise<string | null> {
    return await invoke('test_proxy', { proxy: proxy })
}