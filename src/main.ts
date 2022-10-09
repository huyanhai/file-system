import { createApp } from "vue";

import { setupRouter, router } from "@/router";
import { setupRouterGuard } from "@/router/guard";

import App from "./App.vue";
import "tailwindcss/tailwind.css";
import "@/scss/base.scss";

const bootstrap = () => {
  const app = createApp(App);
  setupRouter(app);
  setupRouterGuard(router);
  app.mount("#app");
};

bootstrap();
