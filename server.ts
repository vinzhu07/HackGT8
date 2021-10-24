import pkg from '@prisma/client';
const { PrismaClient } = pkg;
const prisma = new PrismaClient()

import Handlebars from 'hbs'

import express from 'express'
const app = express()
const port = 3002

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
    var color= [];
    var gender = [];
    var season = [];
    var article = [];
    var sub = [];

    for (const element of likes) {
        var id = element.cloth_id;
        const cloth = await prisma.clothes.findUnique({
            where: {id: id}
        })
        var tempname = cloth.product_display_name;
        var tempimage = "/images/" + cloth.id + ".jpg"
        var tempusage = cloth.usage;
        var tempcolor = cloth.base_color;
        article.push(cloth.article_type);
        sub.push(cloth.sub_category);
        season.push(cloth.season);
        gender.push(cloth.gender);
        name.push(tempname);
        image.push(tempimage);
        usage.push(tempusage);
        color.push(tempcolor);
    }

    var code = "<table class='table' style = 'overflow-y:auto'>\
    <tbody>";

    for (let x = 0; x < size; x++) {
        code += "<tr id = 'shadow'>";
        code = code + "<td> <img src="+image[x]+" id = 'picture'> <p>" + name[x]+ "</p> </td>";
        var nested = "";
        nested = "<table class = 'table' id = 'nested'> <tbody><tr><td>Gender</td><td>Article Type</td><td>Sub Category</td><td>Color</td><td>Season</td><td>Usage</td></tr>";
        nested += "<tr><td>"+gender[x]+"</td><td>"+article[x]+"</td><td>"+sub[x]+"</td><td>"+color[x]+"</td><td>"+season[x]+"</td><td>"+usage[x]+"</td>";
        nested +="</tbody></table>";
        code = code + "<td> "+nested+"</td>"
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
    var color= [];
    var gender = [];
    var season = [];
    var article = [];
    var sub = [];


    for (const element of likes) {
        var id = element.cloth_id;
        const cloth = await prisma.clothes.findUnique({
            where: {id: id}
        })
        var tempname = cloth.product_display_name;
        var tempimage = "/images/" + cloth.id + ".jpg"
        var tempusage = cloth.usage;
        var tempcolor = cloth.base_color;
        
        article.push(cloth.article_type);
        sub.push(cloth.sub_category);
        season.push(cloth.season);
        gender.push(cloth.gender);
        name.push(tempname);
        image.push(tempimage);
        usage.push(tempusage);
        color.push(tempcolor);

    }

    var code = "<table class='table' style = 'overflow-y:auto'>\
    <tbody>";

    for (let x = 0; x < size; x++) {
        code += "<tr id = 'shadow'>";
        code = code + "<td> <img src="+image[x]+" id = 'picture'> <p>" + name[x]+ "</p> </td>";
        var nested = "";
        nested = "<table class = 'table' id = 'nested'> <tbody><tr><td>Gender</td><td>Article Type</td><td>Sub Category</td><td>Color</td><td>Season</td><td>Usage</td></tr>";
        nested += "<tr><td>"+gender[x]+"</td><td>"+article[x]+"</td><td>"+sub[x]+"</td><td>"+color[x]+"</td><td>"+season[x]+"</td><td>"+usage[x]+"</td>";
        nested +="</tbody></table>";
        code = code + "<td> "+nested+"</td>"
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