include!("common/defmt_mock.rs");

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use game_logic::player;

    use core::convert::Infallible;
    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
    use game_logic::two_four_eighteen::Game;
    use rp2350_sensor_hub::game;

    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    const SCALE: u32 = 5;
    const SCREEN_WIDTH: u32 = SCALE * 128;
    const SCREEN_HEIGHT: u32 = SCALE * 64;

    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    type Display = SimulatorDisplay<BinaryColor>;

    fn draw_in_window(display: &Display) -> Result<(), Infallible> {
        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        Window::new("a die", &output_settings).show_static(&display);

        Ok(())
    }

    #[fixture]
    fn init_display() -> Display {
        SimulatorDisplay::new(Size::new(SCREEN_WIDTH, SCREEN_HEIGHT))
    }

    #[fixture]
    fn gen_small_rng() -> SmallRng {
        let seed: u64 = rand::random();
        SmallRng::seed_from_u64(seed)
    }

    #[rstest]
    #[test_log::test]
    fn play_and_draw(
        #[from(init_display)] mut display: Display,
        #[from(gen_small_rng)] small_rng: SmallRng,
    ) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();
        let mut game = Game::new(small_rng);

        game::player::play_and_draw(&mut display, &mut game).unwrap();

        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    #[ignore]
    fn roll_die(
        #[from(init_display)] mut display: Display,
        #[from(gen_small_rng)] small_rng: SmallRng,
    ) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();
        player::roll_die(&mut display, small_rng)?;

        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    #[ignore]
    fn roll_two_dice(
        #[from(init_display)] mut display: Display,
        #[from(gen_small_rng)] small_rng: SmallRng,
    ) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();
        player::roll_two_dice(&mut display, small_rng)?;

        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    #[ignore]
    fn roll_three_dice(
        #[from(init_display)] mut display: Display,
        #[from(gen_small_rng)] small_rng: SmallRng,
    ) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();
        player::roll_three_dice(&mut display, small_rng)?;

        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    #[ignore]
    fn roll_four_dice(
        #[from(init_display)] mut display: Display,
        #[from(gen_small_rng)] small_rng: SmallRng,
    ) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();
        player::roll_four_dice(&mut display, small_rng)?;

        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    #[ignore]
    fn roll_five_dice(
        #[from(init_display)] mut display: Display,
        #[from(gen_small_rng)] small_rng: SmallRng,
    ) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();
        player::roll_five_dice(&mut display, small_rng)?;

        draw_in_window(&display)
    }

    #[rstest]
    #[test_log::test]
    #[ignore]
    fn roll_one_to_five_number_of_dice(
        #[from(init_display)] mut display: Display,
        #[from(gen_small_rng)] small_rng: SmallRng,
    ) -> Result<(), Infallible> {
        let _guard = TEST_MUTEX.lock().unwrap();
        player::roll_one_to_five_number_of_dice(&mut display, small_rng)?;

        draw_in_window(&display)
    }
}
