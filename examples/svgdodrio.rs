use qrcode53bytes::{Qr, SvgDodrioRenderer};

fn main() {
    let qr = Qr::new("https://bestia.dev/mem6/#p04.1234").unwrap();
    let s = SvgDodrioRenderer::new(200, 200).render(&qr);
    let file_name = "other/svgdodrio.svg";
    let _x = std::fs::write(file_name, s);
    println!("File saved: {}", file_name);
    println!("If you want to open it in the browser, vou must add the svg and rect elements manually in the file.{}", "");
}
