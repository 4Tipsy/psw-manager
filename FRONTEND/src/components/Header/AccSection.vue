

<script setup lang="ts">

  ///@ts-ignore
  import ArrowDownSvg from '../../assets/arrow-down.svg?component'

  import { useUserStore } from '../../stores/UserStore'
  import { useModalsStore } from '../../stores/ModalsStore'


  const userImgUrl = window.API_URL + '/__api__/user-serv/get-user-image'
  const userStore = useUserStore()
  const modalsStore = useModalsStore()

</script>




<template>

  <div class="acc-section">


    <template v-if="userStore.reqState == 'pending'">
      pending...
    </template>
    <template v-if="userStore.reqState == 'err'">
      get_user_error
    </template>
    <template v-if="userStore.reqState == 'ok'">


      <div class="avatar-wrapper acc-section__acc-modal-trigger" @click="modalsStore.showUserModal = true">
        <img class="avatar" :src="userImgUrl" alt="avatar">
      </div>
  
      <div class="user-name acc-section__acc-modal-trigger" @click="modalsStore.showUserModal = true">
        {{ userStore.user!!.user_name }}
      </div>
  
      <div class="acc-section__ico-wrapper acc-section__acc-modal-trigger" @click="modalsStore.showUserModal = true">
        <ArrowDownSvg class="acc-section__ico"/>
      </div>


    </template>



  </div>

</template>




<style scoped lang="scss">

  .acc-section {
    height: 55%;
    display: flex;
    align-items: center;
  }

  .avatar-wrapper {
    aspect-ratio: 1 / 1;
    height: 100%;

    border-radius: 10px;
    overflow: hidden;

    box-sizing: border-box;
    border: 3px solid var(--text-color-2);
  }

  .avatar {
    width: 100%;
    height: 100%;
  }

  .user-name {
    text-decoration: underline;
    margin-left: 10px;
  }


  .acc-section__ico-wrapper {
    height: 100%;
    aspect-ratio: 1 / 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .acc-section__ico {
    width: 50%;
    height: 50%;

    fill: var(--white-color);
  }


  .acc-section__acc-modal-trigger {
    cursor: pointer;
  }

</style>
