import type { App } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import { routes } from "@/router/routers";

export const router = createRouter({
  history: createWebHistory(import.meta.env.VITE_APP_BASE_ROUTE),
  routes: [
    {
      component: () => import("@/layout/Index.vue"),
      children: routes,
      path: "/",
    },
  ],
  strict: true, // 禁用尾部的斜线
});

export function setupRouter(app: App) {
  app.use(router);
}
