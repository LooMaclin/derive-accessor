# Derive Accessor

# Usage

Derive struct:

```rust

#[derive(Accessor)]
struct A {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    #[explicit_size = 3]
    f: String,
}

```

It's generates impl block for you struct with `to_array` method what serialize you struct into raw bytes fixed-size array with byteorder bigendian,
and generates getter methods from raw slice for each field.

Generated code example:

```rust

impl A {

    pub fn to_array(&self) -> [u8; 18usize] {
        use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
        let mut result: [u8; 18usize] = [0; 18usize];
        result[0usize] = self.a;
        BigEndian::write_u16(&mut result[1usize..1usize + 2], self.b);
        BigEndian::write_u32(&mut result[3usize..3usize + 4], self.c);
        BigEndian::write_u64(&mut result[7usize..7usize + 8], self.d);
        result[15usize..15usize + 3usize].copy_from_slice(self.f.as_bytes());
        result
    }
    pub fn get_a(value: &[u8]) -> u8 {
        use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
        value[0usize]
    }
    pub fn get_b(value: &[u8]) -> u16 {
        use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
        BigEndian::read_u16(&value[1usize..1usize + 2])
    }
    pub fn get_c(value: &[u8]) -> u32 {
        use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
        BigEndian::read_u32(&value[3usize..3usize + 4])
    }
    pub fn get_d(value: &[u8]) -> u64 {
        use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
        BigEndian::read_u64(&value[7usize..7usize + 8])
    }
    pub fn get_f<'a>(value: &'a [u8]) -> &'a str {
        unsafe { ::std::str::from_utf8_unchecked(&value[15usize..15usize + 3usize]) }
    }
}

```