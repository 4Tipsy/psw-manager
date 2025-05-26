

<script setup lang="ts">


  import TypedRecord from './TypedRecord.vue';
  import RawRecord from './RawRecord.vue';

  import { defineProps, computed, ref, type ComputedRef } from 'vue'
  import Color from 'colorjs.io'
  import clsx from 'clsx'

  // modules
  import { useRecordsStore } from '../../../stores/RecordsStore';
  import { type TypedPswRecord, type RawPswRecord } from '../../../types/PswRecord';




  const props = defineProps<{
    recordSearchValue: string
  }>()

  const recordsStore = useRecordsStore()


  // APPLY SEARCH
  const sortedRecordsToRender: ComputedRef<Array<TypedPswRecord|RawPswRecord>> = computed(() => {

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





  // scroll indicators
  const _scrollableElRef = ref<HTMLElement | null>(null)
  const _scrollableIndicatorTrigger = ref(false)
  const scrollableIndicator: ComputedRef<string | null> = computed(() => {

    _scrollableIndicatorTrigger.value
    let el = _scrollableElRef.value
    let classes: string[] = []

    // if no element
    if (el == null) {
      return null
    }
    // can scroll down
    if (el.scrollTop + el.clientHeight < el.scrollHeight) {
      classes.push("_scrollable-down")
    }
    // can scroll up
    if (el.scrollTop > 0) {
      classes.push("_scrollable-up")
    }

    return classes.join(' ')
  })



</script>






<template>

  
  <div :class='clsx("records-table-wrapper", scrollableIndicator)'>
  <div class="records-table" it-scrollable ref="_scrollableElRef" @scroll="_scrollableIndicatorTrigger = !_scrollableIndicatorTrigger; console.log('scroll')">



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
        <template v-if="record._record_type == 'TYPED'">
          <TypedRecord
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
            :_record_type="record._record_type"
          />
        </template>
        <template v-else-if="record._record_type == 'RAW'">
          <RawRecord
            :owner_id="record.owner_id"
            :record_id="record.record_id"
            :app_ico="record.app_ico"
            :app_name="record.app_name"
            :raw_content="record.raw_content"
            :tags="record.tags"
            :created_at="record.created_at"
            :idx="idx + 1"
            :_record_type="record._record_type"
          />
        </template>
      </template>
      <!-- /rendering records -->
    </template>

    <template v-else>
      <span class="records-table__message">{{ recordsStore.reqError }}</span>
    </template>




  </div>
  </div>


</template>






<style scoped lang="scss">


  // .records-table-wrapper 
  ._scrollable-up::before, ._scrollable-down::after {
    content: '';
    position: absolute;
    opacity: 0.7;
    background-color: var(--highlight-color-1);
    border-radius: 99px;
    height: 7px;

    width: calc(100% - 80px);

    left: 50%; // center me
    transform: translate(-50%, 0); // center me
    margin: 4px 0;
  }
  ._scrollable-up::before {
    top: 0;
  }
  ._scrollable-down::after {
    bottom: 0;
  }



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
    width: 100%;
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
