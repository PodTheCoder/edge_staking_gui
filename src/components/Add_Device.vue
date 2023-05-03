<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appLocalDataDir } from '@tauri-apps/api/path';
import { appWindow } from '@tauri-apps/api/window';
import { wallet } from '@edge/xe-utils';

async function create_wallet() {
  const myWallet = wallet.create()
  return myWallet
}

const deviceMsg = ref("");

async function add_device_from_frontend() {
  // Create wallet in frontend 

  const DeviceWallet = create_wallet();

  const address = (await DeviceWallet).address;
  const privatekey = (await DeviceWallet).privateKey;
  const publickey = (await DeviceWallet).publicKey;
  const appLocalDataDirPath = await appLocalDataDir();


  deviceMsg.value = await invoke("add_device_from_frontend", {
    address: address,
    privatekey: privatekey,
    publickey: publickey,
    window: appWindow,
    datadir: appLocalDataDirPath,
  });
  // greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="card">
    <button type="button" @click="add_device_from_frontend()">Create & Display Device Token</button>
  </div>
  <p>{{ deviceMsg }}</p>
</template>
