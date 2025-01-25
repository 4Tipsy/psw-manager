
import { defineStore } from "pinia"
import { ref } from "vue"



const useModalsStore = defineStore("ModalsStore", () => {


  const showUserModal = ref(false)
  const showCreateNewRecordModal = ref(false)
  const showPatchRecordModal = ref<string | false>(false)
  const showViewRecordModal = ref<string | false>(false)
                              // ^record_id

  return { showUserModal, showCreateNewRecordModal, showPatchRecordModal, showViewRecordModal }
})



export { useModalsStore }