<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window';

const Requirements_response = ref("");

async function get_edge_cli_download_url() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  Requirements_response.value = await invoke("get_edge_cli_download_url", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
}
</script>

<template>
  <div class="card">
    <button type="button" @click="get_edge_cli_download_url()">Get Edge CLI Download URL</button>
  </div>

  <p>{{ Requirements_response }}</p>
</template>
