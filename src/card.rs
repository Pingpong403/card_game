pub mod card {
    use rand::Rng;

    #[derive(PartialEq)]
    pub enum Suits {
        Hearts,
        Diamonds,
        Spades,
        Clubs,
        None,
    }

    pub struct Card {
        pub suit: Suits,
        pub value: u32,
    }

    impl Card {
        pub fn generate(&mut self) -> &mut Card{
            let i = rand::thread_rng().gen_range(0..4);
            self.suit = match i {
                i32::MIN..=-1_i32 => Suits::None,
                0 => Suits::Hearts,
                1 => Suits::Diamonds,
                2 => Suits::Spades,
                3 => Suits::Clubs,
                4_i32..=i32::MAX => Suits::None,
            };
            let val = rand::thread_rng().gen_range(1..=13);
            self.value = val;
            self
        }

        pub fn display(&self) {
            let mut display_value = String::new();
            display_value = match self.value {
                0 => "null".to_string(),
                1..=10 => self.value.to_string(),
                11 => "Jack".to_string(),
                12 => "Queen".to_string(),
                13 => "King".to_string(),
                14_u32..=u32::MAX => "null".to_string(),
            };
            let display_suit = match self.suit {
                Suits::Hearts => "Hearts",
                Suits::Diamonds => "Diamonds",
                Suits::Spades => "Spades",
                Suits::Clubs => "Clubs",
                Suits::None => "no suit",
            };
            println!("{display_value} of {display_suit}");
        }

        pub fn compare(&self, &guess: &u32) -> bool {
            self.suit == match guess {
                0 => Suits::None,
                1 => Suits::Hearts,
                2 => Suits::Diamonds,
                3 => Suits::Spades,
                4 => Suits::Clubs,
                5_u32..=u32::MAX => Suits::None,
            }
        }
    }
}