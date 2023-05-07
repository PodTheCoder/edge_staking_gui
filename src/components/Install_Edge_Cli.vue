<script setup lang="ts">
import { appLocalDataDir } from '@tauri-apps/api/path'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'

async function get_latest_edge_cli() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir()
  await invoke('install_edge_cli_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })
}
</script>

<template>
  <div class="card">
    <button type="button" @click="get_latest_edge_cli()">Get Latest Edge Cli</button>
  </div>
</template>
