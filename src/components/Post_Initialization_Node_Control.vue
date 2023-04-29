<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window';

const Node_Control_Response = ref("");

async function device_start_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  await invoke("device_start", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
}

async function device_stop_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  Node_Control_Response.value = await invoke("device_stop", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
}

async function update_cli_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  Node_Control_Response.value = await invoke("update_edge_cli_from_frontend", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
}

</script>

<template>
  <div class="card">
    <button type="button" @click="device_start_emitter()">Start Node</button>
    <button type="button" @click="device_stop_emitter()">Stop Node</button>
    <button type="button" @click="update_cli_emitter()">Update Node</button>
  </div>
</template>
