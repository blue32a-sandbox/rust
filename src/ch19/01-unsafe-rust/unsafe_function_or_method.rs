fn main() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // error: call to unsafe function is unsafe and requires unsafe function or block
    dangerous();
}
