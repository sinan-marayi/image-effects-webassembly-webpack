const path= require('path')
const HTMLWebpackPlugin=require('html-webpack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports={
entry:'./public/main.js',
output:{
    path:path.resolve(__dirname,'dist'),
    filename:'index.js'
},
plugins:[
    new HtmlWebpackPlugin({
        template:'./public/index.html'
    })
]
}