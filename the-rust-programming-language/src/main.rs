mod chapter_1;
mod chapter_2;
mod chapter_3;
mod chapter_4;
mod chapter_5;
mod chapter_6;

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

    println!("{word}");
    let user1 = chapter_5::structs::build_user(
        String::from("example@example.com"),
        String::from("username"),
    );
    let _user2 = chapter_5::structs::User {
        email: String::from("another@example.com"),
        ..user1
    };
    let black = chapter_5::structs::Color(0, 0, 0);
    let origin = chapter_5::structs::Point(0, 0, 0);

    let scale = 2;

    let rect1 = chapter_5::structs::Rectangle {
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
}
