use qrcode53bytes::{Qr, SvgRenderer};

fn main() {
    let qr = Qr::new("https://bestia.dev/mem6/#p04.1234").unwrap();
    let s = SvgRenderer::new()
        .light_module(Color::new(229, 189, 227))
        .dark_module(Color::new(119, 0, 0))
        .dimensions(200, 200)
        .render(&qr);
    let file_name = "other/new.svg";
    let _x = std::fs::write(file_name, s);
    println!("File saved: {}", file_name);
}
