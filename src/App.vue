<script setup lang="ts">
import Add_Device from './components/Add_Device.vue'
import Auto_Start from './components/Auto_Start.vue'
import Current_Status from './components/Current_Status.vue'
import Install_Edge_Cli from './components/Install_Edge_Cli.vue'
import LaunchWindowVisibility from './components/LaunchWindowVisibility.vue'
import Post_Initialization_Autocheck from './components/Post_Initialization_Autocheck.vue'
import Post_Initialization_Node_Control from './components/Post_Initialization_Node_Control.vue'
import Post_Initialization_Node_Info from './components/Post_Initialization_Node_Info.vue'
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
const Node_Online_Message = ref('')

async function call_start_device_for_first_time() {
  start_device_for_first_time(deviceInitialized, Node_Online_Message)
}

const App_version = ref()
const network = ref('')
const App_name = ref()
const log_location = ref('')
const staking_url = ref('')

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
  // Set device to not initialized.
  // Initialization status is used by functions running on interval.
  await invoke('set_device_not_initialized_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })

  // Stop node if running
  await invoke('device_stop_from_frontend', {
    checklatestbinary: false,
    datadir: appLocalDataDirPath,
    window: appWindow
  })

  // Complete reset of config
  await invoke('reset_config_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })
  sync_initialization_status(deviceInitialized)
}

/**
 * Get network eg. testnet or mainnet
 */
async function get_network() {
  const appLocalDataDirPath = await appLocalDataDir()
  network.value = await invoke('get_network_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })
}

/**
 * Switch network eg. testnet or mainnet
 * Function is inlined because Vue unwraps a reference:
* https://github.com/vuejs/composition-api/issues/605
 */
async function switch_network() {
  const appLocalDataDirPath = await appLocalDataDir()
  if (network.value == 'mainnet') {
    await invoke('set_network_from_frontend', {
      network: 'testnet',
      datadir: appLocalDataDirPath,
      window: appWindow
    })
  }
  else {
    await invoke('set_network_from_frontend', {
      network: 'mainnet',
      datadir: appLocalDataDirPath,
      window: appWindow
    })

  }
  await get_network()
  const network_next_step = `Your network has been set to ${network.value}. Make sure you update the CLI and that your stake is set correctly. This might require going back to setup.`
  await invoke('log_and_emit_from_frontend', {
    message: network_next_step,
    datadir: appLocalDataDirPath,
    window: appWindow
  })
  await get_staking_url()
}

async function get_config_location() {
  const appLocalDataDirPath = await appLocalDataDir()
  log_location.value = await invoke('get_log_location_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })
}

async function get_staking_url() {
  await get_network()
  const mainnet_wallet_url = 'https://wallet.xe.network/staking'
  const testnet = 'testnet'
  const testnet_wallet_url = 'https://wallet.test.network/staking'

  if (network.value == testnet) {
    return staking_url.value = testnet_wallet_url
  }
  else {
    return staking_url.value = mainnet_wallet_url
  }
}

get_app_version()
get_app_name()
get_config_location()
get_network()
sync_initialization_status(deviceInitialized)
sync_launch_minimized_status()
get_staking_url()
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
        <p>1. Create a Host stake at: {{ staking_url }}</p>
      </div>

      <div class="step">
        <p>2. Install the latest Edge CLI.</p>
        <Install_Edge_Cli />
      </div>

      <div class="step">
        <p>3. Get your <i>Device Token</i></p>
        <Add_Device />
      </div>

      <div class="step">
        <p>4. Assign your <i>device token</i>. Wait 2-5 minutes until it is confirmed.</p>
      </div>
      <div class="step">
        <p>5. Start your node.</p>
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
        <Post_Initialization_Node_Info />
      </div>
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
    <div style="left: 8px; white">
      <span style="font-size: small; color: gray;" @click="switch_network()">
        Network: {{ network }} |
      </span>
      <span style="font-size: small; color: gray;">
        GUI Version: {{ App_version }}
      </span>
      <br />
      <span style="font-size: small; color: gray;">
        Log: {{ log_location }}
      </span>
    </div>
  </div>
</template>
