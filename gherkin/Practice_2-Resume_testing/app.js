import express from 'express'
const app = express()
const port = process.env.PORT || '8000'
import web from './routes/web.js'
import webcopy from './routes/webcopy.js'
// import './node_modules/font-awesome/css/font-awesome.css';
// node_modules\font-awesome\css\font-awesome.min.css

//Set Template Engine
app.set('view engine', 'ejs')

//Static Files
app.use(express.static('public'))

//Load Routes
// app.use('/', webcopy)
app.use('/', web)
// app.use('/', web)
// app.get('/',(req,res)=>{
//     res.send("Working");
// })
app.listen(port, ()=>{
    console.log(`Server listening at http://localhost:${port}`)
})

