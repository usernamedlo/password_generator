use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let mut random_password = String::new();
    // let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";

    loop {
        println!("Combien de caractères voulez-vous dans votre mot de passe ? ");
        let mut password_length = String::new();

        io::stdin().read_line(&mut password_length).expect("Echec de la lecture de la ligne");

        let password_length: i32 = match password_length.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        for _ in 0..password_length {
            let random_char = (rng.gen_range(33..127) as u8 as char).to_string();
            random_password.push_str(&random_char);
        }
        println!("{}", random_password);
        break;
    }


}
