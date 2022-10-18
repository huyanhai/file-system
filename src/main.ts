import { createApp } from "vue";
import "virtual:svg-icons-register";
import { Progress } from "ant-design-vue";

import { setupRouter, router } from "@/router";
import { setupRouterGuard } from "@/router/guard";

import App from "./App.vue";
import "tailwindcss/tailwind.css";
import "@/scss/base.scss";
import "ant-design-vue/dist/antd.css";

const bootstrap = () => {
  const app = createApp(App);
  setupRouter(app);
  setupRouterGuard(router);

  app.use(Progress);
  app.mount("#app");
};

bootstrap();
