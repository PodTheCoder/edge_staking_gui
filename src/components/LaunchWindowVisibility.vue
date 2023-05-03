<script setup lang="ts">
import { ref } from "vue";
import { appLocalDataDir } from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from "@tauri-apps/api/tauri";
import { get_launch_minimized_status } from "./window_visibility";

// Initialize consts
const launch_window_visibility = ref(false); // Default state, gets overwritten through get_autostart_status

async function change_minimized_state_backend(launchminimized: boolean) {
  const appLocalDataDirPath = await appLocalDataDir();
  await invoke("set_launch_minimized_status_from_frontend", {
    launchminimized: launchminimized,
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
  set_minimized_status_in_frontend();

}
async function enable_minimized() {
  change_minimized_state_backend(true);
  const appLocalDataDirPath = await appLocalDataDir();

  let app_launches_minimized = await get_launch_minimized_status()
  if (app_launches_minimized) {
    let ok_enabled_minimized = "The Staking GUI will launch minimized. Combine this with autostart for a seamless experience."
    await invoke("log_and_emit_from_frontend", {
      message: ok_enabled_minimized,
      datadir: appLocalDataDirPath,
      window: appWindow,
    });
  }
}

async function disable_minimized() {
  change_minimized_state_backend(false);
  const appLocalDataDirPath = await appLocalDataDir();

  let app_launches_minimized = await get_launch_minimized_status()
  if (!app_launches_minimized) {
    let ok_disabled_minimized = "The Staking GUI will not launch minimized."
    await invoke("log_and_emit_from_frontend", {
      message: ok_disabled_minimized,
      datadir: appLocalDataDirPath,
      window: appWindow,
    });
  }
}

async function set_minimized_status_in_frontend() {
  launch_window_visibility.value = await get_launch_minimized_status();
  return launch_window_visibility
}


set_minimized_status_in_frontend();
</script>

<template>
  <p>Start the GUI minimized.</p>
  <div v-if="!launch_window_visibility" class="card">
    <button type="button" @click="enable_minimized()">Enable Minimized</button>
  </div>
  <div v-else="auto_start_enabled" class="card">
    <button type="button" @click="disable_minimized()">Disable Minimized</button>
  </div>
</template>
