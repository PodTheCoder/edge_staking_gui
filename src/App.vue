<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Install_Edge_Cli from "./components/Install_Edge_Cli.vue";
import Post_Initialization_Node_Control from "./components/Post_Initialization_Node_Control.vue";
import Curstatus from "./components/Curstatus.vue";
import Add_Device from "./components/Add_Device.vue";
import Autostart from "./components/Autostart.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from "@tauri-apps/api/path";
import { appWindow } from "@tauri-apps/api/window";
import { ref } from "vue";


import { sync_launch_minimized_status } from "./components/window_visibility";
import LaunchWindowVisibility from "./components/LaunchWindowVisibility.vue";
import { sync_initialization_status, start_device_for_first_time } from "./components/intialization";
import Post_Initialization_Autocheck from "./components/Post_Initialization_Autocheck.vue";


// Initialize consts
const deviceInitialized = ref(false); // default state is uninitialized
const Node_Online_Message = ref();


async function call_start_device_for_first_time() {
  start_device_for_first_time(deviceInitialized, Node_Online_Message)
}

/**
 * Returns program back to the setup stage.
 */
async function back_to_setup() {
  const appLocalDataDirPath = await appLocalDataDir();
  await invoke("set_device_not_initialized_from_frontend", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
  sync_initialization_status(deviceInitialized);
}

sync_initialization_status(deviceInitialized);
sync_launch_minimized_status();
</script>

<template>
  <div class="sticky container">
    <Suspense>
      <Curstatus />
    </Suspense>
  </div>
  <div v-if="!deviceInitialized" class="container">
    <!-- Initialize device -->

    <div class="step">
      <p>1. Install the latest Edge CLI.</p>
      <Install_Edge_Cli />
    </div>

    <div class="step">
      <p>2. Get your <i>Device Token</i></p>
      <Add_Device />
    </div>

    <div class="step">
      <p>3. Wait 5-10 minutes until your device token assignment is confirmed.</p>
    </div>

    <div class="step">
      <p>4. Start your node.</p>
      <div class="card">
        <button type="button" @click="call_start_device_for_first_time()">Start Node</button>
        <p>{{ Node_Online_Message }}</p>
      </div>
    </div>

  </div>

  <div v-else="deviceInitialized" class="container">
    <div class="step">
      <Post_Initialization_Node_Control />
    </div>
    <div class="step">
      <Autostart />
    </div>
    <div class="step">
      <LaunchWindowVisibility />
    </div>
    <div class="step">
      <p>Anything went wrong? You can go back to the setup.</p>
      <div class="card">
        <button type="button" @click="back_to_setup()">Back to Setup.</button>
      </div>
    </div>
    <Post_Initialization_Autocheck />

  </div>
</template>