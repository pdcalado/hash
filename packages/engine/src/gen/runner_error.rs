#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::cast_sign_loss,
    clippy::empty_enum,
    clippy::used_underscore_binding,
    clippy::redundant_static_lifetimes,
    clippy::redundant_field_names,
    clippy::unused_imports,
    unused_imports
)]
// automatically generated by the FlatBuffers compiler, do not modify

use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

pub enum RunnerErrorOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RunnerError<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RunnerError<'a> {
    type Inner = RunnerError<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> RunnerError<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        RunnerError { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RunnerErrorArgs<'args>,
    ) -> flatbuffers::WIPOffset<RunnerError<'bldr>> {
        let mut builder = RunnerErrorBuilder::new(_fbb);
        if let Some(x) = args.msg {
            builder.add_msg(x);
        }
        builder.finish()
    }

    pub const VT_MSG: flatbuffers::VOffsetT = 4;

    #[inline]
    pub fn msg(&self) -> Option<&'a str> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<&str>>(RunnerError::VT_MSG, None)
    }
}

impl flatbuffers::Verifiable for RunnerError<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<&str>>(&"msg", Self::VT_MSG, false)?
            .finish();
        Ok(())
    }
}
pub struct RunnerErrorArgs<'a> {
    pub msg: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for RunnerErrorArgs<'a> {
    #[inline]
    fn default() -> Self {
        RunnerErrorArgs { msg: None }
    }
}
pub struct RunnerErrorBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RunnerErrorBuilder<'a, 'b> {
    #[inline]
    pub fn add_msg(&mut self, msg: flatbuffers::WIPOffset<&'b str>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(RunnerError::VT_MSG, msg);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RunnerErrorBuilder<'a, 'b> {
        let start = _fbb.start_table();
        RunnerErrorBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<RunnerError<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for RunnerError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("RunnerError");
        ds.field("msg", &self.msg());
        ds.finish()
    }
}
#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_runner_error<'a>(buf: &'a [u8]) -> RunnerError<'a> {
    unsafe { flatbuffers::root_unchecked::<RunnerError<'a>>(buf) }
}

#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_runner_error<'a>(buf: &'a [u8]) -> RunnerError<'a> {
    unsafe { flatbuffers::size_prefixed_root_unchecked::<RunnerError<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `RunnerError`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_error_unchecked`.
pub fn root_as_runner_error(buf: &[u8]) -> Result<RunnerError, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<RunnerError>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `RunnerError` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_runner_error_unchecked`.
pub fn size_prefixed_root_as_runner_error(
    buf: &[u8],
) -> Result<RunnerError, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<RunnerError>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `RunnerError` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_error_unchecked`.
pub fn root_as_runner_error_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<RunnerError<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<RunnerError<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `RunnerError` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_error_unchecked`.
pub fn size_prefixed_root_as_runner_error_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<RunnerError<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<RunnerError<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a RunnerError and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `RunnerError`.
pub unsafe fn root_as_runner_error_unchecked(buf: &[u8]) -> RunnerError {
    flatbuffers::root_unchecked::<RunnerError>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed RunnerError and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `RunnerError`.
pub unsafe fn size_prefixed_root_as_runner_error_unchecked(buf: &[u8]) -> RunnerError {
    flatbuffers::size_prefixed_root_unchecked::<RunnerError>(buf)
}
#[inline]
pub fn finish_runner_error_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<RunnerError<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_runner_error_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<RunnerError<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
