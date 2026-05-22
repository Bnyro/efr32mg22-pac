#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pointer: Pointer,
    command: Command,
    pkctrl: Pkctrl,
    pkstatus: Pkstatus,
    version: Version,
    timer: Timer,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn pointer(&self) -> &Pointer {
        &self.pointer
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn pkctrl(&self) -> &Pkctrl {
        &self.pkctrl
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn pkstatus(&self) -> &Pkstatus {
        &self.pkstatus
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn version(&self) -> &Version {
        &self.version
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn timer(&self) -> &Timer {
        &self.timer
    }
}
#[doc = "POINTER (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pointer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pointer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pointer`] module"]
#[doc(alias = "POINTER")]
pub type Pointer = crate::Reg<pointer::PointerSpec>;
#[doc = "No Description"]
pub mod pointer;
#[doc = "COMMAND (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`] module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "No Description"]
pub mod command;
#[doc = "PKCTRL (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkctrl`] module"]
#[doc(alias = "PKCTRL")]
pub type Pkctrl = crate::Reg<pkctrl::PkctrlSpec>;
#[doc = "No Description"]
pub mod pkctrl;
#[doc = "PKSTATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pkstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkstatus`] module"]
#[doc(alias = "PKSTATUS")]
pub type Pkstatus = crate::Reg<pkstatus::PkstatusSpec>;
#[doc = "No Description"]
pub mod pkstatus;
#[doc = "VERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
#[doc(alias = "VERSION")]
pub type Version = crate::Reg<version::VersionSpec>;
#[doc = "No Description"]
pub mod version;
#[doc = "TIMER (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timer::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer`] module"]
#[doc(alias = "TIMER")]
pub type Timer = crate::Reg<timer::TimerSpec>;
#[doc = "No Description"]
pub mod timer;
