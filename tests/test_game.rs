include!("common/defmt_mock.rs");

#[cfg(test)]
mod tests {
    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
    use game_logic::two_four_eighteen::Game;
    use rand::rngs::SmallRng;
    use rp2350_sensor_hub::game;
    use rstest::{fixture, rstest};

    use rand::SeedableRng;

    const SCALE: u32 = 5;
    const SCREEN_WIDTH: u32 = SCALE * 128;
    const SCREEN_HEIGHT: u32 = SCALE * 64;

    type Display = SimulatorDisplay<BinaryColor>;

    #[fixture]
    fn init_display() -> Display {
        SimulatorDisplay::new(Size::new(SCREEN_WIDTH, SCREEN_HEIGHT))
    }

    #[rstest]
    #[case::seed_17035409315052165818(17035409315052165818)]
    #[case::seed_2056713228146178055(2056713228146178055)]
    #[test_log::test]
    fn play_and_draw(#[from(init_display)] mut display: Display, #[case] seed: u64) {
        let output_settings = OutputSettingsBuilder::new().build();
        let mut game = Game::new(SmallRng::seed_from_u64(seed));

        game::player::play_and_draw(&mut display, &mut game).unwrap();
        let generated_roll_image = display.to_grayscale_output_image(&output_settings);

        let path_to_expected_roll_image = format!("resources/roll_{}.png", seed);
        let expected_roll_image = Display::load_png(path_to_expected_roll_image)
            .unwrap()
            .to_grayscale_output_image(&output_settings);

        assert_eq!(generated_roll_image, expected_roll_image);
    }
}
