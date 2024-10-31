import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt({
   ignores: ["presets"],
   rules: {
      "vue/html-self-closing": "off",
   },
});
