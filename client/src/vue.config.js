module.exports = {
  devServer: {
    proxy: {
      "/test": {
        target: "http://localhost:8080",
        changeOrigin: true
      },
      "/api": {
        target: "http://localhost:8080",
        changeOrigin: true
      },
    }
  }
};
