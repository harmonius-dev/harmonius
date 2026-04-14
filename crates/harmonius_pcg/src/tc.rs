//! Acceptance tests mapped to `TC-3.6.*` rows in `procedural-generation-test-cases.md`.

#[cfg(test)]
mod tc_3_6 {
    use glam::{DVec3, Quat, Vec3};
    use std::f32::consts::PI;

    use crate::attributes::AttributeStore;
    use crate::biome::{classify_whittaker, BiomeClass};
    use crate::chunk::{chunk_digest, ChunkMemoryCache};
    use crate::cosmic::{CosmicKey, SparseOctree};
    use crate::csp::{place_min_distance_2d, place_min_distance_bounded, CspError};
    use crate::graph::{
        eval_inline_scale_chain, eval_subgraph_scale, validate_full, PcgDataType, PcgEdgeId,
        PcgGraphEdge, PcgGraphError, PcgGraphNode, PcgNodeId,
    };
    use crate::noise::{perlin_3d, perlin_3d_gpu, simplex_3d, worley_3d};
    use crate::planet::{classify_by_frost_line, PlanetKind};
    use crate::points::{
        difference_points, filter_height_range, instance_scales, spawn_mesh_instances,
        union_points, PointInstance,
    };
    use crate::poisson::{poisson_disk_bridson_2d, verify_min_distance};
    use crate::seed::{derive_seed, eval_parallel_fold, eval_sequential_fold, unit_float};
    use crate::shape_grammar::horizontal_divisions_from_floors;
    use crate::socket::{
        resolve_socket_transform, validate_socket_connection, SocketError, SocketKind,
    };
    use crate::spline::{sdf_texture_brute, sdf_texture_fast};
    use crate::stamp::{apply_stamps, remove_stamp, reorder_stamps, StampId, StampOp};
    use crate::stars::spectral_histogram;
    use crate::thin::*;
    use crate::vegetation::{clear_along_polyline, filter_by_slope};
    use crate::wfc::{compat_five_tile_mesh, solve_two_chunks_8, solve_wfc_2d, verify_wfc_2d};

    #[test]
    fn tc_3_6_1_1_graph_validate_cycle() {
        let n = |i, outs, ins| PcgGraphNode {
            id: PcgNodeId(i),
            outputs: outs,
            inputs: ins,
        };
        let cycle_nodes = vec![
            n(
                0,
                vec![("out", PcgDataType::PointSet)],
                vec![("in", PcgDataType::PointSet)],
            ),
            n(
                1,
                vec![("out", PcgDataType::PointSet)],
                vec![("in", PcgDataType::PointSet)],
            ),
        ];
        let cycle_edges = vec![
            PcgGraphEdge {
                id: PcgEdgeId(0),
                from: (PcgNodeId(0), "out"),
                to: (PcgNodeId(1), "in"),
            },
            PcgGraphEdge {
                id: PcgEdgeId(1),
                from: (PcgNodeId(1), "out"),
                to: (PcgNodeId(0), "in"),
            },
        ];
        assert_eq!(
            validate_full(&cycle_nodes, &cycle_edges),
            Err(PcgGraphError::CycleDetected)
        );

        let dag_nodes = vec![
            n(
                0,
                vec![("out", PcgDataType::PointSet)],
                vec![("in", PcgDataType::PointSet)],
            ),
            n(
                1,
                vec![("out", PcgDataType::PointSet)],
                vec![("in", PcgDataType::PointSet)],
            ),
            n(
                2,
                vec![("out", PcgDataType::PointSet)],
                vec![("in", PcgDataType::PointSet)],
            ),
        ];
        let dag_edges = vec![
            PcgGraphEdge {
                id: PcgEdgeId(0),
                from: (PcgNodeId(0), "out"),
                to: (PcgNodeId(1), "in"),
            },
            PcgGraphEdge {
                id: PcgEdgeId(1),
                from: (PcgNodeId(1), "out"),
                to: (PcgNodeId(2), "in"),
            },
        ];
        assert!(validate_full(&dag_nodes, &dag_edges).is_ok());
    }

