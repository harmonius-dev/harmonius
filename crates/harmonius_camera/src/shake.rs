//! Shake, noise, impulse, and composite layering.
//!
//! Deterministic noise offsets use summed sinusoids (not gradient Perlin); naming tracks the
//! subsystem vocabulary until a full noise implementation lands.

use glam::Vec3;

use std::f32::consts::TAU;

/// One noise channel definition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoiseChannel {
    /// Frequency (Hz).
    pub frequency: f32,
    /// Amplitude (world units).
    pub amplitude: f32,
}

/// Lightweight summed-sinusoid profile for deterministic tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SinusoidalNoiseProfile {
    /// Primary oscillation channels.
    pub channels: [NoiseChannel; 3],
}

/// Impulse source description (curve sampled linearly).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImpulseSource {
    /// Peak strength at the origin.
    pub peak_strength: f32,
    /// Total duration (seconds).
    pub duration: f32,
    /// Maximum propagation radius (meters).
    pub radius: f32,
}

/// Listener configuration on a virtual camera.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImpulseListener {
    /// Gain multiplier.
    pub gain: f32,
    /// Maximum amplitude clamp.
    pub max_amplitude: f32,
}

/// Sinusoidal oscillation shake.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaveOscillation {
    /// Per-axis position amplitude.
    pub position_amplitude: Vec3,
    /// Per-axis position frequency (Hz).
    pub position_frequency: Vec3,
}

/// One layer in a composite shake stack.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CompositeShakeLayer {
    /// Offset for this layer at the sample time.
    pub offset: Vec3,
    /// When false, the layer is muted.
    pub enabled: bool,
}

/// Deterministic pseudo-noise offset built from sinusoids.
#[must_use]
pub fn evaluate_sinusoidal_noise_offset(
    profile: &SinusoidalNoiseProfile,
    time: f32,
    frequency_gain: f32,
) -> Vec3 {
    let mut out = Vec3::ZERO;
    for channel in &profile.channels {
        let w = TAU * channel.frequency * frequency_gain * time;
        out += Vec3::splat(channel.amplitude * w.sin());
    }
    out
}

/// Applies amplitude gain (0 mutes the profile).
#[must_use]
pub fn evaluate_sinusoidal_noise_with_gain(
    profile: &SinusoidalNoiseProfile,
    time: f32,
    amplitude_gain: f32,
    frequency_gain: f32,
) -> Vec3 {
    evaluate_sinusoidal_noise_offset(profile, time, frequency_gain) * amplitude_gain
}

/// Computes listener shake response with radial falloff.
#[must_use]
pub fn evaluate_impulse_listener_response(
    source: &ImpulseSource,
    listener: &ImpulseListener,
    distance: f32,
    time: f32,
) -> f32 {
    if distance > source.radius || time < 0.0 || time > source.duration {
        return 0.0;
    }
    let falloff = 1.0 - (distance / source.radius.max(1e-4));
    let curve = 1.0 - (time / source.duration.max(1e-4));
    let shake = source.peak_strength * falloff * curve * listener.gain;
    shake.min(listener.max_amplitude).max(0.0)
}

/// Samples wave oscillation displacement on the X axis (first channel).
#[must_use]
pub fn evaluate_wave_oscillation_position(wave: &WaveOscillation, time: f32) -> Vec3 {
    Vec3::new(
        wave.position_amplitude.x * (TAU * wave.position_frequency.x * time).sin(),
        wave.position_amplitude.y * (TAU * wave.position_frequency.y * time).sin(),
        wave.position_amplitude.z * (TAU * wave.position_frequency.z * time).sin(),
    )
}

/// Sums enabled composite shake layers.
#[must_use]
pub fn evaluate_composite_shake(layers: &[CompositeShakeLayer]) -> Vec3 {
    layers
        .iter()
        .filter(|layer| layer.enabled)
        .map(|layer| layer.offset)
        .sum()
}

/// Simple timeline-driven shake envelope for cinematics tests.
#[derive(Clone, Debug, PartialEq)]
pub struct CinematicShakeClip {
    /// Start time (seconds).
    pub start: f32,
    /// End time (seconds).
    pub end: f32,
    /// Peak offset while active.
    pub amplitude: Vec3,
}

