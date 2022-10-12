<template>
  <label>
    <input type="file" @change="fileChange" />
  </label>
  <textarea v-model="url"></textarea>
  <img :src="url" class="tw-w-20 tw-h-20 tw-block" alt="" />
</template>
<script lang="ts" setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { writeBinaryFile, BaseDirectory } from "@tauri-apps/api/fs";

const url = ref();

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

  reader.readAsArrayBuffer(file);

  reader.onload = async () => {
    const u8 = new Uint8Array(reader.result);
    const info = await invoke("save_img", {
      buffer: { name: Array.from(u8) },
    });
    let blob = new Blob([new Uint8Array(info).buffer]);
    console.log(new Uint8Array(info).buffer.byteLength, u8.buffer.byteLength);
    let r1 = new FileReader();
    r1.readAsDataURL(blob);
    r1.onload = () => {
      url.value = r1.result;
    };

    await writeBinaryFile("xxx.jpeg", new Uint8Array(info), { dir: BaseDirectory.Download });
  };
};
</script>

<style lang="scss" scoped></style>
