<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'
import { emit } from '@tauri-apps/api/event'
import { ref } from 'vue'


const Status_Response = ref('')

const eventListenerName = 'program_status_listener'
const defaultStatus = 'Awaiting instructions...'
// Listen to events on current window
await appWindow.listen(
  eventListenerName,
  (event) => Status_Response.value = String(event.payload)
)

// Default status
await emit(eventListenerName, defaultStatus)
</script>

<template>
  <div class="sticky">
    <h1 class="statusbarh1">Pod's Edge Staking GUI</h1>
    <p class="statusbarp">Status: {{ Status_Response }}</p>
  </div>

  <!-- <div class="card">
                                <button type="button" @click="emit_event_from_frontend()">Emit from frontend</button>
                                <button type="button" @click="emit_event_from_backend()">Emit from backend</button>
                              </div> -->
</template>
