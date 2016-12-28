use std::char;

#[derive(Debug)]
pub struct DotMatrix {
    x: usize,
    y: usize,
    matrix: Box<[bool]>
}

impl DotMatrix {
    pub fn new(x: usize, y: usize) -> DotMatrix {
        let vec = vec![false; x*y];
        DotMatrix {
            x: x,
            y: y,
            matrix: vec.into_boxed_slice()
        }
    }

    pub fn set(&mut self, x: usize, y: usize, val: bool) {
        assert!(x < self.x, "Given X value ({}) is larger than the matrix's! Max value allowed: {}", x, self.x-1);
        assert!(y < self.y, "Given Y value ({}) is larger than the matrix's! Max value allowed: {}", y, self.y-1);
        self.matrix[y*self.x + x] = val;
    }

    pub fn get(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.x, "Given X value ({}) is larger than the matrix's! Max value allowed: {}", x, self.x-1);
        assert!(y < self.y, "Given Y value ({}) is larger than the matrix's! Max value allowed: {}", y, self.y-1);
        self.matrix[y*self.x + x]
    }

    fn get_without_assert(&self, x: usize, y: usize) -> bool {
        if !(x < self.x) {
            return false;
        }
        if !(y < self.y) {
            return false;
        }
        self.matrix[y*self.x + x]
    }

    pub fn print(&self) {
        for y in 0..(self.y/4+1) {
            for x in 0..(self.x/2+1) {
                let mut character: u8 = 0;

                if self.get_without_assert(x*2, y*4) {
                    character |= 0b00000001;
                }
                if self.get_without_assert(x*2, y*4+1) {
                    character |= 0b00000010;
                }
                if self.get_without_assert(x*2, y*4+2) {
                    character |= 0b00000100;
                }
                if self.get_without_assert(x*2+1, y*4) {
                    character |= 0b00001000;
                }
                if self.get_without_assert(x*2+1, y*4+1) {
                    character |= 0b00010000;
                }
                if self.get_without_assert(x*2+1, y*4+2) {
                    character |= 0b00100000;
                }
                if self.get_without_assert(x*2, y*4+3) {
                    character |= 0b01000000;
                }
                if self.get_without_assert(x*2+1, y*4+3) {
                    character |= 0b10000000;
                }
                print!("{}", DotMatrix::translate_u8(character));
            }
            println!("");
        }
    }

    fn translate_u8(character: u8) -> char {
        /* Each braille character uses these bits for the dots:
        (bit 76543210) -->

        0 3
        1 4
        2 5
        6 7

        */

        char::from_u32(('⠀' as u32 + character as u32)).unwrap()
    }
}
