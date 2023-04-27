<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Query_Node_Info from "./components/Query_Node_Info.vue";
import Install_Edge_Cli from "./components/Install_Edge_Cli.vue";
import Node_Control from "./components/Node_Control.vue";
import Curstatus from "./components/Curstatus.vue";
import Add_Device from "./components/Add_Device.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from "@tauri-apps/api/path";
import { appWindow } from "@tauri-apps/api/window";

// Initialize default config
async function load_config_frontend() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  await invoke("load_config_frontend", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });

}

async function frontend_create_config_if_not_exists() {
  const appLocalDataDirPath = await appLocalDataDir();
  await invoke("frontend_load_config_if_not_exists", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });

}

frontend_create_config_if_not_exists();
const IsDeviceInitialized = load_config_frontend()

</script>

<template>
  <div class="container">

    <!-- Initialization -->
    <Suspense>
      <Curstatus />
    </Suspense>
    <p>1. Install the latest Edge CLI.</p>
    <Install_Edge_Cli />

    <p>2. Get your <i>Device Code</i></p>
    <Add_Device />

    <p>3. Control your node. Requires that your device is assigned to a stake.</p>
    <Node_Control />

    <!-- Completed initialization -->

    <p>4. Check Your Node Earnings Through Index API. (First derives wallet address)</p>
    <Query_Node_Info />

    <!-- TODO: Add checkbox for auto starting the staking GUI. -->


  </div>
</template>