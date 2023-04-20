<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import { appWindow } from '@tauri-apps/api/window';
import { appLocalDataDir } from '@tauri-apps/api/path';
import { emit } from '@tauri-apps/api/event'

const Status_Response = ref("");

const eventListenerName = "program_status_listener";
const defaultStatus = "Awaiting instructions..."
// Listen to events on current window
await appWindow.listen(
  eventListenerName,
  (event) => Status_Response.value = String(event.payload)
);

// Default status
await emit(eventListenerName, defaultStatus);

// Initialize default config
async function load_config_frontend() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  await invoke("load_config_frontend", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
}

const nodeInitialized = load_config_frontend()

async function emit_event_from_frontend() {
  await emit(eventListenerName,
    'Called from frontend'
  );
}

async function emit_event_from_backend() {
  const appLocalDataDirPath = await appLocalDataDir();
  await invoke("emit_from_backend",
    {
      window: appWindow,
      datadir: appLocalDataDirPath
    });
}
</script>

<template>
  <div class="sticky">
    <h1>Edge Staking GUI</h1>
    <p>Status: {{ Status_Response }}</p>
  </div>

  <!-- <div class="card">
                                <button type="button" @click="emit_event_from_frontend()">Emit from frontend</button>
                                <button type="button" @click="emit_event_from_backend()">Emit from backend</button>
                              </div> -->
</template>
