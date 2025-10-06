use nih_plug::prelude::*;
use std::sync::Arc;

struct EvilHorseVst {
    params: Arc<EvilHorseVstParams>,
}

#[derive(Params)]
struct EvilHorseVstParams {
    #[id = "horse"]
    pub horse: FloatParam,
}

impl Default for EvilHorseVst {
    fn default() -> Self {
        Self {
            params: Arc::new(EvilHorseVstParams::default()),
        }
    }
}

impl Default for EvilHorseVstParams {
    fn default() -> Self {
        Self {
            horse: FloatParam::new("Horse", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
        }
    }
}

impl Plugin for EvilHorseVst {
    const VENDOR: &'static str = env!("CARGO_PKG_AUTHORS");
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "evil@horse.vst";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),

            aux_input_ports: &[],
            aux_output_ports: &[],

            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let horse = self.params.horse.smoothed.next();

            for sample in channel_samples {
                if *sample < horse {
                    *sample = (sample.cos().abs()) % *sample;
                } else {
                    *sample *= horse;
                }
            }
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl ClapPlugin for EvilHorseVst {
    const CLAP_ID: &'static str = "space.uoxide.evil-horse-vst";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("horse");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for EvilHorseVst {
    const VST3_CLASS_ID: [u8; 16] = *b"EvilHorseVsttttt";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(EvilHorseVst);
nih_export_vst3!(EvilHorseVst);
