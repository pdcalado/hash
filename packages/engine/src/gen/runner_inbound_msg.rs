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

use super::batch::*;
use super::metaversion::*;
use super::new_simulation_run::*;
use super::package_config::*;
use super::serialized::*;
use super::shared_context::*;
use super::sync_context_batch::*;
use super::sync_state::*;
use super::sync_state_interim::*;
use super::sync_state_snapshot::*;
use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MIN_RUNNER_INBOUND_MSG_PAYLOAD: u8 = 0;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MAX_RUNNER_INBOUND_MSG_PAYLOAD: u8 = 9;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_RUNNER_INBOUND_MSG_PAYLOAD: [RunnerInboundMsgPayload; 10] = [
    RunnerInboundMsgPayload::NONE,
    RunnerInboundMsgPayload::TaskMsg,
    RunnerInboundMsgPayload::CancelTask,
    RunnerInboundMsgPayload::StateSync,
    RunnerInboundMsgPayload::StateSnapshotSync,
    RunnerInboundMsgPayload::ContextBatchSync,
    RunnerInboundMsgPayload::StateInterimSync,
    RunnerInboundMsgPayload::TerminateSimulationRun,
    RunnerInboundMsgPayload::KillRunner,
    RunnerInboundMsgPayload::NewSimulationRun,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct RunnerInboundMsgPayload(pub u8);
#[allow(non_upper_case_globals)]
impl RunnerInboundMsgPayload {
    pub const NONE: Self = Self(0);
    pub const TaskMsg: Self = Self(1);
    pub const CancelTask: Self = Self(2);
    pub const StateSync: Self = Self(3);
    pub const StateSnapshotSync: Self = Self(4);
    pub const ContextBatchSync: Self = Self(5);
    pub const StateInterimSync: Self = Self(6);
    pub const TerminateSimulationRun: Self = Self(7);
    pub const KillRunner: Self = Self(8);
    pub const NewSimulationRun: Self = Self(9);

    pub const ENUM_MIN: u8 = 0;
    pub const ENUM_MAX: u8 = 9;
    pub const ENUM_VALUES: &'static [Self] = &[
        Self::NONE,
        Self::TaskMsg,
        Self::CancelTask,
        Self::StateSync,
        Self::StateSnapshotSync,
        Self::ContextBatchSync,
        Self::StateInterimSync,
        Self::TerminateSimulationRun,
        Self::KillRunner,
        Self::NewSimulationRun,
    ];
    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::NONE => Some("NONE"),
            Self::TaskMsg => Some("TaskMsg"),
            Self::CancelTask => Some("CancelTask"),
            Self::StateSync => Some("StateSync"),
            Self::StateSnapshotSync => Some("StateSnapshotSync"),
            Self::ContextBatchSync => Some("ContextBatchSync"),
            Self::StateInterimSync => Some("StateInterimSync"),
            Self::TerminateSimulationRun => Some("TerminateSimulationRun"),
            Self::KillRunner => Some("KillRunner"),
            Self::NewSimulationRun => Some("NewSimulationRun"),
            _ => None,
        }
    }
}
impl std::fmt::Debug for RunnerInboundMsgPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}
impl<'a> flatbuffers::Follow<'a> for RunnerInboundMsgPayload {
    type Inner = Self;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { flatbuffers::read_scalar_at::<u8>(buf, loc) };
        Self(b)
    }
}

impl flatbuffers::Push for RunnerInboundMsgPayload {
    type Output = RunnerInboundMsgPayload;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe {
            flatbuffers::emplace_scalar::<u8>(dst, self.0);
        }
    }
}

impl flatbuffers::EndianScalar for RunnerInboundMsgPayload {
    #[inline]
    fn to_little_endian(self) -> Self {
        let b = u8::to_le(self.0);
        Self(b)
    }
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_little_endian(self) -> Self {
        let b = u8::from_le(self.0);
        Self(b)
    }
}

impl<'a> flatbuffers::Verifiable for RunnerInboundMsgPayload {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        u8::run_verifier(v, pos)
    }
}

impl flatbuffers::SimpleToVerifyInSlice for RunnerInboundMsgPayload {}
pub struct RunnerInboundMsgPayloadUnionTableOffset {}

