<template>
  <label>
    <input type="file" @change="fileChange" />
  </label>
  <textarea v-model="url"></textarea>
  <img :src="url" class="tw-w-20 tw-h-20 tw-block" alt="" />

  <input type="text" v-model="video" />
  <button @click="downloadVideo">下载</button>
</template>
<script lang="ts" setup>
import { ref } from "vue";
import { imgExt } from "@/constants/images";
import { invoke } from "@tauri-apps/api/tauri";
import { writeBinaryFile, BaseDirectory } from "@tauri-apps/api/fs";

const url = ref();
const video = ref();

const downloadVideo = async () => {
  await invoke("down", { url: video.value });
};

const downLoad = (blob: Blob) => {
  const link = document.createElement("a");
  link.style.display = "none";
  link.href = URL.createObjectURL(blob);
  link.setAttribute("download", "fileName");
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
};

const fileChange = async (e: any) => {
  const file = e.target.files[0];
  const reader = new FileReader();
  const ext = file.type.split("/")[1];
  const type = imgExt[ext];

  reader.readAsArrayBuffer(file);

  reader.onload = async () => {
    const u8 = new Uint8Array(reader.result);
    const info = await invoke("save_img", {
      buffer: { source: Array.from(u8) },
      ext: type,
    });
    let blob = new Blob([new Uint8Array(info).buffer]);
    let r1 = new FileReader();
    r1.readAsDataURL(blob);
    r1.onload = () => {
      url.value = r1.result;
    };

    await writeBinaryFile(`xxx.${ext}`, new Uint8Array(info), { dir: BaseDirectory.Download });
  };
};
</script>

<style lang="scss" scoped></style>
