const express = require('express');
const app = express();

const port = process.env.PORT;

app.listen(process.env.PORT, () => {
    console.log(`App is listening on ${port}`)
})