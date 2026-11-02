use std::ops::Mul;

use num::cast::AsPrimitive;
use xpans_render::host::{BackwardIndex, FractionalIndex, SampleRate};

use crate::audio_input::{AudioInput, BufferedAudioInput, FractionalAudioInput};

/// Holds the audio input and additional state used to read samples from it.
pub struct AudioInputSocket<Input>
where
    Input: AudioInput,
{
    input: Input,
    frames: usize,
}

impl<Input> AudioInputSocket<Input>
where
    Input: AudioInput,
{
    pub(crate) fn new(input: Input) -> Self {
        Self { input, frames: 0 }
    }
    pub(crate) fn input(&self) -> &Input {
        &self.input
    }
    pub(crate) fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub(crate) fn set_frames(&mut self, frames: usize) {
        self.frames = frames;
    }
}

impl<Input> xpans_render::host::Input<Input::Sample> for AudioInputSocket<Input>
where
    Input: AudioInput,
{
    fn frames(&self) -> usize {
        self.frames
    }
    fn sources(&self) -> usize {
        self.input().channel_count()
    }
    fn sample(&self, frame: usize, source: usize) -> Input::Sample {
        self.input.sample(source, frame)
    }
}

impl<Input: AudioInput> SampleRate for AudioInputSocket<Input> {
    fn sample_rate(&self) -> u32 {
        self.input().sample_rate()
    }
}

impl<Input> BackwardIndex<Input::Sample> for AudioInputSocket<Input>
where
    Input: BufferedAudioInput,
{
    fn relative_sample(&self, frame: usize, source: usize, index: isize) -> Input::Sample {
        self.input.buffered_sample(source, frame, index)
    }

    fn previous_frames(&self) -> usize {
        self.input().max_delay_length()
    }
}
impl<Input> FractionalIndex<Input::Sample, Input::Sample> for AudioInputSocket<Input>
where
    Input: FractionalAudioInput,
    Input::Sample: Copy + 'static + Mul<Output = Input::Sample>,
    usize: AsPrimitive<Input::Sample>,
{
    fn fractional_sample(
        &self,
        frame: usize,
        source: usize,
        index: Input::Sample,
    ) -> Input::Sample {
        self.input.fractional_sample(source, frame, index)
    }
}
