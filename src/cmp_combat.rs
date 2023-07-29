use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use crate::cmp_blood;
use crate::cmp_ball;
use crate::cmp_game_asset::GameAsset;
use crate::cmp_ball::Ball;

#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Status {
    pub hp: f32,
    pub attack: f32,
}


#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Player1;
#[derive(Component, Reflect, Clone, Serialize, Deserialize, Debug)]
pub struct Player2;

fn display_contact_info(entity1: Entity, entity2: Entity, rapier_context: &Res<RapierContext>) {
    /* Find the contact pair, if it exists, between two colliders. */
    if let Some(contact_pair) = rapier_context.contact_pair(entity1, entity2) {
        // The contact pair exists meaning that the broad-phase identified a potential contact.
        if contact_pair.has_any_active_contacts() {
            // The contact pair has active contacts, meaning that it
            // contains contacts for which contact forces were computed.
        }

        // We may also read the contact manifolds to access the contact geometry.
        for manifold in contact_pair.manifolds() {
            println!("Local-space contact normal: {}", manifold.local_n1());
            println!("Local-space contact normal: {}", manifold.local_n2());
            println!("World-space contact normal: {}", manifold.normal());

            // Read the geometric contacts.
            for contact_point in manifold.points() {
                // Keep in mind that all the geometric contact data are expressed in the local-space of the colliders.
                println!("Found local contact point 1: {:?}", contact_point.local_p1());
                println!("Found contact distance: {:?}", contact_point.dist()); // Negative if there is a penetration.
            }
        }
    }
}

pub fn system(
    mut commands: Commands,
    audio: Res<Audio>,
    game_assets: Res<GameAsset>,
    rapier_context: Res<RapierContext>,
    mut p1_q: Query<(Entity, &mut Status, &Transform, Option<&mut Velocity>, Option<&Ball>), (With<Player1>, Without<Player2>)>,
    mut p2_q: Query<(Entity, &mut Status, &Transform, Option<&mut Velocity>, Option<&Ball>), (With<Player2>, Without<Player1>)>,
) {
    for (p1_e, mut p1_c, p1_t, mut p1_v_opt, p1_ball_opt) in p1_q.iter_mut() {
        for (p2_e, mut p2_c, p2_t, mut p2_v_opt, p2_ball_opt) in p2_q.iter_mut() {
            if let Some(contact_pair) = rapier_context.contact_pair(p1_e.clone(), p2_e.clone()) {
                if contact_pair.has_any_active_contacts() {
                    println!("collision detect");
                    let p1_damage = p2_c.attack;
                    let p2_damage = p1_c.attack;

                    p1_c.hp = p1_c.hp - p1_damage;
                    p2_c.hp = p2_c.hp - p2_damage;

                    cmp_blood::add(&mut commands, p1_t.translation.truncate(), p1_damage as usize);
                    cmp_blood::add(&mut commands, p2_t.translation.truncate(), p2_damage as usize);
                }
            }
        }
    }
}
