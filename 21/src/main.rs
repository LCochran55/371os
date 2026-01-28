use std::rc::Rc;
fn main() {

    unsafe {
        //00000066, 00000105, 00000110, 00000107,00000108,00000101,
        //00000032,00000119,00000111,00000114,00000108,00000100,00000033
        //
        //
        //62696e6b6c6520776f726c64

        let values: &[i32] = &[0x6b6e6942, 0x7720656c, 0x6f646c72];

        //let bytes: &[u8] = std::mem::transmute::<&[i32],&[u8]>(values);
        let binkle: &str = std::mem::transmute::<&[u8], &str>(bytes::from);
        //println!("{:?}", binkle_bytes);
        println!("{}", binkle);
    }

}
