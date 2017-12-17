#![feature(attr_literals)]
#![feature(custom_attribute)]

#[macro_use]
extern crate derive_accessor;

#[derive(Accessor)]
struct A {
    #[accessor_size = 1]
    a: u8,
}


#[cfg(test)]
mod tests {

    #[test]
    fn base() {
        #[derive(Accessor)]
        struct A {
            #[accessor_size = 1]
            a: u8,
        }

        assert_eq!(A::a, 1);
    }
}
