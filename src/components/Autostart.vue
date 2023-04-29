<script setup lang="ts">
import { ref } from "vue";
import { enable, isEnabled, disable } from "tauri-plugin-autostart-api";
import { appLocalDataDir } from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from "@tauri-apps/api/tauri";
import { send_notification } from './notification';


// Initialize consts
const auto_start_enabled = ref(false); // Default state, gets overwritten through get_autostart_status

async function get_autostart_status() {
  auto_start_enabled.value = await isEnabled();
  return auto_start_enabled.value;
}


async function enable_autostart() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await enable();
  const appLocalDataDirPath = await appLocalDataDir();
  let ok_enable_autostart = "Your node will automatically launch at system startup.";
  await invoke("log_and_emit_from_frontend", {
    message: ok_enable_autostart,
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
  get_autostart_status(); // to update reference status
}

async function disable_autostart() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await disable();
  const appLocalDataDirPath = await appLocalDataDir();
  let ok_disable_autostart = "Your node will not automatically launch at system startup.";
  await invoke("log_and_emit_from_frontend", {
    message: ok_disable_autostart,
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
  get_autostart_status(); // to update reference status
}

async function check_device_initialization() {
  const appLocalDataDirPath = await appLocalDataDir();
  let has_device_been_initialized = await invoke("load_device_initialization_status", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
  return has_device_been_initialized
}


let isNodeAutostartIntervalActive = false;

/**
 * 
 * Automatically starts node if autostart is set to active.
 * 
 * @param timer_seconds_delay 
 * @param recheck_limit 
 */
async function auto_start_node(timer_seconds_delay: number = 60, recheck_limit: number = 60) {
  const appLocalDataDirPath = await appLocalDataDir();

  let node_starts_automatically = await get_autostart_status();
  let device_is_initialized = await check_device_initialization();

  if (!node_starts_automatically || !device_is_initialized) {
    return // Don't autostart.
  }

  let auto_launch_node = `Staking GUI will try to automatically launch your node every ${timer_seconds_delay} seconds.`
  await invoke("log_and_emit_from_frontend", {
    message: auto_launch_node,
    datadir: appLocalDataDirPath,
    window: appWindow,
  });

  let is_node_latest_version = false;
  let recheck_count = 0;
  if (!isNodeAutostartIntervalActive) {
    isNodeAutostartIntervalActive = true;
    let AutoStartNode = setInterval(async () => {
      recheck_count += 1;
      let recheck_message = "Trying to launch node automatically. Attempt:" + recheck_count;
      await invoke("log_and_emit_from_frontend", {
        message: recheck_message,
        datadir: appLocalDataDirPath,
        window: appWindow,
      });

      let has_node_started_successfully = await start_device();

      // First update node to latest version.
      if (!is_node_latest_version) {
        const appLocalDataDirPath = await appLocalDataDir();
        is_node_latest_version = await invoke("update_edge_cli_from_frontend", {
          datadir: appLocalDataDirPath,
          window: appWindow,
        });
      }

      if (has_node_started_successfully) {
        let ok_node_started_message = "Your node has successfully autostarted! You can now close the staking GUI.";
        await invoke("log_and_emit_from_frontend", {
          message: ok_node_started_message,
          datadir: appLocalDataDirPath,
          window: appWindow,
        });
        send_notification("Node autostarted", "Your Edge node has successfully autostarted!");
        clearInterval(AutoStartNode); // Stop autochecking
      }

      if (recheck_count >= recheck_limit) {
        let error_message = "Could not start your node automatically. Try manually starting the node, do you meet all requirements? You might need to enable Docker launch at start. If the error persists, please contact support.";
        await invoke("log_and_emit_from_frontend", {
          message: error_message,
          datadir: appLocalDataDirPath,
          window: appWindow,
        });
        clearInterval(AutoStartNode); // Stop autochecking
      }
    }, timer_seconds_delay * 1000);


  }
}

async function start_device() {
  const appLocalDataDirPath = await appLocalDataDir();
  let has_device_started_successfully: boolean = await invoke("device_start", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });

  return has_device_started_successfully

}


auto_start_node();
</script>

<template>
  <div v-if="!auto_start_enabled" class="card">
    <button type="button" @click="enable_autostart()">Enable Autostart</button>
  </div>
  <div v-else="auto_start_enabled" class="card">
    <button type="button" @click="disable_autostart()">Disable Autostart</button>
  </div>
</template>
