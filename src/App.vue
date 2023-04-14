<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import Requirements from "./components/Requirements.vue";

</script>

<template>
  <div class="container">
    <h1>Edge Staking GUI</h1>
    <h2>1. Check if your system is ready for staking.</h2>

    <Requirements />

    <h2>2. Check if the latest Edge CLI is installed.</h2>
    <!-- TODO: Compose the url based on the OS + processor + bitness.  -->
    <!-- TODO: Checksum the Edge CLI -->
    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>

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
