# qrcode53bytes

[//]: # (lmake_cargo_toml_to_md start)

**QR code for 53 bytes**  
 ***version: 1.0  date: 2021-01-13 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/qrcode53bytes)***  

[//]: # (lmake_cargo_toml_to_md end)

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready_for_use](https://img.shields.io/badge/ready_for_use-green)

 [![crates.io](https://img.shields.io/crates/v/qrcode53bytes.svg)](https://crates.io/crates/qrcode53bytes)
 [![Documentation](https://docs.rs/qrcode53bytes/badge.svg)](https://docs.rs/qrcode53bytes/)
 [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/qrcode53bytes.svg)](https://web.crev.dev/rust-reviews/crate/qrcode53bytes/)
 [![RustActions](https://github.com/bestia-dev/qrcode53bytes/workflows/rust/badge.svg)](https://github.com/bestia-dev/qrcode53bytes/)

 [![latest doc](https://img.shields.io/badge/latest_docs-GitHub-orange.svg)](https://bestia-dev.github.io/qrcode53bytes/qrcode53bytes/index.html)
 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/qrcode53bytes/blob/master/LICENSE)
 ![qrcode53bytes](https://bestia.dev/webpage_hit_counter/get_svg_image/455565280.svg)

[//]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1474-green.svg)](https://github.com/bestia-dev/qrcode53bytes/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-258-blue.svg)](https://github.com/bestia-dev/qrcode53bytes/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-86-purple.svg)](https://github.com/bestia-dev/qrcode53bytes/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-31-yellow.svg)](https://github.com/bestia-dev/qrcode53bytes/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/qrcode53bytes/)

[//]: # (lmake_lines_of_code end)

Hashtags: #rustlang #tutorial
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

I need to generate a simple QR code for url that has max 53 bytes. I want to do this in wasm.  
QR codes are pretty complicated. So specifying only one single use-case makes the code smaller. But it looses universality.  
Smaller code is good for wasm.  
The url I want to encode is like this:  
`https://bestia.dev/mem6/#p04.1234`  
There is a hash symbol here so I cannot use the `alphanumeric mode`.  
I must use the `byte mode`.  
There are 33 characters. It means there is some free space for future uses.  
The smallest QR code for that is:

- version 3
- 29x29 modules
- ECC Level L
- data bits 440
- 53 bytes
- ISO-8859-1

The code is written by treeman. I just removed all the variants I don't need.  

I use this code my wasm project <https://github.com/bestia-dev/mem6_game>.  

## make

List all the prepared commands and tasks with `$ cargo make`.  
<https://github.com/sagiegurari/cargo-make>

## References

<https://github.com/treeman/rqr>  
<https://www.thonky.com/qr-code-tutorial>  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## changelog

1.1.0 bitvec dependency was yanked. updated to 1.17.4, but later versions have breaking changes  

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
