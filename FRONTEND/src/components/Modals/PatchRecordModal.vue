
<script setup lang="ts">

  import FancyButton from '../../ui/FancyButton.vue'

  import { ref, onMounted } from 'vue'
  import request, { type ResponseError } from 'superagent'

  import { useModalsStore } from '../../stores/ModalsStore'
  import { ADecoder } from '../../utils/ADecoder'


  const modalsStore = useModalsStore()



  // get record
  const _record_reqStatus = ref<'pending'|'ok'|'err'>('pending')
  onMounted(() => {
    request
      .get( window.API_URL + '/__api__/record-serv/get-records/' + modalsStore.showPatchRecordModal )
      .set('accept', 'json')
      .withCredentials()
      .then((res) => {
        let obj = res.body
        patchInput.value = JSON.stringify(obj, null, 2)
        _record_reqStatus.value = 'ok'
      })
      .catch(_ => {
        _record_reqStatus.value = 'err'
      })
  })




  // patch record
  const patchInput = ref('') // input
  const reqResult = ref('')

  function performPatch() {

    try {
      var reqBody = JSON.parse(patchInput.value)
    } catch (e) {
      ///@ts-ignore
      reqResult.value = `[not_sent] ${e.message}`
      return
    }

    request
      .patch( window.API_URL + '/__api__/record-serv/patch-record' )
      .send( reqBody )
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



  // encode helper
  const encodeHelperVal = ref('')
  function handleEncodeHelper() {
    if (encodeHelperVal.value == '') { return }
    const adecoder = new ADecoder()
    encodeHelperVal.value = adecoder.encode( encodeHelperVal.value )
  }


</script>







<template>
  
  
  <div class="shading">
    <template v-if="_record_reqStatus == 'pending'">
      <div style="font-size: 60px">Pending</div>
    </template>
    <template v-else-if="_record_reqStatus == 'err'">
      <div style="font-size: 60px">Error</div>
    </template>




    <template v-else>
      <div class="modal">


        <!-- title -->
        <div class="modal__close" @click="modalsStore.showPatchRecordModal = false">
          [X]
        </div>
        <div class="modal__title">Patch record</div>
        


        <!-- input -->
        <textarea class="patch-input" v-model="patchInput"/>
        <div class="encode-helper-input__info">
          <div>Encode helper (encode string)</div>
          <button class="encode-helper-input__btn" @click="handleEncodeHelper()">[encode]</button>
        </div>
        <input type="text" class="encode-helper-input" v-model="encodeHelperVal"/>


        <!-- btn -->
        <div class="modal__btns">
          <FancyButton :fn="() => {performPatch()}">Perform patch</FancyButton>
        </div>


        <!-- result -->
        <div class="result">{{ reqResult }}</div>



      </div>
    </template>



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

  

  .patch-input {
    width: 100%;
    box-sizing: border-box;
    aspect-ratio: 3 / 2.7;

    margin: 15px 0;
  }
  .encode-helper-input__info {
    font-size: 14px;
    display: flex;
    justify-content: space-between;
  }
  .encode-helper-input__btn {
    background: none;
    border: none;
    font-size: inherit;
    color: inherit;
    cursor: pointer;

    &:hover {
      text-decoration: underline;
    }
  }
  .encode-helper-input {
    width: 100%;
    padding: 2px;
    box-sizing: border-box;
    color: inherit;
    font-size: inherit;
    margin-top: 5px;
  }


  .modal__btns {
    display: flex;
    flex-direction: column;
    gap: 15px;

    margin-top: 20px;
  }



  .result {
    position: absolute;
    left: 0;
    bottom: -100px;
    color: var(--text-color-2);
    font-size: 18px;
  }

</style>
