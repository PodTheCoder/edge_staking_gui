import { appLocalDataDir } from '@tauri-apps/api/path'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api'
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification'

export async function send_notification(title: string, body: string) {
  let permissionGranted = await isPermissionGranted()
  if (!permissionGranted) {
    const permission = await requestPermission()
    permissionGranted = permission === 'granted'
    const appLocalDataDirPath = await appLocalDataDir()
    const error_message = 'Tried to send notification but was unable to.'
    await invoke('log_and_emit_from_frontend', {
      message: error_message,
      datadir: appLocalDataDirPath,
      window: appWindow
    })
  }
  if (permissionGranted) {
    sendNotification({ title: title, body: body })
  }

}
