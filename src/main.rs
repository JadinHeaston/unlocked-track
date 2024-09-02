use clap::Parser;
use lazy_static::lazy_static;
use lettre::{message::header::ContentType, Message, Transport};
use lock::{lock, FailureReason};
use native_dialog::{MessageDialog, MessageType};
use rpassword::read_password;
use std::io::Write;
use std::ptr;
use whoami::fallible;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{ShowWindow, SW_HIDE};

//CONSTANTS
const PIN_LIST: [i32; 1] = [12345];
lazy_static! {
    static ref EMAIL_SMTP_HOST: String = String::from("smtp.example.com");
    static ref EMAIL_SENDER: String = String::from("unlocked-track <unlocked-track@example.com>");
    static ref EMAIL_DESTINATION: String =
        String::from("Cybersecurity <cybersecurity@example.com>");
    static ref EMAIL_SUBJECT: String = String::from("unlocked-track");
}

/// Oops! You left your machine unlocked!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Authentication PIN
    #[arg(short, long, default_value_t = 0)]
    pin: i32,
}
fn main() {
    let args = Args::parse();
    let pin: i32;

    if !PIN_LIST.contains(&args.pin) {
        println!("Enter PIN: ");
        std::io::stdout().flush().unwrap();
        let user_input = read_password().unwrap();

        if PIN_LIST.contains(&user_input.trim().parse::<i32>().unwrap()) {
            pin = user_input.trim().parse::<i32>().unwrap();
        } else {
            println!("Invalid PIN!");
            std::process::exit(1);
        }
    } else {
        pin = args.pin.clone();
    }

    hide_console_window();

    //Sending email.
    send_email(String::from(
        "
Reported By PIN: "
            .to_owned()
            + &pin.to_string()
            + "
realname: " + &whoami::realname().to_string()
            + "
username: " + &whoami::username().to_string()
            + "
devicename: "
            + &whoami::devicename().to_string()
            + "
hostname: " + &fallible::hostname().unwrap()
            + "
Local IP Address: "
            + &local_ip_address::local_ip().unwrap().to_string()
            + "
desktop_env: "
            + &whoami::desktop_env().to_string()
            + "
arch: " + &whoami::arch().to_string()
            + "
distro: " + &whoami::distro().to_string()
            + "
lang: " + &whoami::lang().collect::<String>().to_string()
            + "
platform: " + &whoami::platform().to_string()
            + "
",
    ));

    //Machine
    println!("realname: {:#?}", whoami::realname());
    println!("username: {:#?}", whoami::username());
    println!("devicename: {:#?}", whoami::devicename());
    println!("hostname: {:#?}", fallible::hostname());
    println!("desktop_env: {:#?}", whoami::desktop_env());
    println!("arch: {:#?}", whoami::arch());
    println!("distro: {:#?}", whoami::distro());
    println!("lang: {:#?}", whoami::lang().collect::<Vec<String>>());
    println!("platform: {:#?}", whoami::platform());

    //Networking
    println!(
        "Local IP Address: {:?}",
        local_ip_address::local_ip().unwrap()
    );

    //Locking machine
    println!("Locking machine.");
    let lock_result = lock();

    MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("ATTENTION!")
        .set_text(("Your computer was found unlocked by Cybersecurity.\nRemember to secure your workstation when you step away to protect sensitive information and prevent unauthorized access.\nPress WIN + L to lock your machine.\nDirect any questions towards ".to_string() + &EMAIL_DESTINATION + ".").as_str())
        .show_alert()
        .unwrap();

    match lock_result {
        Err(FailureReason::CannotExecute) => {
            panic!("Failed to execute lock command!")
        }
        Err(FailureReason::LinuxCommandNotFound) => {
            panic!(
                "No applicable command found. Please consider installing \
                    xdg-screensaver, gnome-screensaver, or dm-tool, and try again."
            );
        }
        Ok(()) => (),
    }

    // Finished!
    println!("Finished!");
}

fn send_email(message: String) {
    let email = Message::builder()
        .from(EMAIL_SENDER.parse().unwrap())
        .to(EMAIL_DESTINATION.parse().unwrap())
        .subject(EMAIL_SUBJECT.to_string())
        .header(ContentType::TEXT_PLAIN)
        .body(message)
        .unwrap();

    // Open a remote connection to SMTP
    let mailer = lettre::SmtpTransport::relay(&EMAIL_SMTP_HOST)
        .unwrap()
        .port(25)
        .tls(lettre::transport::smtp::client::Tls::None)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

fn hide_console_window() {
    let window = unsafe { GetConsoleWindow() };
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}
