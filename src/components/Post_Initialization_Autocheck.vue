<script setup lang="ts">
import { appLocalDataDir } from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from "@tauri-apps/api/tauri";
import { check_node_earnings } from "./node_earnings";
import { check_device_initialization, check_node_online_status } from "./intialization";
import { send_notification } from './notification';

let isPostInitializationAutocheckActive = false;
/**
 * 
 * Automatically starts node if autostart is set to active.
 * 
 * @param timer_seconds_delay 
 * @param recheck_limit 
 */
async function PostInitializationAutocheck(timer_seconds_delay: number = 3600) {
  const appLocalDataDirPath = await appLocalDataDir();

  let device_is_initialized = await check_device_initialization();

  if (!device_is_initialized) {
    return // Don't autostart.
  }

  let recheck_count = 0;
  if (!isPostInitializationAutocheckActive) {
    isPostInitializationAutocheckActive = true;
    let Autocheck = setInterval(async () => {
      recheck_count += 1;
      let recheck_msg = "Autochecking for node earnings & node online status. Check nr:" + recheck_count;
      await invoke("log_and_emit_from_frontend", {
        message: recheck_msg,
        datadir: appLocalDataDirPath,
        window: appWindow,
      });

      check_node_earnings();

      // Check online status
      let node_address: string = await invoke("get_node_address_from_frontend", {
        datadir: appLocalDataDirPath,
        window: appWindow,
      });

      if ((await check_node_online_status(node_address)) === false) {
        let err_msg = "Node not online according to index. Check if your node is running. If the issue persists, contact support."
        await invoke("log_and_emit_from_frontend", {
          message: err_msg,
          datadir: appLocalDataDirPath,
          window: appWindow,
        });
        send_notification("Node not online.", "Your Edge node is not currently online according to index. Double check if your node is started. If you keep getting this error message, please contact support.")

      }



    }, timer_seconds_delay * 1000);


  }
}

PostInitializationAutocheck();
</script>

<template></template>