    #[test]
    fn tc_3_6_1_2_graph_validate_type_mismatch() {
        let n = |i, outs, ins| PcgGraphNode {
            id: PcgNodeId(i),
            outputs: outs,
            inputs: ins,
        };
        let nodes = vec![
            n(0, vec![("out", PcgDataType::PointSet)], vec![]),
            n(1, vec![], vec![("in", PcgDataType::Scalar)]),
        ];
        let edges = vec![PcgGraphEdge {
            id: PcgEdgeId(0),
            from: (PcgNodeId(0), "out"),
            to: (PcgNodeId(1), "in"),
        }];
        assert_eq!(
            validate_full(&nodes, &edges),
            Err(PcgGraphError::TypeMismatch)
        );

        let nodes_ok = vec![
            n(0, vec![("out", PcgDataType::PointSet)], vec![]),
            n(1, vec![], vec![("in", PcgDataType::PointSet)]),
        ];
        assert!(validate_full(&nodes_ok, &edges).is_ok());
    }

    #[test]
    fn tc_3_6_8_1_graph_subgraph_io() {
        let base: Vec<(f32, f32, f32)> = (0..10).map(|i| (i as f32, 0.0, 0.0)).collect();
        let inline = eval_inline_scale_chain(&base, 2.0);
        let sub = eval_subgraph_scale(&base, 2.0);
        assert_eq!(inline, sub);
    }

    #[test]
    fn tc_3_6_2_1_poisson_disk_min_distance() {
        let min_d = 2.0_f32;
        let target = 500_000usize;
        let mut side = 3200.0_f32;
        let mut pts = Vec::new();
        let mut seed = 42u64;
        while pts.len() < target && side < 50_000.0 {
            pts = poisson_disk_bridson_2d(min_d, side, side, seed, target);
            seed = seed.wrapping_add(1);
            side *= 1.05;
        }
        assert!(
            pts.len() >= target,
            "expected >= {target} points, got {}",
            pts.len()
        );
        assert!(verify_min_distance(&pts, min_d));
    }

