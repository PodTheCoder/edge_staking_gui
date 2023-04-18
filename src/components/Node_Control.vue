<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from '@tauri-apps/api/path';

const Node_Control_Response = ref("");

async function device_start_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  Node_Control_Response.value = await invoke("device_start", { datadir: appLocalDataDirPath });
}
async function device_stop_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  Node_Control_Response.value = await invoke("device_stop", { datadir: appLocalDataDirPath });
}

async function device_info_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  Node_Control_Response.value = await invoke("device_info", { datadir: appLocalDataDirPath });
}
</script>

<template>
  <div class="card">
    <button type="button" @click="device_start_emitter()">Start </button>
    <br />
    <button type="button" @click="device_stop_emitter()">Stop</button>
    <br />
    <button type="button" @click="device_info_emitter()">Info</button>
  </div>

  <p>{{ Node_Control_Response }}</p>
</template>
