use std::collections::HashMap;
use std::io::BufWriter;

use resvg::usvg::fontdb::{Family, Query};
use svg::node::element::path::Data;
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


    let mut dots = Vec::new();

    let c = 3.0;
    let colors = ["red", "blue", "green", "yellow", "purple", "orange"];
    // let angle: f64 = 137.508_f64.to_radians();
    let angle: f64 = (83.702_f64).to_radians();
    for i in 20..200 {
        let r = c * (i as f64).sqrt();
        let theta = (i as f64) * angle;
        let x = r * theta.cos() + 300.0;
        let y = r * theta.sin() + 300.0;
        dots.push(
            Circle::new()
                .set("cx", x)
                .set("cy", y)
                .set("r", 1)
                .set("fill", colors[i % colors.len()])
        )
    }

    let data = Data::new()
        .move_to((10, 90))
        .quadratic_curve_by((10.0, -10.0, 20.0, 0.0))
        .smooth_quadratic_curve_by((20.0, 0.0))
        ;


    let test_path = Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("d", data);


    let mut document = Document::new()
        .set("viewBox", (0, 0, 2000, 2000))
        .add(defs)
        // .add(circle)
        // .add(circle2)
        // .add(circle3)
        // .add(path)
        // .add(text_node)
        .add(test_path)
        // .add(ogham_text)
        ;
        // .add(text_path);

    for dot in dots {
        document = document.add(dot.clone());
    }

    svg::save("image.svg", &document)?;
    let mut svg_data = Vec::new();
    let writer = BufWriter::new(&mut svg_data);
    svg::write(writer, &document)?;
    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();
    fontdb.load_font_file("./resources/fonts/TengwarAnnatarAltBoldItalic-YzDo.ttf")?;
    fontdb.load_font_file("./resources/fonts/TengwarAnnatarBoldItalic-K7r7.ttf")?;

    let opt = Options::default();
    let tree = Tree::from_data(&svg_data, &opt, &fontdb)?;
    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());
    pixmap.save_png("image.png")?;
    Ok(())
}
