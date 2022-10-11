<template>
  <div class="tw-flex tw-w-full tw-h-full">
    <nav-left />
    <content />
  </div>
</template>
<script lang="ts" setup>
import NavLeft from "./Nav.vue";
import Content from "./Content.vue";

import { type } from "@tauri-apps/api/os";
import { onMounted } from "vue";
import { osType } from "@/store";

onMounted(async () => {
  const system = await type();
  osType.value = system.toLowerCase();
});
</script>

<style lang="scss" scoped></style>
