use xpans_render::host::{InterpretationInfo, ProcessSamples};

use crate::{
    audio_input::AudioInput, audio_input_socket::AudioInputSocket, audio_output::AudioOutput,
    audio_output_socket::AudioOutputSocket, interpretation_storage::InterpretationStorage,
};

/// Uses source interpretations to transform audio samples.
pub trait SampleProcessor<In, Out>
where
    In: AudioInput,
    Out: AudioOutput,
    Self: ProcessSamples<InterpretationStorage, AudioInputSocket<In>, AudioOutputSocket<Out>>
        + InterpretationInfo,
{
}
impl<T, In, Out> SampleProcessor<In, Out> for T
where
    In: AudioInput,
    Out: AudioOutput,
    T: ?Sized
        + ProcessSamples<InterpretationStorage, AudioInputSocket<In>, AudioOutputSocket<Out>>
        + InterpretationInfo,
{
}
