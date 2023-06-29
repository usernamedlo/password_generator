use rand::Rng;
use std::io;

fn main() {
    loop {
        println!("Combien de caractères voulez-vous dans votre mot de passe ? ");
        let mut password_length = String::new();

        io::stdin().read_line(&mut password_length).expect("Echec de la lecture de la ligne");

        let password_length: i32 = match password_length.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer une valeur numérique valide.");
                continue;
            }
        };

        if password_length <= 0 {
            println!("La longueur du mot de passe doit être supérieure à zéro.");
            continue;
        }

        println!("Voulez-vous inclure des majuscules ? (Oui/Non) ");
        let mut include_uppercase = String::new();
        io::stdin().read_line(&mut include_uppercase).expect("Echec de la lecture de la ligne");
        let include_uppercase = include_uppercase.trim().eq_ignore_ascii_case("oui");

        println!("Voulez-vous inclure des caractères spéciaux ? (Oui/Non) ");
        let mut include_special_chars = String::new();
        io::stdin().read_line(&mut include_special_chars).expect("Echec de la lecture de la ligne");
        let include_special_chars = include_special_chars.trim().eq_ignore_ascii_case("oui");

        let password = generate_random_password(
            password_length,
            include_uppercase,
            include_special_chars
        );
        println!("{}", password);
        break;
    }
}

fn generate_random_password(
    length: i32,
    include_uppercase: bool,
    include_special_chars: bool
) -> String {
    let mut rng = rand::thread_rng();
    let mut characters = String::from("abcdefghijklmnopqrstuvwxyz0123456789");

    if include_uppercase {
        characters.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    if include_special_chars {
        characters.push_str("!@#$%^&*()");
    }

    let mut random_password = String::new();

    for _ in 0..length {
        let random_number = rng.gen_range(0..characters.len());
        random_password.push(characters.chars().nth(random_number).unwrap());
    }

    random_password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_len_generate_random_password() {
        let password = generate_random_password(8);
        assert_eq!(password.len(), 8);
    }

    #[test]
    fn test_generate_random_password() {
        let password = generate_random_password(8);
        assert_eq!(password.len(), 8);

        // Exemple de tests sur le contenu du mot de passe
        assert!(password.chars().any(|c| c.is_ascii_lowercase())); // Vérifie la présence d'au moins une lettre minuscule
        assert!(password.chars().any(|c| c.is_ascii_uppercase())); // Vérifie la présence d'au moins une lettre majuscule
        assert!(password.chars().any(|c| c.is_ascii_digit())); // Vérifie la présence d'au moins un chiffre
        // Ajoute d'autres tests selon les critères que tu souhaites vérifier
    }
}
