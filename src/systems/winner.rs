mod pong {
    use amethyst::ecs::prelude::*;

    pub struct Ball {
        pub radius: f32,
        pub velocity: [f32; 2],
    }
    impl Component for Ball {
        type Storage = DenseVecStorage<Self>;
    }

    #[derive(Default)]
    pub struct ScoreBoard {
        pub score_left: i32,
        pub score_right: i32,
    }

    pub struct ScoreText {
        pub p1_score: Entity,
        pub p2_score: Entity,
    }

    pub const ARENA_WIDTH: f32 = 100.0;
    pub const ARENA_HEIGHT: f32 = 100.0;
}

use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    // --snip--
    ecs::{Join, ReadExpect, System, SystemData, World, Write, WriteStorage},
    ui::UiText,
};

use crate::pong::{Ball, ScoreBoard, ScoreText, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(
        &mut self,
        (mut balls, mut locals, mut ui_text, mut scores, score_text): Self::SystemData,
    ) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                scores.score_right = (scores.score_right + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                scores.score_left = (scores.score_left + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0];
                transform.set_translation_x(ARENA_WIDTH / 2.0);
                transform.set_translation_y(ARENA_HEIGHT / 2.0);

                println!(
                    "Score: | {:^3} | {:^3} |",
                    scores.score_left, scores.score_right
                );
            }
        }
    }
}
