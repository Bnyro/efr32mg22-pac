#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    _reserved1: [u8; 0x04],
    async_swpulse: AsyncSwpulse,
    async_swlevel: AsyncSwlevel,
    async_peek: AsyncPeek,
    sync_peek: SyncPeek,
    async_ch0_ctrl: AsyncCh0Ctrl,
    async_ch1_ctrl: AsyncCh1Ctrl,
    async_ch2_ctrl: AsyncCh2Ctrl,
    async_ch3_ctrl: AsyncCh3Ctrl,
    async_ch4_ctrl: AsyncCh4Ctrl,
    async_ch5_ctrl: AsyncCh5Ctrl,
    async_ch6_ctrl: AsyncCh6Ctrl,
    async_ch7_ctrl: AsyncCh7Ctrl,
    async_ch8_ctrl: AsyncCh8Ctrl,
    async_ch9_ctrl: AsyncCh9Ctrl,
    async_ch10_ctrl: AsyncCh10Ctrl,
    async_ch11_ctrl: AsyncCh11Ctrl,
    sync_ch0_ctrl: SyncCh0Ctrl,
    sync_ch1_ctrl: SyncCh1Ctrl,
    sync_ch2_ctrl: SyncCh2Ctrl,
    sync_ch3_ctrl: SyncCh3Ctrl,
    consumer_cmu_caldn: ConsumerCmuCaldn,
    consumer_cmu_calup: ConsumerCmuCalup,
    _reserved23: [u8; 0x04],
    consumer_iadc0_scantrigger: ConsumerIadc0Scantrigger,
    consumer_iadc0_singletrigger: ConsumerIadc0Singletrigger,
    consumer_ldmaxbar_dmareq0: ConsumerLdmaxbarDmareq0,
    consumer_ldmaxbar_dmareq1: ConsumerLdmaxbarDmareq1,
    consumer_letimer0_clear: ConsumerLetimer0Clear,
    consumer_letimer0_start: ConsumerLetimer0Start,
    consumer_letimer0_stop: ConsumerLetimer0Stop,
    consumer_euart0_rx: ConsumerEuart0Rx,
    consumer_euart0_trigger: ConsumerEuart0Trigger,
    consumer_modem_din: ConsumerModemDin,
    _reserved33: [u8; 0x34],
    consumer_rac_clr: ConsumerRacClr,
    consumer_rac_ctiin0: ConsumerRacCtiin0,
    consumer_rac_ctiin1: ConsumerRacCtiin1,
    consumer_rac_ctiin2: ConsumerRacCtiin2,
    consumer_rac_ctiin3: ConsumerRacCtiin3,
    consumer_rac_forcetx: ConsumerRacForcetx,
    consumer_rac_rxdis: ConsumerRacRxdis,
    consumer_rac_rxen: ConsumerRacRxen,
    consumer_rac_seq: ConsumerRacSeq,
    consumer_rac_txen: ConsumerRacTxen,
    consumer_rtcc_cc0: ConsumerRtccCc0,
    consumer_rtcc_cc1: ConsumerRtccCc1,
    consumer_rtcc_cc2: ConsumerRtccCc2,
    _reserved46: [u8; 0x04],
    consumer_core_ctiin0: ConsumerCoreCtiin0,
    consumer_core_ctiin1: ConsumerCoreCtiin1,
    consumer_core_ctiin2: ConsumerCoreCtiin2,
    consumer_core_ctiin3: ConsumerCoreCtiin3,
    consumer_core_m33rxev: ConsumerCoreM33rxev,
    consumer_timer0_cc0: ConsumerTimer0Cc0,
    consumer_timer0_cc1: ConsumerTimer0Cc1,
    consumer_timer0_cc2: ConsumerTimer0Cc2,
    consumer_timer0_dti: ConsumerTimer0Dti,
    consumer_timer0_dtifs1: ConsumerTimer0Dtifs1,
    consumer_timer0_dtifs2: ConsumerTimer0Dtifs2,
    consumer_timer1_cc0: ConsumerTimer1Cc0,
    consumer_timer1_cc1: ConsumerTimer1Cc1,
    consumer_timer1_cc2: ConsumerTimer1Cc2,
    consumer_timer1_dti: ConsumerTimer1Dti,
    consumer_timer1_dtifs1: ConsumerTimer1Dtifs1,
    consumer_timer1_dtifs2: ConsumerTimer1Dtifs2,
    consumer_timer2_cc0: ConsumerTimer2Cc0,
    consumer_timer2_cc1: ConsumerTimer2Cc1,
    consumer_timer2_cc2: ConsumerTimer2Cc2,
    consumer_timer2_dti: ConsumerTimer2Dti,
    consumer_timer2_dtifs1: ConsumerTimer2Dtifs1,
    consumer_timer2_dtifs2: ConsumerTimer2Dtifs2,
    consumer_timer3_cc0: ConsumerTimer3Cc0,
    consumer_timer3_cc1: ConsumerTimer3Cc1,
    consumer_timer3_cc2: ConsumerTimer3Cc2,
    consumer_timer3_dti: ConsumerTimer3Dti,
    consumer_timer3_dtifs1: ConsumerTimer3Dtifs1,
    consumer_timer3_dtifs2: ConsumerTimer3Dtifs2,
    consumer_timer4_cc0: ConsumerTimer4Cc0,
    consumer_timer4_cc1: ConsumerTimer4Cc1,
    consumer_timer4_cc2: ConsumerTimer4Cc2,
    consumer_timer4_dti: ConsumerTimer4Dti,
    consumer_timer4_dtifs1: ConsumerTimer4Dtifs1,
    consumer_timer4_dtifs2: ConsumerTimer4Dtifs2,
    consumer_usart0_clk: ConsumerUsart0Clk,
    consumer_usart0_ir: ConsumerUsart0Ir,
    consumer_usart0_rx: ConsumerUsart0Rx,
    consumer_usart0_trigger: ConsumerUsart0Trigger,
    consumer_usart1_clk: ConsumerUsart1Clk,
    consumer_usart1_ir: ConsumerUsart1Ir,
    consumer_usart1_rx: ConsumerUsart1Rx,
    consumer_usart1_trigger: ConsumerUsart1Trigger,
    consumer_wdog0_src0: ConsumerWdog0Src0,
    consumer_wdog0_src1: ConsumerWdog0Src1,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn async_swpulse(&self) -> &AsyncSwpulse {
        &self.async_swpulse
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn async_swlevel(&self) -> &AsyncSwlevel {
        &self.async_swlevel
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn async_peek(&self) -> &AsyncPeek {
        &self.async_peek
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn sync_peek(&self) -> &SyncPeek {
        &self.sync_peek
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn async_ch0_ctrl(&self) -> &AsyncCh0Ctrl {
        &self.async_ch0_ctrl
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn async_ch1_ctrl(&self) -> &AsyncCh1Ctrl {
        &self.async_ch1_ctrl
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn async_ch2_ctrl(&self) -> &AsyncCh2Ctrl {
        &self.async_ch2_ctrl
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn async_ch3_ctrl(&self) -> &AsyncCh3Ctrl {
        &self.async_ch3_ctrl
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn async_ch4_ctrl(&self) -> &AsyncCh4Ctrl {
        &self.async_ch4_ctrl
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn async_ch5_ctrl(&self) -> &AsyncCh5Ctrl {
        &self.async_ch5_ctrl
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn async_ch6_ctrl(&self) -> &AsyncCh6Ctrl {
        &self.async_ch6_ctrl
    }
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn async_ch7_ctrl(&self) -> &AsyncCh7Ctrl {
        &self.async_ch7_ctrl
    }
    #[doc = "0x38 - No Description"]
    #[inline(always)]
    pub const fn async_ch8_ctrl(&self) -> &AsyncCh8Ctrl {
        &self.async_ch8_ctrl
    }
    #[doc = "0x3c - No Description"]
    #[inline(always)]
    pub const fn async_ch9_ctrl(&self) -> &AsyncCh9Ctrl {
        &self.async_ch9_ctrl
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn async_ch10_ctrl(&self) -> &AsyncCh10Ctrl {
        &self.async_ch10_ctrl
    }
    #[doc = "0x44 - No Description"]
    #[inline(always)]
    pub const fn async_ch11_ctrl(&self) -> &AsyncCh11Ctrl {
        &self.async_ch11_ctrl
    }
    #[doc = "0x48 - No Description"]
    #[inline(always)]
    pub const fn sync_ch0_ctrl(&self) -> &SyncCh0Ctrl {
        &self.sync_ch0_ctrl
    }
    #[doc = "0x4c - No Description"]
    #[inline(always)]
    pub const fn sync_ch1_ctrl(&self) -> &SyncCh1Ctrl {
        &self.sync_ch1_ctrl
    }
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn sync_ch2_ctrl(&self) -> &SyncCh2Ctrl {
        &self.sync_ch2_ctrl
    }
    #[doc = "0x54 - No Description"]
    #[inline(always)]
    pub const fn sync_ch3_ctrl(&self) -> &SyncCh3Ctrl {
        &self.sync_ch3_ctrl
    }
    #[doc = "0x58 - CALDN Consumer Register"]
    #[inline(always)]
    pub const fn consumer_cmu_caldn(&self) -> &ConsumerCmuCaldn {
        &self.consumer_cmu_caldn
    }
    #[doc = "0x5c - CALUP Consumer Register"]
    #[inline(always)]
    pub const fn consumer_cmu_calup(&self) -> &ConsumerCmuCalup {
        &self.consumer_cmu_calup
    }
    #[doc = "0x64 - SCAN Consumer Register"]
    #[inline(always)]
    pub const fn consumer_iadc0_scantrigger(&self) -> &ConsumerIadc0Scantrigger {
        &self.consumer_iadc0_scantrigger
    }
    #[doc = "0x68 - SINGLE Consumer Register"]
    #[inline(always)]
    pub const fn consumer_iadc0_singletrigger(&self) -> &ConsumerIadc0Singletrigger {
        &self.consumer_iadc0_singletrigger
    }
    #[doc = "0x6c - DMAREQ0 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq0(&self) -> &ConsumerLdmaxbarDmareq0 {
        &self.consumer_ldmaxbar_dmareq0
    }
    #[doc = "0x70 - DMAREQ1 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_ldmaxbar_dmareq1(&self) -> &ConsumerLdmaxbarDmareq1 {
        &self.consumer_ldmaxbar_dmareq1
    }
    #[doc = "0x74 - CLEAR Consumer Register"]
    #[inline(always)]
    pub const fn consumer_letimer0_clear(&self) -> &ConsumerLetimer0Clear {
        &self.consumer_letimer0_clear
    }
    #[doc = "0x78 - START Consumer Register"]
    #[inline(always)]
    pub const fn consumer_letimer0_start(&self) -> &ConsumerLetimer0Start {
        &self.consumer_letimer0_start
    }
    #[doc = "0x7c - STOP Consumer Register"]
    #[inline(always)]
    pub const fn consumer_letimer0_stop(&self) -> &ConsumerLetimer0Stop {
        &self.consumer_letimer0_stop
    }
    #[doc = "0x80 - RX Consumer Register"]
    #[inline(always)]
    pub const fn consumer_euart0_rx(&self) -> &ConsumerEuart0Rx {
        &self.consumer_euart0_rx
    }
    #[doc = "0x84 - TRIGGER Consumer Register"]
    #[inline(always)]
    pub const fn consumer_euart0_trigger(&self) -> &ConsumerEuart0Trigger {
        &self.consumer_euart0_trigger
    }
    #[doc = "0x88 - DIN Consumer Register"]
    #[inline(always)]
    pub const fn consumer_modem_din(&self) -> &ConsumerModemDin {
        &self.consumer_modem_din
    }
    #[doc = "0xc0 - CLR Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rac_clr(&self) -> &ConsumerRacClr {
        &self.consumer_rac_clr
    }
    #[doc = "0xc4 - CTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin0(&self) -> &ConsumerRacCtiin0 {
        &self.consumer_rac_ctiin0
    }
    #[doc = "0xc8 - CTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin1(&self) -> &ConsumerRacCtiin1 {
        &self.consumer_rac_ctiin1
    }
    #[doc = "0xcc - CTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin2(&self) -> &ConsumerRacCtiin2 {
        &self.consumer_rac_ctiin2
    }
    #[doc = "0xd0 - CTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rac_ctiin3(&self) -> &ConsumerRacCtiin3 {
        &self.consumer_rac_ctiin3
    }
    #[doc = "0xd4 - FORCETX Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rac_forcetx(&self) -> &ConsumerRacForcetx {
        &self.consumer_rac_forcetx
    }
    #[doc = "0xd8 - RXDIS Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rac_rxdis(&self) -> &ConsumerRacRxdis {
        &self.consumer_rac_rxdis
    }
    #[doc = "0xdc - RXEN Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rac_rxen(&self) -> &ConsumerRacRxen {
        &self.consumer_rac_rxen
    }
    #[doc = "0xe0 - SEQ Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rac_seq(&self) -> &ConsumerRacSeq {
        &self.consumer_rac_seq
    }
    #[doc = "0xe4 - TXEN Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rac_txen(&self) -> &ConsumerRacTxen {
        &self.consumer_rac_txen
    }
    #[doc = "0xe8 - CC0 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rtcc_cc0(&self) -> &ConsumerRtccCc0 {
        &self.consumer_rtcc_cc0
    }
    #[doc = "0xec - CC1 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rtcc_cc1(&self) -> &ConsumerRtccCc1 {
        &self.consumer_rtcc_cc1
    }
    #[doc = "0xf0 - CC2 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_rtcc_cc2(&self) -> &ConsumerRtccCc2 {
        &self.consumer_rtcc_cc2
    }
    #[doc = "0xf8 - CTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_core_ctiin0(&self) -> &ConsumerCoreCtiin0 {
        &self.consumer_core_ctiin0
    }
    #[doc = "0xfc - CTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_core_ctiin1(&self) -> &ConsumerCoreCtiin1 {
        &self.consumer_core_ctiin1
    }
    #[doc = "0x100 - CTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_core_ctiin2(&self) -> &ConsumerCoreCtiin2 {
        &self.consumer_core_ctiin2
    }
    #[doc = "0x104 - CTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_core_ctiin3(&self) -> &ConsumerCoreCtiin3 {
        &self.consumer_core_ctiin3
    }
    #[doc = "0x108 - M33 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_core_m33rxev(&self) -> &ConsumerCoreM33rxev {
        &self.consumer_core_m33rxev
    }
    #[doc = "0x10c - CC0 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer0_cc0(&self) -> &ConsumerTimer0Cc0 {
        &self.consumer_timer0_cc0
    }
    #[doc = "0x110 - CC1 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer0_cc1(&self) -> &ConsumerTimer0Cc1 {
        &self.consumer_timer0_cc1
    }
    #[doc = "0x114 - CC2 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer0_cc2(&self) -> &ConsumerTimer0Cc2 {
        &self.consumer_timer0_cc2
    }
    #[doc = "0x118 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer0_dti(&self) -> &ConsumerTimer0Dti {
        &self.consumer_timer0_dti
    }
    #[doc = "0x11c - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs1(&self) -> &ConsumerTimer0Dtifs1 {
        &self.consumer_timer0_dtifs1
    }
    #[doc = "0x120 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer0_dtifs2(&self) -> &ConsumerTimer0Dtifs2 {
        &self.consumer_timer0_dtifs2
    }
    #[doc = "0x124 - CC0 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer1_cc0(&self) -> &ConsumerTimer1Cc0 {
        &self.consumer_timer1_cc0
    }
    #[doc = "0x128 - CC1 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer1_cc1(&self) -> &ConsumerTimer1Cc1 {
        &self.consumer_timer1_cc1
    }
    #[doc = "0x12c - CC2 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer1_cc2(&self) -> &ConsumerTimer1Cc2 {
        &self.consumer_timer1_cc2
    }
    #[doc = "0x130 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer1_dti(&self) -> &ConsumerTimer1Dti {
        &self.consumer_timer1_dti
    }
    #[doc = "0x134 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs1(&self) -> &ConsumerTimer1Dtifs1 {
        &self.consumer_timer1_dtifs1
    }
    #[doc = "0x138 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer1_dtifs2(&self) -> &ConsumerTimer1Dtifs2 {
        &self.consumer_timer1_dtifs2
    }
    #[doc = "0x13c - CC0 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer2_cc0(&self) -> &ConsumerTimer2Cc0 {
        &self.consumer_timer2_cc0
    }
    #[doc = "0x140 - CC1 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer2_cc1(&self) -> &ConsumerTimer2Cc1 {
        &self.consumer_timer2_cc1
    }
    #[doc = "0x144 - CC2 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer2_cc2(&self) -> &ConsumerTimer2Cc2 {
        &self.consumer_timer2_cc2
    }
    #[doc = "0x148 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer2_dti(&self) -> &ConsumerTimer2Dti {
        &self.consumer_timer2_dti
    }
    #[doc = "0x14c - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs1(&self) -> &ConsumerTimer2Dtifs1 {
        &self.consumer_timer2_dtifs1
    }
    #[doc = "0x150 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer2_dtifs2(&self) -> &ConsumerTimer2Dtifs2 {
        &self.consumer_timer2_dtifs2
    }
    #[doc = "0x154 - CC0 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer3_cc0(&self) -> &ConsumerTimer3Cc0 {
        &self.consumer_timer3_cc0
    }
    #[doc = "0x158 - CC1 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer3_cc1(&self) -> &ConsumerTimer3Cc1 {
        &self.consumer_timer3_cc1
    }
    #[doc = "0x15c - CC2 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer3_cc2(&self) -> &ConsumerTimer3Cc2 {
        &self.consumer_timer3_cc2
    }
    #[doc = "0x160 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer3_dti(&self) -> &ConsumerTimer3Dti {
        &self.consumer_timer3_dti
    }
    #[doc = "0x164 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs1(&self) -> &ConsumerTimer3Dtifs1 {
        &self.consumer_timer3_dtifs1
    }
    #[doc = "0x168 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer3_dtifs2(&self) -> &ConsumerTimer3Dtifs2 {
        &self.consumer_timer3_dtifs2
    }
    #[doc = "0x16c - CC0 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer4_cc0(&self) -> &ConsumerTimer4Cc0 {
        &self.consumer_timer4_cc0
    }
    #[doc = "0x170 - CC1 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer4_cc1(&self) -> &ConsumerTimer4Cc1 {
        &self.consumer_timer4_cc1
    }
    #[doc = "0x174 - CC2 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer4_cc2(&self) -> &ConsumerTimer4Cc2 {
        &self.consumer_timer4_cc2
    }
    #[doc = "0x178 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer4_dti(&self) -> &ConsumerTimer4Dti {
        &self.consumer_timer4_dti
    }
    #[doc = "0x17c - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs1(&self) -> &ConsumerTimer4Dtifs1 {
        &self.consumer_timer4_dtifs1
    }
    #[doc = "0x180 - DTI Consumer Register"]
    #[inline(always)]
    pub const fn consumer_timer4_dtifs2(&self) -> &ConsumerTimer4Dtifs2 {
        &self.consumer_timer4_dtifs2
    }
    #[doc = "0x184 - CLK Consumer Register"]
    #[inline(always)]
    pub const fn consumer_usart0_clk(&self) -> &ConsumerUsart0Clk {
        &self.consumer_usart0_clk
    }
    #[doc = "0x188 - IR Consumer Register"]
    #[inline(always)]
    pub const fn consumer_usart0_ir(&self) -> &ConsumerUsart0Ir {
        &self.consumer_usart0_ir
    }
    #[doc = "0x18c - RX Consumer Register"]
    #[inline(always)]
    pub const fn consumer_usart0_rx(&self) -> &ConsumerUsart0Rx {
        &self.consumer_usart0_rx
    }
    #[doc = "0x190 - TRIGGER Consumer Register"]
    #[inline(always)]
    pub const fn consumer_usart0_trigger(&self) -> &ConsumerUsart0Trigger {
        &self.consumer_usart0_trigger
    }
    #[doc = "0x194 - CLK Consumer Register"]
    #[inline(always)]
    pub const fn consumer_usart1_clk(&self) -> &ConsumerUsart1Clk {
        &self.consumer_usart1_clk
    }
    #[doc = "0x198 - IR Consumer Register"]
    #[inline(always)]
    pub const fn consumer_usart1_ir(&self) -> &ConsumerUsart1Ir {
        &self.consumer_usart1_ir
    }
    #[doc = "0x19c - RX Consumer Register"]
    #[inline(always)]
    pub const fn consumer_usart1_rx(&self) -> &ConsumerUsart1Rx {
        &self.consumer_usart1_rx
    }
    #[doc = "0x1a0 - TRIGGER Consumer Register"]
    #[inline(always)]
    pub const fn consumer_usart1_trigger(&self) -> &ConsumerUsart1Trigger {
        &self.consumer_usart1_trigger
    }
    #[doc = "0x1a4 - SRC0 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_wdog0_src0(&self) -> &ConsumerWdog0Src0 {
        &self.consumer_wdog0_src0
    }
    #[doc = "0x1a8 - SRC1 Consumer Register"]
    #[inline(always)]
    pub const fn consumer_wdog0_src1(&self) -> &ConsumerWdog0Src1 {
        &self.consumer_wdog0_src1
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "ASYNC_SWPULSE (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_swpulse::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_swpulse`] module"]
#[doc(alias = "ASYNC_SWPULSE")]
pub type AsyncSwpulse = crate::Reg<async_swpulse::AsyncSwpulseSpec>;
#[doc = "No Description"]
pub mod async_swpulse;
#[doc = "ASYNC_SWLEVEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_swlevel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_swlevel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_swlevel`] module"]
#[doc(alias = "ASYNC_SWLEVEL")]
pub type AsyncSwlevel = crate::Reg<async_swlevel::AsyncSwlevelSpec>;
#[doc = "No Description"]
pub mod async_swlevel;
#[doc = "ASYNC_PEEK (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_peek::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_peek`] module"]
#[doc(alias = "ASYNC_PEEK")]
pub type AsyncPeek = crate::Reg<async_peek::AsyncPeekSpec>;
#[doc = "No Description"]
pub mod async_peek;
#[doc = "SYNC_PEEK (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_peek::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_peek`] module"]
#[doc(alias = "SYNC_PEEK")]
pub type SyncPeek = crate::Reg<sync_peek::SyncPeekSpec>;
#[doc = "No Description"]
pub mod sync_peek;
#[doc = "ASYNC_CH0_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch0_ctrl`] module"]
#[doc(alias = "ASYNC_CH0_CTRL")]
pub type AsyncCh0Ctrl = crate::Reg<async_ch0_ctrl::AsyncCh0CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch0_ctrl;
#[doc = "ASYNC_CH1_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch1_ctrl`] module"]
#[doc(alias = "ASYNC_CH1_CTRL")]
pub type AsyncCh1Ctrl = crate::Reg<async_ch1_ctrl::AsyncCh1CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch1_ctrl;
#[doc = "ASYNC_CH2_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch2_ctrl`] module"]
#[doc(alias = "ASYNC_CH2_CTRL")]
pub type AsyncCh2Ctrl = crate::Reg<async_ch2_ctrl::AsyncCh2CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch2_ctrl;
#[doc = "ASYNC_CH3_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch3_ctrl`] module"]
#[doc(alias = "ASYNC_CH3_CTRL")]
pub type AsyncCh3Ctrl = crate::Reg<async_ch3_ctrl::AsyncCh3CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch3_ctrl;
#[doc = "ASYNC_CH4_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch4_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch4_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch4_ctrl`] module"]
#[doc(alias = "ASYNC_CH4_CTRL")]
pub type AsyncCh4Ctrl = crate::Reg<async_ch4_ctrl::AsyncCh4CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch4_ctrl;
#[doc = "ASYNC_CH5_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch5_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch5_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch5_ctrl`] module"]
#[doc(alias = "ASYNC_CH5_CTRL")]
pub type AsyncCh5Ctrl = crate::Reg<async_ch5_ctrl::AsyncCh5CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch5_ctrl;
#[doc = "ASYNC_CH6_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch6_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch6_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch6_ctrl`] module"]
#[doc(alias = "ASYNC_CH6_CTRL")]
pub type AsyncCh6Ctrl = crate::Reg<async_ch6_ctrl::AsyncCh6CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch6_ctrl;
#[doc = "ASYNC_CH7_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch7_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch7_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch7_ctrl`] module"]
#[doc(alias = "ASYNC_CH7_CTRL")]
pub type AsyncCh7Ctrl = crate::Reg<async_ch7_ctrl::AsyncCh7CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch7_ctrl;
#[doc = "ASYNC_CH8_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch8_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch8_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch8_ctrl`] module"]
#[doc(alias = "ASYNC_CH8_CTRL")]
pub type AsyncCh8Ctrl = crate::Reg<async_ch8_ctrl::AsyncCh8CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch8_ctrl;
#[doc = "ASYNC_CH9_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch9_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch9_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch9_ctrl`] module"]
#[doc(alias = "ASYNC_CH9_CTRL")]
pub type AsyncCh9Ctrl = crate::Reg<async_ch9_ctrl::AsyncCh9CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch9_ctrl;
#[doc = "ASYNC_CH10_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch10_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch10_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch10_ctrl`] module"]
#[doc(alias = "ASYNC_CH10_CTRL")]
pub type AsyncCh10Ctrl = crate::Reg<async_ch10_ctrl::AsyncCh10CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch10_ctrl;
#[doc = "ASYNC_CH11_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch11_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch11_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@async_ch11_ctrl`] module"]
#[doc(alias = "ASYNC_CH11_CTRL")]
pub type AsyncCh11Ctrl = crate::Reg<async_ch11_ctrl::AsyncCh11CtrlSpec>;
#[doc = "No Description"]
pub mod async_ch11_ctrl;
#[doc = "SYNC_CH0_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ch0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ch0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_ch0_ctrl`] module"]
#[doc(alias = "SYNC_CH0_CTRL")]
pub type SyncCh0Ctrl = crate::Reg<sync_ch0_ctrl::SyncCh0CtrlSpec>;
#[doc = "No Description"]
pub mod sync_ch0_ctrl;
#[doc = "SYNC_CH1_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ch1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ch1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_ch1_ctrl`] module"]
#[doc(alias = "SYNC_CH1_CTRL")]
pub type SyncCh1Ctrl = crate::Reg<sync_ch1_ctrl::SyncCh1CtrlSpec>;
#[doc = "No Description"]
pub mod sync_ch1_ctrl;
#[doc = "SYNC_CH2_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ch2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ch2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_ch2_ctrl`] module"]
#[doc(alias = "SYNC_CH2_CTRL")]
pub type SyncCh2Ctrl = crate::Reg<sync_ch2_ctrl::SyncCh2CtrlSpec>;
#[doc = "No Description"]
pub mod sync_ch2_ctrl;
#[doc = "SYNC_CH3_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ch3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ch3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_ch3_ctrl`] module"]
#[doc(alias = "SYNC_CH3_CTRL")]
pub type SyncCh3Ctrl = crate::Reg<sync_ch3_ctrl::SyncCh3CtrlSpec>;
#[doc = "No Description"]
pub mod sync_ch3_ctrl;
#[doc = "CONSUMER_CMU_CALDN (rw) register accessor: CALDN Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_cmu_caldn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_cmu_caldn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_cmu_caldn`] module"]
#[doc(alias = "CONSUMER_CMU_CALDN")]
pub type ConsumerCmuCaldn = crate::Reg<consumer_cmu_caldn::ConsumerCmuCaldnSpec>;
#[doc = "CALDN Consumer Register"]
pub mod consumer_cmu_caldn;
#[doc = "CONSUMER_CMU_CALUP (rw) register accessor: CALUP Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_cmu_calup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_cmu_calup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_cmu_calup`] module"]
#[doc(alias = "CONSUMER_CMU_CALUP")]
pub type ConsumerCmuCalup = crate::Reg<consumer_cmu_calup::ConsumerCmuCalupSpec>;
#[doc = "CALUP Consumer Register"]
pub mod consumer_cmu_calup;
#[doc = "CONSUMER_IADC0_SCANTRIGGER (rw) register accessor: SCAN Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_iadc0_scantrigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_iadc0_scantrigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_iadc0_scantrigger`] module"]
#[doc(alias = "CONSUMER_IADC0_SCANTRIGGER")]
pub type ConsumerIadc0Scantrigger =
    crate::Reg<consumer_iadc0_scantrigger::ConsumerIadc0ScantriggerSpec>;
