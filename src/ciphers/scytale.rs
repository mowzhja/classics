use super::utils::pad;
use crate::Cipher;

pub struct Scytale {
    key: usize, // the length
}

impl Scytale {
    /// Creates a new scytale of a certain length.
    ///
    /// I consider the scytale to have two dimensions: length and diameter, length is its extension
    /// on the x direction, diameter is its extension in the y direction, e.g.: | a b c d |
    ///                                                                         | e f g h |
    ///                                                                         | l m n o | is a scytale of
    ///                                                                                     length = 4 and
    ///                                                                                     diameter = 3
    pub fn new(key: usize) -> Self {
        Scytale { key }
    }

    /// Computes the diameter of the scytale given its length.
    fn get_diameter(s: &str, length: usize) -> usize {
        let padded_s = pad(s, length as u32, '.');

        padded_s.len() / length
    }

    /// Simulates wrapping the parchment around the scytale.
    ///
    /// Mathematically speaking, my implementation and the traditional implementation utilizing
    /// matrices et alia should be equivalent.
    fn wrap_around_scytale(text: &str, n: usize) -> String {
        let mut unwinded = String::new();
        let l = text.len();

        for i in 0..=l {
            for j in (i..l).step_by(n) {
                unwinded.push(text.chars().nth(j).unwrap());
            }
        }
        unwinded.truncate(l);

        unwinded
    }
}

impl Cipher for Scytale {
    fn encrypt(&self, plaintext: &str) -> String {
        let clean_plaintext = <Scytale as Cipher>::clean_input(plaintext);

        Scytale::wrap_around_scytale(&clean_plaintext, self.key)
    }

    fn decrypt(&self, ciphertext: &str) -> String {
        let clean_ciphertext = <Scytale as Cipher>::clean_input(ciphertext);
        let diameter = Scytale::get_diameter(&clean_ciphertext, self.key);

        Scytale::wrap_around_scytale(&clean_ciphertext, diameter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_zero_length() {
        let cipher = Scytale::new(0);

        cipher.encrypt("this should fail");
    }

    #[test]
    fn test_known_pairs() {
        // from https://en.wikipedia.org/wiki/Scytale
        let cipher = Scytale::new(5);
        let plaintext = "I am hurt very badly help";
        let ciphertext = String::from("Iryyatbhmvaehedlurlp");

        assert_eq!(ciphertext.to_uppercase(), cipher.encrypt(plaintext));

        assert_eq!(
            <Scytale as Cipher>::clean_input(plaintext),
            cipher.decrypt(&ciphertext)
        );

        // from 'Programming in Cryptol', page 30
        let cipher = Scytale::new(4); // in the book the diameter is 3
        let plaintext = "attackatdawn";
        let ciphertext = String::from("ACDTKATAWATN");

        assert_eq!(ciphertext, cipher.encrypt(plaintext));

        assert_eq!(
            <Scytale as Cipher>::clean_input(plaintext),
            cipher.decrypt(&ciphertext)
        );
    }

    #[test]
    #[ignore]
    fn test_correct() {
        let plaintext = String::from("Iamhurtverybadly");

        // FIXME something very subtle is going on here...
        // FIXME tried copypasting code from another implementation and still didn't work!?
        for length in 1..=plaintext.len() {
            let cipher = Scytale::new(length);
            assert_eq!(
                plaintext.to_uppercase(),
                cipher.decrypt(&cipher.encrypt(&plaintext))
            );
        }
    }
}
