<script setup lang="ts">
import { ref } from "vue";
import { enable, isEnabled, disable } from "tauri-plugin-autostart-api";
import { appLocalDataDir } from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from "@tauri-apps/api/tauri";
import { send_notification } from './notification';


// Initialize consts
const auto_start_enabled = ref(false); // Default state, gets overwritten through get_autostart_status


/// Syncronizes the frontend and backend based on config. Returns the frontend state.
async function sync_autostart_status() {

  /// Change autostart status in frontend. 
  async function set_autostart_status_in_frontend(autostart_status_to_set: boolean) {
    if (autostart_status_to_set === true) {
      await enable();
    }
    else {
      await disable();
    }
    return await isEnabled();
  }
  const appLocalDataDirPath = await appLocalDataDir();

  // get status from config
  let config_autostart_status: boolean = await invoke("get_autostart_status_from_frontend", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
  let frontend_autostart_status = await isEnabled();

  // check if config matches frontend state
  if (config_autostart_status != frontend_autostart_status) {
    if (config_autostart_status === true) {
      frontend_autostart_status = await set_autostart_status_in_frontend(config_autostart_status);
    }
    else {
      frontend_autostart_status = await set_autostart_status_in_frontend(config_autostart_status);
    }

    // Make frontend reflect actual status
    if (frontend_autostart_status === true) {
      let ok_enable_autostart = "Your node will automatically launch when you login. Make sure to also automatically start Docker Desktop when you log in. (https://docs.docker.com/desktop/settings/windows/#general).";
      await invoke("log_and_emit_from_frontend", {
        message: ok_enable_autostart,
        datadir: appLocalDataDirPath,
        window: appWindow,
      });
    } else {
      let ok_disable_autostart = "Your node will not automatically launch at system startup.";
      await invoke("log_and_emit_from_frontend", {
        message: ok_disable_autostart,
        datadir: appLocalDataDirPath,
        window: appWindow,
      });
    }
  }

  auto_start_enabled.value = frontend_autostart_status;
  return frontend_autostart_status;
}

/// Set autostart in config, and sync up config and backend
async function change_autostart_in_config_and_sync(set_autostart_status_to: boolean) {
  const appLocalDataDirPath = await appLocalDataDir();
  await invoke("set_autostart_status_from_frontend", {
    autostartstatus: set_autostart_status_to,
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
  await sync_autostart_status();

}

async function enable_autostart() {
  change_autostart_in_config_and_sync(true);
}

async function disable_autostart() {
  change_autostart_in_config_and_sync(false);
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
async function auto_start_node(timer_seconds_delay: number = 30, recheck_limit: number = 60) {
  const appLocalDataDirPath = await appLocalDataDir();

  let node_starts_automatically = await sync_autostart_status();
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

      if (recheck_count == 4) {
        send_notification("Node Autostart Failing", "Will keep trying. Are you connected to the internet and is Docker running?");
      }

      let recheck_message = "Trying to launch node automatically. Attempt:" + recheck_count;
      await invoke("log_and_emit_from_frontend", {
        message: recheck_message,
        datadir: appLocalDataDirPath,
        window: appWindow,
      });


      // First update node to latest version.
      if (!is_node_latest_version) {
        const appLocalDataDirPath = await appLocalDataDir();
        is_node_latest_version = await invoke("update_edge_cli_from_frontend", {
          datadir: appLocalDataDirPath,
          window: appWindow,
        });

        if (is_node_latest_version) {
          let node_updated_msg = "Your Edge CLI is the latest version. Attempting to start your node."
          await invoke("log_and_emit_from_frontend", {
            message: node_updated_msg,
            datadir: appLocalDataDirPath,
            window: appWindow,
          });
        }
      }

      let has_node_started_successfully = await start_device();

      if (has_node_started_successfully) {
        let ok_node_started_message = "Your node has successfully autostarted! You can now close the staking GUI.";
        await invoke("log_and_emit_from_frontend", {
          message: ok_node_started_message,
          datadir: appLocalDataDirPath,
          window: appWindow,
        });
        send_notification("Node Autostarted", "Your Edge node has successfully autostarted!");
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
    checklatestbinary: false,
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
