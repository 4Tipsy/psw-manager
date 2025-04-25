
import { defineStore } from "pinia"
import { ref } from "vue"

import { type TypedPswRecord, type RawPswRecord } from "../types/PswRecord"





const useRecordsStore = defineStore("RecordsStore", () => {

  const _totalSize = ref<number | null>(null)
  const records = ref<Array<TypedPswRecord|RawPswRecord> | null>(null)

  const reqError = ref<string | null>(null)
  const reqState = ref<'pending'|'ok'|'err'>('pending')



  return { _totalSize, records, reqError, reqState }
})




export { useRecordsStore }