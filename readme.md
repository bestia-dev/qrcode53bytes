# qrcode53bytes

[comment]: # (lmake_lines_of_code start)

[comment]: # (lmake_lines_of_code end)

*Things are changing fast. 2020-02-17 LucianoBestia ver.1.0.0.*  

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

I use this code my wasm project <https://github.com/LucianoBestia/mem6_game>.  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## Examples

     cargo run --example svg

## References

<https://github.com/treeman/rqr>  
<https://www.thonky.com/qr-code-tutorial>  

## changelog

1.1.0 bitvec dependency was yanked. updated to 1.17.4, but later versions have breaking changes  