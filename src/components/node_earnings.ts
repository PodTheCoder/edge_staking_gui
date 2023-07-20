import { appLocalDataDir } from '@tauri-apps/api/path'
import { appWindow } from '@tauri-apps/api/window'
import { get_node_wallet_from_config } from './intialization'
import { invoke } from '@tauri-apps/api'
import { send_notification } from './notification'
import { exchangeRate, tx } from '@edge/index-utils'
import { get_index_url } from './utils'




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


  const index_url: string = await get_index_url()

  const txs = await tx.transactions(index_url, wallet_from_config)
  const exchange_rate_usd_to_xe = (await exchangeRate.current(index_url)).rate
  let found_one_or_more_new_node_earnings = false

  for (const tx of txs.results.reverse()) {
    const config_latest_processed_transaction_timestamp = await get_last_node_payment()
    if (await is_node_transaction(tx)) {
      if (tx.timestamp > config_latest_processed_transaction_timestamp) {
        found_one_or_more_new_node_earnings = true
        await send_node_earning_notification(tx)
      }
    }
    if (await is_lottery_transaction(tx)) {
      if (tx.timestamp > config_latest_processed_transaction_timestamp) {
        found_one_or_more_new_node_earnings = true
        await send_lottery_earning_notification(tx)
      }
    }
  }

  return found_one_or_more_new_node_earnings

  async function is_node_transaction(tx: tx.Tx) {
    const edge_payout_wallet = 'xe_7Bb7B633834f251a8D4132028f5749Ac03010982'
    const node_earnings_memo_str = 'Node Earnings'
    return ((typeof tx.data.memo === 'string') && tx.data.memo.includes(node_earnings_memo_str) && (tx.sender == edge_payout_wallet))
  }

  async function is_lottery_transaction(tx: tx.Tx) {
    const lottery_winnings_memo_str = 'Lottery Winnings'
    const edge_lottery_wallet = 'xe_127eaC00394edf285B1B5936617329e01c9756c5'
    return ((typeof tx.data.memo === 'string') && tx.data.memo.includes(lottery_winnings_memo_str) && (tx.sender == edge_lottery_wallet))
  }

  async function send_node_earning_notification(tx: tx.Tx) {
    const current_tx_amount = tx.amount
    await set_last_node_payment(tx.timestamp)
    const pretty_date = new Date(tx.timestamp)
    const pretty_node_earnings = current_tx_amount / 1000000
    const pretty_node_earnings_in_dollars = pretty_node_earnings * exchange_rate_usd_to_xe
    const ok_message = `You earned ${pretty_node_earnings.toFixed(6)} XE (${pretty_node_earnings_in_dollars.toFixed(6)}\$)! \nThe transaction was received on ${pretty_date.toString()}.`
    await invoke('log_and_emit_from_frontend', {
      message: ok_message,
      datadir: appLocalDataDirPath,
      window: appWindow
    })
    send_notification('Received Edge Node Earnings', ok_message)
  }

  async function send_lottery_earning_notification(tx: tx.Tx) {
    const current_tx_amount = tx.amount
    await set_last_node_payment(tx.timestamp)
    const pretty_date = new Date(tx.timestamp)
    const pretty_lottery_earnings = current_tx_amount / 1000000
    const pretty_lottery_earnings_in_dollars = pretty_lottery_earnings * exchange_rate_usd_to_xe
    const ok_message = `You earned ${pretty_lottery_earnings.toFixed(6)} XE (${pretty_lottery_earnings_in_dollars.toFixed(6)}\$)! \nThe transaction was received on ${pretty_date.toString()}.`
    await invoke('log_and_emit_from_frontend', {
      message: ok_message,
      datadir: appLocalDataDirPath,
      window: appWindow
    })
    send_notification('Congratulations! You won the Edge Lottery!', ok_message)
  }
}