// struct TaskID, aligned to 1
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct TaskID(pub [u8; 16]);
impl Default for TaskID {
    fn default() -> Self {
        Self([0; 16])
    }
}
impl std::fmt::Debug for TaskID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaskID")
            .field("inner", &self.inner())
            .finish()
    }
}

impl flatbuffers::SimpleToVerifyInSlice for TaskID {}
impl flatbuffers::SafeSliceAccess for TaskID {}
impl<'a> flatbuffers::Follow<'a> for TaskID {
    type Inner = &'a TaskID;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        <&'a TaskID>::follow(buf, loc)
    }
}
impl<'a> flatbuffers::Follow<'a> for &'a TaskID {
    type Inner = &'a TaskID;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        flatbuffers::follow_cast_ref::<TaskID>(buf, loc)
    }
}
impl<'b> flatbuffers::Push for TaskID {
    type Output = TaskID;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const TaskID as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b TaskID {
    type Output = TaskID;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const TaskID as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for TaskID {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.in_buffer::<Self>(pos)
    }
}
impl<'a> TaskID {
    #[allow(clippy::too_many_arguments)]
    pub fn new(inner: &[i8; 16]) -> Self {
        let mut s = Self([0; 16]);
        s.set_inner(&inner);
        s
    }

    pub fn inner(&'a self) -> flatbuffers::Array<'a, i8, 16> {
        flatbuffers::Array::follow(&self.0, 0)
    }

    pub fn set_inner(&mut self, items: &[i8; 16]) {
        flatbuffers::emplace_scalar_array(&mut self.0, 0, items);
    }
}

pub enum TaskMsgOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TaskMsg<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TaskMsg<'a> {
    type Inner = TaskMsg<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> TaskMsg<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TaskMsg { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TaskMsgArgs<'args>,
    ) -> flatbuffers::WIPOffset<TaskMsg<'bldr>> {
        let mut builder = TaskMsgBuilder::new(_fbb);
        if let Some(x) = args.payload {
            builder.add_payload(x);
        }
        if let Some(x) = args.metaversioning {
            builder.add_metaversioning(x);
        }
        if let Some(x) = args.task_id {
            builder.add_task_id(x);
        }
        builder.add_package_sid(args.package_sid);
        builder.finish()
    }

    pub const VT_PACKAGE_SID: flatbuffers::VOffsetT = 4;
    pub const VT_TASK_ID: flatbuffers::VOffsetT = 6;
    pub const VT_METAVERSIONING: flatbuffers::VOffsetT = 8;
    pub const VT_PAYLOAD: flatbuffers::VOffsetT = 10;

    #[inline]
    pub fn package_sid(&self) -> u16 {
        self._tab
            .get::<u16>(TaskMsg::VT_PACKAGE_SID, Some(0))
            .unwrap()
    }
    #[inline]
    pub fn task_id(&self) -> Option<&'a TaskID> {
        self._tab.get::<TaskID>(TaskMsg::VT_TASK_ID, None)
    }
    #[inline]
    pub fn metaversioning(&self) -> Option<StateInterimSync<'a>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<StateInterimSync>>(TaskMsg::VT_METAVERSIONING, None)
    }
    #[inline]
    pub fn payload(&self) -> Serialized<'a> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<Serialized>>(TaskMsg::VT_PAYLOAD, None)
            .unwrap()
    }
}

