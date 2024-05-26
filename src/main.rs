use std::io::BufWriter;

use resvg::usvg::fontdb::{Family, Query};
use svg::node::element::tag::Path;
use svg::Document;
use svg::node::element::{Circle, Path, Text, TextPath};
use resvg::usvg::{fontdb, Options, Transform, Tree};
use resvg::tiny_skia::{Pixmap, PixmapMut};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circle = Circle::new()
        .set("cx", 500)
        .set("cy", 500)
        .set("r", 100)
        .set("id", "circle1")
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
    
    let text_path = TextPath::new("!Hello, World!")   
        .set("x", 0)
        .set("y", 600)
        .set("href", "#circle1")
        .set("text-anchor", "start")
        .set("font-family", "Tengwar Annatar")
        .set("font-size", 30)
        .set("fill", "black");

    let text_node = Text::new("")
        .add(text_path.clone());


    // let data = Data::new()
        // .
        // .move_to((10, 10))
        // .line_by((0, 50))
        // .line_by((50, 0))
        // .line_by((0, -50))
        // .close();

    // let path = Path::new()
    //     .set("fill", "none")
    //     .set("stroke", "black")
    //     .set("stroke-width", 1)
    //     .set("d", circle);

    let document = Document::new()
        .set("viewBox", (0, 0, 2000, 2000))
        .add(circle)
        .add(path)
        .add(text_node)
        ;
        // .add(text_path);


    svg::save("image.svg", &document)?;
    let mut svg_data = Vec::new();
    let writer = BufWriter::new(&mut svg_data);
    svg::write(writer, &document)?;
    println!("{:?}", svg_data.len());

    let mut fontdb = fontdb::Database::new();
    // fontdb.load_system_fonts();
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
    Ok(())
}
