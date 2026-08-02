include!("common/defmt_mock.rs");

#[cfg(test)]
mod tests {
    use core::convert::Infallible;
    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
    use game_logic::two_four_eighteen::Game;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;
    use rp2350_sensor_hub::game;
    use rstest::{fixture, rstest};

    const SCALE: u32 = 5;
    const SCREEN_WIDTH: u32 = SCALE * 128;
    const SCREEN_HEIGHT: u32 = SCALE * 64;

    type Display = SimulatorDisplay<BinaryColor>;

    #[fixture]
    fn init_display() -> Display {
        SimulatorDisplay::new(Size::new(SCREEN_WIDTH, SCREEN_HEIGHT))
    }

    #[rstest]
    #[test_log::test]
    fn play_and_draw(#[from(init_display)] mut display: Display) -> Result<(), Infallible> {
        let seed: u64 = 2056713228146178055;
        let mut game = Game::new(SmallRng::seed_from_u64(seed));
        let output_settings = OutputSettingsBuilder::new().build();

        game::player::play_and_draw(&mut display, &mut game).unwrap();
        let output_image = display.to_grayscale_output_image(&output_settings);

        //let expected =

        assert_eq!(output_image.size(), Size::new(SCREEN_WIDTH, SCREEN_HEIGHT));
        Ok(())
    }
}
