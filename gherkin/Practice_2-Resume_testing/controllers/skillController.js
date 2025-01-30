const skillController = (req,res) => {
    // Defaul views fileLoaded in app.js, so no need to import
    res.render('skill', {'title' : "Mitanshu Kelaiya - Skill"})
}
export {skillController}