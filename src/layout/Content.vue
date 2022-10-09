<template>
  <div class="tw-flex-1">
    <router-view />
  </div>
</template>
<script lang="ts" setup>
import { ref } from "vue";
const count = ref(0);
</script>

<style lang="scss" scoped></style>
