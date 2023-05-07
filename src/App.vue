<script setup lang="ts">
import Add_Device from './components/Add_Device.vue'
import Auto_Start from './components/Auto_Start.vue'
import Current_Status from './components/Current_Status.vue'
import Install_Edge_Cli from './components/Install_Edge_Cli.vue'
import LaunchWindowVisibility from './components/LaunchWindowVisibility.vue'
import Post_Initialization_Autocheck from './components/Post_Initialization_Autocheck.vue'
import Post_Initialization_Node_Control from './components/Post_Initialization_Node_Control.vue'
import { appLocalDataDir } from '@tauri-apps/api/path'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'
import { sync_launch_minimized_status } from './components/window_visibility'
import { getName, getVersion } from '@tauri-apps/api/app'
import { start_device_for_first_time, sync_initialization_status } from './components/intialization'




// Initialize consts
// default state is uninitialized
const deviceInitialized = ref(false)
const Node_Online_Message = ref()

async function call_start_device_for_first_time() {
  start_device_for_first_time(deviceInitialized, Node_Online_Message)
}

const App_version = ref()
const App_name = ref()
/**
 * Get Application Version
 */
async function get_app_version() {
  App_version.value = await getVersion()
}

/**
 * Get Application Name
 */
async function get_app_name() {
  App_name.value = await getName()
}

/**
 * Returns program back to the setup stage.
 */
async function back_to_setup() {
  const appLocalDataDirPath = await appLocalDataDir()
  await invoke('set_device_not_initialized_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })
  sync_initialization_status(deviceInitialized)
}

get_app_version()
get_app_name()
sync_initialization_status(deviceInitialized)
sync_launch_minimized_status()
</script>

<template>
  <div>
    <div class="sticky container">
      <Suspense>
        <Current_Status />
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
          <button type="button" @click="call_start_device_for_first_time()">
            Start Node
          </button>
          <p>{{ Node_Online_Message }}</p>
        </div>
      </div>
    </div>

    <div v-else class="container">
      <div class="step">
        <Post_Initialization_Node_Control />
      </div>
      <div class="step">
        <Auto_Start />
      </div>
      <div class="step">
        <LaunchWindowVisibility />
      </div>
      <div class="step">
        <p>Anything went wrong? You can go back to the setup.</p>
        <div class="card">
          <button type="button" @click="back_to_setup()">
            Back to Setup.
          </button>
        </div>
      </div>
      <Post_Initialization_Autocheck />
    </div>
    <div style="position:absolute; right: 8px">
      <p style="font-size: small; color: gray;">
        v. {{ App_version }}
      </p>
    </div>
  </div>
</template>
