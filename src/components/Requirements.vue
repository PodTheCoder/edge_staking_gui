<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const Requirements_response = ref("");
const name = ref("");

async function check_requirements() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  Requirements_response.value = await invoke("initialize_prereqs");
}
</script>

<template>
  <div class="card">
    <button type="button" @click="check_requirements()">Check System</button>
  </div>

  <p>{{ Requirements_response }}</p>
</template>
