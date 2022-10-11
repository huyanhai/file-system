<template>
  <div class="tw-flex tw-flex-col tw-flex-initial tw-w-1/4 tw-bg-white tw-h-full tw-p-4 tw-box-border" :class="{ 'tw-pt-8': osType === 'darwin' }">
    <router-link
      :to="item.path"
      class="tw-text-black/[0.5] tw-text-sm tw-flex tw-py-3 tw-items-center tw-px-2 tw-no-underline normal"
      :class="{ active: item.path.includes(path) }"
      v-for="item in links"
    >
      <svg-icon :name="item.icon" class="tw-w-5 tw-h-5 tw-mr-2" :color="item.path.includes(path) ? '#fff' : '#00000090'" />
      {{ item.name }}
    </router-link>
  </div>
</template>
<script lang="ts" setup>
import SvgIcon from "@/components/Svg.vue";

import { computed, ref, watch } from "vue";
import { useRouter } from "vue-router";
import { osType } from "@/store";

const router = useRouter();

const links = ref([
  {
    name: "图片",
    path: "/pic",
    icon: "pic",
  },
  {
    name: "视频",
    path: "/video",
    icon: "video",
  },
  {
    name: "书籍",
    path: "/book",
    icon: "book",
  },
]);

const path = computed(() => router.currentRoute.value.path);
</script>

<style lang="scss" scoped>
.normal {
  filter: grayscale(100%);
}
.active {
  @apply tw-bg-blue-700 tw-rounded-md tw-text-white;
  filter: grayscale(0%);
}
</style>
