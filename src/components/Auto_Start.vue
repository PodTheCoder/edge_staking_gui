<script setup lang="ts">
import { appLocalDataDir } from '@tauri-apps/api/path'
import { appWindow } from '@tauri-apps/api/window'
import { check_device_initialization } from './intialization'
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'
import { send_notification } from './notification'
import { disable, enable, isEnabled } from 'tauri-plugin-autostart-api'


// Initialize consts
// Default state is false, gets overwritten through get_autostart_status
const auto_start_enabled = ref(false)


/// Syncronizes the frontend and backend based on config. Returns the frontend state.
async function sync_autostart_status() {

  /// Change autostart status in frontend.
  async function set_autostart_status_in_frontend(autostart_status_to_set: boolean) {
    if (autostart_status_to_set === true) {
      await enable()
    }
    else {
      await disable()
    }
    return await isEnabled()
  }
  const appLocalDataDirPath = await appLocalDataDir()

  // get status from config
  const config_autostart_status: boolean = await invoke('get_autostart_status_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })
  let frontend_autostart_status = await isEnabled()

  // check if config matches frontend state
  if (config_autostart_status != frontend_autostart_status) {
    if (config_autostart_status === true) {
      frontend_autostart_status = await set_autostart_status_in_frontend(config_autostart_status)
    }
    else {
      frontend_autostart_status = await set_autostart_status_in_frontend(config_autostart_status)
    }

    // Make frontend reflect actual status
    if (frontend_autostart_status === true) {
      const ok_enable_autostart = 'Your node will automatically launch when you login. Make sure to also automatically start Docker Desktop when you log in. (https://docs.docker.com/desktop/settings/windows/#general).'
      await invoke('log_and_emit_from_frontend', {
        message: ok_enable_autostart,
        datadir: appLocalDataDirPath,
        window: appWindow
      })
    }
    else {
      const ok_disable_autostart = 'Your node will not automatically launch at system startup.'
      await invoke('log_and_emit_from_frontend', {
        message: ok_disable_autostart,
        datadir: appLocalDataDirPath,
        window: appWindow
      })
    }
  }

  auto_start_enabled.value = frontend_autostart_status
  return frontend_autostart_status
}

/// Set autostart in config, and sync up config and backend
async function change_autostart_in_config_and_sync(set_autostart_status_to: boolean) {
  const appLocalDataDirPath = await appLocalDataDir()
  await invoke('set_autostart_status_from_frontend', {
    autostartstatus: set_autostart_status_to,
    datadir: appLocalDataDirPath,
    window: appWindow
  })
  await sync_autostart_status()

}

async function enable_autostart() {
  change_autostart_in_config_and_sync(true)
}

async function disable_autostart() {
  change_autostart_in_config_and_sync(false)
}


let isNodeAutostartIntervalActive = false

/**
 *
 * Automatically starts node if autostart is set to active.
 *
 * @param timer_seconds_delay
 * @param recheck_limit
 */
async function auto_start_node(timer_seconds_delay = 30, recheck_limit = 60) {
  const appLocalDataDirPath = await appLocalDataDir()

  const node_starts_automatically = await sync_autostart_status()

  if (!node_starts_automatically) {
    // Don't autostart.
    return
  }

  const auto_launch_node_msg = `Staking GUI will try to automatically launch your node every ${timer_seconds_delay} seconds.`
  await invoke('log_and_emit_from_frontend', {
    message: auto_launch_node_msg,
    datadir: appLocalDataDirPath,
    window: appWindow
  })


  let recheck_count = 0
  if (!isNodeAutostartIntervalActive) {
    isNodeAutostartIntervalActive = true
    const AutoStartNode = setInterval(async () => {
      recheck_count += 1

      const device_is_initialized = await check_device_initialization()
      if (!device_is_initialized) {
        clearInterval(AutoStartNode)
      }

      if (recheck_count == 4) {
        send_notification('Node Autostart Failing', 'Will keep trying. Are you connected to the internet and is Docker running?')
      }

      const recheck_message = 'Trying to launch node automatically. Attempt:' + recheck_count
      await invoke('log_and_emit_from_frontend', {
        message: recheck_message,
        datadir: appLocalDataDirPath,
        window: appWindow
      })

      const has_node_started_successfully = await start_device()

      if (has_node_started_successfully) {
        const ok_node_started_message = 'Your node has successfully autostarted! You can now close the staking GUI.'
        await invoke('log_and_emit_from_frontend', {
          message: ok_node_started_message,
          datadir: appLocalDataDirPath,
          window: appWindow
        })
        send_notification('Node Autostarted', 'Your Edge node has successfully autostarted!')
        // Stop autochecking
        clearInterval(AutoStartNode)
      }

      if (recheck_count >= recheck_limit) {
        const error_message = 'Could not start your node automatically. Try manually starting the node, do you meet all requirements? You might need to enable Docker launch at start. If the error persists, please contact support.'
        await invoke('log_and_emit_from_frontend', {
          message: error_message,
          datadir: appLocalDataDirPath,
          window: appWindow
        })
        // Stop autochecking
        clearInterval(AutoStartNode)
      }
    }, timer_seconds_delay * 1000)


  }
}

async function start_device() {
  const appLocalDataDirPath = await appLocalDataDir()
  const has_device_start_from_frontended_successfully: boolean = await invoke('device_start_from_frontend', {
    checklatestbinary: false,
    datadir: appLocalDataDirPath,
    window: appWindow
  })

  return has_device_start_from_frontended_successfully

}


auto_start_node()
</script>

<template>
  <div>
    <p>Automatically launch your node when your computer starts.</p>
    <div v-if="!auto_start_enabled" class="card">
      <button type="button" @click="enable_autostart()">
        Enable Autostart (Recommended)
      </button>
    </div>
    <div v-else class="card">
      <button type="button" @click="disable_autostart()">
        Disable Autostart
      </button>
    </div>
  </div>
</template>
