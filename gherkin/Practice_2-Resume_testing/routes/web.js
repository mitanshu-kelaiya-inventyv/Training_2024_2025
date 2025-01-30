import express from 'express'
const router = express.Router();
import { homeController } from '../controllers/homeController.js';
import { contactController } from '../controllers/contactController.js';
import { skillController } from '../controllers/skillController.js';
import { servicesController } from '../controllers/servicesController.js';

// router.use('/', (req,res,next)=>{
//     next();
// });

router.get('/', homeController);
router.get('/contact', contactController);
router.get('/skill', skillController);
router.get('/services', servicesController);

export default router