use bevy::prelude::*;

pub mod prelude {
    pub use crate::*;
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(animate_sprites.system());
    }
}

pub type AnimationFrameRange = (usize, usize);

pub struct AnimationState {
    pub animations: Vec<AnimationFrameRange>,
    pub current_animation: usize,
    pub current_idx: usize,
    pub is_playing: bool,
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState {
            animations: Vec::default(),
            current_animation: 0,
            current_idx: 0,
            is_playing: false,
        }
    }
}

impl AnimationState {
    // sets the animation on an animation state and moves to the first frame
    pub fn set_animation(&mut self, animation_idx: usize) -> bool {
        if animation_idx >= self.animations.len() {
            println!(
                "Unknown animation {}, max idx {}",
                animation_idx,
                self.animations.len()
            );
            return false;
        }

        self.current_animation = animation_idx;
        self.current_idx = self.animations[animation_idx].0;
        true
    }

    // increments the animation frame
    pub fn incr(&mut self) {
        if self.current_idx == self.animations[self.current_animation].1 {
            self.current_idx = self.animations[self.current_animation].0;
        } else {
            self.current_idx += 1;
        }
    }

    pub fn get_frame_index(&self) -> u32 {
        if !self.is_playing {
            return 0;
        }

        return (self.animations[self.current_animation].0 + self.current_idx) as u32;
    }
}

fn animate_sprites(mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut AnimationState)>) {
    for (timer, mut sprite, mut state) in &mut query.iter() {
        if timer.finished {
            state.incr();
            sprite.index = state.get_frame_index();
        }
    }
}

pub fn spawn_animated_spritesheet(
    mut commands: Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    frame_duration: f32,
    animation_frames: Vec<(usize, usize)>,
    location: Vec3,
) {
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(1.0).with_translation(location),
            ..Default::default()
        })
        .with(Timer::from_seconds(frame_duration, true))
        .with(AnimationState {
            animations: animation_frames,
            current_animation: 1,
            current_idx: 0,
            is_playing: true,
        });
}
