#[unsafe(no_mangle)]
extern "C" fn _aullrem(dividend: u64, divisor: u64) -> u64 {
    if divisor == 0 {
        return 0;
    }
    dividend % divisor
}

#[unsafe(no_mangle)]
extern "C" fn _aulldiv(dividend: u64, divisor: u64) -> u64 {
    dividend / divisor
}
