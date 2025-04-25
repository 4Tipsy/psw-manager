

<script setup lang="ts">

  import AccSection from "./AccSection.vue"
  import request from "superagent"

  function _ping() {
    request
      .get( window.API_URL + '/__api__/ping' )
      .end((err, res) => {
        console.log(res);
      })
  }

</script>




<template>
  
  <header class="header-wrapper">
    <div class="header">

      <div class="logo">
        <img class="logo__img" src="/perfectly-downscaled-shit.jpeg" alt="" @click="_ping()">
        <div class="logo__text">Silly Password Manager</div>
      </div>
      


      <div class="_gap" style="width: 15%"/>


      <AccSection/>


    </div>
  </header>

</template>





<style scoped lang="scss">

  .header-wrapper {
    width: 100vw;
    height: 100px;

    display: flex;
    align-items: center;
    justify-content: center;

    background: linear-gradient(var(--main-color-1) 47%, rgba(0, 0, 0, 0.35) 100%);
  
    color: var(--white-color);

    font-size: 22px;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;

    width: var(--content-width); // !!!
    height: 90%;
  }

  .logo {
    height: 65%;
    display: flex;
    align-items: center;
  }

  .logo__img {
    aspect-ratio: 1 / 1;
    height: 100%;
    border-radius: 25%;
  }

  .logo__text {
    font-family: "Onest Medium";
    font-size: 28px;

    margin-left: 15px;
  }

</style>
