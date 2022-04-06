const express = require('express');
const {catalogView } = require('../controllers/catalogController');
const router = express.Router();
router.get('/catalog', catalogView);
module.exports = router;