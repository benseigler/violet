use core::num::NonZeroUsize;
use xpans_render::mode::Sources;

use crate::{Source, spatial_input::SpatialInput};

pub struct SpatialInputSocket<Input> {
    input: Input,
    frames: usize,
}

impl<Input> SpatialInputSocket<Input> {
    pub(crate) fn new(input: Input) -> Self {
        Self { input, frames: 0 }
    }
    pub(crate) fn input(&self) -> &Input {
        &self.input
    }
    pub(crate) fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }
    pub(crate) fn set_frames(&mut self, frames: NonZeroUsize) {
        self.frames = frames.get();
    }
}

impl<Input, T> Sources for SpatialInputSocket<Input>
where
    Input: SpatialInput<Scalar = T>,
{
    type Source = Source<T>;

    fn sources(&self) -> usize {
        self.input.channel_count()
    }

    fn frames(&self) -> usize {
        self.frames
    }

    fn get(&self, frame: usize, source: usize) -> Self::Source {
        self.input.source(source, frame)
    }
}
