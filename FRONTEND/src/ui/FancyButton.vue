

<script setup lang="ts">

  import { defineProps } from "vue"
  



  const props = defineProps<{
    fn: Function,
  }>()




</script>




<template>
  
  <button @click="props.fn()" class="ui-fancy-btn">
    <slot></slot>
  </button>

</template>




<style lang="scss">

  .ui-fancy-btn {
    background: none;
    border: none;
    cursor: pointer;

    padding: 10px;
    font-family: "Onest Medium";
    font-size: 18px;

    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;

    border-radius: 7px;
    color: var(--white-color);
  }

  .ui-fancy-btn > svg {
    height: 20px;
    width: 20px;
    > path { fill: inherit; }
  }

  // coloring
  .ui-fancy-btn {
    background: linear-gradient(var(--highlight-color-1) 20%, rgba(0, 0, 0, 0.61) 100%);
    box-shadow: var(--standard-box-shadow);
  }

  // interaction
  .ui-fancy-btn {
    transition: 0.15s transform;

    &:hover {
      transform: scale(1.04);
    }
  }

</style>
