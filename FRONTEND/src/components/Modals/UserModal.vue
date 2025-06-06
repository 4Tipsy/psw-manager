
<script setup lang="ts">

  ///@ts-ignore
  import PlusSvg from "../../assets/plus.svg"

  import request, { type ResponseError } from "superagent"
  import Cookies from "js-cookie"
  import Color from "colorjs.io"

  import FancyButton from "../../ui/FancyButton.vue"
  import SpecialText from "../../ui/SpecialText.vue"
  import { useModalsStore } from "../../stores/ModalsStore"
  import { useUserStore } from "../../stores/UserStore"
  import { useRecordsStore } from "../../stores/RecordsStore"

  import { ref, computed, type ComputedRef } from "vue"
  import { onMounted, onUnmounted } from "vue"

  import { downloadRecords } from "../../utils/downloadRecords"



  const userImgUrl = window.API_URL + '/__api__/user-serv/get-user-image'
  const updateUserImgRef = ref<HTMLInputElement | null>(null)

  const modalsStore = useModalsStore()
  const userStore = useUserStore()
  const recordsStore = useRecordsStore()





  // close on Esc
  function handleEsc(e: KeyboardEvent) {
    if (e.key == 'Escape') {
      modalsStore.showUserModal = false
    }
  }
  onMounted(() => {
    window.addEventListener('keydown', handleEsc)
  })
  onUnmounted(() => {
    window.removeEventListener('keydown', handleEsc)
  })







  function handleUpdateUserImg() {
    const file = updateUserImgRef.value!!.files?.item(0)

    if (file == null || file == undefined) { return }

    request
      .post( window.API_URL + '/__api__/user-serv/update-user-image' )
      ///@ts-ignore
      .field('File', file)
      .withCredentials()
      .then(_ => {
        window.location.reload()
      })
      .catch((e: ResponseError) => {
        if (e.response) {
          alert(e.response!!.body.err)
        } else {
          alert(e.message)
        }
      })
  }





  function handleLogout() {
    if (confirm("Confirm logout? (will terminate all saved data and cookies)")) {
      Cookies.remove("psw-manager.access_token")
      localStorage.removeItem("psw-manager.adecoder_phrase")
      window.location.reload()
    }
  }

  function handleDownloadRecords() {
    if (confirm("Confirm download records?")) {
      downloadRecords(
        userStore.user!!,
        recordsStore.records!!,
        recordsStore._totalSize!!
      )
    }
  }
  function handleImportMultipleRecords() {
    if (confirm("This is extremely experimental feature. Do you want to continue?")) {
      modalsStore.showUserModal = false
      modalsStore.showImportMultipleRecordsModal = true
    }
  }

  



  const adecoderInput = ref<string>( localStorage.getItem("psw-manager.adecoder_phrase") || "" )
  function saveAdecoderInput() {
    localStorage.setItem("psw-manager.adecoder_phrase", adecoderInput.value)
    adecoderStatus.value = 'none' // make 
    adecoderStatus.value = 'saved'
  }
  function deleteAdecoderInput() {
    localStorage.removeItem("psw-manager.adecoder_phrase")
    adecoderInput.value = ""
    adecoderStatus.value = 'deleted'
  }

  const adecoderStatus = ref<'saved'|'deleted'|'none'>('none')
  const adecoderStatusText = computed(() => {
    let i = localStorage.getItem("psw-manager.adecoder_phrase")
    if (adecoderStatus.value == 'saved') {
      return `saved "${i}"`
    }
    else if (adecoderStatus.value == 'deleted') {
      return "deleted"
    }
    else {
      return ""
    }
  })






  const userNameColor: ComputedRef<string> = computed(() => {

    let c = window.getComputedStyle(document.body).getPropertyValue("--text-color-1")
    if (c === "") { c = "white" }
    let color = new Color(c)
    color.darken(0.05)
    return color.toString()
  })

  const adecoderAboutSrc = window.LINKS__ADECODER_ABOUT

</script>




