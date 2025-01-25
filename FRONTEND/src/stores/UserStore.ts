
import { defineStore } from "pinia"
import { ref } from "vue"

import { type User } from "../types/User"






const useUserStore = defineStore("UserStore", () => {

  const _resStatus = ref<number | null>(null)
  const user = ref<User | null>(null)

  const reqError = ref<string | null>(null)
  const reqState = ref<'pending'|'ok'|'err'>('pending')



  return { _resStatus, user, reqError, reqState }
})




export { useUserStore }