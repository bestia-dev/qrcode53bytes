use qrcode53bytes::*;

fn main() {
    let qr = Qr::new("https://bestia.dev/mem6/#p04.1234").unwrap();
    let s = StringRenderer::new().render(&qr);
    println!("{}", s);
}
