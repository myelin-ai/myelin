const path = require('path');

module.exports = {
  entry: "./src/main.ts",
  node: false,
  mode: "development",
  module: {
    rules: [{
        test: /\.ts$/,
        exclude: /node_modules/,
        use: 'ts-loader',
      },
      {
        test: /\.wasm$/,
        exclude: /node_modules/,
        type: "javascript/auto",
        loader: "file-loader",
        options: {
          publicPath: "dist/"
        }
      }
    ]
  },
  output: {
    publicPath: '/dist/',
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  resolve: {
    extensions: ['.ts', '.js', '.wasm'],
  },
};
