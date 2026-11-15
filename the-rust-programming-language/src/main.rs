mod chapter_1;
mod chapter_2;
mod chapter_3;
mod chapter_4;

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
    chapter_3::functions::another_function(5);
    chapter_3::functions::print_labeled_measurement(5, 'h');
    chapter_3::functions::statements_and_expressions();
    let x = chapter_3::functions::five();

    println!("The value of x is: {x}");

    let x = chapter_3::functions::plus_one(5);

    println!("The value of x is: {x}");

    chapter_3::control_flow::if_condition();
    chapter_3::control_flow::if_in_let();
    chapter_3::control_flow::loops_loop();
    chapter_3::control_flow::labeled_loops();
    chapter_3::control_flow::while_loops();
    chapter_3::control_flow::for_loops();

    let s = String::from("hello world");

    let word = chapter_4::slice::first_word(&s); // word will get the value 5

    println!("{word}")
}
