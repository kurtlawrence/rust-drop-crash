use libloading::*;
use std::collections::*;

type Bt = BTreeMap<String, String>;
type Hm = HashMap<String, String>;
type V = Vec<String>;

fn main() {
    println!("Hello, world!");

    let lib = Library::new("dep/target/debug/libdep.so").unwrap();

    let mut x = get_vec(&lib);
    println!("Got the vec");
    x.push("Hello, world".into());
    println!("Pushed onto it");
    drop(x);
    println!("That dropped fine");

    let x = get_vec(&lib);
    drop(x); 
    println!("Dropped empty vec fine");

    let mut x = get_hashmap(&lib);
    println!("Got the hash map");
    x.insert("Hello".into(), "World".into());
    println!("Inserted into it");
    drop(x);
    println!("That dropped fine");

    let x = get_hashmap(&lib);
    drop(x); 
    println!("Dropped empty hash map fine");

    let mut x = get_btreemap(&lib);
    println!("Got the btree map");
    x.insert("Hello".into(), "World".into());
    println!("Inserted into it");
    drop(x);
    println!("That dropped fine");

    let x = get_btreemap(&lib);
    drop(x); 
    println!("Dropped empty btree map fine");

    drop(lib); // hold onto lib until end
}

fn get_btreemap(lib: &Library) -> Bt {
    unsafe {
        let func: Symbol<unsafe extern fn() -> Bt> = lib.get(b"btreemap").unwrap();
        func()
    }
}

fn get_vec(lib: &Library) -> V {
    unsafe {
        let func: Symbol<unsafe extern fn() -> V> = lib.get(b"vec").unwrap();
        func()
    }
}

fn get_hashmap(lib: &Library) -> Hm {
    unsafe {
        let func: Symbol<unsafe extern fn() -> Hm> = lib.get(b"hashmap").unwrap();
        func()
    }
}
