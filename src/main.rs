fn main() {
    let conninfo = std::ffi::CString::new("").unwrap();
    let conn = unsafe { pq_sys::PQconnectdb(conninfo.as_ptr()) };
    println!("{conn:?}");
}
