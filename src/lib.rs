#[no_mangle]
pub extern "C" fn fun1() {
    println!("fun1 was called");
}

#[no_mangle]
pub extern "C" fn fun4() {
    unsafe {
        fun2();
        fun3();
    }
}


extern "C" {
    fn fun2();
    fn fun3();
}