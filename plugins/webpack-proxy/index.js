env = require("../../env")

// This one is used when running locally.
// It routes /api to centraldev.ppsspp.org, where we run a development instance
// of the main backend server.
module.exports = function (context, options) {
    return {
        name: "custom-webpack-plugin",
        configureWebpack(_config, _isServer, _utils) {
            console.log("Modifying webpack config to proxy API calls.");
            return {
                mergeStrategy: { "devServer.proxy": "replace" },
                devServer: {
                    proxy: {
                        "/api": {
                            target: "http://centraldev.ppsspp.org",
                            secure: false,
                            changeOrigin: true,
                            logLevel: "debug",
                        },
                    },
                },
            };
        },
    };
};
