export const routes = [
  {
    path: "/pic",
    name: "Pic",
    component: () => import("@/views/pic/Index.vue"),
    meta: {
      title: "图片",
      keepAlive: true,
    },
  },
  {
    path: "/video",
    name: "Video",
    component: () => import("@/views/pic/Index.vue"),
    meta: {
      title: "图片",
      keepAlive: true,
    },
  },
  {
    path: "/book",
    name: "Book",
    component: () => import("@/views/pic/Index.vue"),
    meta: {
      title: "图片",
      keepAlive: true,
    },
  },
];
