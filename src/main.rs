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
    width: f64,
    text: &str,
) {
    let mut height = 0;
    let mut line = String::new();
    for word in text.split_whitespace() {
        let new_line = match line.len() {
            0 => word.to_string(),
            _ => line.clone() + " " + word,
        };

        let extents = cr.text_extents(new_line.as_str()).unwrap();
        if extents.width() > width {
            cr.move_to(pos.x, pos.y + 12.0 + height);
            cr.show_text(line.as_str()).unwrap();
            height += 12.0;
            line = word.to_string();
        } else {
            line = new_line;
        }
    }

    cr.move_to(pos.x, pos.y + 12.0 + height);
    cr.show_text(line.as_str()).unwrap();

    height += 12.0;

    cr.rectangle(pos.x, pos.y, width, height);
    cr.stroke().unwrap();
}

fn main() {
    let args = Args::parse();
}
