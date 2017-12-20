#![feature(attr_literals)]
#![feature(custom_attribute)]

#[macro_use]
extern crate derive_accessor;
extern crate byteorder;

#[derive(Accessor)]
struct A {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    #[explicit_size = 3]
    f: String,
}

fn base() {
    let test_a = A {
        a: 0,
        b: 1,
        c: 2,
        d: 3,
        f: "abc".to_string()
    };
}


#[cfg(test)]
mod tests {

    #[derive(Accessor)]
    struct A {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
        #[explicit_size = 3]
        f: String,
    }

    #[test]
    fn base() {
        let test_a = A {
            a: 0,
            b: 1,
            c: 2,
            d: 3,
            f: "abc".to_string()
        };
        let resulting_array = test_a.to_array();
        println!("resulting array: {:?}", resulting_array);
        assert_eq!(resulting_array, [0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 97, 98, 99]);
        assert_eq!(0, A::get_a(&resulting_array));
        assert_eq!(1, A::get_b(&resulting_array));
        assert_eq!(2, A::get_c(&resulting_array));
        assert_eq!(3, A::get_d(&resulting_array));
        assert_eq!("abc", A::get_f(&resulting_array));
    }
}
