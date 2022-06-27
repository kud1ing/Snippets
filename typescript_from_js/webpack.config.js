const path = require('path');

module.exports = {
    entry: './index.js',
    mode: 'production',
    /*mode: 'development',*/
    output: {
        filename: 'result.js',
        path: path.resolve(__dirname, 'dist'),
    },
};
