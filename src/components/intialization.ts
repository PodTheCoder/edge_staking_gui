import { invoke } from "@tauri-apps/api";
import { appLocalDataDir } from "@tauri-apps/api/path";
import { appWindow } from "@tauri-apps/api/window";
import { Ref } from "vue";
import { send_notification } from "./notification";
import { session, stake } from "@edge/index-utils";

export async function set_wallet_address(deviceInitializedref: Ref<boolean>) {
    const appLocalDataDirPath = await appLocalDataDir();
    deviceInitializedref.value = await invoke("get_device_initialization_status_from_frontend", {
        datadir: appLocalDataDirPath,
        window: appWindow,
    });
}


export async function get_node_wallet_from_config() {
    const appLocalDataDirPath = await appLocalDataDir();
    let wallet_addr_from_config: string = await invoke("get_wallet_address_from_frontend", {
        datadir: appLocalDataDirPath,
        window: appWindow,
    });
    return wallet_addr_from_config
}

/**
 * get the node wallet. Returns true if successful
 */
async function derive_and_set_node_wallet_based_on_node_address(node_address: string) {
    const appLocalDataDirPath = await appLocalDataDir();

    const sess = await session.session('https://index.xe.network', node_address)

    const node_stake = sess.node.stake;

    const myStake = await stake.stake('https://index.xe.network', node_stake)

    const derived_wallet_addr = myStake.wallet;

    let err_str_1 = "Unset";
    let err_str_2 = "CouldNotLoadWalletAddressFromConfig";

    if (derived_wallet_addr === err_str_1 || derived_wallet_addr === err_str_2) {
        return false //error
    }

    console.log(JSON.stringify(derived_wallet_addr))
    await invoke("set_wallet_address_from_frontend", {
        walletaddress: derived_wallet_addr,
        datadir: appLocalDataDirPath,
        window: appWindow,
    });

    let wallet_from_config = await get_node_wallet_from_config();

    if (wallet_from_config === derived_wallet_addr) {
        let ok_message = "Wallet address derived based on node address:" + wallet_from_config;
        await invoke("log_and_emit_from_frontend", {
            message: ok_message,
            datadir: appLocalDataDirPath,
            window: appWindow,
        });
        return true
    }
    else {
        let err_message = "Config wallet different from derived wallet after setting. Config: " + wallet_from_config + "Derived:" + derived_wallet_addr;
        await invoke("log_and_emit_from_frontend", {
            message: err_message,
            datadir: appLocalDataDirPath,
            window: appWindow,
        });
        return false
    }



}

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

            let is_node_online = await check_node_online_status(node_address);
            if (is_node_online) {
                let could_wallet_address_be_derived = await derive_and_set_node_wallet_based_on_node_address(node_address);

                if (could_wallet_address_be_derived) {
                    // set initialized flag
                    await invoke("set_device_fully_initialized_from_frontend", {
                        datadir: appLocalDataDirPath,
                        window: appWindow,
                    });
                    sync_initialization_status(deviceInitializedref);
                    send_notification("Node Setup Completed", "Your Edge node setup has completed!");
                    Disable_Autocheck_Node_online(AutoCheckNodeOnline); // Stop autochecking
                } else {
                    let online_but_could_not_derive_msg = "Node is viewed as online, but unable to derive wallet addreses which is needed for node earning notifications.";
                    await invoke("log_and_emit_from_frontend", {
                        message: online_but_could_not_derive_msg,
                        datadir: appLocalDataDirPath,
                        window: appWindow,
                    });
                }
            } else {
                let err_msg_node_not_found_yet = "Node not seen yet. The Staking GUI automatically rechecks the online status. If you start your node for the first time this can take up to an hour."
                await invoke("log_and_emit_from_frontend", {
                    message: err_msg_node_not_found_yet,
                    datadir: appLocalDataDirPath,
                    window: appWindow,
                });
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
export async function check_node_online_status(node_address: string) {
    const appLocalDataDirPath = await appLocalDataDir();
    try {
        const sess = await session.session('https://index.xe.network', node_address)
        console.log(JSON.stringify(sess))
        console.log(sess.lastActive)
        let is_node_online = sess.online;

        if ((typeof is_node_online === 'boolean') && is_node_online) {
            let ok_message = "Node session exists. However, node is not online.";
            await invoke("log_and_emit_from_frontend", {
                message: ok_message,
                datadir: appLocalDataDirPath,
                window: appWindow,
            });
            return true
        } else {
            let error_message = "Node session exists. However, node is not online.";
            await invoke("log_and_emit_from_frontend", {
                message: error_message,
                datadir: appLocalDataDirPath,
                window: appWindow,
            });
            return false
        }

    } catch (e) {
        let error_string = JSON.stringify(e);
        let err_msg_1 = "Node not found http error code:" + error_string;

        await invoke("log_and_emit_from_frontend", {
            message: err_msg_1,
            datadir: appLocalDataDirPath,
            window: appWindow,
        });


        return false
    }


}

export async function check_device_initialization() {
    const appLocalDataDirPath = await appLocalDataDir();
    let has_device_been_initialized = await invoke("get_device_initialization_status_from_frontend", {
        datadir: appLocalDataDirPath,
        window: appWindow,
    });
    return has_device_been_initialized
}
