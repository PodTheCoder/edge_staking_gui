<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from '@tauri-apps/api/path';

const installMsg = ref("");

async function get_latest_edge_cli() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  installMsg.value = await invoke("install_edge_cli", { datadir: appLocalDataDirPath });
  // greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="card">
    <button type="button" @click="get_latest_edge_cli()">Get Latest Edge Cli</button>
  </div>

  <p>{{ installMsg }}</p>
</template>
