module.exports = {
  "clip-mash": {
    input: "../api-docs.json",
    output: {
      target: "./src/api.ts",
      override: {
        mutator: {
          path: "./src/custom-client.ts",
          name: "customInstance",
        },
      },
    },
  },
}
