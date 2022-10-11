const IS_PROD = process.env.NODE_ENV !== "development";
const twBaseName = "";
module.exports = {
  mode: IS_PROD ? "" : "jit",
  important: false,
  content: ["./public/**/*.html", "./src/**/*.{vue,js,ts,tsx}"],
  theme: {
    container: {
      center: true,
    },
    extend: {
      color: {
        [`${twBaseName}2b5be9`]: "#2b5be9",
        [`${twBaseName}111`]: "#111111",
      },
      fontSize: {
        [`${twBaseName}base`]: ["12px", { lineHeight: 0 }],
      },
    },
  },
  variants: {
    extend: {},
  },
  prefix: "tw-",
  plugins: [],
  corePlugins: {
    preflight: false,
  },
};
