<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const Requirements_response = ref("");

async function device_start_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  Requirements_response.value = await invoke("device_start");
}
async function device_stop_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  Requirements_response.value = await invoke("device_stop");
}

async function device_info_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  Requirements_response.value = await invoke("device_info");
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

  <p>{{ Requirements_response }}</p>
</template>
