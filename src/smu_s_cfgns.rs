#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    nsstatus: Nsstatus,
    nslock: Nslock,
    nsif: Nsif,
    nsien: Nsien,
    _reserved4: [u8; 0x2c],
    ppunspatd0: Ppunspatd0,
    ppunspatd1: Ppunspatd1,
    _reserved6: [u8; 0xf8],
    ppunsfs: Ppunsfs,
    _reserved7: [u8; 0x0c],
    bmpunspatd0: Bmpunspatd0,
}
impl RegisterBlock {
    #[doc = "0x04 - Register for status flags."]
    #[inline(always)]
    pub const fn nsstatus(&self) -> &Nsstatus {
        &self.nsstatus
    }
    #[doc = "0x08 - Register used to lock/unlock access to the register file."]
    #[inline(always)]
    pub const fn nslock(&self) -> &Nslock {
        &self.nslock
    }
    #[doc = "0x0c - Register for interrupt status flags."]
    #[inline(always)]
    pub const fn nsif(&self) -> &Nsif {
        &self.nsif
    }
    #[doc = "0x10 - Register used for enabling/disabling interrupts."]
    #[inline(always)]
    pub const fn nsien(&self) -> &Nsien {
        &self.nsien
    }
    #[doc = "0x40 - Set peripheral bits to 1 to mark as privileged access only."]
    #[inline(always)]
    pub const fn ppunspatd0(&self) -> &Ppunspatd0 {
        &self.ppunspatd0
    }
    #[doc = "0x44 - Set peripheral bits to 1 to mark as privileged access only."]
    #[inline(always)]
    pub const fn ppunspatd1(&self) -> &Ppunspatd1 {
        &self.ppunspatd1
    }
    #[doc = "0x140 - Read this register to query the fault status."]
    #[inline(always)]
    pub const fn ppunsfs(&self) -> &Ppunsfs {
        &self.ppunsfs
    }
    #[doc = "0x150 - Write to set BMPU priveledged attributes."]
    #[inline(always)]
    pub const fn bmpunspatd0(&self) -> &Bmpunspatd0 {
        &self.bmpunspatd0
    }
}
#[doc = "NSSTATUS (r) register accessor: Register for status flags.\n\nYou can [`read`](crate::Reg::read) this register and get [`nsstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsstatus`] module"]
#[doc(alias = "NSSTATUS")]
pub type Nsstatus = crate::Reg<nsstatus::NsstatusSpec>;
#[doc = "Register for status flags."]
pub mod nsstatus;
#[doc = "NSLOCK (w) register accessor: Register used to lock/unlock access to the register file.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nslock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nslock`] module"]
#[doc(alias = "NSLOCK")]
pub type Nslock = crate::Reg<nslock::NslockSpec>;
#[doc = "Register used to lock/unlock access to the register file."]
pub mod nslock;
#[doc = "NSIF (rw) register accessor: Register for interrupt status flags.\n\nYou can [`read`](crate::Reg::read) this register and get [`nsif::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsif::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsif`] module"]
#[doc(alias = "NSIF")]
pub type Nsif = crate::Reg<nsif::NsifSpec>;
#[doc = "Register for interrupt status flags."]
pub mod nsif;
#[doc = "NSIEN (rw) register accessor: Register used for enabling/disabling interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`nsien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nsien`] module"]
#[doc(alias = "NSIEN")]
pub type Nsien = crate::Reg<nsien::NsienSpec>;
#[doc = "Register used for enabling/disabling interrupts."]
pub mod nsien;
#[doc = "PPUNSPATD0 (rw) register accessor: Set peripheral bits to 1 to mark as privileged access only.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppunspatd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppunspatd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppunspatd0`] module"]
#[doc(alias = "PPUNSPATD0")]
pub type Ppunspatd0 = crate::Reg<ppunspatd0::Ppunspatd0Spec>;
#[doc = "Set peripheral bits to 1 to mark as privileged access only."]
pub mod ppunspatd0;
#[doc = "PPUNSPATD1 (rw) register accessor: Set peripheral bits to 1 to mark as privileged access only.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppunspatd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppunspatd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppunspatd1`] module"]
#[doc(alias = "PPUNSPATD1")]
pub type Ppunspatd1 = crate::Reg<ppunspatd1::Ppunspatd1Spec>;
#[doc = "Set peripheral bits to 1 to mark as privileged access only."]
pub mod ppunspatd1;
#[doc = "PPUNSFS (r) register accessor: Read this register to query the fault status.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppunsfs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppunsfs`] module"]
#[doc(alias = "PPUNSFS")]
pub type Ppunsfs = crate::Reg<ppunsfs::PpunsfsSpec>;
#[doc = "Read this register to query the fault status."]
pub mod ppunsfs;
#[doc = "BMPUNSPATD0 (rw) register accessor: Write to set BMPU priveledged attributes.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpunspatd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmpunspatd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmpunspatd0`] module"]
#[doc(alias = "BMPUNSPATD0")]
pub type Bmpunspatd0 = crate::Reg<bmpunspatd0::Bmpunspatd0Spec>;
#[doc = "Write to set BMPU priveledged attributes."]
pub mod bmpunspatd0;
