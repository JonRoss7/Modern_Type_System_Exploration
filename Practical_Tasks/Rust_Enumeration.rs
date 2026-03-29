enum Message {
    Quit,
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Action: Quitting the program."),
        Message::Write(text) => println!("Action: Writing message -> {}", text),
    }
}

fn main() {
    let msg_write = Message::Write(String::from("System initialized."));
    let msg_quit = Message::Quit;

    process_message(msg_write);
    process_message(msg_quit);
}
