use bevy::prelude::*;

pub mod prelude {
    pub use crate::*;
}

pub struct Player;
pub struct Npc;

pub struct Side(usize);

pub const SIDE_NEUTRAL: usize = 0;
pub const SIDE_1: usize = 1;
pub const SIDE_2: usize = 2;
pub const SIDE_3: usize = 3;
pub const SIDE_4: usize = 4;
pub const SIDE_5: usize = 5;
pub const SIDE_6: usize = 6;
pub const SIDE_7: usize = 7;

/// A resource which stores the relationships between sides
/// The indices are the side idx (i.e. side 1 is 0b0000_0001),
/// and the value of the Vec has 1 in the bits where the sides
/// are enemies, e.g. 0b0000_0010 indicates side1 is enemies with side2.
///
/// Side0 is neutral
pub struct SideRelationships(Vec<u8>);

/// Holds the possible relationships between sides
#[derive(Debug)]
pub enum SideRelationship {
    Neutral,
    Allied,
    Enemy,
}

impl SideRelationships {
    /// gets the relationship between two sides
    pub fn get_relationship(
        &self,
        side: usize,
        target_side: usize,
        relationships: &Vec<u8>,
    ) -> SideRelationship {
        if target_side == SIDE_NEUTRAL || side == SIDE_NEUTRAL {
            return SideRelationship::Neutral;
        }

        let reln = relationships[side as usize];
        return if reln & (1 << target_side) > 0 {
            SideRelationship::Enemy
        } else {
            SideRelationship::Allied
        };
    }

    /// sets the relationship between the two sides (works in both directions)
    pub fn set_relationship(&mut self, from_side: usize, to_side: usize, is_enemy: bool) {
        if is_enemy {
            self.0[from_side] |= 0b1 << to_side;
            self.0[to_side] |= 0b1 << from_side;
        } else {
            self.0[from_side] &= !(0b1 << to_side);
            self.0[to_side] &= !(0b1 << from_side);
        }
    }
}

/// Sets a changed allegiance between side 1 (.0) and side 2 (.0)
/// Sets the allegiance the friendly if bool is true, enemy otherwise
pub struct ChangeAllegiance(usize, usize, bool);

pub struct AllegiancePlugin;

impl Plugin for AllegiancePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(SideRelationships(vec![
            0,
            0b0000_0010, // enemies with side 2
            0b0000_0001, // enemies with side 1
            0,
            0,
            0,
            0,
            0,
            0,
        ]))
        .add_system(change_allegiance.system());
    }
}

fn change_allegiance(
    mut commands: Commands,
    mut sides: ResMut<SideRelationships>,
    entity: Entity,
    request: &ChangeAllegiance,
) {
    sides.set_relationship(request.0, request.1, request.2);
    commands.despawn(entity);
}
