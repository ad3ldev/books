mod chapter_1;
mod chapter_2;
mod chapter_3;

fn main() {
    chapter_1::hello::hello();
    chapter_2::guessing_game::guess_the_number();
    chapter_3::variables::variables();
    chapter_3::variables::shadowing();
    println!(
        "Three hours have {} seconds",
        chapter_3::variables::THREE_HOURS_IN_SECONDS
    );
    chapter_3::data_types::interger_types();
    chapter_3::data_types::float_types();
    chapter_3::data_types::bool_types();
    chapter_3::data_types::char_types();
    chapter_3::data_types::componund_types();
    chapter_3::data_types::array_types();
    chapter_3::numeric_operations::numeric_operations();
}
