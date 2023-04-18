<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from '@tauri-apps/api/path';

const Requirements_Response = ref("");

async function check_requirements() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

  const appLocalDataDirPath = await appLocalDataDir();
  Requirements_Response.value = await invoke("check_requirements", { datadir: appLocalDataDirPath });
}
</script>

<template>
  <div class="card">
    <button type="button" @click="check_requirements()">Check System</button>
  </div>

  <p>{{ Requirements_Response }}</p>
</template>
