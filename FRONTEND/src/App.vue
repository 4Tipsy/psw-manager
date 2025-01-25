
<script setup lang="ts">
  
  import Header from "./components/Header/Header.vue"
  import Content from "./components/Content/Content.vue"
  import NoUserContent from "./components/NoUserContent/NoUserContent.vue"
  import Footer from "./components/Footer/Footer.vue"

  import request, { type ResponseError } from "superagent"
  import { onMounted } from "vue"

  import { useRecordsStore } from "./stores/RecordsStore"
  import { useUserStore } from "./stores/UserStore"



  // request records
  const recordsStore = useRecordsStore()
  onMounted(() => {
    request
      .get( window.API_URL + "/__api__/record-serv/get-records" )
      .set('accept', 'json')
      .withCredentials()
      .then((res) => {
        recordsStore.reqState = 'ok'
        recordsStore.records = res.body // on ok
        recordsStore._totalSize = parseInt(res.headers["content-length"])
      })
      .catch((e: ResponseError) => {
        recordsStore.reqState = 'err'
        recordsStore.reqError = e.message
      })
  })
  // request user
  const userStore = useUserStore()
  onMounted(() => {
    request
      .get( window.API_URL + "/__api__/user-serv/get-current-user" )
      .set('accept', 'json')
      .withCredentials()
      .then((res) => {
        userStore.reqState = 'ok'
        userStore.user = res.body // on ok
        userStore._resStatus = res.status
      })
      .catch((e: ResponseError) => {
        userStore.reqState = 'err'
        userStore.reqError = e.message
        userStore._resStatus = e.response?.status || null
      })
  })

</script>




<template>
  <div class="__body__">

    <Header/>

    <template v-if="userStore.reqState == 'ok'">
      <Content/>
    </template>
    <template v-if="userStore.reqState == 'err'">
      <NoUserContent/>
    </template>
    <template v-if="userStore.reqState == 'pending'">
      pending...
    </template>
    
    
    <Footer/>

  </div>
</template>





<style scoped lang="scss">

  .__body__ {
    width: 100vw;
    height: 100vh;

    background-color: var(--main-color-2);

    display: grid;
    grid-template-rows: auto 1fr auto;
    gap: 10px;
  }


</style>
