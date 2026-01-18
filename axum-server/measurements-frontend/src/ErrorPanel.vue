<script setup lang="ts">
defineProps<{
  message: string
}>()
</script>
<template>
  <article class="center-align error-container">
    <h6>{{ message }}</h6>
  </article>
</template>
<style scoped></style>
