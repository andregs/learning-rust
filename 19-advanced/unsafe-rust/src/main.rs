use std::slice;

fn main() {
    unsafe {
        raw_pointers();
    }

    // safe code calling safe function that hides unsafe behavior
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3); // native fn
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    let (a, b) = split_at_mut(r, 3); // our fn
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        external_code();
    }

    global_vars();
}

unsafe fn raw_pointers() {
    let mut num = 5;
    // this doesn't compile, unless we use raw pointers (we might have data race)
    // let foo = &mut num;
    // let bar = &num;
    // println!("{} {}", foo, bar);

    // immutable raw pointer
    let r1 = &num as *const i32;

    // mutable raw pointer
    let r2 = &mut num as *mut i32;

    println!(
        "Content of r1 and r2 are {:?} = {:?} and {:?} = {:?}",
        r1, *r1, r2, *r2
    );

    // this compiles but probably results in seg fault
    // let address = 0x012345usize;
    // let r = address as *const i32; // this raw pointer probably points to invalid memory location
    // println!("r {:?} = {:?}", r, *r);
}

// 1st attemp fails: we cannot borrow "slice" as mutable twice
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
// 
//     assert!(mid <= len);
// 
//     let left = &mut slice[..mid];
//     let right = &mut slice[mid..];
// 
//     (left, right)
// }

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

unsafe fn external_code() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    // we are really calling C code from Rust
    println!("Absolute value of -4 according to C: {}", abs(-4));
}

#[no_mangle] // keep fn name intact after compilation
pub extern "C" fn call_from_c() {
    println!("Now C is calling our Rust code");
    // now we can compile this fn to a shared lib and link from C
}

// static variables are globals
static HELLO_WORLD: &str = "Hello, World!";
static mut COUNTER: u32 = 0;

fn global_vars() {
    // immutable static is safe
    println!("just saying: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        // mutable static is unsafe (they can be shared among threads i.e. data race)
        println!("Count is {}", COUNTER);
    }
}

fn add_to_count(v: u32) {
    unsafe {
        COUNTER += v;
    }
}

// we can also create unsafe traits
unsafe trait Foo {}
unsafe impl Foo for i32 {}
