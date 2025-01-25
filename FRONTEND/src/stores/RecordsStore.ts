
import { defineStore } from "pinia"
import { ref } from "vue"

import { type PswRecord } from "../types/PswRecord"





const useRecordsStore = defineStore("RecordsStore", () => {

  const _totalSize = ref<number | null>(null)
  const records = ref<PswRecord[] | null>(null)

  const reqError = ref<string | null>(null)
  const reqState = ref<'pending'|'ok'|'err'>('pending')



  return { _totalSize, records, reqError, reqState }
})




export { useRecordsStore }