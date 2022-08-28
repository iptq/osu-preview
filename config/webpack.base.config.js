const webpack = require("webpack");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require("path");

const root = path.resolve(__dirname, "..");

module.exports = (env) => {
  const config = {
    entry: {
      popup: [
        path.resolve(root, "popup/index.js"),
        path.resolve(root, "popup/styles/main.sass"),
      ],
      background: path.resolve(root, "background/background.js"),
      content: path.resolve(root, "background/content.js"),
    },

    mode: env,

    output: {
      publicPath: "",
      path: path.resolve(root, "build"),
      filename: "[name].js",
    },

    module: {
      rules: [
        {
          test: /\.js?$/,
          exclude:
            env === "production"
              ? /(node_modules|bower_components)/
              : undefined,
          use: {
            loader: "babel-loader",
            options: {
              presets: ["@babel/preset-env"],
            },
          },
        },
        {
          test: /\.s[ac]ss$/,
          use: [
            MiniCssExtractPlugin.loader,
            {
              loader: "css-loader",
              options: { url: false },
            },
            "sass-loader",
          ],
        },
        {
          test: /\.(png|svg|woff2|ttf)$/,
          type: "asset/resource",
        },
      ],
    },

    devtool: env === "development" ? "inline-cheap-source-map" : undefined,

    plugins: [
      new CopyWebpackPlugin({
        patterns: [
          {
            context: "./static/",
            from: "**/*",
            to: "./",
          },
          {
            context: "./assets/",
            from: "**/*",
            to: "./assets",
          },
        ],
      }),

      new webpack.DefinePlugin({
        __DEV__: false,
        __CHROME__: JSON.stringify(
          JSON.parse(process.env.BUILD_CHROME || true)
        ),
        __FIREFOX__: JSON.stringify(JSON.parse(process.env.BUILD_FF || false)),
      }),

      new MiniCssExtractPlugin({
        filename: "[name].css",
      }),

      new WasmPackPlugin({
        crateDirectory: path.resolve(root, "backend"),
      }),
    ],

    resolve: {
      extensions: [".js", ".sass", ".scss"],
    },

    experiments: {
      asyncWebAssembly: true,
    },
  };
  return config;
};
