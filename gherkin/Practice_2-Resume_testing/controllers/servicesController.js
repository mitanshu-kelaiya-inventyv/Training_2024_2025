const servicesController = (req,res) => {
    // Defaul views fileLoaded in app.js, so no need to import
    res.render('services', {'title' : "Services"})
}
export {servicesController}