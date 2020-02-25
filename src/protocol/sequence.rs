pub struct Sequence {
    pub binary_sequence: Vec<u8>,
    pub length: usize
}

impl Sequence {
    pub fn new() -> Sequence {
        Sequence {
            binary_sequence: Vec::new(),
            length: 0
        }
    }

    /**
     * Push 1 byte at the end of the sequence (u8)
     */
    pub fn push_u8(&mut self, n: u8) {
        self.binary_sequence.push(n);
        self.length += 1;
    }

    /**
     * Push 2 bytes at the end of the sequence (u16)
     */
    pub fn push_u16(&mut self, n: u16) {
        let byte_1: u8 = (n >> 8) as u8;
        let byte_2: u8 = n as u8;
        self.push_u8(byte_1);
        self.push_u8(byte_2);
    }

    /**
     * Insert 4 bytes (u32) starting at a given index of the sequence
     */
    pub fn insert_u32(&mut self, n: u32, start: usize) {
        let bytes: [u8; 4] = n.to_be_bytes();

        // TODO: maybe insert(index: usize, sequence: &Vec<T>) ?
        self.binary_sequence.insert(start, bytes[0]);
        self.binary_sequence.insert(start + 1, bytes[1]);
        self.binary_sequence.insert(start + 2, bytes[2]);
        self.binary_sequence.insert(start + 3, bytes[3]);

        self.length += 4;
    }

    /**
     * Push char code at the end of the sequence
     */
    pub fn push_char(&mut self, c: char) {
        self.binary_sequence.push(c as u8);
        self.length += 1;
    }

    /**
     * Read a u32 from sequence starting at index
     */
    pub fn read_u32(&mut self, index: usize) -> u32 {
        let bytes: &[u8] = &self.binary_sequence[index..index  + 5];
        ((bytes[0] as u32) << 24) +
        ((bytes[1] as u32) << 16) +
        ((bytes[2] as u32) <<  8) +
        ((bytes[3] as u32) <<  0)
    }

    /**
     * Read a u16 from sequence starting at index
     */
    pub fn read_u16(&mut self, index: usize) -> u16 {
        let bytes: &[u8] = &self.binary_sequence[index..index + 3];
        (bytes[0] as u16) << 8 + (bytes[1] as u16) << 0
    }

    /**
     * Read a u8 from sequence starting at index
     */
    pub fn read_u8(&mut self, index: usize) -> u8 {
        *self.binary_sequence.get(index).unwrap()
    }

    /**
     * Return a mutable reference to internal binary sequence
     */
    pub fn as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.binary_sequence
    }

    /**
     * Return the sequence as a reference
     */
    pub fn as_ref(&mut self) -> &[u8] {
        &self.binary_sequence[..]
    }
}