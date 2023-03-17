extern "C" {
    fn _app_num();
}

pub fn get_app_num() -> usize {
    unsafe { (_app_num as usize as *const usize).read() }
}

pub fn get_app_data(id: usize) -> &'static [u8] {
    let app_num = get_app_num();
    assert!(id < app_num);
    let app_start = unsafe { (_app_num as usize as *const usize).add(id + 1) };
    unsafe {
        core::slice::from_raw_parts((*app_start) as *const u8, *(app_start.add(1)) - *app_start)
    }
}
