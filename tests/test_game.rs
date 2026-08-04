include!("common/defmt_mock.rs");

#[cfg(test)]
mod tests {
    use embedded_graphics::pixelcolor::Gray8;
    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
    use embedded_graphics_simulator::{
        OutputImage, OutputSettings, OutputSettingsBuilder, SimulatorDisplay,
    };
    use game_logic::two_four_eighteen::Game;
    use rand::rngs::SmallRng;
    use rp2350_sensor_hub::game::{self, player::GameResult};
    use rstest::{fixture, rstest};

    use rand::SeedableRng;

    const SCALE: u32 = 5;
    const SCREEN_WIDTH: u32 = SCALE * 128;
    const SCREEN_HEIGHT: u32 = SCALE * 64;

    type Display = SimulatorDisplay<BinaryColor>;

    fn get_expected_image(filename: &str, output_settings: &OutputSettings) -> OutputImage<Gray8> {
        let path_to_expected_roll_image = format!("resources/{}", filename);
        Display::load_png(path_to_expected_roll_image)
            .unwrap()
            .to_grayscale_output_image(output_settings)
    }

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

        let expected_roll_image =
            get_expected_image(format!("roll_{}.png", seed).as_str(), &output_settings);

        assert_eq!(generated_roll_image, expected_roll_image);
    }

    #[rstest]
    #[test_log::test]
    fn winning_game(#[from(init_display)] mut display: Display) {
        let output_settings = OutputSettingsBuilder::new().build();
        let seed = 488748144120125711;
        let mut game = Game::new(SmallRng::seed_from_u64(seed));
        let mut game_result = GameResult::Playing;
        let mut roll = 0;

        while game_result == GameResult::Playing {
            game_result = game::player::play_and_draw(&mut display, &mut game).unwrap();
            let generated_roll_image = display.to_grayscale_output_image(&output_settings);
            let expected_roll_image = get_expected_image(
                format!("winning_roll_{}.png", roll).as_str(),
                &output_settings,
            );

            assert_eq!(generated_roll_image, expected_roll_image);

            roll += 1;
        }
        assert_eq!(game_result, GameResult::Won);
    }

    //generated_roll_image
    //   .save_png(format!("resources/fish_roll_{}.png", roll))
    //  .unwrap();
    #[rstest]
    #[test_log::test]
    fn fish_game(#[from(init_display)] mut display: Display) {
        let output_settings = OutputSettingsBuilder::new().build();
        let seed = 6375483379391604375;
        let mut game = Game::new(SmallRng::seed_from_u64(seed));
        let mut game_result = GameResult::Playing;
        let mut roll = 0;

        while game_result == GameResult::Playing {
            game_result = game::player::play_and_draw(&mut display, &mut game).unwrap();
            let generated_roll_image = display.to_grayscale_output_image(&output_settings);
            let expected_roll_image =
                get_expected_image(format!("fish_roll_{}.png", roll).as_str(), &output_settings);

            assert_eq!(generated_roll_image, expected_roll_image);
            roll += 1;
        }
        assert_eq!(game_result, GameResult::Fish);
    }
}
