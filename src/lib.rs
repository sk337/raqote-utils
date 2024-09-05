use std::arch::x86_64;

use euclid::Transform2D;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::outline::OutlineSink;
use pathfinder_geometry::{transform2d, vector::Vector2F};
use raqote::{DrawOptions, DrawTarget, PathBuilder, Source};
use regex::Regex;
use rustybuzz::{Face, UnicodeBuffer};

/// Create a raqote::Path from a Svg path data string
///
/// # Arguments
///
/// * `svg_raw_path` - A string slice that holds the Svg path data
///
/// # Example
///
/// ```
/// let B = create_path_from_string("M105 57.0273V453.751H252.659C448.259 461.723 428.124 276.022 352.856 253.513V243.197C424.768 204.274 423.809 54.6826 252.659 57.0273H105Z");
/// ```
///---
///
/// supports the following Svg path data commands:
/// m,M,l,L,v,V,h,H,c,C,s,S
// TODO: Implement ops  T, t, A, a
pub fn create_path_from_string(svg_raw_path: &str) -> raqote::Path {
    let svg_regex = format!(
        r"(?:[mMlL]\s?{nr} {nr}|[vVhH]\s?{nr}|[cC]\s?{nr} {nr} {nr} {nr} {nr} {nr}|[Ss]\s?{nr} {nr} {nr} {nr})",
        nr = r"(?:d?[1-9]\d*(?:\.\d*)?)"
    );
    let reg = Regex::new(svg_regex.as_str());
    let mut elements: Vec<String> = Vec::new();

    reg.unwrap().captures_iter(svg_raw_path).for_each(|cap| {
        elements.push(cap.get(0).unwrap().as_str().to_string());
    });

    let mut path = PathBuilder::new();

    let mut last_x = 0.0;
    let mut last_y = 0.0;

    for element in elements.into_iter() {
        let command = element.chars().nth(0).unwrap();
        let element = element.chars().skip(1).collect::<String>();
        let values = element
            .trim()
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<f32>().unwrap())
            .collect::<Vec<f32>>();

        match command.to_string().as_str() {
            "m" => {
                last_x += values[0];
                last_y += values[1];
                path.move_to(last_x, last_y);
            }
            "M" => {
                last_x = values[0];
                last_y = values[1];
                path.move_to(last_x, last_y);
            }
            "l" => {
                last_x += values[0];
                last_y += values[1];
                path.line_to(last_x, last_y);
            }
            "L" => {
                last_x = values[0];
                last_y = values[1];
                path.line_to(last_x, last_y);
            }
            "v" => {
                last_y += values[0];
                path.line_to(last_x, last_y);
            }
            "V" => {
                last_y = values[0];
                path.line_to(last_x, last_y);
            }
            "h" => {
                last_x += values[0];
                path.line_to(last_x, last_y);
            }
            "H" => {
                last_x = values[0];
                path.line_to(last_x, last_y);
            }
            "c" => {
                let x1 = last_x + values[0];
                let y1 = last_y + values[1];
                let x2 = last_x + values[2];
                let y2 = last_y + values[3];
                last_x += values[4];
                last_y += values[5];
                path.cubic_to(x1, y1, x2, y2, last_x, last_y);
            }
            "C" => {
                let x1 = values[0];
                let y1 = values[1];
                let x2 = values[2];
                let y2 = values[3];
                last_x = values[4];
                last_y = values[5];
                path.cubic_to(x1, y1, x2, y2, last_x, last_y);
            }
            "s" => {
                let x1 = last_x + values[0];
                let y1 = last_y + values[1];
                last_x += values[2];
                last_y += values[3];
                path.quad_to(x1, y1, last_x, last_y);
            }
            "S" => {
                let x1 = values[0];
                let y1 = values[1];
                last_x = values[2];
                last_y = values[3];
                path.quad_to(x1, y1, last_x, last_y);
            }
            _ => {}
        }
    }

    path.finish()
}

