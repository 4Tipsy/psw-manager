

<script setup lang="ts">

  import { defineProps, computed } from 'vue'
  import Color from 'colorjs.io'




  const props = defineProps<{
    tcolor: string,
  }>()

  const bgColor = computed(() => {

    let c = new Color(props.tcolor)
    c.alpha = 0.18
    return c.toString()
  })

</script>






<template>
  <div class="ui-special-text" :style="{color: props.tcolor, backgroundColor: bgColor}">
    <slot></slot>
  </div>
</template>






<style lang="scss">
  .ui-special-text {
    padding: 4px 10px;
    border-radius: 999px;
  }
</style>
