use raqote::*;
use raqote_utils::*;

pub fn main() {
    let mut dt = DrawTarget::new(512, 512);

    let font_path = "/usr/share/fonts/FiraCodeNerdFont-Retina.ttf";

    let circle = build_circle(100.0, 256.0, 256.0);

    let logo1 = create_path_from_string("M105 57.0273V453.751H252.659C448.259 461.723 428.124 276.022 352.856 253.513V243.197C424.768 204.274 423.809 54.6826 252.659 57.0273H105Z");

    dt.fill(
        &circle,
        &Source::Solid(SolidSource {
            r: 0x00,
            g: 0x00,
            b: 0x00,
            a: 0xFF,
        }),
        &DrawOptions::new(),
    );

    dt.fill(
        &logo1,
        &Source::Solid(SolidSource {
            r: 0x81,
            g: 0x0E,
            b: 0x68,
            a: 0xFF,
        }),
        &DrawOptions::new(),
    );

    create_text_ligatures(
        "Hello, World\nline2\n==>\n#[",
        50.,
        50.,
        &font_path,
        20.,
        &mut dt,
        &Source::Solid(SolidSource {
            r: 0xff,
            g: 0xff,
            b: 0xff,
            a: 0xFF,
        }),
    );

    let _ = dt.write_png("png.png");
}
