extern {
    fn onig_init() -> u32;
}

fn test_things() {
    let foo = unsafe {
        onig_init()
    };
    println!("Result: {}", foo);
}

#[cfg(test)]
mod test_lib {

    #[test]
    fn it_works() {
        assert!(true);
        super::test_things();
    }
}
