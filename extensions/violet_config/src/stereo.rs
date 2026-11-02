use violet_core::spatial_input::SpatialInput;
use violet_core::{SampleProcessor, SourceInterpreter};
use violet_core::{audio_input::AudioInput, audio_output::AudioOutput};
use xpans_renderconfig::stereo::{Stereo, StereoMode};

use crate::{BuildInterpreter, BuildProcessor, GetDelaySamples, GetOutputChannels, pan_law};

impl<In> BuildInterpreter<In> for Stereo
where
    In: SpatialInput<Scalar = f32>,
{
    fn build_interpreter(&self) -> Box<dyn SourceInterpreter<In>> {
        match self.mode {
            StereoMode::Directional => Box::new(xpans_stereo::Directional::default()),
            StereoMode::Positional => Box::new(xpans_stereo::Positional::default()),
        }
    }
}

impl GetOutputChannels for Stereo {
    fn get_output_channels(&self) -> usize {
        2
    }
}

impl GetDelaySamples for Stereo {
    fn get_delay_samples(&self, _sample_rate: u32) -> usize {
        0
    }
}

impl<In, Out> BuildProcessor<In, Out> for Stereo
where
    In: AudioInput<Sample = f32>,
    Out: AudioOutput<Sample = f32>,
{
    fn build_processor(&self) -> Box<dyn SampleProcessor<In, Out> + Send> {
        Box::new(xpans_stereo::Processor::new(pan_law(self.pan_law)))
    }
}
