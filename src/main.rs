use std::collections::HashMap;
use std::io::BufWriter;
use itertools::Itertools;

use resvg::usvg::fontdb::{Family, Query};
use svg::node::element::path::Data;
use svg::node::element::tag::Path;
use svg::Document;
use svg::node::element::{Circle, Definitions, Path, Style, Text, TextPath};
use resvg::usvg::{fontdb, Options, Transform, Tree};
use resvg::tiny_skia::{Pixmap, PixmapMut};
use phf::phf_map;

use crate::ogham::into_ogham;
mod lambda_calculus_parser;

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

    let lorem_ipsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    // to make sure the line connects neatly we will have to warp
    // example: https://henry.codes/writing/how-to-distort-text-with-svg/
    let ogham_text = TextPath::new(into_ogham(lorem_ipsum.to_string()))
        .set("x", 0)
        .set("y", 600)
        // .set("href", "#test_path")
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


    let mut dots: Vec<Circle> = Vec::new();

    let c = 11.0;
    let colors = ["red", "blue", "green", "yellow", "purple", "orange"];
    // let angle: f64 = 137.508_f64.to_radians();
    let angle: f64 = (83.702_f64).to_radians();
    let mut curves = vec![vec![]; 30];
    for i in 10..300 {
        let r = c * (i as f64).sqrt();
        let theta = (i as f64) * angle;
        let x = r * theta.cos() + 300.0;
        let y = r * theta.sin() + 300.0;
        curves[i % 30].push((x, y));

        // dots.push(
        //     Circle::new()
        //         .set("cx", x)
        //         .set("cy", y)
        //         .set("r", 1)
        //         .set("fill", colors[i % colors.len()])
        //     // .set("stroke", "black")
        // );
    }

    
    let mut data = Data::new();
    let mut debug_data = Data::new();

    // try 3: Centripetal Catmull–Rom spline

    for curve in curves.iter() {
        data = to_catmul_rom_spline(data, curve);
    }


    let test_path = Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        // .set("stroke", "none")
        .set("id", "test_path")
        .set("d", data);

    let debug_path = Path::new()
        .set("fill", "none")
        .set("stroke", "blue")
        .set("d", debug_data);


    let mut document = Document::new()
        .set("viewBox", (0, 0, 2000, 2000))
        .add(defs)
        // .add(circle)
        // .add(circle2)
        // .add(circle3)
        // .add(path)
        .add(text_node)
        .add(test_path)
        // .add(debug_path)
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
    println!("Done rendering!");
    Ok(())
}

const ALPHA: f64 = 0.5;
fn to_catmul_rom_spline(mut data: Data, points: &[(f64, f64)]) -> Data {
    let mut t_i = vec![0.0];
    for i in 1..points.len() {
        let (x1, y1) = points[i - 1];
        let (x2, y2) = points[i];
        let d = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt().powf(ALPHA);
        t_i.push(t_i[i - 1] + d);
    }
    data = data.move_to(points[0]);

    for i in 1..points.len()-2 {
        let (x0, y0) = points[i - 1];
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];
        let (x3, y3) = points[i + 2];

        let t0 = t_i[i - 1];
        let t1 = t_i[i];
        let t2 = t_i[i + 1];
        let t3 = t_i[i + 2];

        let c1 = (t2 - t1) / (t2 - t0);
        let c2 = (t1 - t0) / (t2 - t0);

        let d1 = (t3 - t2) / (t3 - t1);
        let d2 = (t2 - t1) / (t3 - t1);

        let m1 = (
            (t2 - t1) * (c1 * (x1 - x0) / (t1 - t0) + c2 * (x2 - x1) / (t2 - t1)),
            (t2 - t1) * (c1 * (y1 - y0) / (t1 - t0) + c2 * (y2 - y1) / (t2 - t1)),
        );

        let m2 = (
            (t2 - t1) * (d1 * (x2 - x1) / (t2 - t1) + d2 * (x3 - x2) / (t3 - t2)),
            (t2 - t1) * (d1 * (y2 - y1) / (t2 - t1) + d2 * (y3 - y2) / (t3 - t2)),
        );

        // let q0 = (x1, x2);
        let q1 = (
            x1 + m1.0 / 3.0,
            y1 + m1.1 / 3.0,
        );
        let q2 = (
            x2 - m2.0 / 3.0,
            y2 - m2.1 / 3.0,
        );
        let q3 = (x2, y2);
        data = data.cubic_curve_to((q1.0, q1.1, q2.0, q2.1, q3.0, q3.1));
    }

    data
}
