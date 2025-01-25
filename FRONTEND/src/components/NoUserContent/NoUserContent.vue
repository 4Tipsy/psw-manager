
<script setup lang="ts">

  ///@ts-ignore
  import UserSvg from '../../assets/user.svg'

  import { ref } from 'vue'

  import { useUserStore } from '../../stores/UserStore'
  import FancyButton from '../../ui/FancyButton.vue'
  import Login from './Login.vue'
  import Register from './Register.vue'

  const userStore = useUserStore()




  const showLoginModal = ref(false)
  const showRegisterModal = ref(false)



  const adecoderAboutLink = window.LINKS__ADECODER_ABOUT

</script>




<template>
  

  <div class="content-wrapper">
    <div class="content">


      <!-- error message -->
      <div class="error-title">
        Error occurred while fetching user!
      </div>
      <div class="error-text">
        [<span class="_alt-color">{{ userStore._resStatus }}</span>]
        <span class="_alt-color">{{ userStore.reqError }}</span>
      </div>


      <!-- buttons -->
      <div class="btns-section">

        <div class="btns-section__info">
          You might consider:
        </div>

        <div class="btns-section__btns-wrapper">

          <FancyButton :fn="() => {showLoginModal = true}">
            <UserSvg style="color: red "/>
            Log Into
          </FancyButton>

          <span>or</span>

          <FancyButton :fn="() => {showRegisterModal = true}">
            <UserSvg style="color: red "/>
            Register
          </FancyButton>

        </div>

      </div>



      <!-- about adecoder -->
      <div class="adecoder-about">Also, consider reading about ADecoder concept, I mean it!</div>
      <div class="adecoder-about__link">
        Link!
        <a :href="adecoderAboutLink" class="_alt-color">{{ adecoderAboutLink }}</a>
      </div>




      <!-- modals -->
      <template v-if="showLoginModal">
        <Login :untriggerShow="() => {showLoginModal = false}"/>
      </template>
      <template v-if="showRegisterModal">
        <Register :untriggerShow="() => {showRegisterModal = false}"/>
      </template>







    </div>
  </div>


</template>




<style scoped lang="scss">

  ._alt-color {
    color: var(--text-color-2);
  }


  .content-wrapper {
    display: flex;
    justify-content: center;
  }

  .content {
    width: var(--content-width);

    color: var(--text-color-1);

    display: flex;
    flex-direction: column;

    // !
    font-size: 20px;
    align-items: center;
    justify-content: center;

    border: 2px dashed var(--text-color-1); // !!!
    border-radius: 20px; // !!!
  }

  .btns-section {
    margin-top: 40px;
  }
  .btns-section__info {
    margin-bottom: 8px;
    text-align: center;
  }
  .btns-section__btns-wrapper {
    display: flex;
    gap: 15px;
    align-items: center;

    svg {
      fill: var(--white-color) !important;
    }
  }



  .adecoder-about {
    margin-top: 40px;
  }


</style>
