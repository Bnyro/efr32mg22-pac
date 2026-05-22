#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    if_: If,
    ien: Ien,
    _reserved2: [u8; 0x08],
    chiprevhw: Chiprevhw,
    chiprev: Chiprev,
    _reserved4: [u8; 0x08],
    cfgsystic: Cfgsystic,
    _reserved5: [u8; 0x01dc],
    ctrl: Ctrl,
    _reserved6: [u8; 0x04],
    dmem0retnctrl: Dmem0retnctrl,
    _reserved7: [u8; 0x04],
    dmem0eccaddr: Dmem0eccaddr,
    dmem0eccctrl: Dmem0eccctrl,
    _reserved9: [u8; 0x01e8],
    radioramretnctrl: Radioramretnctrl,
    _reserved10: [u8; 0x04],
    radioeccctrl: Radioeccctrl,
    _reserved11: [u8; 0x04],
    seqrameccaddr: Seqrameccaddr,
    frcrameccaddr: Frcrameccaddr,
    _reserved13: [u8; 0x01e8],
    rootdata0: Rootdata0,
    rootdata1: Rootdata1,
    rootlockstatus: Rootlockstatus,
}
impl RegisterBlock {
    #[doc = "0x00 - Read to get system status."]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x04 - Write to enable interrupts."]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x10 - Read to get the hard-wired chip revision."]
    #[inline(always)]
    pub const fn chiprevhw(&self) -> &Chiprevhw {
        &self.chiprevhw
    }
    #[doc = "0x14 - Read to get the chip revision programmed by feature configuration."]
    #[inline(always)]
    pub const fn chiprev(&self) -> &Chiprev {
        &self.chiprev
    }
    #[doc = "0x20 - Configure the source of the system tick for the M33."]
    #[inline(always)]
    pub const fn cfgsystic(&self) -> &Cfgsystic {
        &self.cfgsystic
    }
    #[doc = "0x200 - Configure to provide general RAM configuration."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x208 - Configure to provide general RAM retention configuration."]
    #[inline(always)]
    pub const fn dmem0retnctrl(&self) -> &Dmem0retnctrl {
        &self.dmem0retnctrl
    }
    #[doc = "0x210 - Read to get status of the DMEM0 ECC error address."]
    #[inline(always)]
    pub const fn dmem0eccaddr(&self) -> &Dmem0eccaddr {
        &self.dmem0eccaddr
    }
    #[doc = "0x214 - Configure to set RAM ECC control."]
    #[inline(always)]
    pub const fn dmem0eccctrl(&self) -> &Dmem0eccctrl {
        &self.dmem0eccctrl
    }
    #[doc = "0x400 - Configure SEQRAM Retention controls."]
    #[inline(always)]
    pub const fn radioramretnctrl(&self) -> &Radioramretnctrl {
        &self.radioramretnctrl
    }
    #[doc = "0x408 - Configure to set RAM ECC control."]
    #[inline(always)]
    pub const fn radioeccctrl(&self) -> &Radioeccctrl {
        &self.radioeccctrl
    }
    #[doc = "0x410 - Read to get status of the SEQRAM ECC error address."]
    #[inline(always)]
    pub const fn seqrameccaddr(&self) -> &Seqrameccaddr {
        &self.seqrameccaddr
    }
    #[doc = "0x414 - Read to get status of the FRCRAM ECC error address."]
    #[inline(always)]
    pub const fn frcrameccaddr(&self) -> &Frcrameccaddr {
        &self.frcrameccaddr
    }
    #[doc = "0x600 - Data in this register is passed to the trusted root firmware upon reset."]
    #[inline(always)]
    pub const fn rootdata0(&self) -> &Rootdata0 {
        &self.rootdata0
    }
    #[doc = "0x604 - Data in this register is passed to the trusted root firmware upon reset."]
    #[inline(always)]
    pub const fn rootdata1(&self) -> &Rootdata1 {
        &self.rootdata1
    }
    #[doc = "0x608 - This register returns the status of the SE managed locks."]
    #[inline(always)]
    pub const fn rootlockstatus(&self) -> &Rootlockstatus {
        &self.rootlockstatus
    }
}
#[doc = "IF (rw) register accessor: Read to get system status.\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Read to get system status."]
pub mod if_;
#[doc = "IEN (rw) register accessor: Write to enable interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Write to enable interrupts."]
pub mod ien;
#[doc = "CHIPREVHW (rw) register accessor: Read to get the hard-wired chip revision.\n\nYou can [`read`](crate::Reg::read) this register and get [`chiprevhw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chiprevhw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chiprevhw`] module"]
#[doc(alias = "CHIPREVHW")]
pub type Chiprevhw = crate::Reg<chiprevhw::ChiprevhwSpec>;
#[doc = "Read to get the hard-wired chip revision."]
pub mod chiprevhw;
#[doc = "CHIPREV (rw) register accessor: Read to get the chip revision programmed by feature configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`chiprev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chiprev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chiprev`] module"]
#[doc(alias = "CHIPREV")]
pub type Chiprev = crate::Reg<chiprev::ChiprevSpec>;
#[doc = "Read to get the chip revision programmed by feature configuration."]
pub mod chiprev;
#[doc = "CFGSYSTIC (rw) register accessor: Configure the source of the system tick for the M33.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgsystic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgsystic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgsystic`] module"]
#[doc(alias = "CFGSYSTIC")]
pub type Cfgsystic = crate::Reg<cfgsystic::CfgsysticSpec>;
#[doc = "Configure the source of the system tick for the M33."]
pub mod cfgsystic;
#[doc = "CTRL (rw) register accessor: Configure to provide general RAM configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Configure to provide general RAM configuration."]
pub mod ctrl;
#[doc = "DMEM0RETNCTRL (rw) register accessor: Configure to provide general RAM retention configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmem0retnctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmem0retnctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmem0retnctrl`] module"]
#[doc(alias = "DMEM0RETNCTRL")]
pub type Dmem0retnctrl = crate::Reg<dmem0retnctrl::Dmem0retnctrlSpec>;
#[doc = "Configure to provide general RAM retention configuration."]
pub mod dmem0retnctrl;
#[doc = "DMEM0ECCADDR (r) register accessor: Read to get status of the DMEM0 ECC error address.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmem0eccaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmem0eccaddr`] module"]
#[doc(alias = "DMEM0ECCADDR")]
pub type Dmem0eccaddr = crate::Reg<dmem0eccaddr::Dmem0eccaddrSpec>;
#[doc = "Read to get status of the DMEM0 ECC error address."]
pub mod dmem0eccaddr;
#[doc = "DMEM0ECCCTRL (rw) register accessor: Configure to set RAM ECC control.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmem0eccctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmem0eccctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmem0eccctrl`] module"]
#[doc(alias = "DMEM0ECCCTRL")]
pub type Dmem0eccctrl = crate::Reg<dmem0eccctrl::Dmem0eccctrlSpec>;
#[doc = "Configure to set RAM ECC control."]
pub mod dmem0eccctrl;
#[doc = "RADIORAMRETNCTRL (rw) register accessor: Configure SEQRAM Retention controls.\n\nYou can [`read`](crate::Reg::read) this register and get [`radioramretnctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radioramretnctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@radioramretnctrl`] module"]
#[doc(alias = "RADIORAMRETNCTRL")]
pub type Radioramretnctrl = crate::Reg<radioramretnctrl::RadioramretnctrlSpec>;
#[doc = "Configure SEQRAM Retention controls."]
pub mod radioramretnctrl;
#[doc = "RADIOECCCTRL (rw) register accessor: Configure to set RAM ECC control.\n\nYou can [`read`](crate::Reg::read) this register and get [`radioeccctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`radioeccctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@radioeccctrl`] module"]
#[doc(alias = "RADIOECCCTRL")]
pub type Radioeccctrl = crate::Reg<radioeccctrl::RadioeccctrlSpec>;
#[doc = "Configure to set RAM ECC control."]
pub mod radioeccctrl;
#[doc = "SEQRAMECCADDR (r) register accessor: Read to get status of the SEQRAM ECC error address.\n\nYou can [`read`](crate::Reg::read) this register and get [`seqrameccaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqrameccaddr`] module"]
#[doc(alias = "SEQRAMECCADDR")]
pub type Seqrameccaddr = crate::Reg<seqrameccaddr::SeqrameccaddrSpec>;
#[doc = "Read to get status of the SEQRAM ECC error address."]
pub mod seqrameccaddr;
#[doc = "FRCRAMECCADDR (r) register accessor: Read to get status of the FRCRAM ECC error address.\n\nYou can [`read`](crate::Reg::read) this register and get [`frcrameccaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frcrameccaddr`] module"]
#[doc(alias = "FRCRAMECCADDR")]
pub type Frcrameccaddr = crate::Reg<frcrameccaddr::FrcrameccaddrSpec>;
#[doc = "Read to get status of the FRCRAM ECC error address."]
pub mod frcrameccaddr;
#[doc = "ROOTDATA0 (rw) register accessor: Data in this register is passed to the trusted root firmware upon reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`rootdata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rootdata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rootdata0`] module"]
#[doc(alias = "ROOTDATA0")]
pub type Rootdata0 = crate::Reg<rootdata0::Rootdata0Spec>;
#[doc = "Data in this register is passed to the trusted root firmware upon reset."]
pub mod rootdata0;
#[doc = "ROOTDATA1 (rw) register accessor: Data in this register is passed to the trusted root firmware upon reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`rootdata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rootdata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rootdata1`] module"]
#[doc(alias = "ROOTDATA1")]
pub type Rootdata1 = crate::Reg<rootdata1::Rootdata1Spec>;
#[doc = "Data in this register is passed to the trusted root firmware upon reset."]
pub mod rootdata1;
#[doc = "ROOTLOCKSTATUS (r) register accessor: This register returns the status of the SE managed locks.\n\nYou can [`read`](crate::Reg::read) this register and get [`rootlockstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rootlockstatus`] module"]
#[doc(alias = "ROOTLOCKSTATUS")]
pub type Rootlockstatus = crate::Reg<rootlockstatus::RootlockstatusSpec>;
#[doc = "This register returns the status of the SE managed locks."]
pub mod rootlockstatus;
