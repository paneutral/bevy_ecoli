use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::{EntropyComponent, ForkableRng, GlobalEntropy};
use rand::{distributions::Uniform, Rng};
use std::{f32::consts::TAU, ops::Range, time::Duration};

use crate::asset_loader::SAssets;

const TIMESTEP: f32 = 32.0;
// const VELOCITY_SCALAR: f32 = 5.0;
const SPD: f32 = 6.0;



#[derive(Resource,Debug)]
struct SpawnCage(Cuboid);




#[derive(Component, Debug)]
struct LogisticFnParam {
    beta0: f32,
    beta1: f32,
}

impl Default for LogisticFnParam {
    fn default() -> Self {
        let p_to_log_odds = |i: f32| {( i / (1. - i)).ln()};

        let beta0 = p_to_log_odds(1f32/32.);
        let beta1 = p_to_log_odds(31f32/32.) - beta0;
        Self { beta0, beta1 }
    }
}

#[derive(Component, Debug)]
enum MovementPhase {
    Run(Vec3),
    Tumble(Quat),
}

impl Default for MovementPhase {
    fn default() -> Self {
        MovementPhase::Run(Vec3::ZERO)
    }
}

#[derive(Component, Debug)]
pub struct PreviousConcentration(f32);

#[derive(Component, Debug)]
pub struct EColi;

pub struct EColiPlugin;

impl Plugin for EColiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(5)))
            .insert_resource(Time::<Fixed>::from_hz(TIMESTEP.into()))
            .insert_resource(SpawnCage(Cuboid::new(50., 50., 50.)))
            .add_systems(PostStartup, spawn_ecoli)
            .add_systems(Update, update_position_rotation)
            .add_systems(FixedUpdate, update_ecoli);
    }
}

fn spawn_ecoli(
    mut commands: Commands,
    assets: Res<SAssets>,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut global_rng: ResMut<GlobalEntropy<WyRand>>,
    cage: Res<SpawnCage>
) {

    // commands.spawn(( PbrBundle {
    //             mesh: meshes.add(cage.0),
    //             material: debug_material.clone(),
    //             transform: Transform::from_xyz(
    //                 -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
    //                 2.0,
    //                 Z_EXTENT / 2.,
    //             )
    //             .with_rotation(Quat::from_rotation_x(-PI / 4.)),
    //             ..default()
    //         },
    //         Shape));

    (0..200).for_each(|_| {
        let translation = cage.0.sample_interior(&mut *global_rng);
        

        let mut rng = global_rng.fork_rng();
        let rotation = get_rand_rotation(&mut rng);
        info!("test");
        commands.spawn((
            SceneBundle {
                scene: assets.ecoli.clone(),
                transform: Transform {
                    translation,
                    rotation,
                    scale: Vec3 { x: 0.3, y: 0.3, z: 0.3 },
                    ..default()
                },
                ..default()
            },
            PreviousConcentration(translation.length()),
            rng,
            MovementPhase::default(),
            LogisticFnParam::default(),
            EColi,
        ));
    });
}

fn update_ecoli(
    mut query: Query<
        (
            &mut EntropyComponent<WyRand>,
            &Transform,
            &mut MovementPhase,
            &mut PreviousConcentration,
            &LogisticFnParam,
        ),
        With<EColi>,
    >,
) {
    query.par_iter_mut().for_each(
        |(mut rng, transform, mut mvmnt_phase, mut prev_concentration, params)| {
            let concentration = transform.translation.length().recip() * 1_000_000. ; //TODO better diffusion
            let delta_concentration = (concentration - prev_concentration.0) * TIMESTEP;
            prev_concentration.0 = concentration;

            //logistic function

            let p = (1.0 + (-params.beta0 - params.beta1 * -delta_concentration).exp()).recip();

            info!("conc: {} p: {}", delta_concentration, p);

            //toggle movement
            if rng.gen_bool(p.into()) {
                *mvmnt_phase = match *mvmnt_phase {
                    MovementPhase::Run(_) => {
                        MovementPhase::Tumble(get_rand_rotation(&mut rng))
                    }
                    MovementPhase::Tumble(_) => {
                        MovementPhase::Run(*transform.forward())
                    }
                }
            }

        },
    )
}



#[inline]
fn get_rand_rotation(rng: &mut EntropyComponent<WyRand>) -> Quat {
    // subgroup algorithm for random quaternion rotation

    let (s1, c1) = rng.gen_range(0f32..TAU).sin_cos();
    let (s2, c2) = rng.gen_range(0f32..TAU).sin_cos();

    let x0 = rng.gen_range(0f32..=1.);
    let (r1, r2) = ((1. - x0).sqrt(), x0.sqrt());

    Quat::from_xyzw(s1 * r1, c1 * r1, s2 * r2, c2 * r2)
}

fn update_position_rotation(mut query: Query<(&MovementPhase, &mut Transform)>, time: Res<Time>) {
    query
        .par_iter_mut()
        .for_each(|(mv_phase, mut transform)| match mv_phase {
            MovementPhase::Run(velocity) => {
                transform.translation += *velocity * (time.delta_seconds() * SPD);
            }
            MovementPhase::Tumble(rotational_velocity) => {
                transform.rotation *= Quat::IDENTITY.lerp(*rotational_velocity, (time.delta_seconds() * SPD));
            }
        })
}
