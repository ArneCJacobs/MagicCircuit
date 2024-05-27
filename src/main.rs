use std::collections::HashMap;
use std::io::BufWriter;

use resvg::usvg::fontdb::{Family, Query};
use svg::node::element::tag::Path;
use svg::Document;
use svg::node::element::{Circle, Definitions, Path, Style, Text, TextPath};
use resvg::usvg::{fontdb, Options, Transform, Tree};
use resvg::tiny_skia::{Pixmap, PixmapMut};
use phf::phf_map;

use crate::ogham::into_ogham;

mod ogham;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circle = Circle::new()
        .set("cx", 500)
        .set("cy", 500)
        .set("r", 100)
        .set("id", "circle1")
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1);

    let circle3 = Circle::new()
        .set("cx", 800)
        .set("cy", 500)
        .set("r", 105.3)
        .set("id", "circle3")
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1.5);

    let circle2 = Circle::new()
        .set("cx", 800)
        .set("cy", 500)
        .set("r", 100)
        .set("id", "circle2")
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1);


    // let text = Text::new("!Hello, world!")
    //     .set("x", 0)
    //     .set("y", 600)
    //     .set("text-anchor", "start")
    //     .set("font-family", "Tengwar Annatar")
    //     .set("font-size", 30)
    //     .set("fill", "black");

    let path = Path::new()
        .set("id", "path1")
        .set("fill", "none")
        .set("stroke", "red")
        .set("d", "M10,90 Q90,90 90,45 Q90,10 50,10 Q10,10 10,40 Q10,70 45,70 Q70,70 75,50");
    
    let text_path = TextPath::new("Hello, World, neat this is cool!")   
        .set("x", 0)
        .set("y", 600)
        .set("href", "#circle1")
        .set("text-anchor", "start")
        .set("font-family", "Tengwar Annatar")
        .set("font-size", 14)
        .set("fill", "black");

    // to make sure the line connects neatly we will have to warp
    // example: https://henry.codes/writing/how-to-distort-text-with-svg/
    let ogham_text = TextPath::new(into_ogham("Hello, World! I am glad to be here".to_string()))
        .set("x", 0)
        .set("y", 600)
        .set("href", "#circle2")
        .set("text-anchor", "start")
        .set("font-family", "Tengwar Annatar")
        // .set("letter-spacing", "-4")
        .set("font-size", 15)
        .set("fill", "black");

    let text_node = Text::new("")
        .add(ogham_text.clone())
        .add(text_path.clone());



    let style = Style::new(r#"
        @font-face {
            font-family: "Tengwar Annatar";
            src: url('./resources/fonts/TengwarAnnatarBoldItalic-K7r7.ttf') format("truetype");
        }
    "#).set("type", "text/css");

    let defs = Definitions::new()
        .add(style);


    let document = Document::new()
        .set("viewBox", (0, 0, 2000, 2000))
        .add(defs)
        .add(circle)
        .add(circle2)
        .add(circle3)
        .add(path)
        .add(text_node)

        // .add(ogham_text)
        ;
        // .add(text_path);




    svg::save("image.svg", &document)?;
    let mut svg_data = Vec::new();
    let writer = BufWriter::new(&mut svg_data);
    svg::write(writer, &document)?;
    println!("{:?}", svg_data.len());

    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();
    fontdb.load_font_file("./resources/fonts/TengwarAnnatarAltBoldItalic-YzDo.ttf")?;
    fontdb.load_font_file("./resources/fonts/TengwarAnnatarBoldItalic-K7r7.ttf")?;
    let query = Query {
        families: &[Family::Name("Tengwar Annatar")],
        ..Default::default()
    };
    // for face in fontdb.faces() {
    //     println!("{:?}", face);
    // }
    println!("{:?}", fontdb.query(&query));
    let opt = Options::default();
    let tree = Tree::from_data(&svg_data, &opt, &fontdb)?;
    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());
    pixmap.save_png("image.png")?;

    println!("{:?}", into_ogham("Hello, World!".to_string()));
    Ok(())
}
