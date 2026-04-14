//! Deterministic 2D rendering, tilemap, camera, and lightweight physics helpers for Harmonius.
//!
//! Implements the API surface described in `docs/design/rendering/2d.md` with pure functions
//! suitable for unit tests (`TC-10.5.*`).

#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod animation;
mod camera;
mod collision;
mod draw_sort;
mod grid;
mod joint;
mod lighting;
mod physics;
mod query;
mod sprite;
mod tilemap;
mod types;

pub use animation::{
    advance_animation, advance_animation_oneshot, pingpong_sequence, AnimationEvent, ClipMeta,
    PlaybackMode, PlaybackState, SpriteAnimationState,
};
pub use camera::{parallax_layer_offset, pixel_perfect_camera_position, shake_amplitude_linear};
pub use collision::{
    build_edge_chains_from_solid_grid, convex_polygon_overlap_sat, reduced_collider_count,
    EdgeChain2d,
};
pub use draw_sort::{sort_draw_order, Drawable2d};
pub use grid::{
    hex_neighbors_pointy_top_axial, iso_diamond_screen_to_tile, iso_diamond_tile_to_screen,
};
pub use joint::{simulate_revolute_with_angle_limits, RevoluteLimitSim};
pub use lighting::{light_intensity_quadratic, segment_hits_polygon_first};
pub use physics::{
    step_dynamic_gravity, step_one_way_platform, swept_circle_hits_vertical_wall, SweptCircleWall,
};
pub use query::{circle_overlap_entities, raycast_aabb, RayHit2d};
pub use sprite::{sort_sprite_draw_order, sprite_batch_by_atlas, sprite_batch_by_atlas_and_blend};
pub use tilemap::{apply_auto_tile_cardinal, cull_tilemap_chunks, TilemapChunkDesc};
pub use types::{BlendMode, BodyType2d, GridType, LayerZ, RigidBody2d, Shape2d, TileFlags, TileId};

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{IVec2, Vec2};
    use std::f32::consts::PI;

    use crate::joint::RevoluteLimitSim;
    use crate::physics::SweptCircleWall;

    #[test]
    fn test_sprite_batch_by_atlas() {
        let mut sprites = Vec::new();
        for i in 0..100 {
            sprites.push((i, 0));
        }
        for i in 100..200 {
            sprites.push((i, 1));
        }
        let batches = sprite_batch_by_atlas(&sprites);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0].entities.len(), 100);
        assert_eq!(batches[1].entities.len(), 100);
        assert_ne!(batches[0].atlas, batches[1].atlas);
    }

    #[test]
    fn test_sprite_batch_by_blend_mode() {
        let mut s = Vec::new();
        for i in 0..50 {
            s.push((i, 0, BlendMode::AlphaBlend));
        }
        for i in 50..100 {
            s.push((i, 0, BlendMode::Additive));
        }
        let batches = sprite_batch_by_atlas_and_blend(&s);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0].blend, BlendMode::AlphaBlend);
        assert_eq!(batches[1].blend, BlendMode::Additive);
    }

    #[test]
    fn test_sprite_z_order_sort() {
        let layers = [
            LayerZ {
                sort_layer: 0,
                z_order: 0.5,
            },
            LayerZ {
                sort_layer: 0,
                z_order: 0.1,
            },
            LayerZ {
                sort_layer: 0,
                z_order: 0.9,
            },
        ];
        let order = sort_sprite_draw_order(&layers);
        assert_eq!(order, vec![1, 0, 2]);
    }

    #[test]
    fn test_sprite_layer_priority() {
        let layers = [
            LayerZ {
                sort_layer: 5,
                z_order: 0.0,
            },
            LayerZ {
                sort_layer: 1,
                z_order: 0.99,
            },
        ];
        let order = sort_sprite_draw_order(&layers);
        assert_eq!(order, vec![1, 0]);
    }

    #[test]
    fn test_animation_loop_mode() {
        let clip = ClipMeta {
            fps: 10.0,
            frame_count: 4,
        };
        let mut state = SpriteAnimationState {
            frame_index: 0,
            elapsed: 0.0,
            mode: PlaybackMode::Loop,
            state: PlaybackState::Playing,
        };
        let mut ev = Vec::new();
        advance_animation(&mut state, &clip, 0.5, &mut ev, &[]);
        assert_eq!(state.frame_index, 1);
        assert!((state.elapsed - 0.0).abs() < 1e-5);
    }

    #[test]
    fn test_animation_pingpong_mode() {
        let seq = pingpong_sequence(4, 8);
        assert_eq!(seq, vec![0, 1, 2, 3, 2, 1, 0, 1]);
    }

    #[test]
    fn test_animation_oneshot_mode() {
        let clip = ClipMeta {
            fps: 10.0,
            frame_count: 3,
        };
        let mut state = SpriteAnimationState {
            frame_index: 0,
            elapsed: 0.0,
            mode: PlaybackMode::OneShot,
            state: PlaybackState::Playing,
        };
        let mut ev = Vec::new();
        advance_animation_oneshot(&mut state, &clip, 0.5, &mut ev, &[]);
        assert_eq!(state.state, PlaybackState::Finished);
        assert_eq!(state.frame_index, 2);
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0].name, "AnimationFinished");
    }

    #[test]
    fn test_animation_event_fires() {
        let clip = ClipMeta {
            fps: 10.0,
            frame_count: 4,
        };
        let mut state = SpriteAnimationState {
            frame_index: 0,
            elapsed: 0.0,
            mode: PlaybackMode::Loop,
            state: PlaybackState::Playing,
        };
        let mut ev = Vec::new();
        let clip_events: [(u32, &str); 1] = [(2, "footstep")];
        advance_animation(&mut state, &clip, 0.3, &mut ev, &clip_events);
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0].name, "footstep");
    }

    #[test]
    fn test_tilemap_chunk_cull() {
        let mut chunks = Vec::new();
        for y in 0..4 {
            for x in 0..4 {
                chunks.push(TilemapChunkDesc {
                    chunk_pos: IVec2::new(x, y),
                    width: 32,
                    height: 32,
                });
            }
        }
        let tile = Vec2::splat(1.0);
        let vis = cull_tilemap_chunks(&chunks, tile, Vec2::ZERO, Vec2::new(64.0, 64.0));
        assert_eq!(vis.len(), 4);
        for c in &vis {
            assert!(c.x < 2 && c.y < 2);
        }
    }

    #[test]
    fn test_tilemap_auto_tile() {
        let mut tiles = vec![TileId(1); 9];
        apply_auto_tile_cardinal(&mut tiles, 3, 3, TileId(1), TileId(5));
        assert_eq!(tiles[4], TileId(5));
    }

    #[test]
    fn test_iso_diamond_screen_to_tile() {
        let t = IVec2::new(2, 3);
        let tile_size = Vec2::new(64.0, 32.0);
        let s = iso_diamond_tile_to_screen(t, tile_size);
        let back = iso_diamond_screen_to_tile(s, tile_size);
        assert_eq!(back, t);
    }

    #[test]
    fn test_hex_pointy_top_neighbors() {
        let c = IVec2::new(2, 2);
        let n = hex_neighbors_pointy_top_axial(c);
        let set: std::collections::HashSet<IVec2> = n.into_iter().collect();
        let expected: std::collections::HashSet<IVec2> = [
            IVec2::new(3, 2),
            IVec2::new(1, 2),
            IVec2::new(2, 1),
            IVec2::new(3, 1),
            IVec2::new(2, 3),
            IVec2::new(1, 3),
        ]
        .into_iter()
        .collect();
        assert_eq!(set, expected);
    }

    #[test]
    fn test_camera_parallax_scroll() {
        let off = parallax_layer_offset(Vec2::new(100.0, 0.0), 0.5);
        assert!((off.x - 50.0).abs() < 1e-5);
    }

    #[test]
    fn test_camera_shake_decay() {
        let a = shake_amplitude_linear(10.0, 0.5, 0.25);
        assert!((a - 5.0).abs() < 0.11);
    }

    #[test]
    fn test_pixel_perfect_snap() {
        let p = pixel_perfect_camera_position(Vec2::new(10.4, 7.8));
        assert_eq!(p, Vec2::new(10.0, 8.0));
    }

    #[test]
    fn test_rigidbody_dynamic_gravity() {
        let mut body = RigidBody2d {
            body_type: BodyType2d::Dynamic,
            gravity_scale: 1.0,
            velocity: Vec2::ZERO,
            ccd_enabled: false,
        };
        let g = Vec2::new(0.0, -9.81);
        let dt = 1.0 / 60.0;
        step_dynamic_gravity(&mut body, g, dt, 60);
        assert!(body.velocity.y > -9.85 && body.velocity.y < -9.77);
    }

    #[test]
    fn test_one_way_platform_drop() {
        let mut y = 5.0_f32;
        let mut vy = -2.0_f32;
        let dt = 0.1;
        for _ in 0..200 {
            step_one_way_platform(&mut y, &mut vy, dt, 0.0, true, false);
            if y <= 0.01 && vy.abs() < 0.01 {
                break;
            }
        }
        assert!((y - 0.0).abs() < 0.05);
        let mut y2 = 0.5_f32;
        let mut vy2 = -1.0_f32;
        for _ in 0..50 {
            step_one_way_platform(&mut y2, &mut vy2, dt, 0.0, true, true);
        }
        assert!(y2 < -0.01);
    }

    #[test]
    fn test_ccd_no_tunnel() {
        let t = swept_circle_hits_vertical_wall(SweptCircleWall {
            x: 0.0,
            vx: 5000.0,
            radius: 0.05,
            wall_x: 5.0,
            wall_half_width: 0.5,
            dt: 1.0 / 60.0,
        });
        assert!(t.is_some());
    }

    #[test]
    fn test_polygon_collider_overlap() {
        let a = vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(-1.0, 1.0),
        ];
        let b = vec![
            Vec2::new(-0.5, -1.0),
            Vec2::new(1.5, -1.0),
            Vec2::new(1.5, 1.0),
            Vec2::new(-0.5, 1.0),
        ];
        let (hit, n) = convex_polygon_overlap_sat(&a, &b).unwrap();
        assert!(hit);
        assert!((n.x.abs() - 1.0).abs() < 0.01 && n.y.abs() < 0.01);
    }

    #[test]
    fn test_tilemap_edge_chain_reduce() {
        let w = 100usize;
        let h = 100usize;
        let solid = vec![true; w * h];
        let n = reduced_collider_count(w, h, &solid);
        assert!(n <= 1000);
        assert_eq!(n, 1);
    }

    #[test]
    fn test_joint_revolute_limits() {
        let (angle, _) = simulate_revolute_with_angle_limits(RevoluteLimitSim {
            angle: 0.0,
            omega: 0.0,
            applied_torque: 100.0,
            inertia: 0.01,
            dt: 1.0 / 60.0,
            lower: -PI / 4.0,
            upper: PI / 4.0,
            steps: 60 * 30,
        });
        assert!((angle - PI / 4.0).abs() < 0.01);
    }

    #[test]
    fn test_2d_raycast_hit() {
        let hit = raycast_aabb(
            Vec2::ZERO,
            Vec2::new(1.0, 0.0),
            10.0,
            Vec2::new(5.0, -1.0),
            Vec2::new(6.0, 1.0),
            42,
        )
        .unwrap();
        assert!((hit.point.x - 5.0).abs() < 0.01);
        assert!((hit.normal.x + 1.0).abs() < 0.01);
        assert_eq!(hit.entity, 42);
    }

    #[test]
    fn test_2d_overlap_query() {
        let positions = vec![
            (1u32, Vec2::new(0.0, 0.0)),
            (2u32, Vec2::new(1.0, 0.0)),
            (3u32, Vec2::new(0.5, 0.5)),
            (4u32, Vec2::new(5.0, 0.0)),
            (5u32, Vec2::new(-3.0, 0.0)),
        ];
        let got = circle_overlap_entities(&positions, Vec2::ZERO, 2.0);
        assert_eq!(got, vec![1, 2, 3]);
    }

    #[test]
    fn test_light2d_radius_falloff() {
        let r = 10.0;
        let i0 = light_intensity_quadratic(0.0, r);
        let i5 = light_intensity_quadratic(5.0, r);
        let i10 = light_intensity_quadratic(10.0, r);
        assert!((i0 - 1.0).abs() < 0.01);
        assert!((i5 - 0.25).abs() < 0.01);
        assert!((i10 - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_shadow_caster_occluder() {
        let light = Vec2::ZERO;
        let sample = Vec2::new(10.0, 0.0);
        let poly = vec![
            Vec2::new(4.5, -1.0),
            Vec2::new(5.5, -1.0),
            Vec2::new(5.5, 1.0),
            Vec2::new(4.5, 1.0),
        ];
        assert!(segment_hits_polygon_first(light, sample, &poly));
    }

    #[test]
    fn test_particle_z_interleave() {
        let items = [
            (Drawable2d::Sprite(0), 0.3f32),
            (Drawable2d::Particle(1), 0.4f32),
            (Drawable2d::Sprite(2), 0.5f32),
        ];
        let order = sort_draw_order(&items);
        assert_eq!(
            order,
            vec![
                Drawable2d::Sprite(0),
                Drawable2d::Particle(1),
                Drawable2d::Sprite(2)
            ]
        );
    }
}
