use bevy::prelude::*;

pub mod prelude {
    pub use crate::*;
}

pub struct Player;
pub struct Npc;

pub struct Side(u8);

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

fn get_bit_idx(side: u8) -> usize {
    for idx in 0..7 {
        if side >> idx == 0 {
            return idx;
        }
    }

    // 0 implies no side
    return 0;
}

impl SideRelationships {
    /// Returns the first high bit index (starting from the lowest bit),
    /// e.g. 0b0000_0001 should return 1. Used to convert a side u8 to a Vec index

    /// gets the relationship between two sides
    pub fn get_relationship(
        &self,
        side: u8,
        target: u8,
        relationships: &Vec<u8>,
    ) -> SideRelationship {
        if target == 0 || side == 0 {
            return SideRelationship::Neutral;
        }

        let reln = relationships[get_bit_idx(side)];
        return if reln & target > 0 {
            SideRelationship::Enemy
        } else {
            SideRelationship::Allied
        };
    }

    /// sets the relationship between the two sides (works in both directions)
    pub fn set_relationship(&mut self, from_side: u8, to_side: u8, is_enemy: bool) {
        let from_idx = get_bit_idx(from_side);
        let to_idx = get_bit_idx(to_side);

        if is_enemy {
            self.0[from_idx] |= 0b1 << (to_idx - 1);
            self.0[to_idx] |= 0b1 << (from_idx - 1);
        } else {
            self.0[from_idx] &= !(0b1 << (to_idx - 1));
            self.0[to_idx] &= !(0b1 << (from_idx - 1));
        }
    }
}

pub struct ChangeAllegiance(u8, u8, bool);

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
