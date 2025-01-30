const homeController = (req,res) => {
    // Defaul views fileLoaded in app.js, so no need to import
    res.render('home', {'title' : "Mitanshu Kelaiya - Home"})
}
export {homeController}