/// Create a raqote::Path for a circle
///
/// # Arguments
///
/// * `radius` - The radius of the circle
/// * `x` - The x coordinate of the center of the circle
/// * `y` - The y coordinate of the center of the circle
///
/// # Example
///
/// ```
/// use raqote_utils::build_circle;
/// 
/// let circle = build_circle(100.0, 100.0, 100.0);
/// ```
pub fn build_circle(radius: f32, x: f32, y: f32) -> raqote::Path {
    let mut pb = PathBuilder::new();

    let x = x - radius;
    let y = y + radius;

    let offset = 0.5522847498 * radius;

    pb.move_to(x + radius, y);

    pb.cubic_to(
        x + radius + offset,
        y,
        x + (radius * 2.),
        y - radius + offset,
        x + (radius * 2.),
        y - radius,
    );

    pb.cubic_to(
        x + (radius * 2.),
        y - radius - offset,
        x + radius + offset,
        y - (radius * 2.),
        x + radius,
        y - (radius * 2.),
    );

    pb.cubic_to(
        x + radius - offset,
        y - (radius * 2.),
        x,
        y - radius - offset,
        x,
        y - radius,
    );

    pb.cubic_to(x, y - offset, x + radius - offset, y, x + radius, y);

    pb.finish()
}

/// Write text to screen
///
/// # Arguments
///
/// text: Text to write to screen
///
/// x: X cordinate of text
///
/// y: Y cordinate of text
///
/// font: Font to use for rendering,
///
/// font_size: font size in pt
///
/// ctx: Draw target to draw text to
///
/// # Example
///
/// ```
/// create_text(
///     "Hello, World\nline2",
///     50.,
///     50.,
///     "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
///     35.,
///     &mut dt,
///     &Source::Solid(SolidSource {
///         r: 0x00,
///         g: 0x00,
///         b: 0x00,
///         a: 0xFF,
///     }),
/// );
/// ```
pub fn create_text(
    text: &str,
    x: f32,
    y: f32,
    font_path: &str,
    font_size: f32,
    ctx: &mut DrawTarget,
    source: &Source<'_>,
) {
    // convert font_size to px from em

    let line_height = (font_size / 72.) * 96.;



    let lines = text.split("\n").collect::<Vec<&str>>();
    let lines = lines.iter();

    let font_data = std::fs::read(font_path).unwrap();

    let face = Face::from_slice(&font_data, 0).unwrap();
    let font = Font::from_bytes(font_data.clone().into(), 0).unwrap();


    let mut lo = y;
    // let x = x;
    
    
    
    for (li, line) in lines.enumerate() {
        let mut x = x;
        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(&line);
        let glyph_buffer = rustybuzz::shape(&face, &[], buffer);

        for (i, glyph) in glyph_buffer.glyph_infos().iter().enumerate() {
            let glyph_pos = glyph_buffer.glyph_positions()[i];
            let glyph_id = glyph.glyph_id;

            // Get the glyph path using font-kit
            let mut path_builder = PathBuilder::new();

            pub struct MySink<'a> {
                path_builder: &'a mut PathBuilder,
            };

            impl<'a> OutlineSink for MySink<'a> {

                fn move_to(&mut self, to: Vector2F) {
                    self.path_builder.move_to(to.x(), to.y());
                }

                fn line_to(&mut self, to: Vector2F) {
                    self.path_builder.line_to(to.x(), to.y()); 
                }

                fn cubic_curve_to(&mut self, ctrl: pathfinder_geometry::line_segment::LineSegment2F, to: Vector2F) {
                    self.path_builder.cubic_to(ctrl.from().x(), ctrl.from().y(), ctrl.to().x(), ctrl.to().y(), to.x(), to.y());
                }
                fn quadratic_curve_to(&mut self, ctrl: Vector2F, to: Vector2F) {
                    self.path_builder.quad_to(ctrl.x(), ctrl.y(), to.x(), to.y());
                }

                fn close(&mut self) {
                    self.path_builder.close();
                }
            }

            let _ = font.outline(glyph_id, HintingOptions::None,  &mut MySink { path_builder: &mut path_builder });
            
            let path = path_builder.finish();


            let path = path.transform(&Transform2D::new(line_height/2048., 0.0, 0.0, -line_height/2048., x, y+lo-(line_height)));

            println!("{}, {}, {}, {}, {}, {}, {}", line_height/2048., 0.0, 0.0, -line_height/2048., x, y+lo-(line_height), glyph_pos.x_offset);

            ctx.fill(&path, &source, &DrawOptions::new());
            
            x += glyph_pos.x_advance as f32 / 64.;


        }

        // ctx.draw_text(
        //     &font,
        //     font_size,
        //     line,
        //     Point::new(x, y),
        //     source,
        //     &DrawOptions::new(),
        // );
        lo += line_height;
    }
}
