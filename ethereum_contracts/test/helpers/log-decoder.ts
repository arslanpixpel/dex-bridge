import { ethers } from 'ethers'
import { getContracts } from './contracts'
import { Log } from '@ethersproject/abstract-provider'
export interface DecodedLog {
  address: string
  event: string
  signature: string
  args: any
}
export class LogDecoder {
  private _methodIDs: any
  private readonly _interfaces: any

  constructor (abis: any[] = []) {
    this._methodIDs = {}
    this._interfaces = []
    abis.forEach(abi => {
      const methodInterface = new ethers.utils.Interface(abi)
      Object.keys(methodInterface.events).forEach(evtKey => {
        const evt = methodInterface.events[evtKey]
        // @ts-expect-error
        const signature = evt.topic
        // Handles different indexed arguments with same signature from different contracts
        // Like ERC721/ERC20 Transfer
        // eslint-disable-next-line @typescript-eslint/strict-boolean-expressions
        this._methodIDs[signature] = this._methodIDs[signature] || []
        this._methodIDs[signature].push(evt)
        this._interfaces.push(methodInterface)
      })
    })
  }

  decodeLogs (logs: Log[] = []): DecodedLog[] {
    const results: DecodedLog[] = []
    for (const log of logs) {
      for (let i = 0; i < this._interfaces.length; i++) {
        try {
          const parsedLog = this._interfaces[i].parseLog(log)
          if (parsedLog != null) {
            results.push({
              address: log.address.toLowerCase(),
              event: parsedLog.name,
              signature: parsedLog.signature,
              args: parsedLog.args
            })
          }
        } catch (e) {
        }
      }
    }
    return results
  }
}

export const getLogDecoder = async (): Promise<LogDecoder> => {
  const contracts = await getContracts()

  const abis: any[] = []
  Object.keys(contracts).forEach(c => {
    // @ts-expect-error
    abis.push(contracts[c].interface.format('json'))
  })
  const logDecoder = new LogDecoder(abis)
  return logDecoder
}
