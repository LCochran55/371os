fn main() {

    let s = 4;

    let mut  array: [u8; 0x80] = [0u8;0x80];

    array[0] = 0xff;
    array[1] = 0xff;

    for row in 0..16 {
        let mut count = s;
        let byte = array[row];
        println!("row {:?}", row);

        for col in 0..8 {
            println!("col {:?}", col);
            if (byte & (1 << col)) == 0 {
                count -= 1;
            } else {
                count = s;
            }
            println!("count {:?}", count);

            if count == 0 {
                let cur_bit = (8 * row) + col;
                println!("cur_bit {:?}", cur_bit);

                let first_bit = cur_bit - s + 1;
                println!("first_bit {:?}", first_bit);

                for bit in first_bit..(first_bit + s) {
                    array[bit / 8] |= 1 << (bit % 8);
                    println!("bit {:?}", bit);
                    println!("array {:?}", array);
                }

                return
            }
        }
    }
} 
