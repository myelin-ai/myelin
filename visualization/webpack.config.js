const path = require('path');

module.exports = {
  entry: "./src/main.ts",
  module: {
    rules: [{
      test: /\.ts$/,
      use: 'ts-loader',
      exclude: /node_modules/
    }]
  },
  node: false,
  output: {
    publicPath: '/dist/',
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  resolve: {
    extensions: ['.ts', '.js', '.wasm'],
  },
  mode: "development",
};