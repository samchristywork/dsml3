use cairo::{Context, FontSlant, FontWeight, Format, ImageSurface};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file to read
    #[arg(short, long)]
    input: String,

    /// Name of the output file
    #[arg(short, long)]
    output: String,
}

struct Vec2 {
    x: f64,
    y: f64,
}

fn draw_text_box(
    cr: &Context,
    pos: &Vec2,
    pad: &Vec2,
    width: f64,
    text: &str,
) -> f64 {
    let mut height = pad.y;
    let mut line = String::new();
    for word in text.split_whitespace() {
        let new_line = match line.len() {
            0 => word.to_string(),
            _ => line.clone() + " " + word,
        };

        let extents = cr.text_extents(new_line.as_str()).unwrap();
        if extents.width() > width - 2.0 * pad.x {
            cr.move_to(pos.x + pad.x, pos.y + font_height + height);
            cr.show_text(line.as_str()).unwrap();
            height += 12.0;
            line = word.to_string();
        } else {
            line = new_line;
        }
    }

    cr.move_to(pos.x + pad.x, pos.y + font_height + height);
    cr.show_text(line.as_str()).unwrap();

    height += 12.0;
    height += pad.y;

    cr.rectangle(pos.x, pos.y, width, height);
    cr.stroke().unwrap();

    height
}

fn read_float(value: &str, current: f64) -> f64 {
    if value == "" {
        return current;
    }

    if value.chars().nth(0).unwrap() == '+' {
        current + value[1..].parse::<f64>().unwrap()
    } else if value.chars().nth(0).unwrap() == '-' {
        current - value[1..].parse::<f64>().unwrap()
    } else {
        value.parse::<f64>().unwrap()
    }
}

fn main() {
    let args = Args::parse();

    let surface = ImageSurface::create(Format::ARgb32, 100, 100).unwrap();

    let cr = Context::new(&surface).unwrap();

    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.paint().unwrap();

    cr.select_font_face("Arial", FontSlant::Normal, FontWeight::Normal);
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.set_font_size(12.0);
    cr.set_line_width(1.0);

    let mut stream = std::fs::File::create(args.output).unwrap();
    surface.write_to_png(&mut stream).unwrap();
}
