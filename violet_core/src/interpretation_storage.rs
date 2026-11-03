use core::num::NonZeroUsize;
use xpans_render::host;

pub struct InterpretationStorage {
    data: Box<[u8]>,
    length: usize,
    frames: usize,
    sources: usize,
    max_frames: usize,
}

unsafe impl host::Interpretations for InterpretationStorage {
    fn frames(&self) -> usize {
        self.frames
    }
    fn sources(&self) -> usize {
        self.sources
    }
    fn length(&self) -> usize {
        self.length
    }
    unsafe fn get<T>(&self, frame: usize, source: usize, index: usize) -> &T {
        let frame_start = self.length * self.sources * frame;
        let source_start = frame_start + (self.length * source);
        &self.typed_slice::<T>()[source_start + index]
    }
    unsafe fn get_mut<T>(&mut self, frame: usize, source: usize, index: usize) -> &mut T {
        let frame_start = self.length * self.sources * frame;
        let source_start = frame_start + (self.length * source);
        &mut self.typed_slice_mut::<T>()[source_start + index]
    }
}

impl InterpretationStorage {
    pub(crate) fn new(
        info: &impl host::InterpretationInfo,
        sources: usize,
        max_frames: usize,
    ) -> Self {
        let length = info.interpretation_length();
        let layout = info.interpretation_layout();
        let mut data = vec![0_u8; sources * length * max_frames * layout.size()].into_boxed_slice();
        for chunk in data.chunks_exact_mut(layout.size()) {
            unsafe {
                info.initialize_interpretation(chunk);
            }
        }
        Self {
            data,
            length,
            frames: 0,
            sources,
            max_frames,
        }
    }
    fn typed_slice<T: Sized>(&self) -> &[T] {
        let ptr = self.data.as_ptr() as *const T;
        let type_size = size_of::<T>();
        let len = self.data.len() / type_size;
        unsafe { core::slice::from_raw_parts(ptr, len) }
    }
    fn typed_slice_mut<T: Sized>(&mut self) -> &mut [T] {
        let ptr = self.data.as_mut_ptr() as *mut T;
        let type_size = size_of::<T>();
        let len = self.data.len() / type_size;
        unsafe { core::slice::from_raw_parts_mut(ptr, len) }
    }
    pub(crate) fn set_frames(&mut self, frames: NonZeroUsize) {
        self.frames = frames.get();
    }
    pub(crate) fn max_frames(&self) -> usize {
        self.max_frames
    }
}
