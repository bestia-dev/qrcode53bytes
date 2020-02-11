use qrcode53bytes::{Qr, SvgDodrioRenderer};

fn main() {
    let qr = Qr::new("https://bestia.dev/mem6/#p04.1234").unwrap();
    let s = SvgDodrioRenderer::new(200, 200).render(&qr);
    let file_name = "other/new.svg";
    let _x = std::fs::write(file_name, s);
    println!("File saved: {}", file_name);
}
