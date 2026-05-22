#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Debug halt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbghalt {
    #[doc = "0: Continue normal UART operation even if core is halted"]
    Disable = 0,
    #[doc = "1: If core is halted, receive one frame and then halt reception by deactivating RTS. Next frame reception happens when the core is unhalted during single stepping."]
    Enable = 1,
}
impl From<Dbghalt> for bool {
    #[inline(always)]
    fn from(variant: Dbghalt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGHALT` reader - Debug halt"]
pub type DbghaltR = crate::BitReader<Dbghalt>;
impl DbghaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbghalt {
        match self.bits {
            false => Dbghalt::Disable,
            true => Dbghalt::Enable,
        }
    }
    #[doc = "Continue normal UART operation even if core is halted"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dbghalt::Disable
    }
    #[doc = "If core is halted, receive one frame and then halt reception by deactivating RTS. Next frame reception happens when the core is unhalted during single stepping."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dbghalt::Enable
    }
}
#[doc = "Field `DBGHALT` writer - Debug halt"]
pub type DbghaltW<'a, REG> = crate::BitWriter<'a, REG, Dbghalt>;
impl<'a, REG> DbghaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue normal UART operation even if core is halted"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbghalt::Disable)
    }
    #[doc = "If core is halted, receive one frame and then halt reception by deactivating RTS. Next frame reception happens when the core is unhalted during single stepping."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbghalt::Enable)
    }
}
#[doc = "Clear-to-send Invert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsinv {
    #[doc = "0: The CTS pin is active low"]
    Disable = 0,
    #[doc = "1: The CTS pin is active high"]
    Enable = 1,
}
impl From<Ctsinv> for bool {
    #[inline(always)]
    fn from(variant: Ctsinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSINV` reader - Clear-to-send Invert Enable"]
pub type CtsinvR = crate::BitReader<Ctsinv>;
impl CtsinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsinv {
        match self.bits {
            false => Ctsinv::Disable,
            true => Ctsinv::Enable,
        }
    }
    #[doc = "The CTS pin is active low"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctsinv::Disable
    }
    #[doc = "The CTS pin is active high"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctsinv::Enable
    }
}
#[doc = "Field `CTSINV` writer - Clear-to-send Invert Enable"]
pub type CtsinvW<'a, REG> = crate::BitWriter<'a, REG, Ctsinv>;
impl<'a, REG> CtsinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CTS pin is active low"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsinv::Disable)
    }
    #[doc = "The CTS pin is active high"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsinv::Enable)
    }
}
#[doc = "Clear-to-send Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsen {
    #[doc = "0: Ignore CTS"]
    Disable = 0,
    #[doc = "1: Stop transmitting when CTS is inactive"]
    Enable = 1,
}
impl From<Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - Clear-to-send Enable"]
pub type CtsenR = crate::BitReader<Ctsen>;
impl CtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsen {
        match self.bits {
            false => Ctsen::Disable,
            true => Ctsen::Enable,
        }
    }
    #[doc = "Ignore CTS"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctsen::Disable
    }
    #[doc = "Stop transmitting when CTS is inactive"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctsen::Enable
    }
}
#[doc = "Field `CTSEN` writer - Clear-to-send Enable"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctsen>;
impl<'a, REG> CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore CTS"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Disable)
    }
    #[doc = "Stop transmitting when CTS is inactive"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Enable)
    }
}
#[doc = "Request-to-send Invert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsinv {
    #[doc = "0: The RTS pin is active low"]
    Disable = 0,
    #[doc = "1: The RTS pin is active high"]
    Enable = 1,
}
impl From<Rtsinv> for bool {
    #[inline(always)]
    fn from(variant: Rtsinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSINV` reader - Request-to-send Invert Enable"]
pub type RtsinvR = crate::BitReader<Rtsinv>;
impl RtsinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsinv {
        match self.bits {
            false => Rtsinv::Disable,
            true => Rtsinv::Enable,
        }
    }
    #[doc = "The RTS pin is active low"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rtsinv::Disable
    }
    #[doc = "The RTS pin is active high"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtsinv::Enable
    }
}
#[doc = "Field `RTSINV` writer - Request-to-send Invert Enable"]
pub type RtsinvW<'a, REG> = crate::BitWriter<'a, REG, Rtsinv>;
impl<'a, REG> RtsinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The RTS pin is active low"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsinv::Disable)
    }
    #[doc = "The RTS pin is active high"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsinv::Enable)
    }
}
#[doc = "Field `TXDMAWU` reader - Transmitter DMA Wakeup"]
pub type TxdmawuR = crate::BitReader;
#[doc = "Field `TXDMAWU` writer - Transmitter DMA Wakeup"]
pub type TxdmawuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAWU` reader - Receiver DMA Wakeup"]
pub type RxdmawuR = crate::BitReader;
#[doc = "Field `RXDMAWU` writer - Receiver DMA Wakeup"]
pub type RxdmawuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFUBRX` reader - Start Frame Unblock Receiver"]
pub type SfubrxR = crate::BitReader;
#[doc = "Field `SFUBRX` writer - Start Frame Unblock Receiver"]
pub type SfubrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPRSEN` reader - PRS RX Enable"]
pub type RxprsenR = crate::BitReader;
#[doc = "Field `RXPRSEN` writer - PRS RX Enable"]
pub type RxprsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TX FIFO Interrupt Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txfiw {
    #[doc = "0: TXFL status flag and IF are set when the TX FIFO has space for at least one more frame."]
    Oneframe = 0,
    #[doc = "1: TXFL status flag and IF are set when the TX FIFO has space for at least two more frames."]
    Twoframes = 1,
    #[doc = "2: TXFL status flag and IF are set when the TX FIFO has space for at least three more frames."]
    Threeframes = 2,
    #[doc = "3: TXFL status flag and IF are set when the TX FIFO has space for at least four more frames."]
    Fourframes = 3,
}
impl From<Txfiw> for u8 {
    #[inline(always)]
    fn from(variant: Txfiw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txfiw {
    type Ux = u8;
}
impl crate::IsEnum for Txfiw {}
#[doc = "Field `TXFIW` reader - TX FIFO Interrupt Watermark"]
pub type TxfiwR = crate::FieldReader<Txfiw>;
impl TxfiwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfiw {
        match self.bits {
            0 => Txfiw::Oneframe,
            1 => Txfiw::Twoframes,
            2 => Txfiw::Threeframes,
            3 => Txfiw::Fourframes,
            _ => unreachable!(),
        }
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least one more frame."]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == Txfiw::Oneframe
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least two more frames."]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == Txfiw::Twoframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least three more frames."]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == Txfiw::Threeframes
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least four more frames."]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == Txfiw::Fourframes
    }
}
#[doc = "Field `TXFIW` writer - TX FIFO Interrupt Watermark"]
pub type TxfiwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txfiw, crate::Safe>;
impl<'a, REG> TxfiwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least one more frame."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Oneframe)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least two more frames."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Twoframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least three more frames."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Threeframes)
    }
    #[doc = "TXFL status flag and IF are set when the TX FIFO has space for at least four more frames."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut crate::W<REG> {
        self.variant(Txfiw::Fourframes)
    }
}
#[doc = "RX FIFO Interrupt Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxfiw {
    #[doc = "0: RXFL status flag and IF are set when the RX FIFO has at least one frame in it."]
    Oneframe = 0,
    #[doc = "1: RXFL status flag and IF are set when the RX FIFO has at least two frames in it."]
    Twoframes = 1,
    #[doc = "2: RXFL status flag and IF are set when the RX FIFO has at least three frames in it."]
    Threeframes = 2,
    #[doc = "3: RXFL status flag and IF are set when the RX FIFO has four frames in it."]
    Fourframes = 3,
}
impl From<Rxfiw> for u8 {
    #[inline(always)]
    fn from(variant: Rxfiw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxfiw {
    type Ux = u8;
}
impl crate::IsEnum for Rxfiw {}
#[doc = "Field `RXFIW` reader - RX FIFO Interrupt Watermark"]
pub type RxfiwR = crate::FieldReader<Rxfiw>;
impl RxfiwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfiw {
        match self.bits {
            0 => Rxfiw::Oneframe,
            1 => Rxfiw::Twoframes,
            2 => Rxfiw::Threeframes,
            3 => Rxfiw::Fourframes,
            _ => unreachable!(),
        }
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least one frame in it."]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == Rxfiw::Oneframe
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least two frames in it."]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == Rxfiw::Twoframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least three frames in it."]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == Rxfiw::Threeframes
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has four frames in it."]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == Rxfiw::Fourframes
    }
}
#[doc = "Field `RXFIW` writer - RX FIFO Interrupt Watermark"]
pub type RxfiwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxfiw, crate::Safe>;
impl<'a, REG> RxfiwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least one frame in it."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Oneframe)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least two frames in it."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Twoframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has at least three frames in it."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Threeframes)
    }
    #[doc = "RXFL status flag and IF are set when the RX FIFO has four frames in it."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfiw::Fourframes)
    }
}
#[doc = "Request-to-send RX FIFO Watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtsrxfw {
    #[doc = "0: RTS is set if there is space for at least one more frame in the RX FIFO."]
    Oneframe = 0,
    #[doc = "1: RTS is set if there is space for at least two more frames in the RX FIFO."]
    Twoframes = 1,
    #[doc = "2: RTS is set if there is space for at least three more frames in the RX FIFO."]
    Threeframes = 2,
    #[doc = "3: RTS is set if there is space for four more frames in the RX FIFO."]
    Fourframes = 3,
}
impl From<Rtsrxfw> for u8 {
    #[inline(always)]
    fn from(variant: Rtsrxfw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtsrxfw {
    type Ux = u8;
}
impl crate::IsEnum for Rtsrxfw {}
#[doc = "Field `RTSRXFW` reader - Request-to-send RX FIFO Watermark"]
pub type RtsrxfwR = crate::FieldReader<Rtsrxfw>;
impl RtsrxfwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsrxfw {
        match self.bits {
            0 => Rtsrxfw::Oneframe,
            1 => Rtsrxfw::Twoframes,
            2 => Rtsrxfw::Threeframes,
            3 => Rtsrxfw::Fourframes,
            _ => unreachable!(),
        }
    }
    #[doc = "RTS is set if there is space for at least one more frame in the RX FIFO."]
    #[inline(always)]
    pub fn is_oneframe(&self) -> bool {
        *self == Rtsrxfw::Oneframe
    }
    #[doc = "RTS is set if there is space for at least two more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_twoframes(&self) -> bool {
        *self == Rtsrxfw::Twoframes
    }
    #[doc = "RTS is set if there is space for at least three more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_threeframes(&self) -> bool {
        *self == Rtsrxfw::Threeframes
    }
    #[doc = "RTS is set if there is space for four more frames in the RX FIFO."]
    #[inline(always)]
    pub fn is_fourframes(&self) -> bool {
        *self == Rtsrxfw::Fourframes
    }
}
#[doc = "Field `RTSRXFW` writer - Request-to-send RX FIFO Watermark"]
pub type RtsrxfwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtsrxfw, crate::Safe>;
impl<'a, REG> RtsrxfwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTS is set if there is space for at least one more frame in the RX FIFO."]
    #[inline(always)]
    pub fn oneframe(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Oneframe)
    }
    #[doc = "RTS is set if there is space for at least two more frames in the RX FIFO."]
    #[inline(always)]
    pub fn twoframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Twoframes)
    }
    #[doc = "RTS is set if there is space for at least three more frames in the RX FIFO."]
    #[inline(always)]
    pub fn threeframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Threeframes)
    }
    #[doc = "RTS is set if there is space for four more frames in the RX FIFO."]
    #[inline(always)]
    pub fn fourframes(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsrxfw::Fourframes)
    }
}
impl R {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DbghaltR {
        DbghaltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear-to-send Invert Enable"]
    #[inline(always)]
    pub fn ctsinv(&self) -> CtsinvR {
        CtsinvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear-to-send Enable"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Request-to-send Invert Enable"]
    #[inline(always)]
    pub fn rtsinv(&self) -> RtsinvR {
        RtsinvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&self) -> TxdmawuR {
        TxdmawuR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receiver DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&self) -> RxdmawuR {
        RxdmawuR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Start Frame Unblock Receiver"]
    #[inline(always)]
    pub fn sfubrx(&self) -> SfubrxR {
        SfubrxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprsen(&self) -> RxprsenR {
        RxprsenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - TX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn txfiw(&self) -> TxfiwR {
        TxfiwR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 19:20 - RX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn rxfiw(&self) -> RxfiwR {
        RxfiwR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Request-to-send RX FIFO Watermark"]
    #[inline(always)]
    pub fn rtsrxfw(&self) -> RtsrxfwR {
        RtsrxfwR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DbghaltW<'_, Cfg1Spec> {
        DbghaltW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear-to-send Invert Enable"]
    #[inline(always)]
    pub fn ctsinv(&mut self) -> CtsinvW<'_, Cfg1Spec> {
        CtsinvW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear-to-send Enable"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<'_, Cfg1Spec> {
        CtsenW::new(self, 2)
    }
    #[doc = "Bit 3 - Request-to-send Invert Enable"]
    #[inline(always)]
    pub fn rtsinv(&mut self) -> RtsinvW<'_, Cfg1Spec> {
        RtsinvW::new(self, 3)
    }
    #[doc = "Bit 9 - Transmitter DMA Wakeup"]
    #[inline(always)]
    pub fn txdmawu(&mut self) -> TxdmawuW<'_, Cfg1Spec> {
        TxdmawuW::new(self, 9)
    }
    #[doc = "Bit 10 - Receiver DMA Wakeup"]
    #[inline(always)]
    pub fn rxdmawu(&mut self) -> RxdmawuW<'_, Cfg1Spec> {
        RxdmawuW::new(self, 10)
    }
    #[doc = "Bit 11 - Start Frame Unblock Receiver"]
    #[inline(always)]
    pub fn sfubrx(&mut self) -> SfubrxW<'_, Cfg1Spec> {
        SfubrxW::new(self, 11)
    }
    #[doc = "Bit 15 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprsen(&mut self) -> RxprsenW<'_, Cfg1Spec> {
        RxprsenW::new(self, 15)
    }
    #[doc = "Bits 16:17 - TX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn txfiw(&mut self) -> TxfiwW<'_, Cfg1Spec> {
        TxfiwW::new(self, 16)
    }
    #[doc = "Bits 19:20 - RX FIFO Interrupt Watermark"]
    #[inline(always)]
    pub fn rxfiw(&mut self) -> RxfiwW<'_, Cfg1Spec> {
        RxfiwW::new(self, 19)
    }
    #[doc = "Bits 22:23 - Request-to-send RX FIFO Watermark"]
    #[inline(always)]
    pub fn rtsrxfw(&mut self) -> RtsrxfwW<'_, Cfg1Spec> {
        RtsrxfwW::new(self, 22)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {}
