const contactController = (req,res) => {
    // Defaul views fileLoaded in app.js, so no need to import
    res.render('contact', {'title' : "Contact"})
}
export {contactController}