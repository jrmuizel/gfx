// Copyright 2017 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Memory stuff

use bitflags;
use {Resources};

/// A trait for plain-old-data types.
///
/// A POD type does not have invalid bit patterns and can be safely
/// created from arbitrary bit pattern.
pub unsafe trait Pod {}

macro_rules! impl_pod {
    ( ty = $($ty:ty)* ) => { $( unsafe impl Pod for $ty {} )* };
    ( ar = $($tt:expr)* ) => { $( unsafe impl<T: Pod> Pod for [T; $tt] {} )* };
}

impl_pod! { ty = isize usize i8 u8 i16 u16 i32 u32 i64 u64 f32 f64 }
impl_pod! { ar =
    0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
}

unsafe impl<T: Pod, U: Pod> Pod for (T, U) {}

bitflags!(
    // TODO
    pub flags ImageAccess: u16 {
        const RENDER_TARGET_CLEAR = 0x20,
        const RESOLVE_SRC         = 0x100,
        const RESOLVE_DST         = 0x200,
        const COLOR_ATTACHMENT_READ = 0x1,
        const COLOR_ATTACHMENT_WRITE = 0x2,
    }
);

bitflags!(
    pub flags BufferState: u16 {
        const INDEX_BUFFER_READ      = 0x1,
        const VERTEX_BUFFER_READ     = 0x2,
        const CONSTANT_BUFFER_READ   = 0x4,
        const INDIRECT_COMMAND_READ  = 0x8,
    }
);

#[derive(Copy, Clone, Debug)]
pub enum ImageLayout {
    General,
    ColorAttachmentOptimal,
    DepthStencilAttachmentOptimal,
    DepthStencilReadOnlyOptimal,
    ShaderReadOnlyOptimal,
    TransferSrcOptimal,
    TransferDstOptimal,
    Undefined,
    Preinitialized,
    Present,
}

#[derive(Copy, Clone, Debug)]
pub enum ImageStateSrc {
    Present(ImageAccess), // exclusive state
    State(ImageAccess, ImageLayout),
}

#[derive(Copy, Clone, Debug)]
pub enum ImageStateDst {
    Present,
    State(ImageAccess, ImageLayout),
}

pub struct ImageSubResource {

}

// TODO: probably remove this
pub struct MemoryBarrier;

pub struct BufferBarrier<'a, R: Resources> {
    pub state_src: BufferState,
    pub state_dst: BufferState,

    pub buffer: &'a R::Buffer,
    pub offset: usize,
    pub size: usize,
}

pub struct ImageBarrier<'a, R: Resources> {
    pub state_src: ImageStateSrc,
    pub state_dst: ImageStateDst,

    pub image: &'a R::Image,
}
