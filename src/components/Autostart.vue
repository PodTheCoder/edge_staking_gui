<script setup lang="ts">
import { ref } from "vue";
import { enable, isEnabled, disable } from "tauri-plugin-autostart-api";

const Autostart_Status = ref(false);

async function get_autostart_status() {
  Autostart_Status.value = await isEnabled()
}

//TODO: Expand autostart functionality.
async function enable_autostart() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await enable();
}

async function disable_autostart() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await disable();
}

get_autostart_status()
</script>

<template>
  <div class="card">
    <button type="button" @click="enable_autostart()">Enable Autostart</button>
    <button type="button" @click="disable_autostart()">Disable Autostart</button>
    <button type="button" @click="get_autostart_status()">Get Autostart Status</button>

    <p>{{ Autostart_Status }}</p>
  </div>
</template>
