import { invoke } from "@tauri-apps/api";
import { appLocalDataDir } from "@tauri-apps/api/path";
import { appWindow } from "@tauri-apps/api/window";
import { Ref } from "vue";
import { send_notification } from "./notification";
import { session } from "@edge/index-utils";

/**
 * Load and set node initialization status.
 */
export async function sync_initialization_status(deviceInitializedref: Ref<boolean>) {
    const appLocalDataDirPath = await appLocalDataDir();
    deviceInitializedref.value = await invoke("get_device_initialization_status_from_frontend", {
        datadir: appLocalDataDirPath,
        window: appWindow,
    });
}

/**
 * Start the node. Returns a boolean whether the device has successfully started.
 */
async function initial_device_start_from_frontend() {
    const appLocalDataDirPath = await appLocalDataDir();
    let has_device_start_from_frontended_successfully: boolean = await invoke("device_start_from_frontend", {
        checklatestbinary: true,
        datadir: appLocalDataDirPath,
        window: appWindow,
    });

    return has_device_start_from_frontended_successfully

}

/**
 * Initial startup of device.
 */
export async function start_device_for_first_time(deviceInitializedref: Ref<boolean>, nodeOnlineMessageref: Ref<any>) {
    let has_device_start_from_frontended_successfully = await initial_device_start_from_frontend();
    if (has_device_start_from_frontended_successfully == true) {
        complete_initialization_flow(deviceInitializedref, nodeOnlineMessageref);
    } else {
    }
}

/**
 * Flow for checking the node online status. 
 * Autochecks if node is online. 
 * If true, set program stage to post-intialization.
 */
// Check node and set initialization status. If the node is online once, it is assumed to be correctly initialized.
async function complete_initialization_flow(deviceInitializedref: Ref<boolean>, nodeOnlineMessageref: Ref<any>) {
    const appLocalDataDirPath = await appLocalDataDir();
    let node_address: string = await invoke("get_node_address_from_frontend", {
        datadir: appLocalDataDirPath,
        window: appWindow,
    });

    // Check online status of node and set initialization status based on result.
    let error_string = "Unset";
    if (node_address != error_string) {
        // Assume node address is valid.
        let check_message = "Your node was started successfully! Sit back and relax. The Staking GUI will automatically keep checking if your node is online."
        await invoke("log_and_emit_from_frontend", {
            message: check_message,
            datadir: appLocalDataDirPath,
            window: appWindow,
        });

        await auto_recheck_node_online(deviceInitializedref, nodeOnlineMessageref, appLocalDataDirPath, node_address);
    }
    else {
        let error_message = "Node address is not set. Please complete the other setup steps.";
        await invoke("log_and_emit_from_frontend", {
            message: error_message,
            datadir: appLocalDataDirPath,
            window: appWindow,
        });

    }
}

let isNodeOnlineAutocheckActive = false;
/**
 * Automatically check if the node online. If it is, marks initialization as completed.
 * 
 * @param appLocalDataDirPath 
 * @param node_address 
 * @param timer_seconds_delay 
 * @param recheck_limit 
 */
async function auto_recheck_node_online(deviceInitializedref: Ref<boolean>, nodeOnlineMessageref: Ref<any>, appLocalDataDirPath: string, node_address: string, timer_seconds_delay: number = 60, recheck_limit: number = 120) {
    let recheck_count = 0;
    if (!isNodeOnlineAutocheckActive) {
        isNodeOnlineAutocheckActive = true;
        let AutoCheckNodeOnline = setInterval(async () => {
            recheck_count += 1;
            let recheck_message = "Rechecking node online status. Count : " + recheck_count;
            await invoke("log_and_emit_from_frontend", {
                message: recheck_message,
                datadir: appLocalDataDirPath,
                window: appWindow,
            });

            let is_node_online = await helper_check_node_online_status(nodeOnlineMessageref, node_address);

            if (is_node_online) {
                // set initialized flag
                await invoke("set_device_fully_initialized_from_frontend", {
                    datadir: appLocalDataDirPath,
                    window: appWindow,
                });
                sync_initialization_status(deviceInitializedref);
                send_notification("Node Setup Completed", "Your Edge node setup has completed!");
                Disable_Autocheck_Node_online(AutoCheckNodeOnline); // Stop autochecking
            }

            if (recheck_count >= recheck_limit) {
                let error_message = "Could not find your node online after several retries. Please double check if your device code was correctly assigned. Try starting the node again. If the error keeps persisting, contact support.";
                await invoke("log_and_emit_from_frontend", {
                    message: error_message,
                    datadir: appLocalDataDirPath,
                    window: appWindow,
                });
                Disable_Autocheck_Node_online(AutoCheckNodeOnline); // Stop autochecking
            }
        }, timer_seconds_delay * 1000);


    }

    function Disable_Autocheck_Node_online(AutoCheckNodeOnline: NodeJS.Timer) {
        isNodeOnlineAutocheckActive = false;
        clearInterval(AutoCheckNodeOnline);
    }
}

/**
 * 
 * @param node_address XE node address
 * Checks if the XE node address is online.
 */
async function helper_check_node_online_status(Node_Online_Message_ref: Ref<any>, node_address: string) {
    const appLocalDataDirPath = await appLocalDataDir();
    try {
        const sess = await session.session('https://index.xe.network', node_address)
        console.log(JSON.stringify(sess))
        console.log(sess.lastActive)
        sess.online
        // TODO: Check online value
        return true

    } catch (e) {
        let error_string = JSON.stringify(e);
        Node_Online_Message_ref.value = "Node not seen yet. The Staking GUI automatically rechecks the online status. If you start your node for the first time this can take up to an hour."
        let error_message = "Node not found http error code:" + error_string;

        await invoke("log_and_emit_from_frontend", {
            message: error_message,
            datadir: appLocalDataDirPath,
            window: appWindow,
        });

        await invoke("log_and_emit_from_frontend", {
            message: Node_Online_Message_ref.value,
            datadir: appLocalDataDirPath,
            window: appWindow,
        });
        return false
    }


}
