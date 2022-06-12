const path = require('path');

module.exports = {
    mode: "production",
    entry: path.resolve(__dirname, 'index.js'),
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bundle.js',
    },
    experiments: {
        asyncWebAssembly: true
    }
}