
<script setup lang="ts">

  import SpecialText from '../../ui/SpecialText.vue'
  import FancyButton from '../../ui/FancyButton.vue'

  import request, { type ResponseError } from 'superagent'
  import { ref, computed, onMounted } from 'vue'

  import { useModalsStore } from '../../stores/ModalsStore'
  import { type PswRecord } from '../../types/PswRecord'
  import { ADecoder } from '../../utils/ADecoder'
  import { textToColor } from '../../utils/textToColor'



  const modalsStore = useModalsStore()





  const record = ref<PswRecord|null>(null)
  const record_reqStatus = ref<'pending'|'ok'|'err'>('pending')
  onMounted(() => {
    request
      .get( window.API_URL + '/__api__/record-serv/get-records/' + modalsStore.showViewRecordModal )
      .set('accept', 'json')
      .withCredentials()
      .then((res) => {
        record.value = res.body
        record_reqStatus.value = 'ok'
      })
      .catch(_ => {
        record_reqStatus.value = 'err'
      })
  })





  function copyIt(text: string) {
    navigator.clipboard.writeText(text)
  }




  const adecoder = new ADecoder()
  const nameDecoded = computed(() => {
    return adecoder.decode(record.value!!.account_name)
  })
  const loginDecoded = computed(() => {
    return adecoder.decode(record.value!!.encoded_login)
  })
  const passwordDecoded = computed(() => {
    return adecoder.decode(record.value!!.encoded_password)
  })





  function deleteRecord() {

    if (!confirm(`Delete record "${record.value!!.record_id}"?`)) {
      return
    }

    request
      .delete( window.API_URL + '/__api__/record-serv/delete-record' )
      .send({"target_id": record.value!!.record_id})
      .withCredentials()
      .then(_ => {
        window.location.reload()
      })
      .catch((e: ResponseError) => {
        if (e.response) {
          alert(e.response.body.err)
        } else {
          alert(e.message)
        }
      })
  }

</script>




<template>

  <div class="shading">
    <template v-if="record_reqStatus == 'pending'">
      <div style="font-size: 60px">Pending</div>
    </template>
    <template v-else-if="record_reqStatus == 'err'">
      <div style="font-size: 60px">Error</div>
    </template>






    <template v-else>
      <div class="shading">
        <div class="modal">

  
          <!-- title -->
          <div class="modal__close" @click="modalsStore.showViewRecordModal = false">
            [X]
          </div>
          <div class="modal__title">
            Record ({{ record!!.record_id }})
          </div>
  
  
          <!-- fields -->
          <div class="field-wrapper">
            <div class="field__title">
              <span>App name</span>
            </div>
            <div class="field__value">{{ record!!.app_name }}</div>
          </div>
  
          <div class="field-wrapper">
            <div class="field__title">
              <span>Name</span>
              <button @click="copyIt(nameDecoded)">[copy]</button>
            </div>
            <div class="field__value">{{ nameDecoded }}</div>
          </div>
  
          <div class="field-wrapper">
            <div class="field__title">
              <span>Login</span>
              <button @click="copyIt(loginDecoded)">[copy]</button>
            </div>
            <div class="field__value">{{ loginDecoded }}</div>
          </div>
  
          <div class="field-wrapper">
            <div class="field__title">
              <span>Password</span>
              <button @click="copyIt(passwordDecoded)">[copy]</button>
            </div>
            <div class="field__value">{{ passwordDecoded }}</div>
          </div>

          <div class="field-wrapper">
            <div class="field__title">
              <span>Created at</span>
            </div>
            <div class="field__value">{{ record!!.created_at }}</div>
          </div>

          <div class="field-wrapper">
            <div class="field__title">
              <span>Tags</span>
            </div>
            <div class="field__tags-container">
              <template v-for="tag in record!!.tags">
                <SpecialText :tcolor="textToColor(tag)">{{tag}}</SpecialText>
              </template>
            </div>
          </div>
  

          <!-- btns -->
          <div class="btns-section">
            <FancyButton :fn="() => {modalsStore.showViewRecordModal = false; modalsStore.showPatchRecordModal = record!!.record_id}"
            >Patch</FancyButton>
            <FancyButton :fn="() => {deleteRecord()}">Delete</FancyButton>
          </div>


  
        </div>
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

    width: max(300px, 30%);

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

    display: flex;
  }


  .field-wrapper {
    margin: 25px 0;
  }
  .field__title {
    color: var(--text-color-2);
  }
  .field__title button {
    display: inline-block;
    background: none;
    border: none;
    color: var(--text-color-1);
    font-size: inherit;

    cursor: pointer;
    &:hover { text-decoration: underline; }
  }

  .field__tags-container {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;

    padding-top: 8px; // -_-
  }


  .btns-section {
    display: flex;
    gap: 22px;
    margin-top: 20px;

    & *:last-of-type {
      background: rgb(170, 0, 0);
    }
  }

</style>
