#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    _reserved1: [u8; 0x04],
    status: Status,
    _reserved2: [u8; 0x04],
    lock: Lock,
    wdoglock: Wdoglock,
    _reserved4: [u8; 0x08],
    if_: If,
    ien: Ien,
    _reserved6: [u8; 0x28],
    calcmd: Calcmd,
    calctrl: Calctrl,
    calcnt: Calcnt,
    _reserved9: [u8; 0x08],
    clken0: Clken0,
    clken1: Clken1,
    _reserved11: [u8; 0x04],
    sysclkctrl: Sysclkctrl,
    _reserved12: [u8; 0x0c],
    traceclkctrl: Traceclkctrl,
    _reserved13: [u8; 0x0c],
    exportclkctrl: Exportclkctrl,
    _reserved14: [u8; 0x6c],
    dpllrefclkctrl: Dpllrefclkctrl,
    _reserved15: [u8; 0x1c],
    em01grpaclkctrl: Em01grpaclkctrl,
    em01grpbclkctrl: Em01grpbclkctrl,
    _reserved17: [u8; 0x18],
    em23grpaclkctrl: Em23grpaclkctrl,
    _reserved18: [u8; 0x1c],
    em4grpaclkctrl: Em4grpaclkctrl,
    _reserved19: [u8; 0x1c],
    iadcclkctrl: Iadcclkctrl,
    _reserved20: [u8; 0x7c],
    wdog0clkctrl: Wdog0clkctrl,
    _reserved21: [u8; 0x1c],
    euart0clkctrl: Euart0clkctrl,
    _reserved22: [u8; 0x1c],
    rtccclkctrl: Rtccclkctrl,
    _reserved23: [u8; 0x04],
    prortcclkctrl: Prortcclkctrl,
    _reserved24: [u8; 0x14],
    cryptoaccclkctrl: Cryptoaccclkctrl,
    _reserved25: [u8; 0x1c],
    radioclkctrl: Radioclkctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn wdoglock(&self) -> &Wdoglock {
        &self.wdoglock
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn calcmd(&self) -> &Calcmd {
        &self.calcmd
    }
    #[doc = "0x54 - No Description"]
    #[inline(always)]
    pub const fn calctrl(&self) -> &Calctrl {
        &self.calctrl
    }
    #[doc = "0x58 - No Description"]
    #[inline(always)]
    pub const fn calcnt(&self) -> &Calcnt {
        &self.calcnt
    }
    #[doc = "0x64 - No Description"]
    #[inline(always)]
    pub const fn clken0(&self) -> &Clken0 {
        &self.clken0
    }
    #[doc = "0x68 - No Description"]
    #[inline(always)]
    pub const fn clken1(&self) -> &Clken1 {
        &self.clken1
    }
    #[doc = "0x70 - No Description"]
    #[inline(always)]
    pub const fn sysclkctrl(&self) -> &Sysclkctrl {
        &self.sysclkctrl
    }
    #[doc = "0x80 - No Description"]
    #[inline(always)]
    pub const fn traceclkctrl(&self) -> &Traceclkctrl {
        &self.traceclkctrl
    }
    #[doc = "0x90 - No Description"]
    #[inline(always)]
    pub const fn exportclkctrl(&self) -> &Exportclkctrl {
        &self.exportclkctrl
    }
    #[doc = "0x100 - No Description"]
    #[inline(always)]
    pub const fn dpllrefclkctrl(&self) -> &Dpllrefclkctrl {
        &self.dpllrefclkctrl
    }
    #[doc = "0x120 - No Description"]
    #[inline(always)]
    pub const fn em01grpaclkctrl(&self) -> &Em01grpaclkctrl {
        &self.em01grpaclkctrl
    }
    #[doc = "0x124 - No Description"]
    #[inline(always)]
    pub const fn em01grpbclkctrl(&self) -> &Em01grpbclkctrl {
        &self.em01grpbclkctrl
    }
    #[doc = "0x140 - No Description"]
    #[inline(always)]
    pub const fn em23grpaclkctrl(&self) -> &Em23grpaclkctrl {
        &self.em23grpaclkctrl
    }
    #[doc = "0x160 - No Description"]
    #[inline(always)]
    pub const fn em4grpaclkctrl(&self) -> &Em4grpaclkctrl {
        &self.em4grpaclkctrl
    }
    #[doc = "0x180 - No Description"]
    #[inline(always)]
    pub const fn iadcclkctrl(&self) -> &Iadcclkctrl {
        &self.iadcclkctrl
    }
    #[doc = "0x200 - No Description"]
    #[inline(always)]
    pub const fn wdog0clkctrl(&self) -> &Wdog0clkctrl {
        &self.wdog0clkctrl
    }
    #[doc = "0x220 - No Description"]
    #[inline(always)]
    pub const fn euart0clkctrl(&self) -> &Euart0clkctrl {
        &self.euart0clkctrl
    }
    #[doc = "0x240 - No Description"]
    #[inline(always)]
    pub const fn rtccclkctrl(&self) -> &Rtccclkctrl {
        &self.rtccclkctrl
    }
    #[doc = "0x248 - No Description"]
    #[inline(always)]
    pub const fn prortcclkctrl(&self) -> &Prortcclkctrl {
        &self.prortcclkctrl
    }
    #[doc = "0x260 - No Description"]
    #[inline(always)]
    pub const fn cryptoaccclkctrl(&self) -> &Cryptoaccclkctrl {
        &self.cryptoaccclkctrl
    }
    #[doc = "0x280 - No Description"]
    #[inline(always)]
    pub const fn radioclkctrl(&self) -> &Radioclkctrl {
        &self.radioclkctrl
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
#[doc = "LOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "No Description"]
pub mod lock;
#[doc = "WDOGLOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdoglock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdoglock`] module"]
#[doc(alias = "WDOGLOCK")]
pub type Wdoglock = crate::Reg<wdoglock::WdoglockSpec>;
#[doc = "No Description"]
pub mod wdoglock;
#[doc = "IF (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "No Description"]
pub mod ien;
#[doc = "CALCMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calcmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calcmd`] module"]
#[doc(alias = "CALCMD")]
pub type Calcmd = crate::Reg<calcmd::CalcmdSpec>;
#[doc = "No Description"]
pub mod calcmd;
#[doc = "CALCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calctrl`] module"]
#[doc(alias = "CALCTRL")]
pub type Calctrl = crate::Reg<calctrl::CalctrlSpec>;
#[doc = "No Description"]
pub mod calctrl;
#[doc = "CALCNT (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`calcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calcnt`] module"]
#[doc(alias = "CALCNT")]
pub type Calcnt = crate::Reg<calcnt::CalcntSpec>;
#[doc = "No Description"]
pub mod calcnt;
#[doc = "CLKEN0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`clken0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clken0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clken0`] module"]
#[doc(alias = "CLKEN0")]
pub type Clken0 = crate::Reg<clken0::Clken0Spec>;
#[doc = "No Description"]
pub mod clken0;
#[doc = "CLKEN1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`clken1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clken1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clken1`] module"]
#[doc(alias = "CLKEN1")]
pub type Clken1 = crate::Reg<clken1::Clken1Spec>;
#[doc = "No Description"]
pub mod clken1;
#[doc = "SYSCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclkctrl`] module"]
#[doc(alias = "SYSCLKCTRL")]
pub type Sysclkctrl = crate::Reg<sysclkctrl::SysclkctrlSpec>;
#[doc = "No Description"]
pub mod sysclkctrl;
#[doc = "TRACECLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`traceclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceclkctrl`] module"]
#[doc(alias = "TRACECLKCTRL")]
pub type Traceclkctrl = crate::Reg<traceclkctrl::TraceclkctrlSpec>;
#[doc = "No Description"]
pub mod traceclkctrl;
#[doc = "EXPORTCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`exportclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exportclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exportclkctrl`] module"]
#[doc(alias = "EXPORTCLKCTRL")]
pub type Exportclkctrl = crate::Reg<exportclkctrl::ExportclkctrlSpec>;
#[doc = "No Description"]
pub mod exportclkctrl;
#[doc = "DPLLREFCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllrefclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllrefclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllrefclkctrl`] module"]
#[doc(alias = "DPLLREFCLKCTRL")]
pub type Dpllrefclkctrl = crate::Reg<dpllrefclkctrl::DpllrefclkctrlSpec>;
#[doc = "No Description"]
pub mod dpllrefclkctrl;
#[doc = "EM01GRPACLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em01grpaclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em01grpaclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em01grpaclkctrl`] module"]
#[doc(alias = "EM01GRPACLKCTRL")]
pub type Em01grpaclkctrl = crate::Reg<em01grpaclkctrl::Em01grpaclkctrlSpec>;
#[doc = "No Description"]
pub mod em01grpaclkctrl;
#[doc = "EM01GRPBCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em01grpbclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em01grpbclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em01grpbclkctrl`] module"]
#[doc(alias = "EM01GRPBCLKCTRL")]
pub type Em01grpbclkctrl = crate::Reg<em01grpbclkctrl::Em01grpbclkctrlSpec>;
#[doc = "No Description"]
pub mod em01grpbclkctrl;
#[doc = "EM23GRPACLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em23grpaclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em23grpaclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em23grpaclkctrl`] module"]
#[doc(alias = "EM23GRPACLKCTRL")]
pub type Em23grpaclkctrl = crate::Reg<em23grpaclkctrl::Em23grpaclkctrlSpec>;
#[doc = "No Description"]
pub mod em23grpaclkctrl;
#[doc = "EM4GRPACLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4grpaclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4grpaclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4grpaclkctrl`] module"]
#[doc(alias = "EM4GRPACLKCTRL")]
pub type Em4grpaclkctrl = crate::Reg<em4grpaclkctrl::Em4grpaclkctrlSpec>;
#[doc = "No Description"]
pub mod em4grpaclkctrl;
#[doc = "IADCCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`iadcclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iadcclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iadcclkctrl`] module"]
#[doc(alias = "IADCCLKCTRL")]
pub type Iadcclkctrl = crate::Reg<iadcclkctrl::IadcclkctrlSpec>;
#[doc = "No Description"]
pub mod iadcclkctrl;
#[doc = "WDOG0CLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog0clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog0clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog0clkctrl`] module"]
#[doc(alias = "WDOG0CLKCTRL")]
pub type Wdog0clkctrl = crate::Reg<wdog0clkctrl::Wdog0clkctrlSpec>;
#[doc = "No Description"]
pub mod wdog0clkctrl;
#[doc = "EUART0CLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`euart0clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`euart0clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@euart0clkctrl`] module"]
#[doc(alias = "EUART0CLKCTRL")]
pub type Euart0clkctrl = crate::Reg<euart0clkctrl::Euart0clkctrlSpec>;
#[doc = "No Description"]
pub mod euart0clkctrl;
#[doc = "RTCCCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccclkctrl`] module"]
#[doc(alias = "RTCCCLKCTRL")]
pub type Rtccclkctrl = crate::Reg<rtccclkctrl::RtccclkctrlSpec>;
#[doc = "No Description"]
pub mod rtccclkctrl;
#[doc = "PRORTCCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`prortcclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prortcclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prortcclkctrl`] module"]
#[doc(alias = "PRORTCCLKCTRL")]
pub type Prortcclkctrl = crate::Reg<prortcclkctrl::PrortcclkctrlSpec>;
#[doc = "No Description"]
pub mod prortcclkctrl;
#[doc = "CRYPTOACCCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cryptoaccclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryptoaccclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cryptoaccclkctrl`] module"]
#[doc(alias = "CRYPTOACCCLKCTRL")]
pub type Cryptoaccclkctrl = crate::Reg<cryptoaccclkctrl::CryptoaccclkctrlSpec>;
#[doc = "No Description"]
pub mod cryptoaccclkctrl;
#[doc = "RADIOCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`radioclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radioclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@radioclkctrl`] module"]
#[doc(alias = "RADIOCLKCTRL")]
pub type Radioclkctrl = crate::Reg<radioclkctrl::RadioclkctrlSpec>;
#[doc = "No Description"]
pub mod radioclkctrl;
