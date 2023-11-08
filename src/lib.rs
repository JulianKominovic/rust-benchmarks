pub mod buffer_first_then_stdout;
pub mod buffer_reserve_then_stdout;
pub mod directly_stdout;
pub mod directly_stdout_manual_std_lock;
pub mod parallel_buffer_first_then_stdout;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directly_stdout() {
        directly_stdout::directly_stdout();
    }

    #[test]
    fn directly_stdout_manual_std_lock() {
        directly_stdout_manual_std_lock::directly_stdout_manual_stdout_lock();
    }

    #[test]
    fn buffer_first_then_stdout() {
        buffer_first_then_stdout::buffer_first_then_stdout();
    }

    #[test]
    fn buffer_reserve_then_stdout() {
        buffer_reserve_then_stdout::buffer_reserve_then_stdout();
    }
}
