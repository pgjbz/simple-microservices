// For View 
const PRODUCT_URL = process.env.PRODUCT_URL || 'localhost:8000';
const axios = require('axios')


const catalogView = async (_req, res) => {

    let values = [];
    await axios.get(`http://${PRODUCT_URL}/products`)
        .then(r => {
            
            r.data.forEach(element => {
                values.push(element);
            });
        })
        .catch(error => {
            console.error(error)
        })

    res.render("catalog", {
        products: values
    });
}

module.exports = {
    catalogView
};