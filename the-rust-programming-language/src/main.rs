use crate::{
    chapter_07::front_of_house::eat_at_restaurant,
    chapter_09::errors::{file_results, read_username_from_file},
    chapter_10::{lifetimes::longest, traits::traits_example},
};

mod chapter_01;
mod chapter_02;
mod chapter_03;
mod chapter_04;
mod chapter_05;
mod chapter_06;
mod chapter_07;
mod chapter_08;
mod chapter_09;
mod chapter_10;
mod chapter_11;

fn main() {
    chapter_01::hello::hello();
    chapter_02::guessing_game::guess_the_number();
    chapter_03::variables::variables();
    chapter_03::variables::shadowing();
    println!(
        "Three hours have {} seconds",
        chapter_03::variables::THREE_HOURS_IN_SECONDS
    );
    chapter_03::data_types::interger_types();
    chapter_03::data_types::float_types();
    chapter_03::data_types::bool_types();
    chapter_03::data_types::char_types();
    chapter_03::data_types::componund_types();
    chapter_03::data_types::array_types();
    chapter_03::numeric_operations::numeric_operations();
    chapter_03::functions::another_function(5);
    chapter_03::functions::print_labeled_measurement(5, 'h');
    chapter_03::functions::statements_and_expressions();
    let x = chapter_03::functions::five();

    println!("The value of x is: {x}");

    let x = chapter_03::functions::plus_one(5);

    println!("The value of x is: {x}");

    chapter_03::control_flow::if_condition();
    chapter_03::control_flow::if_in_let();
    chapter_03::control_flow::loops_loop();
    chapter_03::control_flow::labeled_loops();
    chapter_03::control_flow::while_loops();
    chapter_03::control_flow::for_loops();

    let s = String::from("hello world");

    let word = chapter_04::slice::first_word(&s); // word will get the value 5

    println!("{word}");
    let user1 = chapter_05::structs::build_user(
        String::from("example@example.com"),
        String::from("username"),
    );
    let _user2 = chapter_05::structs::User {
        email: String::from("another@example.com"),
        ..user1
    };
    let black = chapter_05::structs::Color(0, 0, 0);
    let origin = chapter_05::structs::Point(0, 0, 0);

    let scale = 2;

    let rect1 = chapter_05::structs::Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("rect1 is {rect1:?}");
    dbg!(&rect1);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    eat_at_restaurant();
    // panic_please();
    file_results();
    read_username_from_file();
    traits_example();

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}
