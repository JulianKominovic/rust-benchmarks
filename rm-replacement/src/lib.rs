pub mod rm_parallel;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directly_stdout() {
        rm_parallel::rm_parallel();
    }
}
