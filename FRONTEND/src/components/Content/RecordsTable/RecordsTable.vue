

<script setup lang="ts">


  import Record from './Record.vue';

  import { defineProps, computed, type ComputedRef } from 'vue'
  import Color from 'colorjs.io'

  // modules
  import { useRecordsStore } from '../../../stores/RecordsStore';
  import { type PswRecord } from '../../../types/PswRecord';




  const props = defineProps<{
    recordSearchValue: string
  }>()

  const recordsStore = useRecordsStore()


  // APPLY SEARCH
  const sortedRecordsToRender: ComputedRef<PswRecord[]> = computed(() => {

    // if no search
    if (props.recordSearchValue.trim() == '') {
      return recordsStore.records!!
    }

    // if search
    return recordsStore.records!!.filter(record => {
      for (const tag of record.tags) {
        if (tag.toLocaleLowerCase().indexOf(props.recordSearchValue.trim().toLowerCase()) != -1) {
          return true
        }
      }
    })

  })









  const darkenWhiteColor: ComputedRef<string> = computed(() => {

    let c = window.getComputedStyle(document.body).getPropertyValue("--white-color")
    if (c === "") { c = "white" }
    let color = new Color(c)
    color.darken(0.05)
    return color.toString()
  })



</script>






<template>

  
  <div class="records-table-wrapper">
  <div class="records-table" it-scrollable>



    <!-- table titles -->
    <div class="records-table__titles" :style="`background-color: ${darkenWhiteColor}`">
      <div style="justify-self: center">Idx</div>
      <div>App name</div>
      <div>Name</div>
      <div>Login</div>
      <div>Password</div>
      <div>Tags</div>
      <div>Buttons</div>
      <div style="justify-self: center">View</div>
    </div>






    <template v-if="recordsStore.reqState == 'pending'">
      <span class="records-table__message">loading...</span>
    </template>

    <template v-else-if="recordsStore.reqState == 'ok'">
      <!-- rendering records -->
      <template v-if="recordsStore.records!!.length == 0">
        <span class="records-table__message">empty</span>
      </template>
      <template v-else v-for="record, idx in sortedRecordsToRender" :key="record.record_id">
        <Record
          :owner_id="record.owner_id"
          :record_id="record.record_id"
          :app_ico="record.app_ico"
          :app_name="record.app_name"
          :account_name="record.account_name"
          :encoded_login="record.encoded_login"
          :encoded_password="record.encoded_password"
          :tags="record.tags"
          :created_at="record.created_at"
          :idx="idx + 1"
        />
      </template>
    </template>

    <template v-else>
      <span class="records-table__message">{{ recordsStore.reqError }}</span>
    </template>




  </div>
  </div>


</template>






<style scoped lang="scss">


  .records-table-wrapper {
    flex-grow: 1; // to take all available space, see parent component

    --grid-layout: 60px  repeat(4, 1fr)  2fr    0.6fr  0.4fr;
    //             ^idx  ^credentials    ^tags  ^btn  ^btn
    --grid-gap: 10px;


    background-color: var(--white-color);
    box-shadow: var(--standard-box-shadow);
    border-radius: 10px;

    overflow: hidden;

    position: relative;
  }



  .records-table {
    position: absolute;
    overflow-y: auto;
    max-height: 100%;
    //> * {display: none !important;}
  }



  .records-table__message {
    padding: 80px;
    font-size: 20px;
  }


  .records-table__titles {
    padding: 7px 10px;
    display: grid;
    grid-template-columns: var(--grid-layout);
    gap: var(--grid-gap);

    align-items: center;
  }


</style>
