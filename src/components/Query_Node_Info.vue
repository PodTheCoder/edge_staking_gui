<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window';

const api_response = ref("");
const node_address = ref("");

async function get_node_session_from_index_api() {
  const appLocalDataDirPath = await appLocalDataDir();
  api_response.value = await invoke("query_node_session", {
    name: node_address.value,
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
}
</script>

<template>
  <div class="card">
    <input id="node-address-input" v-model="node_address" placeholder="Enter node address..." />
    <button type="button" @click="get_node_session_from_index_api()">Derive Node Wallet Address</button>
  </div>

  <p>{{ api_response }}</p>
</template>
