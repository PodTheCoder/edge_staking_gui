<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import Requirements from "./components/Requirements.vue";
import Install_Edge_Cli from "./components/Install_Edge_Cli.vue";
import Get_Edge_Cli_Download from "./components/Get_Edge_Cli_Download_Url.vue"
import Node_Control from "./components/Node_Control.vue";
</script>

<template>
  <div class="container">
    <h1>Edge Staking GUI</h1>
  <p>1. Check if your system is ready for staking.</p>

    <Requirements />

    <p>2. Install the latest Edge CLI.</p>
    <p>Note: The automatic install can make the Edge Staking GUI temporarily unresponsive.</p>

    <Install_Edge_Cli />

    <!-- <p>Option 2: Manually download Edge CLI and place in your staking GUI installation folder.</p>
        <Get_Edge_Cli_Download /> -->

    <!-- <p>3. Add your device code.</p>
                <Greet /> -->

    <p>3. Control your node. Currently requires your device to already be setup. In a future version this setup will also
      be automated.</p>
    <Node_Control />

    <!-- TODO: Add checkbox for auto starting the staking GUI. -->


  </div>
</template>