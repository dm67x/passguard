const rules = require('./webpack.rules')

rules.push({
  test: /\.css$/,
  use: [{ loader: 'style-loader' }, { loader: 'css-loader' }],
});

rules.push({
  test: /\.jsx?$/,
  exclude: /node_modules/,
  use: [{ loader: 'babel-loader' }]
})

rules.push({
  test: /\.(woff|woff2|eot|ttf|svg)$/,
  loader: 'file-loader',
  options: { name: '[name].[ext]', outputPath: 'fonts/' }
})

rules.push({
  test: /\.(png|jpe?g|gif)$/i,
  use: [{ loader: 'file-loader' }],
})

module.exports = {
  // Put your normal webpack config below here
  module: {
    rules,
  },
};
