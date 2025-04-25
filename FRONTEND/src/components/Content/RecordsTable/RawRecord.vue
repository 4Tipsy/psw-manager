

<script setup lang="ts">

  ///@ts-ignore
  import UnlockSvg from "../../../assets/unlock.svg"
  ///@ts-ignore
  import MoreSvg from "../../../assets/more.svg"
  
  import { type RawPswRecord } from '../../../types/PswRecord'
  import FancyButton from '../../../ui/FancyButton.vue'
  import SpecialText from '../../../ui/SpecialText.vue'


  import Color from 'colorjs.io'
  import { ref, defineProps, computed, type ComputedRef } from 'vue'

  // modules
  import { ADecoder } from '../../../utils/ADecoder'
  import { useIsDecodeAll } from "../../../stores/IsDecodeAll"
  import { textToColor } from '../../../utils/textToColor'
  import { useModalsStore } from "../../../stores/ModalsStore"






  const props = defineProps<RawPswRecord & {idx: number}>()



  const onHoverColor: ComputedRef<string> = computed(() => {
    let c = window.getComputedStyle(document.body).getPropertyValue("--text-color-2")
    if (c === "") { c = "red" }
    let color = new Color(c)
    color.alpha = 0.15
    return color.toString()
  })
  const isHovered = ref(false)






  const showDecoded = ref<boolean>(false)
  const isDecodeAllStore = useIsDecodeAll()

  const renderedFields = computed(() => {
    let _decoded = false
    if (isDecodeAllStore.isDecodeAll) { _decoded = true }
    if (showDecoded.value) { _decoded = true }

    if (_decoded) {
      const decoder = new ADecoder()
      const decoded_data = decoder.decode(props.raw_content)
      if (decoded_data.length > 50) {
        return ["... ... ..."]
      }
      return [
        decoded_data
      ]

    } else {
      return ["<@encoded>"]
    }
  })





  const modalsStore = useModalsStore()

</script>





<template>
  

  <div class="record"
    :data-record-id="props.record_id"
    :style="isHovered ? {backgroundColor: onHoverColor} : {}"
    @mouseenter="isHovered=true" @mouseleave="isHovered=false"
  >
    <div class="record__idx">{{ props.idx }}</div>
    <div class="record__app-name" it-scrollable>{{ props.app_name }}</div>
    <div class="record__raw-content" it-scrollable>
      <span style="color: var(--text-color-2)">raw record content: </span>
      <span style="margin-left: 7px;">{{ renderedFields[0] }}</span>
    </div>



    <div it-scrollable class="record__tags-container">
      <template v-if="props.tags.length != 0" v-for="tag in tags">
        <SpecialText :tcolor="textToColor(tag)">{{ tag }}</SpecialText>
      </template>
      <template v-else>
        <!-- if no tags -->
        <SpecialText :tcolor="'black'">null</SpecialText>
      </template>
    </div>
    

    <template v-if="isDecodeAllStore.isDecodeAll">
      <FancyButton class="record__encode-btn _inactive" :fn="() => {}">
        <UnlockSvg/>
        <div>Encode</div>
      </FancyButton>
    </template>
    <template v-else-if="showDecoded==false">
      <FancyButton class="record__decode-btn" :fn="() => {showDecoded = true}">
        <UnlockSvg/>
        <div>Decode</div>
      </FancyButton>
    </template>
    <template v-else>
      <FancyButton class="record__encode-btn" :fn="() => {showDecoded = false}">
        <UnlockSvg/>
        <div>Encode</div>
      </FancyButton>
    </template>

    <div class="record__more-actions-btn" @click="modalsStore.showViewRecordModal = props.record_id">
      <MoreSvg/>
    </div>

  </div>


</template>





<style scoped lang="scss">

  .record__app-name, .record__raw-content {
    overflow: auto;
    text-wrap: nowrap;
  }
  .record__raw-content {
    grid-column: span 3;
    display: flex;
    justify-content: center;
  }


  .record {
    display: grid;
    grid-template-columns: var(--grid-layout);
    gap: var(--grid-gap);

    align-items: center;

    padding: 7px 10px;
  }

  .record__encode-btn._inactive {
    cursor: not-allowed;
  }
  .record__decode-btn svg, .record__encode-btn svg {
    fill: var(--white-color);
  }
  .record__encode-btn {
    background: linear-gradient(0deg, var(--highlight-color-2) 35%, rgba(190, 190, 190, 0.25) 100%);
  }

  .record__more-actions-btn  {
    justify-self: center;
    cursor: pointer;
    svg {
      height: 30px;
      width: 30px;
      fill: var(--white-color);
      margin-top: 7px; // -_-
    }
  }

  .record__idx {
    justify-self: center;
  }


  .record__tags-container {
    display: flex;
    gap: 0 8px;

    overflow-x: auto;

    white-space: nowrap
  }


</style>
