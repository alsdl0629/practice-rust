use benchmark::play_game;

fn main() {
    fn main() {
        let args: Vec<String> = std::env::args().collect();
        let i = args
            .get(1)
            .map(|s| s.parse::<u32>())
            .unwrap_or(Ok(15))
            .unwrap_or(15);
        play_game(i, true);
    }
}