<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Query_Node_Info from "./components/Query_Node_Info.vue";
import Install_Edge_Cli from "./components/Install_Edge_Cli.vue";
import Start_Node from "./components/Start_Node.vue";
import Stop_Node from "./components/Stop_Node.vue";
import Curstatus from "./components/Curstatus.vue";
import Add_Device from "./components/Add_Device.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from "@tauri-apps/api/path";
import { appWindow } from "@tauri-apps/api/window";
import { ref } from "vue";

let deviceInitialized: false;

// TODO: Fix bug with frontend code. It returns correct boolean but does not render.
// async function load_initialization_status() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   const appLocalDataDirPath = await appLocalDataDir();
//   deviceInitialized = await invoke("load_device_initialization_status", {
//     datadir: appLocalDataDirPath,
//     window: appWindow,
//   });

//   console.log(deviceInitialized);

//   return deviceInitialized
// }

// deviceInitialized = await load_initialization_status(); // This is bugged.
// console.log(deviceInitialized);


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
      <Start_Node />
    </div>


  </div>

  <div v-else="deviceInitialized" class="container">
    <div class="step">
      <h2>Control your node</h2>
      <Start_Node />
      <Stop_Node />
    </div>
    <p>4. Check Your Node Earnings Through Index API. (First derives wallet address)</p>
    <Query_Node_Info />
    <!-- TODO: Add button to return to device initialization. -->
    <!-- TODO: Add checkbox for auto starting the staking GUI. -->
  </div>
</template>