<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Query_Node_Info from "./components/Query_Node_Info.vue";
import Install_Edge_Cli from "./components/Install_Edge_Cli.vue";
import Stop_Node from "./components/Stop_Node.vue";
import Curstatus from "./components/Curstatus.vue";
import Add_Device from "./components/Add_Device.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from "@tauri-apps/api/path";
import { appWindow } from "@tauri-apps/api/window";
import { ref } from "vue";


import { session } from '@edge/index-utils';


const deviceInitialized = ref(false); // default state is uninitialized

const Node_Online_Message = ref();

async function load_initialization_status() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const appLocalDataDirPath = await appLocalDataDir();
  deviceInitialized.value = await invoke("load_device_initialization_status", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
}

async function start_device() {
  const appLocalDataDirPath = await appLocalDataDir();
  let has_device_started_successfully: boolean = await invoke("device_start", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });

  return has_device_started_successfully

}

async function start_device_for_first_time() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let has_device_started_successfully = await start_device();

  if (has_device_started_successfully == true) {
    Node_Online_Message.value = "Initializing node for the first time. Check the status bar at the top for the latest progress."
    complete_initialization_flow();
  } else {
    Node_Online_Message.value = "Device was not started successfully. Check the status bar at the top for the error code."
  }
}

/**
 * 
 * @param node_address XE node address
 * Checks if the XE node address is online.
 */
async function helper_check_node_online_status(node_address: string) {
  const appLocalDataDirPath = await appLocalDataDir();
  try {
    const sess = await session.session('https://index.xe.network', node_address)
    console.log(JSON.stringify(sess))
    console.log(sess.lastActive)

    // TODO: Check online value

    return true

  } catch (e) {
    let error_string = JSON.stringify(e);
    Node_Online_Message.value = "Node not seen yet. Will automatically recheck."
    let error_message = "Node not found http error code:" + error_string;

    // log and emit to backend.
    await invoke("log_and_emit_from_frontend", {
      message: error_message,
      datadir: appLocalDataDirPath,
      window: appWindow,
    });

    await invoke("log_and_emit_from_frontend", {
      message: Node_Online_Message.value,
      datadir: appLocalDataDirPath,
      window: appWindow,
    });
    return false
  }


}


/**
 * Flow for checking the node online status. 
 * Autochecks if node is online. 
 * If true, set program stage to post-intialization.
 */
// Check node and set initialization status. If the node is online once, it is assumed to be correctly initialized.
async function complete_initialization_flow() {
  const appLocalDataDirPath = await appLocalDataDir();
  let node_address: string = await invoke("load_node_online_status", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });

  // Check online status of node and set initialization status based on result.
  let error_string = "Unset";
  if (node_address != error_string) {
    // Assume node address is valid.
    let check_message = "Your node was started successfully! Sit back and relax. The Staking GUI will automatically keep checking if your node is online."
    await invoke("log_and_emit_from_frontend", {
      message: check_message,
      datadir: appLocalDataDirPath,
      window: appWindow,
    });

    await auto_recheck_node_online(appLocalDataDirPath, node_address);

  }
  else {
    let error_message = "Node address is not set. Please complete the other setup steps.";
    await invoke("log_and_emit_from_frontend", {
      message: error_message,
      datadir: appLocalDataDirPath,
      window: appWindow,
    });

  }
}

let isNodeOnlineAutocheckActive = false;
async function auto_recheck_node_online(appLocalDataDirPath: string, node_address: string) {
  let timer_seconds_delay = 60; // every min
  let recheck_count = 0;
  let recheck_limit = 120; // If node is not online after x rechecks, stop checking.

  if (!isNodeOnlineAutocheckActive) {
    isNodeOnlineAutocheckActive = true;
    let AutoCheckNodeOnline = setInterval(async () => {
      recheck_count += 1;
      let recheck_message = "Rechecking node online status. Count : " + recheck_count;
      await invoke("log_and_emit_from_frontend", {
        message: recheck_message,
        datadir: appLocalDataDirPath,
        window: appWindow,
      });

      let is_node_online = await helper_check_node_online_status(node_address);

      if (is_node_online) {
        // set initialized flag
        await invoke("set_device_fully_initialized", {
          datadir: appLocalDataDirPath,
          window: appWindow,
        });
        load_initialization_status();
        clearInterval(AutoCheckNodeOnline); // Stop autochecking
      }

      if (recheck_count >= recheck_limit) {
        let error_message = "Could not find your node online after several retries. Please double check if your device code was correctly assigned. Try starting the node again. If the error keeps persisting, contact support.";
        await invoke("log_and_emit_from_frontend", {
          message: error_message,
          datadir: appLocalDataDirPath,
          window: appWindow,
        });
        clearInterval(AutoCheckNodeOnline); // Stop autochecking
      }

    }, timer_seconds_delay * 1000);


  }

}

/**
 * Returns program back to the setup stage.
 */
async function back_to_setup() {
  const appLocalDataDirPath = await appLocalDataDir();
  await invoke("set_device_not_initialized", {
    datadir: appLocalDataDirPath,
    window: appWindow,
  });
  load_initialization_status();

}

load_initialization_status(); 
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
        <button type="button" @click="start_device_for_first_time()">Start Node</button>

        <p>{{ Node_Online_Message }}</p>
      </div>
    </div>

  </div>

  <div v-else="deviceInitialized" class="container">
    <div class="step">

      <h2>Your Node Setup Is Complete!</h2>
      <p>From now on, this is your node control panel: </p>
      <div class="card">
        <button type="button" @click="start_device()">Start Node</button>
      </div>
      <Stop_Node />
    </div>
    <div class="step">
      <p>4. Check Your Node Earnings Through Index API. (First derives wallet address)</p>
      <Query_Node_Info />
      <!-- TODO: Add button to return to device initialization. -->
      <!-- TODO: Add checkbox for auto starting the staking GUI. -->
    </div>
    <div class="step">
      <p>Anything went wrong? You can go back to the setup.</p>
      <div class="card">
        <button type="button" @click="back_to_setup()">Back to Setup.</button>
      </div>
    </div>

  </div>
</template>