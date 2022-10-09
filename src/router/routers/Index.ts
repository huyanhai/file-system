export const routes = [
  {
    path: "/",
    name: "Home",
    component: () => import("@/views/home/Index.vue"),
    meta: {
      title: "首页",
      keepAlive: true,
    },
  },
  {
    path: "/pic",
    name: "Pic",
    component: () => import("@/views/pic/Index.vue"),
    meta: {
      title: "图片",
      keepAlive: true,
    },
  },
];
