use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {

    println!("Please enter your db path: ");
    println!("");
    let db_path = get_user_input();

    let db = JiraDatabase::new(db_path);
    let rc_db = Rc::new(db);
    let mut navigator = Navigator::new(Rc::clone(&rc_db));

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        let current_page = navigator.get_current_page();
        match current_page {
            None => break,
            Some(page) => {
                // 2. render page
                if let Err(error) = page.draw_page() {
                    println!("Error rendering page: {}", error);
                    println!("Press Enter to continue...");
                    wait_for_key_press();
                    continue;
                }

                // 3. get user input
                let user_input = get_user_input();

                // 4. pass input to page's input handler
                let current_action = page.handle_input(&user_input);

                match current_action {
                    Err(error) => {
                        println!("Error handling input: {}", error);
                        println!("Press Enter to continue...");
                        wait_for_key_press();
                        continue;
                    }
                    Ok(action) => {
                        // 5. if the page's input handler returns an action let the navigator process the action

                        match action {
                            None => {
                                println!("WRONG ACTION");
                                println!("Press Enter to continue...");
                                wait_for_key_press();
                                continue;
                            }
                            Some(a) => {
                                if let Err(error) = navigator.handle_action(a) {
                                    println!("Error handling action: {}", error);
                                    println!("Press Enter to continue...");
                                    wait_for_key_press();
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
