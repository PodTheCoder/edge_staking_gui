<script setup lang="ts">
import { appLocalDataDir } from '@tauri-apps/api/path'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

const Node_Control_Response = ref('')

async function device_start_from_frontend_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir()
  await invoke('device_start_from_frontend', {
    checklatestbinary: false,
    datadir: appLocalDataDirPath,
    window: appWindow
  })
}

async function device_stop_from_frontend_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir()
  Node_Control_Response.value = await invoke('device_stop_from_frontend', {
    checklatestbinary: false,
    datadir: appLocalDataDirPath,
    window: appWindow
  })
}

async function update_cli_emitter() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir()
  Node_Control_Response.value = await invoke('update_edge_cli_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })
}

</script>

<template>
  <div>
    <h2>Node Control Panel</h2>
    <div class="card">
      <button type="button" @click="device_start_from_frontend_emitter()">
        Start Node
      </button>
      <button type="button" @click="device_stop_from_frontend_emitter()">
        Stop Node
      </button>
      <button type="button" @click="update_cli_emitter()">
        Update CLI
      </button>
    </div>
  </div>
</template>
