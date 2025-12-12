pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[link(name="foo", kind="static")]
unsafe extern "C" { 
    unsafe fn testcall(v: f32);
}

pub fn test_call(v: f32) {
    unsafe {
        testcall(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
