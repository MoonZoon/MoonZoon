use zoon::*;

mod cmp;

blocks!{
    append_blocks!{
        cmp,
    }

    #[s_var]
    fn column_count() -> SVar<i32> {
        3
    }

    #[s_var]
    fn row_count() -> SVar<i32> {
        2
    }

    #[cache]
    fn counter_count() -> Cache<i32> {
        column_count().inner() * row_count().inner()
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
