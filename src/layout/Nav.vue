<template>
  <div class="tw-flex tw-flex-col tw-flex-initial tw-w-1/4 tw-bg-white tw-h-full tw-p-4 tw-box-border">
    <router-link :to="item.path" class="tw-text-black/[0.5]" v-for="item in links">{{ item.name }}</router-link>
  </div>
</template>
<script lang="ts" setup>
import { ref } from "vue";
const links = ref([
  {
    name: "Home",
    path: "/",
  },
  {
    name: "Pic",
    path: "/pic",
  },
]);
</script>

<style lang="scss" scoped></style>