/// Evaluates shake offset for a cinematic clip at absolute timeline `time`.
#[must_use]
pub fn evaluate_cinematic_shake_at(clip: &CinematicShakeClip, time: f32) -> Vec3 {
    if time >= clip.start && time <= clip.end {
        clip.amplitude
    } else {
        Vec3::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_profile() -> SinusoidalNoiseProfile {
        SinusoidalNoiseProfile {
            channels: [
                NoiseChannel {
                    frequency: 1.0,
                    amplitude: 1.0,
                },
                NoiseChannel {
                    frequency: 2.0,
                    amplitude: 0.5,
                },
                NoiseChannel {
                    frequency: 3.0,
                    amplitude: 0.25,
                },
            ],
        }
    }

    /// TC-13.25.18.1 — zero amplitude gain silences sinusoidal output.
    #[test]
    fn tc_13_25_18_1_sinusoidal_mute() {
        let profile = sample_profile();
        let offset = evaluate_sinusoidal_noise_with_gain(&profile, 0.4, 0.0, 1.0);
        assert_eq!(offset, Vec3::ZERO);
    }

    /// TC-13.25.18.2 — doubling frequency doubles oscillation rate metric.
    #[test]
    fn tc_13_25_18_2_sinusoidal_frequency_gain() {
        let profile = SinusoidalNoiseProfile {
            channels: [
                NoiseChannel {
                    frequency: 1.0,
                    amplitude: 1.0,
                },
                NoiseChannel {
                    frequency: 0.0,
                    amplitude: 0.0,
                },
                NoiseChannel {
                    frequency: 0.0,
                    amplitude: 0.0,
                },
            ],
        };
        let a = evaluate_sinusoidal_noise_offset(&profile, 0.125, 1.0).x;
        let b = evaluate_sinusoidal_noise_offset(&profile, 0.125, 2.0).x;
        assert!((a - b).abs() > 1e-3);
    }

    /// TC-13.25.19.1 — closer listeners receive stronger impulses.
    #[test]
    fn tc_13_25_19_1_impulse_distance_attenuation() {
        let source = ImpulseSource {
            peak_strength: 10.0,
            duration: 1.0,
            radius: 20.0,
        };
        let listener = ImpulseListener {
            gain: 1.0,
            max_amplitude: 100.0,
        };
        let near = evaluate_impulse_listener_response(&source, &listener, 5.0, 0.2);
        let far = evaluate_impulse_listener_response(&source, &listener, 10.0, 0.2);
        assert!(near > far);
    }

    /// TC-13.25.19.2 — listeners outside the radius receive nothing.
    #[test]
    fn tc_13_25_19_2_impulse_outside_radius() {
        let source = ImpulseSource {
            peak_strength: 10.0,
            duration: 1.0,
            radius: 5.0,
        };
        let listener = ImpulseListener {
            gain: 1.0,
            max_amplitude: 100.0,
        };
        let shake = evaluate_impulse_listener_response(&source, &listener, 6.0, 0.1);
        assert_eq!(shake, 0.0);
    }

    /// TC-13.25.20.1 — sine wave peaks at quarter period for 1 Hz motion.
    #[test]
    fn tc_13_25_20_1_wave_oscillation() {
        let wave = WaveOscillation {
            position_amplitude: Vec3::new(1.0, 0.0, 0.0),
            position_frequency: Vec3::new(1.0, 0.0, 0.0),
        };
        let sample = evaluate_wave_oscillation_position(&wave, 0.25);
        assert!((sample.x - 1.0).abs() < 1e-3);
    }

    /// TC-13.25.21.1 — composite shake sums enabled layers.
    #[test]
    fn tc_13_25_21_1_composite_shake_additive() {
        let layers = [
            CompositeShakeLayer {
                offset: Vec3::X,
                enabled: true,
            },
            CompositeShakeLayer {
                offset: Vec3::Y,
                enabled: true,
            },
            CompositeShakeLayer {
                offset: Vec3::Z,
                enabled: true,
            },
        ];
        let sum = evaluate_composite_shake(&layers);
        assert!((sum - Vec3::new(1.0, 1.0, 1.0)).length() < 1e-3);

        let mut muted = layers;
        muted[1].enabled = false;
        let partial = evaluate_composite_shake(&muted);
        assert!((partial - Vec3::new(1.0, 0.0, 1.0)).length() < 1e-3);
    }

    /// TC-13.25.22.1 — cinematic shake follows timeline bounds without residual offset.
    #[test]
    fn tc_13_25_22_1_cinematic_shake_timeline() {
        let clip = CinematicShakeClip {
            start: 2.0,
            end: 3.0,
            amplitude: Vec3::new(0.0, 0.2, 0.0),
        };
        assert_eq!(evaluate_cinematic_shake_at(&clip, 2.5), clip.amplitude);
        assert_eq!(evaluate_cinematic_shake_at(&clip, 1.5), Vec3::ZERO);
        assert_eq!(evaluate_cinematic_shake_at(&clip, 3.5), Vec3::ZERO);
        // Scrub backwards across the clip end — no residual offset remains.
        assert_eq!(evaluate_cinematic_shake_at(&clip, 3.6), Vec3::ZERO);
    }
}
