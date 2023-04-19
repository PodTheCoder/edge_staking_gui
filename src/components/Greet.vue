<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window';

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  greetMsg.value = await invoke("greet", {
    name: name.value,
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
}
</script>

<!-- TODO: Rework to Device Add functionality. -->
<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter device code..." />
    <button type="button" @click="greet()">Add</button>
  </div>

  <p>{{ greetMsg }}</p>
</template>
