<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const Requirements_response = ref("");

async function install_edge_cli() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  Requirements_response.value = await invoke("install_edge_cli");
}
</script>

<template>
  <div class="card">
    <button type="button" @click="install_edge_cli()">Install latest Edge Cli</button>
  </div>

  <p>{{ Requirements_response }}</p>
</template>
