#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `TXC` reader - TX Complete IEN"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXC` writer - TX Complete IEN"]
pub type TxcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFL` reader - TX FIFO Level IEN"]
pub type TxflR = crate::BitReader;
#[doc = "Field `TXFL` writer - TX FIFO Level IEN"]
pub type TxflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFL` reader - RX FIFO Level IEN"]
pub type RxflR = crate::BitReader;
#[doc = "Field `RXFL` writer - RX FIFO Level IEN"]
pub type RxflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - RX FIFO Full IEN"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - RX FIFO Full IEN"]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOF` reader - RX FIFO Overflow IEN"]
pub type RxofR = crate::BitReader;
#[doc = "Field `RXOF` writer - RX FIFO Overflow IEN"]
pub type RxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUF` reader - RX FIFO Underflow IEN"]
pub type RxufR = crate::BitReader;
#[doc = "Field `RXUF` writer - RX FIFO Underflow IEN"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOF` reader - TX FIFO Overflow IEN"]
pub type TxofR = crate::BitReader;
#[doc = "Field `TXOF` writer - TX FIFO Overflow IEN"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Parity Error IEN"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Parity Error IEN"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FERR` reader - Framing Error IEN"]
pub type FerrR = crate::BitReader;
#[doc = "Field `FERR` writer - Framing Error IEN"]
pub type FerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPAF` reader - Multi-Processor Addr Frame IEN"]
pub type MpafR = crate::BitReader;
#[doc = "Field `MPAF` writer - Multi-Processor Addr Frame IEN"]
pub type MpafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCF` reader - Collision Check Fail IEN"]
pub type CcfR = crate::BitReader;
#[doc = "Field `CCF` writer - Collision Check Fail IEN"]
pub type CcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIDLE` reader - TX IDLE IEN"]
pub type TxidleR = crate::BitReader;
#[doc = "Field `TXIDLE` writer - TX IDLE IEN"]
pub type TxidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTF` reader - Start Frame IEN"]
pub type StartfR = crate::BitReader;
#[doc = "Field `STARTF` writer - Start Frame IEN"]
pub type StartfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGF` reader - Signal Frame IEN"]
pub type SigfR = crate::BitReader;
#[doc = "Field `SIGF` writer - Signal Frame IEN"]
pub type SigfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOBAUDDONE` reader - Auto Baud Complete IEN"]
pub type AutobauddoneR = crate::BitReader;
#[doc = "Field `AUTOBAUDDONE` writer - Auto Baud Complete IEN"]
pub type AutobauddoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX Complete IEN"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Level IEN"]
    #[inline(always)]
    pub fn txfl(&self) -> TxflR {
        TxflR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Level IEN"]
    #[inline(always)]
    pub fn rxfl(&self) -> RxflR {
        RxflR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Full IEN"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Overflow IEN"]
    #[inline(always)]
    pub fn rxof(&self) -> RxofR {
        RxofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO Underflow IEN"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO Overflow IEN"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error IEN"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Framing Error IEN"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Addr Frame IEN"]
    #[inline(always)]
    pub fn mpaf(&self) -> MpafR {
        MpafR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Collision Check Fail IEN"]
    #[inline(always)]
    pub fn ccf(&self) -> CcfR {
        CcfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX IDLE IEN"]
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 18 - Start Frame IEN"]
    #[inline(always)]
    pub fn startf(&self) -> StartfR {
        StartfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Signal Frame IEN"]
    #[inline(always)]
    pub fn sigf(&self) -> SigfR {
        SigfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto Baud Complete IEN"]
    #[inline(always)]
    pub fn autobauddone(&self) -> AutobauddoneR {
        AutobauddoneR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Complete IEN"]
    #[inline(always)]
    pub fn txc(&mut self) -> TxcW<'_, IenSpec> {
        TxcW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO Level IEN"]
    #[inline(always)]
    pub fn txfl(&mut self) -> TxflW<'_, IenSpec> {
        TxflW::new(self, 1)
    }
    #[doc = "Bit 2 - RX FIFO Level IEN"]
    #[inline(always)]
    pub fn rxfl(&mut self) -> RxflW<'_, IenSpec> {
        RxflW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO Full IEN"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RxfullW<'_, IenSpec> {
        RxfullW::new(self, 3)
    }
    #[doc = "Bit 4 - RX FIFO Overflow IEN"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RxofW<'_, IenSpec> {
        RxofW::new(self, 4)
    }
    #[doc = "Bit 5 - RX FIFO Underflow IEN"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, IenSpec> {
        RxufW::new(self, 5)
    }
    #[doc = "Bit 6 - TX FIFO Overflow IEN"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, IenSpec> {
        TxofW::new(self, 6)
    }
    #[doc = "Bit 8 - Parity Error IEN"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, IenSpec> {
        PerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Framing Error IEN"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FerrW<'_, IenSpec> {
        FerrW::new(self, 9)
    }
    #[doc = "Bit 10 - Multi-Processor Addr Frame IEN"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MpafW<'_, IenSpec> {
        MpafW::new(self, 10)
    }
    #[doc = "Bit 12 - Collision Check Fail IEN"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CcfW<'_, IenSpec> {
        CcfW::new(self, 12)
    }
    #[doc = "Bit 13 - TX IDLE IEN"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TxidleW<'_, IenSpec> {
        TxidleW::new(self, 13)
    }
    #[doc = "Bit 18 - Start Frame IEN"]
    #[inline(always)]
    pub fn startf(&mut self) -> StartfW<'_, IenSpec> {
        StartfW::new(self, 18)
    }
    #[doc = "Bit 19 - Signal Frame IEN"]
    #[inline(always)]
    pub fn sigf(&mut self) -> SigfW<'_, IenSpec> {
        SigfW::new(self, 19)
    }
    #[doc = "Bit 24 - Auto Baud Complete IEN"]
    #[inline(always)]
    pub fn autobauddone(&mut self) -> AutobauddoneW<'_, IenSpec> {
        AutobauddoneW::new(self, 24)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
