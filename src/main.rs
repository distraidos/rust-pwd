use rand::{distr::Alphanumeric, prelude::*};

fn main() {
    let pass = generate_password(13);
    println!("Hola holita, contraseña: {}", pass);
}

fn generate_password(size: usize) -> String {
    let rng = rand::rng();
    rng.sample_iter(Alphanumeric)
        .take(size)
        .inspect(|value| println!("{}, ", value))
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::generate_password;

    #[test]
    fn test_generate_password() {
        let size: usize = 15;
        assert_eq!(generate_password(size).chars().count(), size)
    }
}
