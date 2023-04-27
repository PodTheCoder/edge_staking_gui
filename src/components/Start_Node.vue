<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window';

const Node_Control_Response = ref("");

async function device_start_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  Node_Control_Response.value = await invoke("device_start", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
}
</script>

<template>
  <div class="card">
    <button type="button" @click="device_start_emitter()">Start </button>

    <p>{{ Node_Control_Response }}</p>
  </div>
</template>
