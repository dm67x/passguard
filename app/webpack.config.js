const path = require('path')

module.exports = [
    {
        mode: 'development',
        entry: path.join(__dirname, 'main.js'),
        target: 'electron-main',
        module: {
            rules: [{
                test: /\.js$/,
                exclude: /(node_modules|bower_components)/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: ['@babel/preset-env']
                    }
                }
            }]
        },
        output: {
            path: path.join(__dirname, 'dist'),
            filename: 'passguard.js' 
        }
    }
]