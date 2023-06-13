use std::num::Wrapping;

fn main() {
    /*
     * integer
     * To optimize(economize) memory,
     * It's recommmanded to use correct int range
     * Not put a type too big or small(overflow)
     */
    const DECIMAL: u32 = 98_222; //98222, to use u32(0 - 4294967295)
    println!("DECIMAL:\t{DECIMAL}");
    const HEX: u8 = 0xff; //255, able to use u8(0 - 255)
    println!("HEX:\t\t{HEX}");
    const OCTAL: u8 = 0o77; //63
    println!("OCTAL:\t\t{OCTAL}");
    const BINARY: u8 = 0b1111_0000; //240
    println!("BINARY:\t\t{BINARY}");
    /*
     * `const` is more for hard coding data
     * In those case, `let` is better but I used `const` to test
     */
    let hex32: u32 = HEX.into(); // convert from const to let
    let wrapped: Wrapping<u32> = Wrapping(hex32); // Wrapping to avoid overflow
    println!("wrapped:\t{wrapped}");
}
