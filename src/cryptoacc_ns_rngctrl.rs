#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rngctrl: Rngctrl,
    fifolevel: Fifolevel,
    fifothresh: Fifothresh,
    fifodepth: Fifodepth,
    key0: Key0,
    key1: Key1,
    key2: Key2,
    key3: Key3,
    testdata: Testdata,
    _reserved9: [u8; 0x0c],
    rngstatus: Rngstatus,
    initwaitval: Initwaitval,
    _reserved11: [u8; 0x08],
    swofftmrval: Swofftmrval,
    clkdiv: Clkdiv,
    ais31conf0: Ais31conf0,
    ais31conf1: Ais31conf1,
    ais31conf2: Ais31conf2,
    ais31status: Ais31status,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn rngctrl(&self) -> &Rngctrl {
        &self.rngctrl
    }
    #[doc = "0x04 - Number of 32 bits words of random available in the FIFO. Writing to this register clears the FIFO full interrupt"]
    #[inline(always)]
    pub const fn fifolevel(&self) -> &Fifolevel {
        &self.fifolevel
    }
    #[doc = "0x08 - FIFO level at which the rings are restarted when in the FIFOFull_Off state, expressed in number of 128bit blocks"]
    #[inline(always)]
    pub const fn fifothresh(&self) -> &Fifothresh {
        &self.fifothresh
    }
    #[doc = "0x0c - Maximum number of 32 bits words that can be stored in the FIFO: 2^g_fifodepth"]
    #[inline(always)]
    pub const fn fifodepth(&self) -> &Fifodepth {
        &self.fifodepth
    }
    #[doc = "0x10 - This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[inline(always)]
    pub const fn key0(&self) -> &Key0 {
        &self.key0
    }
    #[doc = "0x14 - This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x18 - This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[inline(always)]
    pub const fn key2(&self) -> &Key2 {
        &self.key2
    }
    #[doc = "0x1c - This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
    #[inline(always)]
    pub const fn key3(&self) -> &Key3 {
        &self.key3
    }
    #[doc = "0x20 - This register is used to feed known data to the conditioning function or to the continuous tests. See manual"]
    #[inline(always)]
    pub const fn testdata(&self) -> &Testdata {
        &self.testdata
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn rngstatus(&self) -> &Rngstatus {
        &self.rngstatus
    }
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn initwaitval(&self) -> &Initwaitval {
        &self.initwaitval
    }
    #[doc = "0x40 - Number of clk cycles to wait before stopping the rings after the FIFO is full"]
    #[inline(always)]
    pub const fn swofftmrval(&self) -> &Swofftmrval {
        &self.swofftmrval
    }
    #[doc = "0x44 - Sample clock divider. The frequency at which the outputs of the rings are sampled is given by Fs = Fpclk/(ClkDiv + 1)"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x48 - No Description"]
    #[inline(always)]
    pub const fn ais31conf0(&self) -> &Ais31conf0 {
        &self.ais31conf0
    }
    #[doc = "0x4c - No Description"]
    #[inline(always)]
    pub const fn ais31conf1(&self) -> &Ais31conf1 {
        &self.ais31conf1
    }
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn ais31conf2(&self) -> &Ais31conf2 {
        &self.ais31conf2
    }
    #[doc = "0x54 - This register is used to obtain diagnostic information about the AIS31 start-up and online tests when g_AIS31=True. Writing to this register clears all fields"]
    #[inline(always)]
    pub const fn ais31status(&self) -> &Ais31status {
        &self.ais31status
    }
}
#[doc = "RNGCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rngctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngctrl`] module"]
#[doc(alias = "RNGCTRL")]
pub type Rngctrl = crate::Reg<rngctrl::RngctrlSpec>;
#[doc = "No Description"]
pub mod rngctrl;
#[doc = "FIFOLEVEL (r) register accessor: Number of 32 bits words of random available in the FIFO. Writing to this register clears the FIFO full interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolevel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifolevel`] module"]
#[doc(alias = "FIFOLEVEL")]
pub type Fifolevel = crate::Reg<fifolevel::FifolevelSpec>;
#[doc = "Number of 32 bits words of random available in the FIFO. Writing to this register clears the FIFO full interrupt"]
pub mod fifolevel;
#[doc = "FIFOTHRESH (r) register accessor: FIFO level at which the rings are restarted when in the FIFOFull_Off state, expressed in number of 128bit blocks\n\nYou can [`read`](crate::Reg::read) this register and get [`fifothresh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifothresh`] module"]
#[doc(alias = "FIFOTHRESH")]
pub type Fifothresh = crate::Reg<fifothresh::FifothreshSpec>;
#[doc = "FIFO level at which the rings are restarted when in the FIFOFull_Off state, expressed in number of 128bit blocks"]
pub mod fifothresh;
#[doc = "FIFODEPTH (r) register accessor: Maximum number of 32 bits words that can be stored in the FIFO: 2^g_fifodepth\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodepth::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifodepth`] module"]
#[doc(alias = "FIFODEPTH")]
pub type Fifodepth = crate::Reg<fifodepth::FifodepthSpec>;
#[doc = "Maximum number of 32 bits words that can be stored in the FIFO: 2^g_fifodepth"]
pub mod fifodepth;
#[doc = "KEY0 (rw) register accessor: This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011...\n\nYou can [`read`](crate::Reg::read) this register and get [`key0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0`] module"]
#[doc(alias = "KEY0")]
pub type Key0 = crate::Reg<key0::Key0Spec>;
#[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
pub mod key0;
#[doc = "KEY1 (rw) register accessor: This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011...\n\nYou can [`read`](crate::Reg::read) this register and get [`key1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`] module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
pub mod key1;
#[doc = "KEY2 (rw) register accessor: This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011...\n\nYou can [`read`](crate::Reg::read) this register and get [`key2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`] module"]
#[doc(alias = "KEY2")]
pub type Key2 = crate::Reg<key2::Key2Spec>;
#[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
pub mod key2;
#[doc = "KEY3 (rw) register accessor: This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011...\n\nYou can [`read`](crate::Reg::read) this register and get [`key3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key3`] module"]
#[doc(alias = "KEY3")]
pub type Key3 = crate::Reg<key3::Key3Spec>;
#[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011..."]
pub mod key3;
#[doc = "TESTDATA (w) register accessor: This register is used to feed known data to the conditioning function or to the continuous tests. See manual\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testdata`] module"]
#[doc(alias = "TESTDATA")]
pub type Testdata = crate::Reg<testdata::TestdataSpec>;
#[doc = "This register is used to feed known data to the conditioning function or to the continuous tests. See manual"]
pub mod testdata;
#[doc = "RNGSTATUS (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rngstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rngstatus`] module"]
#[doc(alias = "RNGSTATUS")]
pub type Rngstatus = crate::Reg<rngstatus::RngstatusSpec>;
#[doc = "No Description"]
pub mod rngstatus;
#[doc = "INITWAITVAL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`initwaitval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`initwaitval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@initwaitval`] module"]
#[doc(alias = "INITWAITVAL")]
pub type Initwaitval = crate::Reg<initwaitval::InitwaitvalSpec>;
#[doc = "No Description"]
pub mod initwaitval;
#[doc = "SWOFFTMRVAL (rw) register accessor: Number of clk cycles to wait before stopping the rings after the FIFO is full\n\nYou can [`read`](crate::Reg::read) this register and get [`swofftmrval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swofftmrval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swofftmrval`] module"]
#[doc(alias = "SWOFFTMRVAL")]
pub type Swofftmrval = crate::Reg<swofftmrval::SwofftmrvalSpec>;
#[doc = "Number of clk cycles to wait before stopping the rings after the FIFO is full"]
pub mod swofftmrval;
#[doc = "CLKDIV (rw) register accessor: Sample clock divider. The frequency at which the outputs of the rings are sampled is given by Fs = Fpclk/(ClkDiv + 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Sample clock divider. The frequency at which the outputs of the rings are sampled is given by Fs = Fpclk/(ClkDiv + 1)"]
pub mod clkdiv;
#[doc = "AIS31CONF0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ais31conf0`] module"]
#[doc(alias = "AIS31CONF0")]
pub type Ais31conf0 = crate::Reg<ais31conf0::Ais31conf0Spec>;
#[doc = "No Description"]
pub mod ais31conf0;
#[doc = "AIS31CONF1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ais31conf1`] module"]
#[doc(alias = "AIS31CONF1")]
pub type Ais31conf1 = crate::Reg<ais31conf1::Ais31conf1Spec>;
#[doc = "No Description"]
pub mod ais31conf1;
#[doc = "AIS31CONF2 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31conf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31conf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ais31conf2`] module"]
#[doc(alias = "AIS31CONF2")]
pub type Ais31conf2 = crate::Reg<ais31conf2::Ais31conf2Spec>;
#[doc = "No Description"]
pub mod ais31conf2;
#[doc = "AIS31STATUS (rw) register accessor: This register is used to obtain diagnostic information about the AIS31 start-up and online tests when g_AIS31=True. Writing to this register clears all fields\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ais31status`] module"]
#[doc(alias = "AIS31STATUS")]
pub type Ais31status = crate::Reg<ais31status::Ais31statusSpec>;
#[doc = "This register is used to obtain diagnostic information about the AIS31 start-up and online tests when g_AIS31=True. Writing to this register clears all fields"]
pub mod ais31status;