#[doc = "SCAN Consumer Register"]
pub mod consumer_iadc0_scantrigger;
#[doc = "CONSUMER_IADC0_SINGLETRIGGER (rw) register accessor: SINGLE Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_iadc0_singletrigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_iadc0_singletrigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_iadc0_singletrigger`] module"]
#[doc(alias = "CONSUMER_IADC0_SINGLETRIGGER")]
pub type ConsumerIadc0Singletrigger =
    crate::Reg<consumer_iadc0_singletrigger::ConsumerIadc0SingletriggerSpec>;
#[doc = "SINGLE Consumer Register"]
pub mod consumer_iadc0_singletrigger;
#[doc = "CONSUMER_LDMAXBAR_DMAREQ0 (rw) register accessor: DMAREQ0 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_ldmaxbar_dmareq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_ldmaxbar_dmareq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_ldmaxbar_dmareq0`] module"]
#[doc(alias = "CONSUMER_LDMAXBAR_DMAREQ0")]
pub type ConsumerLdmaxbarDmareq0 =
    crate::Reg<consumer_ldmaxbar_dmareq0::ConsumerLdmaxbarDmareq0Spec>;
#[doc = "DMAREQ0 Consumer Register"]
pub mod consumer_ldmaxbar_dmareq0;
#[doc = "CONSUMER_LDMAXBAR_DMAREQ1 (rw) register accessor: DMAREQ1 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_ldmaxbar_dmareq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_ldmaxbar_dmareq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_ldmaxbar_dmareq1`] module"]
#[doc(alias = "CONSUMER_LDMAXBAR_DMAREQ1")]
pub type ConsumerLdmaxbarDmareq1 =
    crate::Reg<consumer_ldmaxbar_dmareq1::ConsumerLdmaxbarDmareq1Spec>;