impl flatbuffers::Verifiable for TaskMsg<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u16>(&"package_sid", Self::VT_PACKAGE_SID, false)?
            .visit_field::<TaskID>(&"task_id", Self::VT_TASK_ID, false)?
            .visit_field::<flatbuffers::ForwardsUOffset<StateInterimSync>>(
                &"metaversioning",
                Self::VT_METAVERSIONING,
                false,
            )?
            .visit_field::<flatbuffers::ForwardsUOffset<Serialized>>(
                &"payload",
                Self::VT_PAYLOAD,
                true,
            )?
            .finish();
        Ok(())
    }
}
pub struct TaskMsgArgs<'a> {
    pub package_sid: u16,
    pub task_id: Option<&'a TaskID>,
    pub metaversioning: Option<flatbuffers::WIPOffset<StateInterimSync<'a>>>,
    pub payload: Option<flatbuffers::WIPOffset<Serialized<'a>>>,
}
impl<'a> Default for TaskMsgArgs<'a> {
    #[inline]
    fn default() -> Self {
        TaskMsgArgs {
            package_sid: 0,
            task_id: None,
            metaversioning: None,
            payload: None, // required field
        }
    }
}
pub struct TaskMsgBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TaskMsgBuilder<'a, 'b> {
    #[inline]
    pub fn add_package_sid(&mut self, package_sid: u16) {
        self.fbb_
            .push_slot::<u16>(TaskMsg::VT_PACKAGE_SID, package_sid, 0);
    }
    #[inline]
    pub fn add_task_id(&mut self, task_id: &TaskID) {
        self.fbb_
            .push_slot_always::<&TaskID>(TaskMsg::VT_TASK_ID, task_id);
    }
    #[inline]
    pub fn add_metaversioning(
        &mut self,
        metaversioning: flatbuffers::WIPOffset<StateInterimSync<'b>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<StateInterimSync>>(
                TaskMsg::VT_METAVERSIONING,
                metaversioning,
            );
    }
    #[inline]
    pub fn add_payload(&mut self, payload: flatbuffers::WIPOffset<Serialized<'b>>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<Serialized>>(TaskMsg::VT_PAYLOAD, payload);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TaskMsgBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TaskMsgBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<TaskMsg<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_.required(o, TaskMsg::VT_PAYLOAD, "payload");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for TaskMsg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("TaskMsg");
        ds.field("package_sid", &self.package_sid());
        ds.field("task_id", &self.task_id());
        ds.field("metaversioning", &self.metaversioning());
        ds.field("payload", &self.payload());
        ds.finish()
    }
}
pub enum CancelTaskOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct CancelTask<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for CancelTask<'a> {
    type Inner = CancelTask<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> CancelTask<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        CancelTask { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args CancelTaskArgs<'args>,
    ) -> flatbuffers::WIPOffset<CancelTask<'bldr>> {
        let mut builder = CancelTaskBuilder::new(_fbb);
        if let Some(x) = args.task_id {
            builder.add_task_id(x);
        }
        builder.finish()
    }

    pub const VT_TASK_ID: flatbuffers::VOffsetT = 4;

    #[inline]
    pub fn task_id(&self) -> Option<&'a TaskID> {
        self._tab.get::<TaskID>(CancelTask::VT_TASK_ID, None)
    }
}

impl flatbuffers::Verifiable for CancelTask<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<TaskID>(&"task_id", Self::VT_TASK_ID, false)?
            .finish();
        Ok(())
    }
}
pub struct CancelTaskArgs<'a> {
    pub task_id: Option<&'a TaskID>,
}
impl<'a> Default for CancelTaskArgs<'a> {
    #[inline]
    fn default() -> Self {
        CancelTaskArgs { task_id: None }
    }
}
pub struct CancelTaskBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> CancelTaskBuilder<'a, 'b> {
    #[inline]
    pub fn add_task_id(&mut self, task_id: &TaskID) {
        self.fbb_
            .push_slot_always::<&TaskID>(CancelTask::VT_TASK_ID, task_id);
    }
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> CancelTaskBuilder<'a, 'b> {
        let start = _fbb.start_table();
        CancelTaskBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<CancelTask<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for CancelTask<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("CancelTask");
        ds.field("task_id", &self.task_id());
        ds.finish()
    }
}
pub enum KillRunnerOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct KillRunner<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for KillRunner<'a> {
    type Inner = KillRunner<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> KillRunner<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        KillRunner { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        _args: &'args KillRunnerArgs,
    ) -> flatbuffers::WIPOffset<KillRunner<'bldr>> {
        let mut builder = KillRunnerBuilder::new(_fbb);
        builder.finish()
    }
}

