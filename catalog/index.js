const express = require('express');
const app = express();
const PORT = process.env.PORT || 3000;


app.set('view engine', 'ejs');
app.use('/', require('./routes/catalog'));

app.listen(PORT, console.log("Server don start for port: " + PORT))