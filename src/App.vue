<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Query_Node_Info from "./components/Query_Node_Info.vue";
import Requirements from "./components/Requirements.vue";
import Install_Edge_Cli from "./components/Install_Edge_Cli.vue";
import Get_Edge_Cli_Download from "./components/Get_Edge_Cli_Download_Url.vue"
import Node_Control from "./components/Node_Control.vue";
import Curstatus from "./components/Curstatus.vue";
</script>

<template>
  <div class="container">
    <Suspense>
      <Curstatus />
    </Suspense>
    <p>1. Check if your system is ready for staking.</p>


    <Requirements />

    <p>2. Install the latest Edge CLI.</p>
    <p>Note: The automatic install can make the Edge Staking GUI temporarily unresponsive.</p>

    <Install_Edge_Cli />

    <!-- <p>Option 2: Manually download Edge CLI and place in your staking GUI installation folder.</p>
                                <Get_Edge_Cli_Download /> -->


    <p>3. Control your node. Currently requires your device to already be setup. In a future version this setup will also
      be automated.</p>
    <Node_Control />

    <p>4. Check Node Info Through Index API</p>
    <Query_Node_Info />

    <!-- TODO: Add checkbox for auto starting the staking GUI. -->


  </div>
</template>