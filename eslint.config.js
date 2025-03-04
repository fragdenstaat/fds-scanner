import pluginVue from 'eslint-plugin-vue'
export default [
  // add more generic rulesets here, such as:
  // js.configs.recommended,
  ...pluginVue.configs['flat/recommended'],
  {
    "root": true,
    "parser": "vue-eslint-parser",
    "parserOptions": {
        "parser": "@typescript-eslint/parser",
    },
    "plugins": ["@typescript-eslint", "prettier"],
    "extends": [
        "plugin:vue/strongly-recommended",
        "eslint:recommended",
        "@vue/typescript/recommended",
        "prettier",
    ],
    "rules": {
      // override/add rules settings here, such as:
      "prettier/prettier": "error",
      "vue/no-unused-vars": "error",
      "vue/no-undef-components": "error"
    }
  }
]
