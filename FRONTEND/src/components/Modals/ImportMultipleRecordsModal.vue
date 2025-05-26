
<script setup lang="ts">

  import { useModalsStore } from '../../stores/ModalsStore'
  import FancyButton from '../../ui/FancyButton.vue'
  import SpecialText from '../../ui/SpecialText.vue'
  import { ADecoder } from '../../utils/ADecoder'

  import { ref, onMounted, onUnmounted } from 'vue'
  import request, { type ResponseError, type Response } from 'superagent'



  const recordsInput = ref('[]') // input
  const modalsStore = useModalsStore()




  // close on Esc
  function handleEsc(e: KeyboardEvent) {
    if (e.key == 'Escape') {
      modalsStore.showImportMultipleRecordsModal = false
    }
  }
  onMounted(() => {
    window.addEventListener('keydown', handleEsc)
  })
  onUnmounted(() => {
    window.removeEventListener('keydown', handleEsc)
  })






  async function multiPerform() {

    // check if json
    try { JSON.parse(recordsInput.value) } catch { alert("Invalid JSON provided"); return }

    // get
    const records = JSON.parse(recordsInput.value)

    // check if array
    if (Array.isArray(records) == false) {
      alert("Invalid JSON provided. Not an array")
      return
    }

    // adecoder
    const adecoderPhrase = prompt("ADecoder phrase (be careful!)")
    if (adecoderPhrase != prompt("Repeat it")) {
      alert('ADecoder phrases are not matching')
      return
    }
    if (adecoderPhrase == null) { return }
    let adecoder = new ADecoder(adecoderPhrase)


    // perform
    for (let [idx, record] of records.entries()) {

      // // raw
      if (record._record_type === 'RAW') {

        var reqBody = JSON.stringify({
          app_ico: record.app_ico,
          app_name: record.app_name,
          raw_content: adecoder.encode(record.raw_content),
          tags: record.tags,
        })

      // // typed
      } else if (record._record_type === 'TYPED') {

        var reqBody = JSON.stringify({
          app_ico: record.app_ico,
          app_name: record.app_name,
          account_name: adecoder.encode(record.account_name),
          encoded_login: adecoder.encode(record.encoded_login),
          encoded_password: adecoder.encode(record.encoded_password),
          tags: record.tags,
        })


      // // invalid
      } else {
        continue
      }


      // // REQUEST
      request
        .post( window.API_URL + '/__api__/record-serv/create-new-record' )
        .send(reqBody)
        .withCredentials()
        .end((err: ResponseError, res: Response) => {
          console.log("Performed idx=" + idx)
          if (err) {
            console.error(err)
          } else {
            console.log(res)
          }
        })
      


    }


    // end
    alert("All records are being imported. Check console for details. Then, RELOAD the page to see records.")
  }

</script>





<template>

  <div class="shading">
    <div class="modal">



      <!-- title -->
      <div class="modal__close" @click="modalsStore.showImportMultipleRecordsModal = false">
        [X]
      </div>
      <div class="modal__title">Import multiple records</div>
        

      <!-- info -->
      <div class="info">
        Paste your records below. The records should be in JSON format.<br/>
        Syntax check is not performed on client.<br/><br/>

        Input type is: 
        <span style="display: flex; margin-top: 5px">
          <SpecialText tcolor="black">
            {{ "Array<TypedPswRecord | RawPswRecord>" }}
          </SpecialText>
        </span>

      </div>


      <!-- input -->
      <textarea class="records-input" v-model="recordsInput"/>


      <!-- btn -->
      <div class="modal__btns">
        <FancyButton :fn="() => {multiPerform()}">Perform imports</FancyButton>
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


  .info {
    font-family: "Onest Medium";
    font-size: 16px;

    margin: 15px 0;
  }


  .records-input {
    width: 100%;
    box-sizing: border-box;
    aspect-ratio: 3 / 2.7;

    margin: 15px 0;
  }


  .modal__btns {
    display: flex;
    flex-direction: column;
    gap: 15px;

    margin-top: 20px;
  }


</style>
