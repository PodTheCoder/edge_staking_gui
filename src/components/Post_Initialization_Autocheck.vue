<script setup lang="ts">
import { appLocalDataDir } from '@tauri-apps/api/path'
import { appWindow } from '@tauri-apps/api/window'
import { check_node_earnings } from './node_earnings'
import { invoke } from '@tauri-apps/api/tauri'
import { send_notification } from './notification'
import { check_device_initialization, check_node_online_status } from './intialization'

let isPostInitializationAutocheckActive = false
/**
 *
 * Automatically starts node if autostart is set to active.
 *
 * @param timer_seconds_delay
 * @param recheck_limit
 */
async function PostInitializationAutocheck(timer_seconds_delay = 3600) {
  const appLocalDataDirPath = await appLocalDataDir()

  const device_is_initialized = await check_device_initialization()

  if (!device_is_initialized) {
    // Don't autostart.
    return
  }

  let recheck_count = 0
  if (!isPostInitializationAutocheckActive) {
    isPostInitializationAutocheckActive = true
    // @ts-ignore
    const AutoCheckNode = setInterval(async () => {
      recheck_count += 1

      const device_is_initialized = await check_device_initialization()
      if (!device_is_initialized) {
        // @ts-ignore
        clearInterval(AutoCheckNode)
      }


      const recheck_msg = 'Autochecking for node earnings & node online status. Check nr:' + recheck_count
      await invoke('log_and_emit_from_frontend', {
        message: recheck_msg,
        datadir: appLocalDataDirPath,
        window: appWindow
      })

      check_node_earnings()

      // Check online status
      const node_address: string = await invoke('get_node_address_from_frontend', {
        datadir: appLocalDataDirPath,
        window: appWindow
      })

      if ((await check_node_online_status(node_address)) === false) {
        const err_msg = 'Node not online according to index. Check if your node is running. If the issue persists, contact support on the Edge Discord: https://ed.ge/discord.'
        await invoke('log_and_emit_from_frontend', {
          message: err_msg,
          datadir: appLocalDataDirPath,
          window: appWindow
        })
        send_notification('Node not online.', 'Your Edge node is not currently online according to index. Double check if your node is started. If you keep getting this error message, please contact support on the Edge Discord: https://ed.ge/discord.')

      }



    }, timer_seconds_delay * 1000)


  }
}

PostInitializationAutocheck()
</script>

<template>
  <div></div>
</template>
