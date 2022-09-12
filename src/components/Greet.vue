<script setup lang="ts">
import { ref, onMounted, watchEffect } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

type TDirs = {
  name: string;
  path: string;
  is_dir: boolean;
};

const dirs = ref<TDirs[]>([]);
const content = ref("");

const parentPath = ref("");

const fetchDirs = async (item: TDirs) => {
  if (item?.path.startsWith("/")) {
    item.path = item.path.slice(1);
  }
  const path = `${item?.path ? `${item?.path}/` : ""}${item.name}`;
  if (item.is_dir) {
    dirs.value = await invoke("dir_lists", {
      name: path,
    });
  } else {
    content.value = await invoke("content", {
      path,
    });
  }
};

const goBack = async () => {
  content.value = "";
  let path = parentPath.value.split("/").filter((item) => item);
  const parent = path.splice(0, path.length - 1);
  dirs.value = await invoke("dir_lists", {
    name: parent.length > 0 ? `${parent.join("/")}` : "",
  });
};

watchEffect(() => {
  if (dirs.value.length > 0) {
    parentPath.value = (dirs.value || [])[0]?.path;
  }
});

onMounted(() => {
  fetchDirs({
    name: "",
    path: "",
    is_dir: true,
  });
});
</script>

<template>
  <button @click="goBack">返回</button>
  <div>{{ parentPath }}</div>
  <template v-if="!content">
    <a v-for="item in dirs" @click="fetchDirs(item)">
      <img src="../assets/dir.svg" v-if="item.is_dir" />
      <img src="../assets/file.svg" v-else />
      {{ item.name }}
    </a>
  </template>
  <p v-else>
    {{ content }}
  </p>
</template>

<style lang="scss">
a {
  display: flex;
  align-items: center;
  cursor: pointer;
  margin: 5px 0;
  img {
    width: 20px;
    display: block;
    margin-right: 10px;
  }
}
</style>
