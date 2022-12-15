pub struct NibbleArray {
    size: usize,
    arr: Vec<u8>,
}

impl NibbleArray {
    pub fn new(size: usize) -> NibbleArray {
        let arr = vec![0; size / 2 + 1];
        NibbleArray { size, arr }
    }

    pub fn get(&self, index: usize) -> u8 {
        let i = index / 2;
        let j = index % 2;
        if j == 0 {
            self.arr[i] & 0x0F
        } else {
            self.arr[i] >> 4
        }
    }

    pub fn set(&mut self, index: usize, value: u8) {
        let i = index / 2;
        let j = index % 2;
        if j == 0 {
            self.arr[i] = (self.arr[i] & 0xF0) | (value & 0x0F);
        } else {
            self.arr[i] = (self.arr[i] & 0x0F) | ((value & 0x0F) << 4);
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.arr
    }

    pub fn inflate(&self, dest: &mut Vec<u8>) {
        dest.reserve(self.size);

        for i in 0..self.size {
            dest.push(self.get(i));
        }
    }

    pub fn reset(&mut self) {
        self.arr.fill(0);
    }
}
