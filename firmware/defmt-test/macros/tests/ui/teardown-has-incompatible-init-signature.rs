fn main() {}

#[defmt_test_macros::tests]
mod tests {
    #[init]
    fn init() -> u32 {
        0_u32
    }

    #[teardown]
    fn say(value: &mut u16) {
        assert!(true);
    }
}
