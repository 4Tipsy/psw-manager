
import { defineStore } from "pinia"
import { ref } from "vue"



const useIsDecodeAll = defineStore("IsDecodeAll", () => {
  const isDecodeAll = ref<boolean>(false)
  return { isDecodeAll }
})




export { useIsDecodeAll }