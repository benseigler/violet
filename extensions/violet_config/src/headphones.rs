use violet_core::{
    SampleProcessor, SourceInterpreter, audio_input::FractionalAudioInput,
    audio_output::AudioOutput, spatial_input::SpatialInput,
};
use xpans_headphones::{calculate_delay_samples, distance::DistanceCurve as DistanceCurveTrait};
use xpans_renderconfig::headphones::{DistanceCurve, Headphones};

use crate::{BuildInterpreter, BuildProcessor, GetDelaySamples, GetOutputChannels, pan_law};

impl<In> BuildInterpreter<In> for Headphones
where
    In: SpatialInput<Scalar = f32>,
{
    fn build_interpreter(&self) -> Box<dyn SourceInterpreter<In>> {
        Box::new(xpans_headphones::Interpreter::new())
    }
}

impl<In, Out> BuildProcessor<In, Out> for Headphones
where
    In: FractionalAudioInput<Sample = f32>,
    Out: AudioOutput<Sample = f32>,
{
    fn build_processor(&self) -> Box<dyn SampleProcessor<In, Out> + Send> {
        let processor = xpans_headphones::Processor::new(
            pan_law(self.pan_law),
            self.max_itd_nanos,
            distance_curve(self.distance_curve),
            self.distance_effect,
            self.min_distance,
            self.max_distance,
        );
        Box::new(processor)
    }
}

impl GetOutputChannels for Headphones {
    fn get_output_channels(&self) -> usize {
        2
    }
}

impl GetDelaySamples for Headphones {
    fn get_delay_samples(&self, sample_rate: u32) -> usize {
        calculate_delay_samples(self.max_itd_nanos, sample_rate)
    }
}

fn distance_curve(distance_curve: DistanceCurve) -> Box<dyn DistanceCurveTrait<f32> + Send> {
    use xpans_headphones::distance::{Exponential, Linear, Sine, SquareRoot};
    match distance_curve {
        DistanceCurve::Linear => Box::new(Linear),
        DistanceCurve::Exponential => Box::new(Exponential),
        DistanceCurve::Sine => Box::new(Sine),
        DistanceCurve::SquareRoot => Box::new(SquareRoot),
    }
}
