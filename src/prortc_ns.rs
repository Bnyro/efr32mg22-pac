#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    cfg: Cfg,
    cmd: Cmd,
    status: Status,
    if_: If,
    ien: Ien,
    precnt: Precnt,
    cnt: Cnt,
    combcnt: Combcnt,
    syncbusy: Syncbusy,
    lock: Lock,
    cc0_ctrl: Cc0Ctrl,
    cc0_ocvalue: Cc0Ocvalue,
    cc0_icvalue: Cc0Icvalue,
    cc1_ctrl: Cc1Ctrl,
    cc1_ocvalue: Cc1Ocvalue,
    cc1_icvalue: Cc1Icvalue,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn precnt(&self) -> &Precnt {
        &self.precnt
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn combcnt(&self) -> &Combcnt {
        &self.combcnt
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn cc0_ctrl(&self) -> &Cc0Ctrl {
        &self.cc0_ctrl
    }
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn cc0_ocvalue(&self) -> &Cc0Ocvalue {
        &self.cc0_ocvalue
    }
    #[doc = "0x38 - No Description"]
    #[inline(always)]
    pub const fn cc0_icvalue(&self) -> &Cc0Icvalue {
        &self.cc0_icvalue
    }
    #[doc = "0x3c - No Description"]
    #[inline(always)]
    pub const fn cc1_ctrl(&self) -> &Cc1Ctrl {
        &self.cc1_ctrl
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn cc1_ocvalue(&self) -> &Cc1Ocvalue {
        &self.cc1_ocvalue
    }
    #[doc = "0x44 - No Description"]
    #[inline(always)]
    pub const fn cc1_icvalue(&self) -> &Cc1Icvalue {
        &self.cc1_icvalue
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "No Description"]
pub mod en;
#[doc = "CFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "No Description"]
pub mod cfg;
#[doc = "CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
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
#[doc = "PRECNT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`precnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`precnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@precnt`] module"]
#[doc(alias = "PRECNT")]
pub type Precnt = crate::Reg<precnt::PrecntSpec>;
#[doc = "No Description"]
pub mod precnt;
#[doc = "CNT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "No Description"]
pub mod cnt;
#[doc = "COMBCNT (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`combcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@combcnt`] module"]
#[doc(alias = "COMBCNT")]
pub type Combcnt = crate::Reg<combcnt::CombcntSpec>;
#[doc = "No Description"]
pub mod combcnt;
#[doc = "SYNCBUSY (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "No Description"]
pub mod syncbusy;
#[doc = "LOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "No Description"]
pub mod lock;
#[doc = "CC0_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ctrl`] module"]
#[doc(alias = "CC0_CTRL")]
pub type Cc0Ctrl = crate::Reg<cc0_ctrl::Cc0CtrlSpec>;
#[doc = "No Description"]
pub mod cc0_ctrl;
#[doc = "CC0_OCVALUE (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ocvalue::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ocvalue::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ocvalue`] module"]
#[doc(alias = "CC0_OCVALUE")]
pub type Cc0Ocvalue = crate::Reg<cc0_ocvalue::Cc0OcvalueSpec>;
#[doc = "No Description"]
pub mod cc0_ocvalue;
#[doc = "CC0_ICVALUE (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_icvalue::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_icvalue`] module"]
#[doc(alias = "CC0_ICVALUE")]
pub type Cc0Icvalue = crate::Reg<cc0_icvalue::Cc0IcvalueSpec>;
#[doc = "No Description"]
pub mod cc0_icvalue;
#[doc = "CC1_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ctrl`] module"]
#[doc(alias = "CC1_CTRL")]
pub type Cc1Ctrl = crate::Reg<cc1_ctrl::Cc1CtrlSpec>;
#[doc = "No Description"]
pub mod cc1_ctrl;
#[doc = "CC1_OCVALUE (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ocvalue::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ocvalue::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ocvalue`] module"]
#[doc(alias = "CC1_OCVALUE")]
pub type Cc1Ocvalue = crate::Reg<cc1_ocvalue::Cc1OcvalueSpec>;
#[doc = "No Description"]
pub mod cc1_ocvalue;
#[doc = "CC1_ICVALUE (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_icvalue::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_icvalue`] module"]
#[doc(alias = "CC1_ICVALUE")]
pub type Cc1Icvalue = crate::Reg<cc1_icvalue::Cc1IcvalueSpec>;
#[doc = "No Description"]
pub mod cc1_icvalue;
