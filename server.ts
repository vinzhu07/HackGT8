import pkg from '@prisma/client';
const { PrismaClient } = pkg;
const prisma = new PrismaClient()

import Handlebars from 'hbs'

import express from 'express'
const app = express()
const port = 3000

app.set('view engine', 'hbs');

app.use('/static', express.static('static'))
app.use('/images', express.static('./data/fashion-dataset/images'))
app.use(express.json());

app.get('/', async (req, res) => {
    const clothesCount = await prisma.clothes.count();
    const skip = Math.floor(Math.random() * clothesCount);
    const cloth = await prisma.clothes.findFirst({
        skip: skip
    })

    res.render("home", {
        id: cloth.id,
        image: "/images/" + cloth.id + ".jpg",
        name: cloth.product_display_name,
        subtitle: cloth.usage,
        subtitleTwo: cloth.season
    })

    
})


app.get('/likes', async (req, res) => {
    const likes = await prisma.swipes.findMany({
        where: { love_status: true },
    });
    const size = likes.length;
    var name = [];
    var image = [];
    var usage = [];

    for (const element of likes) {
        var id = element.cloth_id;
        const cloth = await prisma.clothes.findUnique({
            where: {id: id}
        })
        var tempname = cloth.product_display_name;
        var tempimage = "/images/" + cloth.id + ".jpg"
        var tempusage = cloth.usage;
        name.push(tempname);
        image.push(tempimage);
        usage.push(tempusage);
    }

    var code = "<table class='table' style = 'overflow-y:auto'>\
    <tbody>";

    for (let x = 0; x < size; x++) {
        code += "<tr>";
        code = code + "<td> <img src="+image[x]+" id = 'picture'> </td>";
        code += "</tr>";

    }

    var end = "</tbody>\
  </table>";

    code += end;
    
    //console.log(code);

    res.render("likes", {
        table: new Handlebars.SafeString(code)
    })

    
})

app.get('/dislikes', async (req, res) => {
    
    const likes = await prisma.swipes.findMany({
        where: { love_status: false},
    });
    const size = likes.length;
    var name = [];
    var image = [];
    var usage = [];

    for (const element of likes) {
        var id = element.cloth_id;
        const cloth = await prisma.clothes.findUnique({
            where: {id: id}
        })
        var tempname = cloth.product_display_name;
        var tempimage = "/images/" + cloth.id + ".jpg"
        var tempusage = cloth.usage;
        name.push(tempname);
        image.push(tempimage);
        usage.push(tempusage);
    }

    var code = "<table class='table' style = 'overflow-y:auto'>\
    <tbody>";

    for (let x = 0; x < size; x++) {
        code += "<tr>";
        code = code + "<td> <img src="+image[x]+" id = 'picture'> </td>";
        code += "</tr>";

    }

    var end = "</tbody>\
  </table>";

    code += end;
    
    //console.log(code);

    res.render("dislikes", {
        table: new Handlebars.SafeString(code)
    })

    
})

app.post('/item_swipe_status', async (req, res) => {

    await prisma.swipes.create({
        data: {
            love_status: req.body.status,
            cloth_id: req.body.id
        }
    })

    res.sendStatus(200)
})

app.listen(port, () => {
    console.log(`Listening on http://server.zpparel.com:${port}`)
})