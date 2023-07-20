import { invoke } from "@tauri-apps/api"
import { appLocalDataDir } from "@tauri-apps/api/path"
import { appWindow } from "@tauri-apps/api/window"

/**
 * 
 * @returns index_url: string // index.xe.network or index.test.network
 */
export async function get_index_url() {
    const appLocalDataDirPath = await appLocalDataDir()
    const index_url: string = await invoke('get_index_url_from_frontend', {
        datadir: appLocalDataDirPath,
        window: appWindow
    })
    return index_url

}