impl flatbuffers::Verifiable for KillRunner<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?.finish();
        Ok(())
    }
}
pub struct KillRunnerArgs {}
impl<'a> Default for KillRunnerArgs {
    #[inline]
    fn default() -> Self {
        KillRunnerArgs {}
    }
}
pub struct KillRunnerBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> KillRunnerBuilder<'a, 'b> {
    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> KillRunnerBuilder<'a, 'b> {
        let start = _fbb.start_table();
        KillRunnerBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<KillRunner<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for KillRunner<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("KillRunner");
        ds.finish()
    }
}
pub enum TerminateSimulationRunOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TerminateSimulationRun<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TerminateSimulationRun<'a> {
    type Inner = TerminateSimulationRun<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> TerminateSimulationRun<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TerminateSimulationRun { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        _args: &'args TerminateSimulationRunArgs,
    ) -> flatbuffers::WIPOffset<TerminateSimulationRun<'bldr>> {
        let mut builder = TerminateSimulationRunBuilder::new(_fbb);
        builder.finish()
    }
}

impl flatbuffers::Verifiable for TerminateSimulationRun<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?.finish();
        Ok(())
    }
}
pub struct TerminateSimulationRunArgs {}
impl<'a> Default for TerminateSimulationRunArgs {
    #[inline]
    fn default() -> Self {
        TerminateSimulationRunArgs {}
    }
}
pub struct TerminateSimulationRunBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TerminateSimulationRunBuilder<'a, 'b> {
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> TerminateSimulationRunBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TerminateSimulationRunBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<TerminateSimulationRun<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for TerminateSimulationRun<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("TerminateSimulationRun");
        ds.finish()
    }
}
pub enum RunnerInboundMsgOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RunnerInboundMsg<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RunnerInboundMsg<'a> {
    type Inner = RunnerInboundMsg<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> RunnerInboundMsg<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        RunnerInboundMsg { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RunnerInboundMsgArgs,
    ) -> flatbuffers::WIPOffset<RunnerInboundMsg<'bldr>> {
        let mut builder = RunnerInboundMsgBuilder::new(_fbb);
        if let Some(x) = args.payload {
            builder.add_payload(x);
        }
        builder.add_sim_sid(args.sim_sid);
        builder.add_payload_type(args.payload_type);
        builder.finish()
    }

    pub const VT_SIM_SID: flatbuffers::VOffsetT = 4;
    pub const VT_PAYLOAD_TYPE: flatbuffers::VOffsetT = 6;
    pub const VT_PAYLOAD: flatbuffers::VOffsetT = 8;

    #[inline]
    pub fn sim_sid(&self) -> u32 {
        self._tab
            .get::<u32>(RunnerInboundMsg::VT_SIM_SID, Some(0))
            .unwrap()
    }
    #[inline]
    pub fn payload_type(&self) -> RunnerInboundMsgPayload {
        self._tab
            .get::<RunnerInboundMsgPayload>(
                RunnerInboundMsg::VT_PAYLOAD_TYPE,
                Some(RunnerInboundMsgPayload::NONE),
            )
            .unwrap()
    }
    #[inline]
    pub fn payload(&self) -> flatbuffers::Table<'a> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(
                RunnerInboundMsg::VT_PAYLOAD,
                None,
            )
            .unwrap()
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_task_msg(&self) -> Option<TaskMsg<'a>> {
        if self.payload_type() == RunnerInboundMsgPayload::TaskMsg {
            let u = self.payload();
            Some(TaskMsg::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_cancel_task(&self) -> Option<CancelTask<'a>> {
        if self.payload_type() == RunnerInboundMsgPayload::CancelTask {
            let u = self.payload();
            Some(CancelTask::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_state_sync(&self) -> Option<StateSync<'a>> {
        if self.payload_type() == RunnerInboundMsgPayload::StateSync {
            let u = self.payload();
            Some(StateSync::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_state_snapshot_sync(&self) -> Option<StateSnapshotSync<'a>> {
        if self.payload_type() == RunnerInboundMsgPayload::StateSnapshotSync {
            let u = self.payload();
            Some(StateSnapshotSync::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_context_batch_sync(&self) -> Option<ContextBatchSync<'a>> {
        if self.payload_type() == RunnerInboundMsgPayload::ContextBatchSync {
            let u = self.payload();
            Some(ContextBatchSync::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_state_interim_sync(&self) -> Option<StateInterimSync<'a>> {
        if self.payload_type() == RunnerInboundMsgPayload::StateInterimSync {
            let u = self.payload();
            Some(StateInterimSync::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_terminate_simulation_run(&self) -> Option<TerminateSimulationRun<'a>> {
        if self.payload_type() == RunnerInboundMsgPayload::TerminateSimulationRun {
            let u = self.payload();
            Some(TerminateSimulationRun::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_kill_runner(&self) -> Option<KillRunner<'a>> {
        if self.payload_type() == RunnerInboundMsgPayload::KillRunner {
            let u = self.payload();
            Some(KillRunner::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn payload_as_new_simulation_run(&self) -> Option<NewSimulationRun<'a>> {
        if self.payload_type() == RunnerInboundMsgPayload::NewSimulationRun {
            let u = self.payload();
            Some(NewSimulationRun::init_from_table(u))
        } else {
            None
        }
    }
}

impl flatbuffers::Verifiable for RunnerInboundMsg<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<u32>(&"sim_sid", Self::VT_SIM_SID, false)?
            .visit_union::<RunnerInboundMsgPayload, _>(
                &"payload_type",
                Self::VT_PAYLOAD_TYPE,
                &"payload",
                Self::VT_PAYLOAD,
                true,
                |key, v, pos| match key {
                    RunnerInboundMsgPayload::TaskMsg => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<TaskMsg>>(
                            "RunnerInboundMsgPayload::TaskMsg",
                            pos,
                        ),
                    RunnerInboundMsgPayload::CancelTask => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<CancelTask>>(
                            "RunnerInboundMsgPayload::CancelTask",
                            pos,
                        ),
                    RunnerInboundMsgPayload::StateSync => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<StateSync>>(
                            "RunnerInboundMsgPayload::StateSync",
                            pos,
                        ),
                    RunnerInboundMsgPayload::StateSnapshotSync => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<StateSnapshotSync>>(
                            "RunnerInboundMsgPayload::StateSnapshotSync",
                            pos,
                        ),
                    RunnerInboundMsgPayload::ContextBatchSync => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<ContextBatchSync>>(
                            "RunnerInboundMsgPayload::ContextBatchSync",
                            pos,
                        ),
                    RunnerInboundMsgPayload::StateInterimSync => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<StateInterimSync>>(
                            "RunnerInboundMsgPayload::StateInterimSync",
                            pos,
                        ),
                    RunnerInboundMsgPayload::TerminateSimulationRun => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<
                        TerminateSimulationRun,
                    >>(
                        "RunnerInboundMsgPayload::TerminateSimulationRun",
                        pos,
                    ),
                    RunnerInboundMsgPayload::KillRunner => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<KillRunner>>(
                            "RunnerInboundMsgPayload::KillRunner",
                            pos,
                        ),
                    RunnerInboundMsgPayload::NewSimulationRun => v
                        .verify_union_variant::<flatbuffers::ForwardsUOffset<NewSimulationRun>>(
                            "RunnerInboundMsgPayload::NewSimulationRun",
                            pos,
                        ),
                    _ => Ok(()),
                },
            )?
            .finish();
        Ok(())
    }
}
pub struct RunnerInboundMsgArgs {
    pub sim_sid: u32,
    pub payload_type: RunnerInboundMsgPayload,
    pub payload: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for RunnerInboundMsgArgs {
    #[inline]
    fn default() -> Self {
        RunnerInboundMsgArgs {
            sim_sid: 0,
            payload_type: RunnerInboundMsgPayload::NONE,
            payload: None, // required field
        }
    }
}
pub struct RunnerInboundMsgBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RunnerInboundMsgBuilder<'a, 'b> {
    #[inline]
    pub fn add_sim_sid(&mut self, sim_sid: u32) {
        self.fbb_
            .push_slot::<u32>(RunnerInboundMsg::VT_SIM_SID, sim_sid, 0);
    }
    #[inline]
    pub fn add_payload_type(&mut self, payload_type: RunnerInboundMsgPayload) {
        self.fbb_.push_slot::<RunnerInboundMsgPayload>(
            RunnerInboundMsg::VT_PAYLOAD_TYPE,
            payload_type,
            RunnerInboundMsgPayload::NONE,
        );
    }
    #[inline]
    pub fn add_payload(&mut self, payload: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(RunnerInboundMsg::VT_PAYLOAD, payload);
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> RunnerInboundMsgBuilder<'a, 'b> {
        let start = _fbb.start_table();
        RunnerInboundMsgBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<RunnerInboundMsg<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_
            .required(o, RunnerInboundMsg::VT_PAYLOAD, "payload");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for RunnerInboundMsg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("RunnerInboundMsg");
        ds.field("sim_sid", &self.sim_sid());
        ds.field("payload_type", &self.payload_type());
        match self.payload_type() {
            RunnerInboundMsgPayload::TaskMsg => {
                if let Some(x) = self.payload_as_task_msg() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerInboundMsgPayload::CancelTask => {
                if let Some(x) = self.payload_as_cancel_task() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerInboundMsgPayload::StateSync => {
                if let Some(x) = self.payload_as_state_sync() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerInboundMsgPayload::StateSnapshotSync => {
                if let Some(x) = self.payload_as_state_snapshot_sync() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerInboundMsgPayload::ContextBatchSync => {
                if let Some(x) = self.payload_as_context_batch_sync() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerInboundMsgPayload::StateInterimSync => {
                if let Some(x) = self.payload_as_state_interim_sync() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerInboundMsgPayload::TerminateSimulationRun => {
                if let Some(x) = self.payload_as_terminate_simulation_run() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerInboundMsgPayload::KillRunner => {
                if let Some(x) = self.payload_as_kill_runner() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            RunnerInboundMsgPayload::NewSimulationRun => {
                if let Some(x) = self.payload_as_new_simulation_run() {
                    ds.field("payload", &x)
                } else {
                    ds.field(
                        "payload",
                        &"InvalidFlatbuffer: Union discriminant does not match value.",
                    )
                }
            }
            _ => {
                let x: Option<()> = None;
                ds.field("payload", &x)
            }
        };
        ds.finish()
    }
}
#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_runner_inbound_msg<'a>(buf: &'a [u8]) -> RunnerInboundMsg<'a> {
    unsafe { flatbuffers::root_unchecked::<RunnerInboundMsg<'a>>(buf) }
}

#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_runner_inbound_msg<'a>(buf: &'a [u8]) -> RunnerInboundMsg<'a> {
    unsafe { flatbuffers::size_prefixed_root_unchecked::<RunnerInboundMsg<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `RunnerInboundMsg`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_inbound_msg_unchecked`.
pub fn root_as_runner_inbound_msg(
    buf: &[u8],
) -> Result<RunnerInboundMsg, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<RunnerInboundMsg>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `RunnerInboundMsg` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_runner_inbound_msg_unchecked`.
pub fn size_prefixed_root_as_runner_inbound_msg(
    buf: &[u8],
) -> Result<RunnerInboundMsg, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<RunnerInboundMsg>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `RunnerInboundMsg` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_inbound_msg_unchecked`.
pub fn root_as_runner_inbound_msg_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<RunnerInboundMsg<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<RunnerInboundMsg<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `RunnerInboundMsg` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_runner_inbound_msg_unchecked`.
pub fn size_prefixed_root_as_runner_inbound_msg_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<RunnerInboundMsg<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<RunnerInboundMsg<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a RunnerInboundMsg and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `RunnerInboundMsg`.
pub unsafe fn root_as_runner_inbound_msg_unchecked(buf: &[u8]) -> RunnerInboundMsg {
    flatbuffers::root_unchecked::<RunnerInboundMsg>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed RunnerInboundMsg and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `RunnerInboundMsg`.
pub unsafe fn size_prefixed_root_as_runner_inbound_msg_unchecked(buf: &[u8]) -> RunnerInboundMsg {
    flatbuffers::size_prefixed_root_unchecked::<RunnerInboundMsg>(buf)
}
#[inline]
pub fn finish_runner_inbound_msg_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<RunnerInboundMsg<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_runner_inbound_msg_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<RunnerInboundMsg<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
