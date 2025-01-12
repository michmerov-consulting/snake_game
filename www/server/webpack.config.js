var path = require("path");
var CopyWebPackPlugin = require("copy-webpack-plugin");
module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "./server"),
        filename: "bootstrap.js"
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
    mode: "development",
    plugins: [new CopyWebPackPlugin({
            patterns: [
                { from: "./index.html", to: "./" },
                { from: "./public", to: "./" }
            ]
        })]
};
