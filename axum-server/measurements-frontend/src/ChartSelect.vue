<script setup lang="ts">
const properties = defineProps<{
  name: string
}>()
</script>
<template>
  <fieldset>
    <nav>
      <label class="radio">
        <input type="radio" :name="properties.name" checked />
        <span>Median And Band Plot</span>
      </label>
      <label class="radio">
        <input type="radio" :name="properties.name" />
        <span>Decimation</span>
      </label>
    </nav>
  </fieldset>
</template>
