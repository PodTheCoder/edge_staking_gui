<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import Requirements from "./components/Requirements.vue";
import Install_Edge_Cli from "./components/Install_Edge_Cli.vue";
import Get_Edge_Cli_Download from "./components/Get_Edge_Cli_Download_Url.vue"
</script>

<template>
  <div class="container">
    <h1>Edge Staking GUI</h1>
    <p>1. Check if your system is ready for staking.</p>

    <Requirements />

    <p>2. Install the latest Edge CLI.</p>
    <p>Option 1: Automatic install. (Administrator permission required)</p>
    <p>Note: The automatic install can make the Edge Staking GUI temporarily unresponsive.</p>

  <Install_Edge_Cli />

  <p>Option 2: Manually download Edge CLI and place in your staking GUI installation folder.</p>
  <Get_Edge_Cli_Download />

  <!-- TODO: Compose the url based on the OS + processor + bitness.  -->
  <!-- TODO: Checksum the Edge CLI -->
  <!-- <div class="row">
      <a href="https://vitejs.dev" target="_blank">
                <img src="/vite.svg" class="logo vite" alt="Vite logo" />
                                  </a>
                                  <a href="https://tauri.app" target="_blank">
                                    <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
                                  </a>
                                  <a href="https://vuejs.org/" target="_blank">
                                    <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
                                  </a>
                                </div> -->

    <p>3. Add your device code.</p>
    <Greet />

    <p>4. Stake!</p>

    <!-- TODO: Add checkbox for auto starting the staking GUI. -->


  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
