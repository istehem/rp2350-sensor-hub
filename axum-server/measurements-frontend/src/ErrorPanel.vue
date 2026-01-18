<script setup lang="ts">
import type { ApiError } from './assets.ts'

defineProps<{
  error: ApiError
}>()
</script>

<template>
  <article class="center-align error-container">
    <h6>{{ error.message }}</h6>
  </article>
</template>
<style scoped></style>
