import { invoke } from "@tauri-apps/api";
import { appLocalDataDir } from "@tauri-apps/api/path";
import { appWindow } from "@tauri-apps/api/window";


export async function get_launch_minimized_status() {
    const appLocalDataDirPath = await appLocalDataDir();
    let launch_minimized_status: boolean = await invoke("get_launch_minimized_status_from_frontend", {
        datadir: appLocalDataDirPath,
        window: appWindow,
    });

    return launch_minimized_status
}

/**
 * Load and set launch minimized settings
 */
export async function sync_launch_minimized_status() {
    let window_must_be_minimized = await get_launch_minimized_status();
    if (window_must_be_minimized) {
        const appLocalDataDirPath = await appLocalDataDir();
        let ok_message = "App was launched in a minimized state."
        await invoke("log_and_emit_from_frontend", {
            message: ok_message,
            datadir: appLocalDataDirPath,
            window: appWindow,
        });
        await appWindow.hide();
        await appWindow.minimize();
    } else {
        await appWindow.show();
        await appWindow.setFocus();
        await appWindow.unminimize();
    }
}
