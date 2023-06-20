<script setup lang="ts">
import { appLocalDataDir } from '@tauri-apps/api/path'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'

const device_address = ref('')
const stake_id = ref('')
const wallet_address = ref('')

async function get_device_address_from_config() {
  const appLocalDataDirPath = await appLocalDataDir()
  device_address.value = await invoke('get_node_address_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })
}

async function get_stake_id_from_config() {
  const appLocalDataDirPath = await appLocalDataDir()
  stake_id.value = await invoke('get_stake_id_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })
}

async function get_wallet_address_from_config() {
  const appLocalDataDirPath = await appLocalDataDir()
  wallet_address.value = await invoke('get_wallet_address_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })
}

get_device_address_from_config()
get_stake_id_from_config()
get_wallet_address_from_config()
</script>

<template>
  <div>
    <h2>Node Info</h2>
    <div class="card" style="line-height:10px">
      <p>Device address: {{ device_address }}</p>
      <p>Stake ID: {{ stake_id }}</p>
      <p>Wallet_address: {{ wallet_address }}</p>
    </div>
  </div>
</template>
