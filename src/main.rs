mod subcode;

use subcode::SubcodeSymbol;

fn main() {
    let bs = [
        0x00, 0x40, 0x80, 0xC0, 0x68, 0xFF, 0x03, 0x08, 0x45, 0x48, 0x5C, 0x60, 0xD3,
    ];
    for b in bs.iter() {
        let scs = SubcodeSymbol(*b);
        println!("{:#04x} is {:?}", b, scs);
    }
}
