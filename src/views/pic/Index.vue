<template>
  <div class="tw-h-full tw-overflow-y-scroll" id="dropDom" ref="dropDom" @drop="drop" @dragover="dragover">
    <div v-for="item in fileArr" class="tw-flex tw-items-center tw-bg-white tw-p-2 tw-m-2 tw-rounded-md tw-justify-between">
      <div class="tw-flex tw-items-center tw-w-4/12">
        <div class="tw-w-10 tw-h-10 tw-flex tw-items-center tw-justify-center">
          <img class="tw-flex tw-max-w-full tw-max-h-full" :src="item.base64" alt="" />
        </div>
        <p class="tw-text-xs tw-m-0 tw-w-1/2 tw-mx-2 tw-overflow-hidden tw-whitespace-nowrap tw-text-ellipsis">{{ item.name }}</p>
      </div>
      <div class="tw-flex tw-items-center tw-w-5/12">
        <p class="tw-m-0 tw-text-xs tw-w-10 tw-text-right">{{ item.size / 1000 > 1 ? `${Math.ceil(item.size / 1000)}kb` : `${item.size % 1000}b` }}</p>
        <div class="tw-w-1/3 tw-mx-2">
          <a-progress :percent="100" :status="item.byteLength ? 'success' : 'active'" :show-info="false" />
        </div>
        <p class="tw-m-0 tw-text-xs tw-w-5" v-if="item.byteLength">{{ item.byteLength / 1000 > 1 ? `${Math.ceil(item.byteLength / 1000)}kb` : `${item.byteLength % 1000}b` }}</p>
        <p class="tw-m-0 tw-text-xs tw-ml-4 tw-w-10 tw-text-right" v-if="item.byteLength">{{ Math.ceil((1 - item.byteLength / item.size) * 100) }}%</p>
      </div>
      <a class="tw-w-1/12 tw-text-right tw-text-xs" @click="download(item)">下载</a>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { ref } from "vue";
import { imgExt } from "@/constants/images";
import { invoke } from "@tauri-apps/api/tauri";
import { save } from "@tauri-apps/api/dialog";
import { sep } from "@tauri-apps/api/path";
import { writeBinaryFile, BaseDirectory } from "@tauri-apps/api/fs";

interface fileInfo {
  file: File;
  base64: string;
  buffer: Uint8Array;
  ext: number;
  size: number;
  name: string;
  type: number;
  byteLength: number;
}

const fileArr = ref<{
  [key: string]: fileInfo;
}>({});

const readAsArrayBuffer = (file: File, type: number) => {
  const reader = new FileReader();
  reader.readAsArrayBuffer(file);
  return new Promise((resolve) => {
    reader.onload = async () => {
      const u8 = new Uint8Array(reader.result as ArrayBuffer);
      const info = await invoke("save_img", {
        buffer: { source: Array.from(u8) },
        ext: type,
      });

      resolve(new Uint8Array(info));
    };
  });
};

const readAsDataURL = (file: File) => {
  const reader = new FileReader();
  reader.readAsDataURL(file);
  return new Promise((resolve) => {
    reader.onload = async () => {
      resolve(reader.result);
    };
  });
};

const dragover = (e: DragEvent) => {
  e.preventDefault();
};

const drop = async (e: DragEvent) => {
  e.stopPropagation();
  e.preventDefault();

  let files = e.dataTransfer?.files;

  for (const file of files) {
    const ext = file.type.split("/")[1];
    const type = imgExt[ext];
    const base64 = (await readAsDataURL(file)) as string;
    fileArr.value[file.name] = {
      file,
      ext,
      base64,
      size: file.size,
      name: file.name,
      type,
    };
  }

  for (const item in fileArr.value) {
    const info = fileArr.value[item];
    const bufferArray = (await readAsArrayBuffer(info.file, info.type)) as Uint8Array;
    const buffer = new Uint8Array(bufferArray);
    fileArr.value[item].buffer = buffer;
    fileArr.value[item].byteLength = buffer.byteLength;
  }
};

const download = async (info: fileInfo) => {
  const dir = await save();
  if (dir) {
    const dirArr = dir.split(sep);
    dirArr.pop();
    await writeBinaryFile({
      contents: info.buffer,
      path: `${dirArr.join(sep)}${sep}${info.name}`,
    });
  }
};

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
