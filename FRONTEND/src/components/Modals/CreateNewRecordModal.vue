
<script setup lang="ts">

  import { useModalsStore } from '../../stores/ModalsStore'
  import FancyButton from '../../ui/FancyButton.vue'
  import { ADecoder } from '../../utils/ADecoder'

  import request, { type ResponseError } from 'superagent'
  import { ref } from 'vue'



  const modalsStore = useModalsStore()



  const appIcoInput = ref('')
  const appNameInput = ref('')
  const accountNameInput = ref('')
  const loginInput = ref('')
  const passwordInput = ref('')
  const tagsInput = ref('')

  const reqResult = ref('')



  function performCreate() {
    reqResult.value = ''

    // if inputs are empty
    if (appIcoInput.value == '' || appNameInput.value == '' || accountNameInput.value == '' || loginInput.value == '' || passwordInput.value == '') {
      reqResult.value = 'Fill all the inputs'
      return
    }

    // adecoder
    const adecoderPhrase = prompt("ADecoder phrase (be careful!)")
    if (adecoderPhrase != prompt("Repeat it")) {
      reqResult.value = 'ADecoder phrases are not matching'
      return
    }
    if (adecoderPhrase == null) { return }
    let adecoder = new ADecoder( adecoderPhrase )

    // build body
    let _appIco: string | null = appIcoInput.value
    if (appIcoInput.value == 'null') {
      _appIco = null
    }

    let _tags = tagsInput.value.split(" ").filter(t => {if (t != '') {return true}})

    const reqBody = {
      app_ico: _appIco,
      app_name: appNameInput.value,
      account_name: adecoder.encode(accountNameInput.value),
      encoded_login: adecoder.encode(loginInput.value),
      encoded_password: adecoder.encode(passwordInput.value),
      tags: _tags,
    }

    // request
    request
      .post( window.API_URL + '/__api__/record-serv/create-new-record' )
      .send(reqBody)
      .withCredentials()
      .then(_ => {
        window.location.reload()
      })
      .catch((e: ResponseError) => {
        if (e.response) {
        reqResult.value = `[${e.status}] ${e.response.body.err}`
        } else {
        reqResult.value = `[not_sent] ${e.message}`
        }
      })
  }

</script>





<template>
  

  <div class="shading">
    <div class="modal">

      <!-- title -->
      <div class="modal__close" @click="modalsStore.showCreateNewRecordModal = false">
        [X]
      </div>
      <div class="modal__title">Create record</div>


      <!-- inputs -->
      <div class="field-wrapper">
        <div class="field__title">App ico ('null' for empty)</div>
        <input class="field__input" type="text" v-model="appIcoInput">
      </div>

      <div class="field-wrapper">
        <div class="field__title">App name</div>
        <input class="field__input" type="text" v-model="appNameInput">
      </div>

      <div class="field-wrapper">
        <div class="field__title">Account name</div>
        <input class="field__input" type="text" v-model="accountNameInput">
      </div>

      <div class="field-wrapper">
        <div class="field__title">Login</div>
        <input class="field__input" type="text" v-model="loginInput">
      </div>

      <div class="field-wrapper">
        <div class="field__title">Password</div>
        <input class="field__input" v-model="passwordInput">
      </div>

      <div class="field-wrapper">
        <div class="field__title">Tags</div>
        <textarea class="field__tags-input" v-model="tagsInput"/>
        <div class="_tags-info">use spaces to separate tags (yep, spaces in tags are not allowed)</div>
      </div>


      <!-- btn -->
      <div class="modal__btns">
        <FancyButton :fn="() => {performCreate()}">Perform create</FancyButton>
      </div>
      

      <!-- result -->
      <div class="result">{{ reqResult }}</div>



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



  .field-wrapper {
    margin: 25px 0;
  }
  .field__title {
    color: var(--text-color-2);
  }
  .field__input {
    width: 100%;
    padding: 2px;
    box-sizing: border-box;
    color: inherit;
    font-size: inherit;
    margin-top: 5px;
  }
  .field__tags-input {
    width: 100%;
    width: 100%;
    padding: 2px;
    box-sizing: border-box;
    color: inherit;
    font-size: inherit;
    font-family: inherit;
    margin-top: 5px;
  }
  ._tags-info {
    font-size: 14px;
    margin-top: 4px;
  }


  .modal__btns {
    display: flex;
    flex-direction: column;
    gap: 15px;

    margin-top: 30px;
  }



  .result {
    position: absolute;
    bottom: -35px;
    color: var(--text-color-2);
    font-size: 20px;
  }

</style>
