import { appLocalDataDir } from '@tauri-apps/api/path'
import { appWindow } from '@tauri-apps/api/window'
import { get_node_wallet_from_config } from './intialization'
import { invoke } from '@tauri-apps/api'
import { send_notification } from './notification'
import { tx } from '@edge/index-utils'




async function get_last_node_payment() {
  const appLocalDataDirPath = await appLocalDataDir()
  const last_node_payment_from_config: number = await invoke('get_last_node_payment_from_frontend', {
    datadir: appLocalDataDirPath,
    window: appWindow
  })
  return last_node_payment_from_config
}

async function set_last_node_payment(node_payment_timestamp: number) {
  const appLocalDataDirPath = await appLocalDataDir()
  await invoke('set_last_node_payment_from_frontend', {
    lastnodepayment: node_payment_timestamp,
    datadir: appLocalDataDirPath,
    window: appWindow
  })
}

/**
 * If a new node earning is found, updates logs + status + sends notification.
 * @returns true if found higher node earning than in config.
 */
export async function check_node_earnings() {
  const appLocalDataDirPath = await appLocalDataDir()
  const wallet_from_config = await get_node_wallet_from_config()

  const txs = await tx.transactions('https://index.xe.network', wallet_from_config)

  let api_latest_transaction_timestamp = 0
  let amount_of_latest_transaction_timestamp = 0
  const node_earnings_memo_str = 'Node Earnings'

  for (const tx of txs.results) {
    const current_memo = tx.data.memo

    if ((typeof current_memo === 'string') && current_memo.includes(node_earnings_memo_str)) {
      // is a node earning memo
      console.log(tx.timestamp)
      const current_transaction_timestamp = tx.timestamp
      if (current_transaction_timestamp > api_latest_transaction_timestamp) {
        api_latest_transaction_timestamp = current_transaction_timestamp
        amount_of_latest_transaction_timestamp = tx.amount
      }
    }
  }

  const config_latest_transaction_timestamp = await get_last_node_payment()
  console.log(api_latest_transaction_timestamp)
  if (api_latest_transaction_timestamp > config_latest_transaction_timestamp) {
    console.log('Found new transaction!')
    // Found new node earning transaction!
    await set_last_node_payment(api_latest_transaction_timestamp)
    const pretty_date = new Date(api_latest_transaction_timestamp)
    const pretty_node_earnings = amount_of_latest_transaction_timestamp / 1000000
    const ok_message = `You earned ${pretty_node_earnings} XE! \nThe transaction was received on ${pretty_date.toString()}.`
    await invoke('log_and_emit_from_frontend', {
      message: ok_message,
      datadir: appLocalDataDirPath,
      window: appWindow
    })
    send_notification('Received Node Earnings', ok_message)
    return true
  }
  else {
    return false
  }
}
