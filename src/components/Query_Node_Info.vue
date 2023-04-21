<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window';

const api_response = ref("");
const node_address = ref("");

async function get_node_session_from_index_api() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  api_response.value = await invoke("greet", {
    name: node_address.value,
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
}
</script>

<!-- TODO: Rework to Device Add functionality. -->
<template>
  <div class="card">
    <input id="greet-input" v-model="node_address" placeholder="Enter node address..." />
    <button type="button" @click="get_node_session_from_index_api()">Get Session Info</button>
  </div>

  <p>{{ api_response }}</p>
</template>
