use rand::Rng;

const SYMBOLS: [char; 40] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'w', 'x', 'y', 'z', '0', '1', '2', '3',
    '4', '5', '6', '7', '8', '9', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

pub struct ShortCodeGenerator;

impl ShortCodeGenerator {
    pub fn generate() -> String {
        let mut rand = rand::thread_rng();
        let mut code = String::with_capacity(8);
        for _ in 0..8 {
            let index = rand.gen_range(0..40);
            code.push(SYMBOLS[index]);
        }

       code
    }
}