
<script setup lang="ts">

  import FancyButton from '../../ui/FancyButton.vue'

  import request, { type ResponseError } from 'superagent'
  import { defineProps, ref } from 'vue'


  const props = defineProps<{
    untriggerShow: Function
  }>()


  const nameInput = ref("")
  const emailInput = ref("")
  const passwordInput = ref("")
  const fetchResult = ref("")






  function performFetchRegister() {
    fetchResult.value = ""


    if (nameInput.value == '' || emailInput.value == '' || passwordInput.value == '') {
      fetchResult.value = 'Fill all the inputs'
      return
    }


    request
      .post( window.API_URL + '/__api__/user-serv/create-new-user' )
      .send( {"user_name": nameInput.value, "user_email": emailInput.value, "password": passwordInput.value} )
      .withCredentials()
      .then(_ => {
        fetchResult.value = "[200] ok"
        alert("As email confirmation is not implemented, go and ask to verify account BY HANDS, lol ;)")
      })
      .catch((e: ResponseError) => {
        if (e.response) {
          fetchResult.value = `[${e.status}] ${e.response.body.err}`
        } else {
          fetchResult.value = `[not_sent] ${e.message}`
        }
      })
  }



</script>




<template>
  
  <div class="shading">
    <div class="modal">


      <div class="modal__close" @click="props.untriggerShow()">
        [X]
      </div>
      <div class="modal__title">Register</div>


      <!-- inputs -->
      <div class="input-wrapper">
        <div class="input__text">
          User name
        </div>
        <input type="text" class="input__input" v-model="nameInput">
      </div>

      <div class="input-wrapper">
        <div class="input__text">
          Email
        </div>
        <input type="text" class="input__input" v-model="emailInput">
      </div>

      <div class="input-wrapper">
        <div class="input__text">
          Password
        </div>
        <input type="password" class="input__input" v-model="passwordInput">
      </div>



      <!-- btn -->
      <div class="login-btn-wrapper">
        <FancyButton :fn="() => {performFetchRegister()}">
          Create new user
        </FancyButton>
      </div>


      <!-- result -->
      <div class="result">{{ fetchResult }}</div>


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
  }


  .input-wrapper {
    margin: 30px 0;
  }
  .input__text {
    font-size: 18px;
  }
  .input__input {
    box-sizing: border-box;
    width: 100%;
    padding: 8px 10px;
    color: inherit;
    font-size: 16px;

    margin-top: 4px;
  }


  .login-btn-wrapper {
    width: 100%;
    display: flex;
    flex-direction: column;
  }


  .result {
    position: absolute;
    bottom: -35px;
    color: var(--text-color-2);
  }

</style>