#[doc = "DMAREQ1 Consumer Register"]
pub mod consumer_ldmaxbar_dmareq1;
#[doc = "CONSUMER_LETIMER0_CLEAR (rw) register accessor: CLEAR Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_letimer0_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_letimer0_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_letimer0_clear`] module"]
#[doc(alias = "CONSUMER_LETIMER0_CLEAR")]
pub type ConsumerLetimer0Clear = crate::Reg<consumer_letimer0_clear::ConsumerLetimer0ClearSpec>;
#[doc = "CLEAR Consumer Register"]
pub mod consumer_letimer0_clear;
#[doc = "CONSUMER_LETIMER0_START (rw) register accessor: START Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_letimer0_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_letimer0_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_letimer0_start`] module"]
#[doc(alias = "CONSUMER_LETIMER0_START")]
pub type ConsumerLetimer0Start = crate::Reg<consumer_letimer0_start::ConsumerLetimer0StartSpec>;
#[doc = "START Consumer Register"]
pub mod consumer_letimer0_start;
#[doc = "CONSUMER_LETIMER0_STOP (rw) register accessor: STOP Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_letimer0_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_letimer0_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_letimer0_stop`] module"]
#[doc(alias = "CONSUMER_LETIMER0_STOP")]
pub type ConsumerLetimer0Stop = crate::Reg<consumer_letimer0_stop::ConsumerLetimer0StopSpec>;
#[doc = "STOP Consumer Register"]
pub mod consumer_letimer0_stop;
#[doc = "CONSUMER_EUART0_RX (rw) register accessor: RX Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_euart0_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_euart0_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_euart0_rx`] module"]
#[doc(alias = "CONSUMER_EUART0_RX")]
pub type ConsumerEuart0Rx = crate::Reg<consumer_euart0_rx::ConsumerEuart0RxSpec>;
#[doc = "RX Consumer Register"]
pub mod consumer_euart0_rx;
#[doc = "CONSUMER_EUART0_TRIGGER (rw) register accessor: TRIGGER Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_euart0_trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_euart0_trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_euart0_trigger`] module"]
#[doc(alias = "CONSUMER_EUART0_TRIGGER")]
pub type ConsumerEuart0Trigger = crate::Reg<consumer_euart0_trigger::ConsumerEuart0TriggerSpec>;
#[doc = "TRIGGER Consumer Register"]
pub mod consumer_euart0_trigger;
#[doc = "CONSUMER_MODEM_DIN (rw) register accessor: DIN Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_modem_din::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_modem_din::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_modem_din`] module"]
#[doc(alias = "CONSUMER_MODEM_DIN")]
pub type ConsumerModemDin = crate::Reg<consumer_modem_din::ConsumerModemDinSpec>;
#[doc = "DIN Consumer Register"]
pub mod consumer_modem_din;
#[doc = "CONSUMER_RAC_CLR (rw) register accessor: CLR Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_clr`] module"]
#[doc(alias = "CONSUMER_RAC_CLR")]
pub type ConsumerRacClr = crate::Reg<consumer_rac_clr::ConsumerRacClrSpec>;
#[doc = "CLR Consumer Register"]
pub mod consumer_rac_clr;
#[doc = "CONSUMER_RAC_CTIIN0 (rw) register accessor: CTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_ctiin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_ctiin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_ctiin0`] module"]
#[doc(alias = "CONSUMER_RAC_CTIIN0")]
pub type ConsumerRacCtiin0 = crate::Reg<consumer_rac_ctiin0::ConsumerRacCtiin0Spec>;
#[doc = "CTI Consumer Register"]
pub mod consumer_rac_ctiin0;
#[doc = "CONSUMER_RAC_CTIIN1 (rw) register accessor: CTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_ctiin1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_ctiin1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_ctiin1`] module"]
#[doc(alias = "CONSUMER_RAC_CTIIN1")]
pub type ConsumerRacCtiin1 = crate::Reg<consumer_rac_ctiin1::ConsumerRacCtiin1Spec>;
#[doc = "CTI Consumer Register"]
pub mod consumer_rac_ctiin1;
#[doc = "CONSUMER_RAC_CTIIN2 (rw) register accessor: CTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_ctiin2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_ctiin2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_ctiin2`] module"]
#[doc(alias = "CONSUMER_RAC_CTIIN2")]
pub type ConsumerRacCtiin2 = crate::Reg<consumer_rac_ctiin2::ConsumerRacCtiin2Spec>;
#[doc = "CTI Consumer Register"]
pub mod consumer_rac_ctiin2;
#[doc = "CONSUMER_RAC_CTIIN3 (rw) register accessor: CTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_ctiin3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_ctiin3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_ctiin3`] module"]
#[doc(alias = "CONSUMER_RAC_CTIIN3")]
pub type ConsumerRacCtiin3 = crate::Reg<consumer_rac_ctiin3::ConsumerRacCtiin3Spec>;
#[doc = "CTI Consumer Register"]
pub mod consumer_rac_ctiin3;
#[doc = "CONSUMER_RAC_FORCETX (rw) register accessor: FORCETX Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_forcetx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_forcetx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_forcetx`] module"]
#[doc(alias = "CONSUMER_RAC_FORCETX")]
pub type ConsumerRacForcetx = crate::Reg<consumer_rac_forcetx::ConsumerRacForcetxSpec>;
#[doc = "FORCETX Consumer Register"]
pub mod consumer_rac_forcetx;
#[doc = "CONSUMER_RAC_RXDIS (rw) register accessor: RXDIS Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_rxdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_rxdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_rxdis`] module"]
#[doc(alias = "CONSUMER_RAC_RXDIS")]
pub type ConsumerRacRxdis = crate::Reg<consumer_rac_rxdis::ConsumerRacRxdisSpec>;
#[doc = "RXDIS Consumer Register"]
pub mod consumer_rac_rxdis;
#[doc = "CONSUMER_RAC_RXEN (rw) register accessor: RXEN Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_rxen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_rxen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_rxen`] module"]
#[doc(alias = "CONSUMER_RAC_RXEN")]
pub type ConsumerRacRxen = crate::Reg<consumer_rac_rxen::ConsumerRacRxenSpec>;
#[doc = "RXEN Consumer Register"]
pub mod consumer_rac_rxen;
#[doc = "CONSUMER_RAC_SEQ (rw) register accessor: SEQ Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_seq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_seq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_seq`] module"]
#[doc(alias = "CONSUMER_RAC_SEQ")]
pub type ConsumerRacSeq = crate::Reg<consumer_rac_seq::ConsumerRacSeqSpec>;
#[doc = "SEQ Consumer Register"]
pub mod consumer_rac_seq;
#[doc = "CONSUMER_RAC_TXEN (rw) register accessor: TXEN Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_txen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_txen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rac_txen`] module"]
#[doc(alias = "CONSUMER_RAC_TXEN")]
pub type ConsumerRacTxen = crate::Reg<consumer_rac_txen::ConsumerRacTxenSpec>;
#[doc = "TXEN Consumer Register"]
pub mod consumer_rac_txen;
#[doc = "CONSUMER_RTCC_CC0 (rw) register accessor: CC0 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rtcc_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rtcc_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rtcc_cc0`] module"]
#[doc(alias = "CONSUMER_RTCC_CC0")]
pub type ConsumerRtccCc0 = crate::Reg<consumer_rtcc_cc0::ConsumerRtccCc0Spec>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_rtcc_cc0;
#[doc = "CONSUMER_RTCC_CC1 (rw) register accessor: CC1 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rtcc_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rtcc_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rtcc_cc1`] module"]
#[doc(alias = "CONSUMER_RTCC_CC1")]
pub type ConsumerRtccCc1 = crate::Reg<consumer_rtcc_cc1::ConsumerRtccCc1Spec>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_rtcc_cc1;
#[doc = "CONSUMER_RTCC_CC2 (rw) register accessor: CC2 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rtcc_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rtcc_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_rtcc_cc2`] module"]
#[doc(alias = "CONSUMER_RTCC_CC2")]
pub type ConsumerRtccCc2 = crate::Reg<consumer_rtcc_cc2::ConsumerRtccCc2Spec>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_rtcc_cc2;
#[doc = "CONSUMER_CORE_CTIIN0 (rw) register accessor: CTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_ctiin0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_ctiin0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_core_ctiin0`] module"]
#[doc(alias = "CONSUMER_CORE_CTIIN0")]
pub type ConsumerCoreCtiin0 = crate::Reg<consumer_core_ctiin0::ConsumerCoreCtiin0Spec>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin0;
#[doc = "CONSUMER_CORE_CTIIN1 (rw) register accessor: CTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_ctiin1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_ctiin1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_core_ctiin1`] module"]
#[doc(alias = "CONSUMER_CORE_CTIIN1")]
pub type ConsumerCoreCtiin1 = crate::Reg<consumer_core_ctiin1::ConsumerCoreCtiin1Spec>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin1;
#[doc = "CONSUMER_CORE_CTIIN2 (rw) register accessor: CTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_ctiin2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_ctiin2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_core_ctiin2`] module"]
#[doc(alias = "CONSUMER_CORE_CTIIN2")]
pub type ConsumerCoreCtiin2 = crate::Reg<consumer_core_ctiin2::ConsumerCoreCtiin2Spec>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin2;
#[doc = "CONSUMER_CORE_CTIIN3 (rw) register accessor: CTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_ctiin3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_ctiin3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_core_ctiin3`] module"]
#[doc(alias = "CONSUMER_CORE_CTIIN3")]
pub type ConsumerCoreCtiin3 = crate::Reg<consumer_core_ctiin3::ConsumerCoreCtiin3Spec>;
#[doc = "CTI Consumer Register"]
pub mod consumer_core_ctiin3;
#[doc = "CONSUMER_CORE_M33RXEV (rw) register accessor: M33 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_m33rxev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_m33rxev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_core_m33rxev`] module"]
#[doc(alias = "CONSUMER_CORE_M33RXEV")]
pub type ConsumerCoreM33rxev = crate::Reg<consumer_core_m33rxev::ConsumerCoreM33rxevSpec>;
#[doc = "M33 Consumer Register"]
pub mod consumer_core_m33rxev;
#[doc = "CONSUMER_TIMER0_CC0 (rw) register accessor: CC0 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER0_CC0")]
pub type ConsumerTimer0Cc0 = crate::Reg<consumer_timer0_cc0::ConsumerTimer0Cc0Spec>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_timer0_cc0;
#[doc = "CONSUMER_TIMER0_CC1 (rw) register accessor: CC1 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER0_CC1")]
pub type ConsumerTimer0Cc1 = crate::Reg<consumer_timer0_cc1::ConsumerTimer0Cc1Spec>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_timer0_cc1;
#[doc = "CONSUMER_TIMER0_CC2 (rw) register accessor: CC2 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER0_CC2")]
pub type ConsumerTimer0Cc2 = crate::Reg<consumer_timer0_cc2::ConsumerTimer0Cc2Spec>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_timer0_cc2;
#[doc = "CONSUMER_TIMER0_DTI (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_dti`] module"]
#[doc(alias = "CONSUMER_TIMER0_DTI")]
pub type ConsumerTimer0Dti = crate::Reg<consumer_timer0_dti::ConsumerTimer0DtiSpec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer0_dti;
#[doc = "CONSUMER_TIMER0_DTIFS1 (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER0_DTIFS1")]
pub type ConsumerTimer0Dtifs1 = crate::Reg<consumer_timer0_dtifs1::ConsumerTimer0Dtifs1Spec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer0_dtifs1;
#[doc = "CONSUMER_TIMER0_DTIFS2 (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer0_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer0_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer0_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER0_DTIFS2")]
pub type ConsumerTimer0Dtifs2 = crate::Reg<consumer_timer0_dtifs2::ConsumerTimer0Dtifs2Spec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer0_dtifs2;
#[doc = "CONSUMER_TIMER1_CC0 (rw) register accessor: CC0 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER1_CC0")]
pub type ConsumerTimer1Cc0 = crate::Reg<consumer_timer1_cc0::ConsumerTimer1Cc0Spec>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_timer1_cc0;
#[doc = "CONSUMER_TIMER1_CC1 (rw) register accessor: CC1 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER1_CC1")]
pub type ConsumerTimer1Cc1 = crate::Reg<consumer_timer1_cc1::ConsumerTimer1Cc1Spec>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_timer1_cc1;
#[doc = "CONSUMER_TIMER1_CC2 (rw) register accessor: CC2 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER1_CC2")]
pub type ConsumerTimer1Cc2 = crate::Reg<consumer_timer1_cc2::ConsumerTimer1Cc2Spec>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_timer1_cc2;
#[doc = "CONSUMER_TIMER1_DTI (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_dti`] module"]
#[doc(alias = "CONSUMER_TIMER1_DTI")]
pub type ConsumerTimer1Dti = crate::Reg<consumer_timer1_dti::ConsumerTimer1DtiSpec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer1_dti;
#[doc = "CONSUMER_TIMER1_DTIFS1 (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER1_DTIFS1")]
pub type ConsumerTimer1Dtifs1 = crate::Reg<consumer_timer1_dtifs1::ConsumerTimer1Dtifs1Spec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer1_dtifs1;
#[doc = "CONSUMER_TIMER1_DTIFS2 (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer1_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER1_DTIFS2")]
pub type ConsumerTimer1Dtifs2 = crate::Reg<consumer_timer1_dtifs2::ConsumerTimer1Dtifs2Spec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer1_dtifs2;
#[doc = "CONSUMER_TIMER2_CC0 (rw) register accessor: CC0 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER2_CC0")]
pub type ConsumerTimer2Cc0 = crate::Reg<consumer_timer2_cc0::ConsumerTimer2Cc0Spec>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_timer2_cc0;
#[doc = "CONSUMER_TIMER2_CC1 (rw) register accessor: CC1 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER2_CC1")]
pub type ConsumerTimer2Cc1 = crate::Reg<consumer_timer2_cc1::ConsumerTimer2Cc1Spec>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_timer2_cc1;
#[doc = "CONSUMER_TIMER2_CC2 (rw) register accessor: CC2 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER2_CC2")]
pub type ConsumerTimer2Cc2 = crate::Reg<consumer_timer2_cc2::ConsumerTimer2Cc2Spec>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_timer2_cc2;
#[doc = "CONSUMER_TIMER2_DTI (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_dti`] module"]
#[doc(alias = "CONSUMER_TIMER2_DTI")]
pub type ConsumerTimer2Dti = crate::Reg<consumer_timer2_dti::ConsumerTimer2DtiSpec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer2_dti;
#[doc = "CONSUMER_TIMER2_DTIFS1 (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER2_DTIFS1")]
pub type ConsumerTimer2Dtifs1 = crate::Reg<consumer_timer2_dtifs1::ConsumerTimer2Dtifs1Spec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer2_dtifs1;
#[doc = "CONSUMER_TIMER2_DTIFS2 (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer2_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer2_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer2_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER2_DTIFS2")]
pub type ConsumerTimer2Dtifs2 = crate::Reg<consumer_timer2_dtifs2::ConsumerTimer2Dtifs2Spec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer2_dtifs2;
#[doc = "CONSUMER_TIMER3_CC0 (rw) register accessor: CC0 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER3_CC0")]
pub type ConsumerTimer3Cc0 = crate::Reg<consumer_timer3_cc0::ConsumerTimer3Cc0Spec>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_timer3_cc0;
#[doc = "CONSUMER_TIMER3_CC1 (rw) register accessor: CC1 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER3_CC1")]
pub type ConsumerTimer3Cc1 = crate::Reg<consumer_timer3_cc1::ConsumerTimer3Cc1Spec>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_timer3_cc1;
#[doc = "CONSUMER_TIMER3_CC2 (rw) register accessor: CC2 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER3_CC2")]
pub type ConsumerTimer3Cc2 = crate::Reg<consumer_timer3_cc2::ConsumerTimer3Cc2Spec>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_timer3_cc2;
#[doc = "CONSUMER_TIMER3_DTI (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_dti`] module"]
#[doc(alias = "CONSUMER_TIMER3_DTI")]
pub type ConsumerTimer3Dti = crate::Reg<consumer_timer3_dti::ConsumerTimer3DtiSpec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer3_dti;
#[doc = "CONSUMER_TIMER3_DTIFS1 (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER3_DTIFS1")]
pub type ConsumerTimer3Dtifs1 = crate::Reg<consumer_timer3_dtifs1::ConsumerTimer3Dtifs1Spec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer3_dtifs1;
#[doc = "CONSUMER_TIMER3_DTIFS2 (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer3_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER3_DTIFS2")]
pub type ConsumerTimer3Dtifs2 = crate::Reg<consumer_timer3_dtifs2::ConsumerTimer3Dtifs2Spec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer3_dtifs2;
#[doc = "CONSUMER_TIMER4_CC0 (rw) register accessor: CC0 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_cc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_cc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_cc0`] module"]
#[doc(alias = "CONSUMER_TIMER4_CC0")]
pub type ConsumerTimer4Cc0 = crate::Reg<consumer_timer4_cc0::ConsumerTimer4Cc0Spec>;
#[doc = "CC0 Consumer Register"]
pub mod consumer_timer4_cc0;
#[doc = "CONSUMER_TIMER4_CC1 (rw) register accessor: CC1 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_cc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_cc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_cc1`] module"]
#[doc(alias = "CONSUMER_TIMER4_CC1")]
pub type ConsumerTimer4Cc1 = crate::Reg<consumer_timer4_cc1::ConsumerTimer4Cc1Spec>;
#[doc = "CC1 Consumer Register"]
pub mod consumer_timer4_cc1;
#[doc = "CONSUMER_TIMER4_CC2 (rw) register accessor: CC2 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_cc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_cc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_cc2`] module"]
#[doc(alias = "CONSUMER_TIMER4_CC2")]
pub type ConsumerTimer4Cc2 = crate::Reg<consumer_timer4_cc2::ConsumerTimer4Cc2Spec>;
#[doc = "CC2 Consumer Register"]
pub mod consumer_timer4_cc2;
#[doc = "CONSUMER_TIMER4_DTI (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_dti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_dti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_dti`] module"]
#[doc(alias = "CONSUMER_TIMER4_DTI")]
pub type ConsumerTimer4Dti = crate::Reg<consumer_timer4_dti::ConsumerTimer4DtiSpec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer4_dti;
#[doc = "CONSUMER_TIMER4_DTIFS1 (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_dtifs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_dtifs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_dtifs1`] module"]
#[doc(alias = "CONSUMER_TIMER4_DTIFS1")]
pub type ConsumerTimer4Dtifs1 = crate::Reg<consumer_timer4_dtifs1::ConsumerTimer4Dtifs1Spec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer4_dtifs1;
#[doc = "CONSUMER_TIMER4_DTIFS2 (rw) register accessor: DTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer4_dtifs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer4_dtifs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_timer4_dtifs2`] module"]
#[doc(alias = "CONSUMER_TIMER4_DTIFS2")]
pub type ConsumerTimer4Dtifs2 = crate::Reg<consumer_timer4_dtifs2::ConsumerTimer4Dtifs2Spec>;
#[doc = "DTI Consumer Register"]
pub mod consumer_timer4_dtifs2;
#[doc = "CONSUMER_USART0_CLK (rw) register accessor: CLK Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_usart0_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_usart0_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_usart0_clk`] module"]
#[doc(alias = "CONSUMER_USART0_CLK")]
pub type ConsumerUsart0Clk = crate::Reg<consumer_usart0_clk::ConsumerUsart0ClkSpec>;
#[doc = "CLK Consumer Register"]
pub mod consumer_usart0_clk;
#[doc = "CONSUMER_USART0_IR (rw) register accessor: IR Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_usart0_ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_usart0_ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_usart0_ir`] module"]
#[doc(alias = "CONSUMER_USART0_IR")]
pub type ConsumerUsart0Ir = crate::Reg<consumer_usart0_ir::ConsumerUsart0IrSpec>;
#[doc = "IR Consumer Register"]
pub mod consumer_usart0_ir;
#[doc = "CONSUMER_USART0_RX (rw) register accessor: RX Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_usart0_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_usart0_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_usart0_rx`] module"]
#[doc(alias = "CONSUMER_USART0_RX")]
pub type ConsumerUsart0Rx = crate::Reg<consumer_usart0_rx::ConsumerUsart0RxSpec>;
#[doc = "RX Consumer Register"]
pub mod consumer_usart0_rx;
#[doc = "CONSUMER_USART0_TRIGGER (rw) register accessor: TRIGGER Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_usart0_trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_usart0_trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_usart0_trigger`] module"]
#[doc(alias = "CONSUMER_USART0_TRIGGER")]
pub type ConsumerUsart0Trigger = crate::Reg<consumer_usart0_trigger::ConsumerUsart0TriggerSpec>;
#[doc = "TRIGGER Consumer Register"]
pub mod consumer_usart0_trigger;
#[doc = "CONSUMER_USART1_CLK (rw) register accessor: CLK Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_usart1_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_usart1_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_usart1_clk`] module"]
#[doc(alias = "CONSUMER_USART1_CLK")]
pub type ConsumerUsart1Clk = crate::Reg<consumer_usart1_clk::ConsumerUsart1ClkSpec>;
#[doc = "CLK Consumer Register"]
pub mod consumer_usart1_clk;
#[doc = "CONSUMER_USART1_IR (rw) register accessor: IR Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_usart1_ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_usart1_ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_usart1_ir`] module"]
#[doc(alias = "CONSUMER_USART1_IR")]
pub type ConsumerUsart1Ir = crate::Reg<consumer_usart1_ir::ConsumerUsart1IrSpec>;
#[doc = "IR Consumer Register"]
pub mod consumer_usart1_ir;
#[doc = "CONSUMER_USART1_RX (rw) register accessor: RX Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_usart1_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_usart1_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_usart1_rx`] module"]
#[doc(alias = "CONSUMER_USART1_RX")]
pub type ConsumerUsart1Rx = crate::Reg<consumer_usart1_rx::ConsumerUsart1RxSpec>;
#[doc = "RX Consumer Register"]
pub mod consumer_usart1_rx;
#[doc = "CONSUMER_USART1_TRIGGER (rw) register accessor: TRIGGER Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_usart1_trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_usart1_trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_usart1_trigger`] module"]
#[doc(alias = "CONSUMER_USART1_TRIGGER")]
pub type ConsumerUsart1Trigger = crate::Reg<consumer_usart1_trigger::ConsumerUsart1TriggerSpec>;
#[doc = "TRIGGER Consumer Register"]
pub mod consumer_usart1_trigger;
#[doc = "CONSUMER_WDOG0_SRC0 (rw) register accessor: SRC0 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_wdog0_src0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_wdog0_src0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_wdog0_src0`] module"]
#[doc(alias = "CONSUMER_WDOG0_SRC0")]
pub type ConsumerWdog0Src0 = crate::Reg<consumer_wdog0_src0::ConsumerWdog0Src0Spec>;
#[doc = "SRC0 Consumer Register"]
pub mod consumer_wdog0_src0;
#[doc = "CONSUMER_WDOG0_SRC1 (rw) register accessor: SRC1 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_wdog0_src1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_wdog0_src1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@consumer_wdog0_src1`] module"]
#[doc(alias = "CONSUMER_WDOG0_SRC1")]
pub type ConsumerWdog0Src1 = crate::Reg<consumer_wdog0_src1::ConsumerWdog0Src1Spec>;
#[doc = "SRC1 Consumer Register"]
pub mod consumer_wdog0_src1;
