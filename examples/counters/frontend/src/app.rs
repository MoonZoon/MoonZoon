use zoon::*;

mod cmp;

blocks!{
    // append_blocks![
    //     cmp,
    // ]

    pub fn __append_blocks(blocks: __Blocks) -> __Blocks {
        cmp::__blocks(blocks)
    }

    #[s_var]
    fn column_count() -> SVar<i32> {
        3
    }

    #[s_var]
    fn row_count() -> SVar<i32> {
        2
    }

    #[s_var]
    fn counter_count() -> SVar<i32> {
        6
    }

    #[update]
    fn set_column_count(count: i32) {
        column_count().set(count);
    }

    #[update]
    fn set_row_count(count: i32) {
        row_count().set(count);
    }

}
