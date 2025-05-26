

<script setup lang="ts">

  import FancyButton from "../../ui/FancyButton.vue"
  import SpecialText from "../../ui/SpecialText.vue"
  import RecordsTable from "./RecordsTable/RecordsTable.vue"

  import UserModal from "../Modals/UserModal.vue"
  import ViewRecordModal from "../Modals/ViewRecordModal.vue"
  import PatchRecordModal from "../Modals/PatchRecordModal.vue"
  import CreateNewRecordModal from "../Modals/CreateNewRecordModal.vue"
  import ImportMultipleRecordsModal from "../Modals/ImportMultipleRecordsModal.vue"

  import { useRecordsStore } from "../../stores/RecordsStore"
  import { useIsDecodeAll } from "../../stores/IsDecodeAll"
  import { useModalsStore } from "../../stores/ModalsStore"

  ///@ts-ignore
  import SearchSvg from "../../assets/search.svg?component"
  ///@ts-ignore
  import UnlockSvg from "../../assets/unlock.svg?component"
  ///@ts-ignore
  import PlusSvg from "../../assets/plus.svg?component"





  import clsx from "clsx"
  import { ref, computed, type ComputedRef } from "vue"






  const searchActive = ref(false)

  const _mainColor1: ComputedRef<string> = computed(() => {
    let c = window.getComputedStyle(document.body).getPropertyValue("--text-color-1")
    if (c === "") { c = "black" }
    return c
  })

  const searchValue = ref("")








  const recordsStore = useRecordsStore()
  const recordsTotalInfo: ComputedRef<string> = computed(() => {
    if (recordsStore.reqState == 'ok') {

      const sizeBytes = recordsStore._totalSize!! / 1000
      return `${recordsStore.records!!.length} records | ${sizeBytes.toFixed(2)} KB`

    } else {
      return "..."
    }
  })




  const isDecodeAllStore = useIsDecodeAll()
  const modalsStore = useModalsStore()

</script>




<template>
  
  <div class="content-wrapper">
    <div class="content">
      <div>


        <div class="content__text-area">

          <div class="content__main-text">
            <div>Your passwords</div>
            <SpecialText :tcolor="_mainColor1" style="font-size: 18px; margin-left: 15px;">
              {{ recordsTotalInfo }}
            </SpecialText>
          </div>
  
          <div class="content__under-main-text">
            All data on server is encrypted. Decoding happens only on client side, worry not
          </div>

        </div>



        <div class="content__btns-area">

          <div class="content__btns-area__subwrapper">

            <!-- search -->
            <div :class="clsx('search-wrapper', searchActive && 'search-wrapper_active')">
              <SearchSvg class="search__ico"/>
              <input type="search" class="search__input" placeholder="Search... (by tags)"
              @focus="searchActive=true" @blur="searchActive=false" v-model="searchValue"/>
            </div>

            <template v-if="isDecodeAllStore.isDecodeAll==false">
              <FancyButton :fn="() => {isDecodeAllStore.isDecodeAll = true}" style="text-wrap: nowrap">
                <UnlockSvg/>
                <div>Decode all</div>
              </FancyButton>
            </template>
            <template v-if="isDecodeAllStore.isDecodeAll==true">
              <FancyButton :fn="() => {isDecodeAllStore.isDecodeAll = false}" style="text-wrap: nowrap" class="_encode-btn">
                <UnlockSvg/>
                <div>Encode all</div>
              </FancyButton>
            </template>


          </div>


          <FancyButton :fn="() => { modalsStore.showCreateNewRecordModal = true }" class="content__btns-area__new-record-btn">
            <PlusSvg/>
            <div>New record</div>
          </FancyButton>

        </div>




      </div>





      <!-- records table (flex-grow) -->
      <RecordsTable :recordSearchValue="searchValue"/>





      <!-- modals -->
      <template v-if="modalsStore.showUserModal">
        <UserModal/>
      </template>
      <template v-if="modalsStore.showViewRecordModal">
        <ViewRecordModal/>
      </template>
      <template v-if="modalsStore.showPatchRecordModal">
        <PatchRecordModal/>
      </template>
      <template v-if="modalsStore.showCreateNewRecordModal">
        <CreateNewRecordModal/>
      </template>
      <template v-if="modalsStore.showImportMultipleRecordsModal">
        <ImportMultipleRecordsModal/>
      </template>



    </div>
  </div>

</template>




<style scoped lang="scss">

  ._encode-btn {
    background: linear-gradient(0deg, var(--highlight-color-2) 35%, rgba(190, 190, 190, 0.25) 100%);
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
  }

  .content__main-text {
    font-size: 28px;
    display: flex;
    align-items: center;
  }

  .content__under-main-text {
    font-size: 18px;
    margin-top: 10px;
  }

  /*.content__text-area {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
  }*/



  /*
  * btns-area
  */

  .content__btns-area {
    display: flex;
    justify-content: space-between;

    margin: 22px 0;

    svg {
      fill: var(--white-color);
    }
  }
  .content__btns-area__gap {
    flex-grow: 1;
  }

  .content__btns-area__new-record-btn {
    background: linear-gradient(0deg, var(--highlight-color-2) 35%, rgba(190, 190, 190, 0.25) 100%);
    padding: 10px 25px;
  }

  .content__btns-area__subwrapper {
    display: flex;
    gap: 30px;
  }

  @media (max-width: 650px) {
    .content__btns-area {
      flex-direction: column-reverse;
      gap: 20px;
      &__subwrapper {
        justify-content: space-between;
        gap: 20px;
      }
    }

  }

  /*
  * search
  */

  .search-wrapper {
    display: flex;
    align-items: center;

    background-color: var(--white-color);
    border-radius: 999px;
    box-shadow: var(--standard-box-shadow);

    font-size: 16px;
    border: 2px solid var(--white-color);
  }
  .search-wrapper_active {
    border: 2px solid var(--highlight-color-1);
  }

  .search__ico {
    margin: 0 8px;
  }

  .search__input {
    font-size: inherit;
    height: 100%;
    flex-grow: 1;
    border: none;
    background-color: transparent;
    font-family: "Onest Regular";
    color: var(--text-color-1);
    outline: none;
  }




</style>
