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

Generates:
1) `to_array` function to serialize you struct to array with Byteorder::BigEndian;
2) new struct with one field (inner: &'a [u8]) and impl block with getter methods from `inner` reference for each field;

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
}
pub struct ARef<'a> {
    inner: &'a [u8],
}
impl<'a> ARef<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        ARef { inner: slice }
    }
    pub fn get_a(&self) -> u8 {
        use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
        self.inner[0usize]
    }
    pub fn get_b(&self) -> u16 {
        use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
        BigEndian::read_u16(&self.inner[1usize..1usize + 2])
    }
    pub fn get_c(&self) -> u32 {
        use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
        BigEndian::read_u32(&self.inner[3usize..3usize + 4])
    }
    pub fn get_d(&self) -> u64 {
        use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
        BigEndian::read_u64(&self.inner[7usize..7usize + 8])
    }
    pub fn get_f(&self) -> &str {
        unsafe { ::std::str::from_utf8_unchecked(&self.inner[15usize..15usize + 3usize]) }
    }
}


```