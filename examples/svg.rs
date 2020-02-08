use qrcode53bytes::{Color, Qr, SvgRenderer};

fn main() {
    let qr = Qr::new("https://bestia.dev/mem6/#p04.1234").unwrap();
    let s = SvgRenderer::new()
        .light_module(Color::new(229, 189, 227))
        .dark_module(Color::new(119, 0, 0))
        .dimensions(200, 200)
        .render(&qr);
    let x = std::fs::write("other/new.svg", s);
    //println!("{}", s);
}
