use nih_plug::prelude::*;
use nih_plug_webview::WebViewEditor;
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

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        let editor = WebViewEditor::new(
            nih_plug_webview::HTMLSource::String(include_str!("horse.html")),
            (256, 256),
        )
        .with_background_color((200, 0, 0, 255))
        .with_event_loop(move |ctx, setter, _window| {
            while let Ok(event) = ctx.next_event() {
                if let Some(value) = event.get("data").map(|a| a.as_f64()).flatten() {
                    setter.begin_set_parameter(&params.horse);
                    setter.set_parameter_normalized(&params.horse, value as f32);
                    setter.end_set_parameter(&params.horse);
                } else {
                    ctx.send_json(event);
                }
            }
        });

        Some(Box::new(editor))
    }
}

impl Vst3Plugin for EvilHorseVst {
    const VST3_CLASS_ID: [u8; 16] = *b"EvilHorseVsttttt";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_vst3!(EvilHorseVst);
