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

    /// Width of the output file
    #[arg(long)]
    width: Option<f64>,

    /// Height of the output file
    #[arg(long)]
    height: Option<f64>,
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

    let scale = 2.0;
    let width = (args.width.unwrap_or(595.0) * scale) as i32;
    let height = (args.height.unwrap_or(842.0) * scale) as i32;
    let surface = ImageSurface::create(Format::ARgb32, width, height).unwrap();

    let cr = Context::new(&surface).unwrap();
    cr.scale(scale, scale);

    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.paint().unwrap();

    cr.select_font_face("Arial", FontSlant::Normal, FontWeight::Normal);
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.set_font_size(12.0);
    cr.set_line_width(1.0);

    let content = std::fs::read_to_string(args.input.clone());
    let content = match content {
        Ok(content) => content,
        Err(_) => {
            println!("Error reading file: {}", args.input);
            return;
        }
    };

    let lines: Vec<&str> = content.split('\n').collect();

    let mut cursor = Vec2 { x: 100.0, y: 100.0 };
    for line in lines.iter() {
        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() != 2 {
            println!("Invalid line: {}", line);
            continue;
        }

        let key = parts[0];
        let value = parts[1];

        match key {
            "x" => cursor.x = read_float(value, cursor.x),
            "y" => cursor.y = read_float(value, cursor.y),
            "size" => {
                cr.set_font_size(value.parse::<f64>().unwrap());
            }
            "text" => {
                cr.move_to(cursor.x, cursor.y);
                cr.show_text(value).unwrap();
            }
            _ => {}
        }
    }

    let mut stream = std::fs::File::create(args.output).unwrap();
    surface.write_to_png(&mut stream).unwrap();
}