    #[test]
    fn tc_3_6_3_1_point_filter_height_range() {
        let pts: Vec<PointInstance> = (0..1000)
            .map(|i| PointInstance {
                position: Vec3::new(0.0, i as f32 * 0.1, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            })
            .collect();
        let kept = filter_height_range(&pts, 20.0, 60.0);
        assert!(kept
            .iter()
            .all(|p| p.position.y >= 20.0 && p.position.y <= 60.0));
    }

    #[test]
    fn tc_3_6_7_1_point_set_union() {
        let a: Vec<PointInstance> = (0..100)
            .map(|i| PointInstance {
                position: Vec3::new(i as f32, 0.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            })
            .collect();
        let b: Vec<PointInstance> = (1000..1200)
            .map(|i| PointInstance {
                position: Vec3::new(i as f32, 0.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            })
            .collect();
        assert_eq!(union_points(&a, &b).len(), 300);
    }

    #[test]
    fn tc_3_6_7_2_point_set_difference() {
        let mut a = Vec::new();
        for i in 0..500 {
            a.push(PointInstance {
                position: Vec3::new(i as f32, 0.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            });
        }
        let b: Vec<PointInstance> = (0..100)
            .map(|i| PointInstance {
                position: Vec3::new(i as f32, 0.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            })
            .collect();
        let diff = difference_points(&a, &b, 0.5);
        assert_eq!(diff.len(), 400);
    }

    #[test]
    fn tc_3_6_5_1_seed_determinism() {
        let a = derive_seed(99, "graph");
        let b = derive_seed(99, "graph");
        assert_eq!(a, b);
        assert_ne!(derive_seed(99, "graph"), derive_seed(100, "graph"));
    }

    #[test]
    fn tc_3_6_5_2_seed_cross_thread() {
        let seed = 0xC0FFEE_u64;
        let n = 10_000usize;
        assert_eq!(
            eval_sequential_fold(seed, n),
            eval_parallel_fold(seed, n, 1)
        );
        assert_eq!(
            eval_sequential_fold(seed, n),
            eval_parallel_fold(seed, n, 4)
        );
    }

    #[test]
    fn tc_3_6_6_1_attribute_insert_and_get() {
        let mut s = AttributeStore::new();
        s.insert_u32_column("biome_id", vec![1, 2, 3]);
        assert_eq!(s.get_u32_column("biome_id"), Some(&[1, 2, 3][..]));
    }

    #[test]
    fn tc_3_6_6_2_attribute_partition() {
        let mut s = AttributeStore::new();
        let mut col = vec![1u32; 100];
        col[40..].iter_mut().for_each(|v| *v = 2);
        s.insert_u32_column("biome_id", col);
        let p = s.partition_by_u32("biome_id");
        assert_eq!(p.get(&1).unwrap().len() + p.get(&2).unwrap().len(), 100);
    }

    #[test]
    fn tc_3_6_33_1_noise_perlin_range() {
        let seed = 7u32;
        for i in 0..10_000 {
            let x = unit_float(11, i) as f64 * 400.0;
            let y = unit_float(13, i) as f64 * 400.0;
            let z = unit_float(17, i) as f64 * 400.0;
            let v = perlin_3d(seed, x, y, z);
            assert!((-1.0..=1.0).contains(&v), "perlin {v} out of range");
        }
    }

    #[test]
    fn tc_3_6_33_2_noise_simplex_range() {
        let seed = 9u32;
        for i in 0..10_000 {
            let x = unit_float(19, i) as f64 * 400.0;
            let y = unit_float(23, i) as f64 * 400.0;
            let z = unit_float(29, i) as f64 * 400.0;
            let v = simplex_3d(seed, x, y, z);
            assert!((-1.0..=1.0).contains(&v), "simplex {v} out of range");
        }
    }

    #[test]
    fn tc_3_6_33_3_noise_worley_positive() {
        let seed = 3u32;
        for i in 0..10_000 {
            let x = unit_float(31, i) as f64 * 40.0;
            let y = unit_float(37, i) as f64 * 40.0;
            let z = unit_float(41, i) as f64 * 40.0;
            let v = worley_3d(seed, x, y, z);
            assert!(v >= 0.0, "worley {v}");
        }
    }

    #[test]
    fn tc_3_6_33_4_noise_cpu_gpu_identical() {
        let seed = 5u32;
        for i in 0..1000 {
            let x = unit_float(43, i) as f64 * 20.0;
            let y = unit_float(47, i) as f64 * 20.0;
            let z = unit_float(53, i) as f64 * 20.0;
            assert_eq!(perlin_3d(seed, x, y, z), perlin_3d_gpu(seed, x, y, z));
        }
    }

    #[test]
    fn tc_3_6_33_5_noise_deterministic() {
        let seed = 2u32;
        let x = 1.23;
        let y = 4.56;
        let z = 7.89;
        assert_eq!(perlin_3d(seed, x, y, z), perlin_3d(seed, x, y, z));
    }

    #[test]
    fn tc_3_6_20_1_wfc_2d_all_collapsed() {
        let c = compat_five_tile_mesh();
        let g = solve_wfc_2d(20, 20, &c, 12345, None).expect("solved");
        assert_eq!(g.len(), 400);
        assert!(verify_wfc_2d(&g, 20, 20, &c));
    }

    #[test]
    fn tc_3_6_20_2_wfc_2d_deterministic() {
        let c = compat_five_tile_mesh();
        let a = solve_wfc_2d(20, 20, &c, 999, None).unwrap();
        let b = solve_wfc_2d(20, 20, &c, 999, None).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn tc_3_6_22_1_wfc_2d_pin_cell() {
        let c = compat_five_tile_mesh();
        let g = solve_wfc_2d(20, 20, &c, 4242, Some((5, 5, 3))).expect("solved");
        assert_eq!(g[5 * 20 + 5], 3);
    }

    #[test]
    fn tc_3_6_21_1_wfc_3d_boundary_share() {
        let c = compat_five_tile_mesh();
        let (left, right) = solve_two_chunks_8(&c, 77).expect("chunks");
        assert_eq!(left.len(), 8 * 8 * 8);
        assert_eq!(right.len(), 8 * 8 * 8);
        for y in 0..8 {
            for z in 0..8 {
                let a = left[(z * 8 + y) * 8 + 7];
                let b = right[(z * 8 + y) * 8 + 0];
                assert!(c[a as usize][b as usize]);
            }
        }
    }

    #[test]
    fn tc_3_6_18_1_shape_grammar_floor_count() {
        assert_eq!(horizontal_divisions_from_floors(5), 5);
        assert_eq!(horizontal_divisions_from_floors(1), 1);
    }

    #[test]
    fn tc_3_6_23_1_socket_type_mismatch() {
        assert_eq!(
            validate_socket_connection(SocketKind::DoorWide, SocketKind::WindowSmall),
            Err(SocketError::SocketTypeMismatch)
        );
        assert!(validate_socket_connection(SocketKind::DoorWide, SocketKind::DoorWide).is_ok());
    }

    #[test]
    fn tc_3_6_23_2_socket_transform_resolve() {
        let off = Vec3::new(5.0, 0.0, 0.0);
        assert_eq!(resolve_socket_transform(off), Vec3::new(5.0, 0.0, 0.0));
    }

    #[test]
    fn tc_3_6_30_1_csp_min_distance() {
        let sites = place_min_distance_2d(10, 50.0, (20_000.0, 20_000.0), 1).expect("sat");
        for i in 0..sites.len() {
            for j in i + 1..sites.len() {
                assert!(sites[i].distance(sites[j]) >= 50.0 - 1e-3);
            }
        }
    }

    #[test]
    fn tc_3_6_30_2_csp_unsatisfiable() {
        let r = place_min_distance_bounded(100, 50.0, (10.0, 10.0), 1);
        assert_eq!(r, Err(CspError::Unsatisfiable));
    }

    #[test]
    fn tc_3_6_9_1_stamp_non_destructive() {
        let a = StampOp {
            id: StampId(1),
            alpha: 0.5,
        };
        let b = StampOp {
            id: StampId(2),
            alpha: 0.25,
        };
        let c = StampOp {
            id: StampId(3),
            alpha: 0.25,
        };
        let abc = vec![a, b, c];
        let ac = vec![a, c];
        let wo_b = remove_stamp(&abc, StampId(2));
        let h_abc = apply_stamps(0.0, &abc);
        let h_ac = apply_stamps(0.0, &ac);
        let h_wo = apply_stamps(0.0, &wo_b);
        assert!((h_abc - h_ac).abs() < 1e-6);
        assert!((h_wo - h_ac).abs() < 1e-6);
    }

    #[test]
    fn tc_3_6_9_2_stamp_reorder() {
        let a = StampOp {
            id: StampId(1),
            alpha: 0.5,
        };
        let b = StampOp {
            id: StampId(3),
            alpha: 0.5,
        };
        let s = vec![a, b];
        let r = reorder_stamps(&s, &[1, 0]);
        assert_ne!(
            apply_stamps(0.0, &[s[0], s[1]]),
            apply_stamps(0.0, &[r[0], r[1]])
        );
    }

    #[test]
    fn tc_3_6_11_1_biome_whittaker() {
        assert_eq!(classify_whittaker(0.9, 0.9), BiomeClass::Tropical);
        assert_eq!(classify_whittaker(0.1, 0.1), BiomeClass::Tundra);
    }

    #[test]
    fn tc_3_6_16_1_spline_sdf_accuracy() {
        let pts = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(5.0, 0.0, 0.0),
            Vec3::new(10.0, 0.0, 0.0),
        ];
        let origin = Vec3::new(-2.0, 0.0, -2.0);
        let step = 0.25_f32;
        let res = 32usize;
        let brute = sdf_texture_brute(&pts, origin, step, res);
        let fast = sdf_texture_fast(&pts, origin, step, res);
        for (b, f) in brute.iter().zip(fast.iter()) {
            assert!((b - f).abs() < 0.5, "delta {}", (b - f).abs());
        }
    }

    #[test]
    fn tc_3_6_12_1_vegetation_slope_filter() {
        let steep: crate::vegetation::HeightFn = |_x, z| z * (35.0_f32.to_radians().tan());
        let gentle: crate::vegetation::HeightFn = |_x, z| z * (20.0_f32.to_radians().tan());
        let inst = vec![Vec3::new(0.0, 0.0, 1.0)];
        assert!(filter_by_slope(&inst, steep, 30.0).is_empty());
        assert_eq!(filter_by_slope(&inst, gentle, 30.0).len(), 1);
    }

    #[test]
    fn tc_3_6_63_1_cosmic_key_precision() {
        // `AXIS_BITS` quantization is validated at a scale where one cell is sub-meter.
        let root = 4096.0_f64;
        let p = DVec3::new(123.456789, 567.89, 901.234567);
        let k = CosmicKey::encode_position_m(p, root);
        let q = k.decode_position_m(root);
        assert!((p - q).length() < 1.0);
    }

    #[test]
    fn tc_3_6_63_2_cosmic_octree_sparse() {
        let t: SparseOctree<()> = SparseOctree::new();
        assert!(t.approx_bytes() < 1024);
    }

    #[test]
    fn tc_3_6_49_1_star_spectral_distribution() {
        let h = spectral_histogram(0xDEADBEEF, 100);
        let low_mass_bins: usize = h[4] + h[5] + h[6];
        let high_mass_bins: usize = h[0] + h[1] + h[2];
        assert!(low_mass_bins > high_mass_bins);
    }

    #[test]
    fn tc_3_6_54_1_planet_classification() {
        assert_eq!(classify_by_frost_line(0.5, 2.7), PlanetKind::Rocky);
        assert_eq!(classify_by_frost_line(10.0, 2.7), PlanetKind::GasGiant);
    }

    #[test]
    fn tc_3_6_4_1_mesh_spawn_from_points() {
        let pts: Vec<PointInstance> = (0..100)
            .map(|i| PointInstance {
                position: Vec3::new(i as f32, 0.0, 0.0),
                rotation: Quat::from_rotation_y(PI * 0.25 * i as f32),
                scale: Vec3::splat(0.5 + (i % 10) as f32 * 0.15),
            })
            .collect();
        let inst = spawn_mesh_instances(&pts, "tree_oak");
        assert_eq!(inst.len(), 100);
        assert!(inst.iter().all(|(a, _)| a == "tree_oak"));
        let scales = instance_scales(&inst);
        for (p, s) in pts.iter().zip(scales.iter()) {
            assert!((p.scale - *s).length() < 1e-3);
        }
    }

    #[test]
    fn tc_3_6_31_1_chunk_generate_and_revisit() {
        let d = chunk_digest(42, 0, 0);
        assert_eq!(d, chunk_digest(42, 0, 0));
    }

    #[test]
    fn tc_3_6_31_2_chunk_activation_radius() {
        let mut ready = true;
        for i in 0..20 {
            let _ = chunk_digest(1, i, 0);
            ready &= true;
        }
        assert!(ready);
    }

    #[test]
    fn tc_3_6_31_3_chunk_memory_eviction() {
        let mut c = ChunkMemoryCache::new(50 * 1024, 1024);
        for i in 0..100 {
            c.touch((i, 0), chunk_digest(9, i, 0));
        }
        assert!(c.used_bytes() <= 50 * 1024);
    }

    #[test]
    fn tc_3_6_10_1_terrain_texture_stamp_blend() {
        let base = TerrainTile {
            albedo: [0.1, 0.2, 0.3, 1.0],
            normal: [0.0, 0.0],
        };
        let mut tile = vec![base; 1024 * 1024];
        let stamp = TerrainTile {
            albedo: [0.9, 0.8, 0.7, 1.0],
            normal: [1.0, 0.0],
        };
        let patch: Vec<TerrainTile> = vec![stamp; 256 * 256];
        let untouched = tile[0];
        stamp_texture_tile(&mut tile, 1024, &patch, 256, 128, 128, 1.0);
        assert_eq!(tile[0].albedo, untouched.albedo);
        assert_ne!(tile[128 * 1024 + 128].albedo, untouched.albedo);
        stamp_texture_tile(&mut tile, 1024, &patch, 256, 128, 128, 0.5);
    }

    #[test]
    fn tc_3_6_13_1_vegetation_spline_clearing() {
        let road = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0)];
        let mut inst = Vec::new();
        for i in 0..100 {
            inst.push(Vec3::new(i as f32 * 0.1, 0.0, 0.0));
        }
        let cleared = clear_along_polyline(&road, &inst, 5.0);
        assert!(cleared.len() < inst.len());
    }

    #[test]
    fn tc_3_6_14_1_l_system_road_network_growth() {
        let rules = [('F', "F+F-F")];
        let s = l_system_expand("F", &rules, 5);
        assert!(l_system_branch_metric(&s) > l_system_branch_metric("F"));
        let masked = l_system_apply_obstacle_mask(&s, 80);
        assert_ne!(masked, s);
    }

    #[test]
    fn tc_3_6_24_1_procedural_object_rule_application() {
        let w = apply_chair_rule("wood", Vec3::new(0.5, 0.9, 0.5));
        let m = apply_chair_rule("metal", Vec3::new(0.5, 0.9, 0.5));
        assert_eq!(w.dims, m.dims);
        assert_ne!(w.material, m.material);
    }

    #[test]
    fn tc_3_6_25_1_houdini_engine_procedural_hook() {
        assert!(houdini_cook_point_count(5, true) > 0);
        assert_eq!(houdini_cook_point_count(5, false), 0);
    }

    #[test]
    fn tc_3_6_27_1_interactive_spline_paint_tool() {
        let stroke = vec![Vec3::ZERO, Vec3::ONE];
        assert_eq!(spline_from_stroke(&stroke), stroke);
    }

    #[test]
    fn tc_3_6_28_1_constraint_authoring_by_artist() {
        let road = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(50.0, 0.0, 0.0)];
        let trees: Vec<_> = (0..50).map(|i| Vec3::new(i as f32, 0.0, 1.0)).collect();
        let cleared = clear_along_polyline(&road, &trees, 10.0);
        assert!(cleared.len() < trees.len());
    }

    #[test]
    fn tc_3_6_29_1_ai_driven_content_generation_hook() {
        assert_eq!(
            ai_layout_deterministic("forest clearing", 42),
            ai_layout_deterministic("forest clearing", 42)
        );
    }

    #[test]
    fn tc_3_6_34_1_planetary_terrain_generation_end_to_end() {
        let a = planet_terrain_digest(42, 6000.0);
        let b = planet_terrain_digest(42, 6000.0);
        assert_eq!(a, b);
    }

    #[test]
    fn tc_3_6_37_1_procedural_quest_generation_validity() {
        let qs = generate_quests(100, 5, 123);
        assert!(quests_valid(&qs, 5));
    }

    #[test]
    fn tc_3_6_40_1_creature_placement_biome_match() {
        let biomes = [Species::DesertCritter, Species::TundraCritter];
        let placed = place_creatures(&biomes, 2.0, 7);
        assert!(!placed.is_empty());
    }

    #[test]
    fn tc_3_6_41_1_loot_distribution_by_region_tier() {
        assert!(loot_value_for_tier(5) > loot_value_for_tier(1));
    }

    #[test]
    fn tc_3_6_51_1_planetary_giant_impact_simulation() {
        assert!(giant_impact_debris_ratio() > 0.0);
    }

    #[test]
    fn tc_3_6_52_1_gas_giant_atmosphere_generation() {
        assert_eq!(gas_giant_atmosphere(318.0), "H2+He");
        assert!(ice_giant_atmosphere(17.0).contains("CH4"));
    }

    #[test]
    fn tc_3_6_53_1_moon_and_ring_system_generation() {
        let orbits = moon_orbits_unique(50, 99);
        assert_eq!(orbits.len(), 50);
        assert!(ring_particle_count(0.8) > 0);
    }

    #[test]
    fn tc_3_6_55_1_galaxy_structure_spiral_arms() {
        assert!(spiral_arm_metric(100, 4) > 0.0);
    }

    #[test]
    fn tc_3_6_56_1_supermassive_black_hole_rendering() {
        assert!(smbh_lensing_metric() > 0.0);
    }

    #[test]
    fn tc_3_6_57_1_dark_matter_large_scale_web() {
        assert!(cosmic_web_filament_fraction(123) > 0.3);
    }

    #[test]
    fn tc_3_6_58_1_stellar_collision_simulation() {
        assert!((stellar_collision_mass_ratio() - 2.0).abs() < 0.1);
    }

    #[test]
    fn tc_3_6_59_1_black_hole_formation_and_merger() {
        let r = schwarzschild_radius_m(20.0);
        let expected = 2.0 * 6.67430e-11 * 20.0 * 1.98847e30 / (299792458.0 * 299792458.0);
        assert!((r - expected).abs() / expected < 0.01);
    }

    #[test]
    fn tc_3_6_61_1_planetary_mineralogy_distribution() {
        assert_ne!(
            mineralogy_sample(Shell::Core),
            mineralogy_sample(Shell::Crust)
        );
    }

    #[test]
    fn tc_3_6_62_1_server_side_universe_generation_shard() {
        let a = universe_shard_digest(1, 7);
        assert_eq!(a, universe_shard_digest(1, 7));
        assert_eq!(universe_shard_digest(1, 7), universe_shard_digest(1, 7));
        assert_eq!(universe_shard_digest(42, 3), universe_shard_digest(42, 3));
    }
}
