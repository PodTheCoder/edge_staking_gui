<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const Requirements_response = ref("");

async function get_edge_cli_download_url() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  Requirements_response.value = await invoke("get_edge_cli_download_url");
}
</script>

<template>
  <div class="card">
    <button type="button" @click="get_edge_cli_download_url()">Get Edge CLI Download URL</button>
  </div>

  <p>{{ Requirements_response }}</p>
</template>
