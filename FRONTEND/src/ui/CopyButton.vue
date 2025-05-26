

<script setup lang="ts">

  import { defineProps, ref } from 'vue'


  const props = defineProps<{
    text: string
  }>()

  const btnText = ref('[copy?]')

  function handleClick() {

    // copy
    navigator.clipboard.writeText(props.text)

    // change button text
    btnText.value = '[copied]'
    setTimeout(() => {
      btnText.value = '[copy?]'
    }, 1000)
  }


</script>




<template>
  
  <button @click="handleClick()">
    {{ btnText }}
  </button>

</template>




<style scoped>
</style>