<template>

  
  <div class="shading">
    <div class="modal">

      <div class="modal__close" @click="modalsStore.showUserModal = false">
        [X]
      </div>
      <div class="modal__title">Account settings</div>




      <div class="avatar-section">
        <div class="avatar-wrapper" @click="updateUserImgRef?.click()">
          <img class="avatar" :src="userImgUrl" alt="avatar">
          <input class="avatar__input" type="file" ref="updateUserImgRef" @change="handleUpdateUserImg()"/>
        </div>
        <div class="user-name">
          <SpecialText :tcolor="userNameColor">
            {{ userStore.user!!.user_name }}
          </SpecialText>
        </div>
        <div class="user-email">email: {{ userStore.user!!.user_email }}</div>
        <div class="user-id">id: {{ userStore.user!!.user_id }}</div>
      </div>



      <div class="adecoder-section">
        <div class="adecoder__info">
          ADecoder <a :href="adecoderAboutSrc" style="color: inherit; text-decoration: none">(?)</a> <span class="adecoder__info__status">{{ adecoderStatusText }}</span>
        </div>
        <input class="adecoder__input" type="text" v-model="adecoderInput"/>
        <div class="adecoder__btns">
          <div @click="saveAdecoderInput">[saveit]</div>
          <div @click="deleteAdecoderInput">[delete]</div>
        </div>
      </div>


      <div class="modal__btns">
        <FancyButton :fn="handleDownloadRecords">Download records</FancyButton>
        <FancyButton :fn="handleImportMultipleRecords">Import multiple records</FancyButton>
        <FancyButton :fn="handleLogout">Log out</FancyButton>
      </div>


    </div>
  </div>


</template>




<style scoped lang="scss">

  .shading {
    position: absolute;
    width: 100vw;
    height: 100vh;
    top: 0;
    left: 0;
    background-color: var(--shading-color);

    display: flex;
    align-items: center;
    justify-content: center;
  }

  .modal {
    position: relative;

    width: max(350px, 30%);

    background-color: var(--white-color);
    border-radius: 10px;

    padding: 20px;
  }

  .modal__close {
    position: absolute;
    top: 20px;
    right: 20px;
    cursor: pointer;

    font-family: "Onest Medium";
    font-size: 20px;
  }

  .modal__title {
    font-family: "Onest Medium";
    font-size: 22px;
  }



  .avatar-section {
    margin: 30px 0;

    display: grid;
    gap: 0 15px;
    
    grid-template-columns: auto 1fr;
    grid-template-areas: "avatar user-name"
                         "avatar user-email"
                         "avatar user-id";
  }
  .avatar-wrapper {
    grid-area: avatar;
    height: 100%;
    max-height: 80px;
    aspect-ratio: 1 / 1;
    border-radius: 20px;
    overflow: hidden;

    box-sizing: border-box;
    border: 3px solid var(--text-color-2);

    position: relative;
  }
  .avatar {
    height: 100%;
    width: 100%;
  }
  .user-name {
    font-size: 20px;
    display: flex;
  }
  .user-email {
    font-size: 20px;
  }
  .user-id {
    font-size: 20px;
    margin-bottom: 4px;
    color: var(--text-color-2);
  }
  .avatar__input {
    display: none;
  }
  .avatar-wrapper:hover img {
    cursor: pointer;
    opacity: 0.4;
  }
  .avatar-wrapper:hover::after {
    content: "Change";
    position: absolute;
    cursor: pointer;

    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);

    color: black;
    font-size: 18px;
  }


  .adecoder-section {
    display: grid;
    gap: 10px 30px;
    grid-template-columns: 1fr auto;
    grid-template-areas: "info info"
                         "input btn";
  }
  .adecoder__info {
    grid-area: info;
  }
  .adecoder__input {
    grid-area: input;
    width: 100%;
    padding: 5px;
  }
  .adecoder__info__status {
    color: var(--text-color-2);

  }
  .adecoder__btns {
    cursor: pointer;
    text-wrap: wrap;
  }
  .adecoder__btns > div:hover {
    text-decoration: underline;
  }


  .modal__btns {
    display: flex;
    flex-direction: column;
    gap: 15px;

    margin-top: 40px;
  }
  .modal__btns > *:hover {
    transform: scale(1.025);
  }


</style>
