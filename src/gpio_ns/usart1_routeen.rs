#[doc = "Register `USART1_ROUTEEN` reader"]
pub type R = crate::R<Usart1RouteenSpec>;
#[doc = "Register `USART1_ROUTEEN` writer"]
pub type W = crate::W<Usart1RouteenSpec>;
#[doc = "Field `CSPEN` reader - CS pin enable control bit"]
pub type CspenR = crate::BitReader;
#[doc = "Field `CSPEN` writer - CS pin enable control bit"]
pub type CspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSPEN` reader - RTS pin enable control bit"]
pub type RtspenR = crate::BitReader;
#[doc = "Field `RTSPEN` writer - RTS pin enable control bit"]
pub type RtspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEN` reader - RX pin enable control bit"]
pub type RxpenR = crate::BitReader;
#[doc = "Field `RXPEN` writer - RX pin enable control bit"]
pub type RxpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPEN` reader - SCLK pin enable control bit"]
pub type ClkpenR = crate::BitReader;
#[doc = "Field `CLKPEN` writer - SCLK pin enable control bit"]
pub type ClkpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPEN` reader - TX pin enable control bit"]
pub type TxpenR = crate::BitReader;
#[doc = "Field `TXPEN` writer - TX pin enable control bit"]
pub type TxpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CS pin enable control bit"]
    #[inline(always)]
    pub fn cspen(&self) -> CspenR {
        CspenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTS pin enable control bit"]
    #[inline(always)]
    pub fn rtspen(&self) -> RtspenR {
        RtspenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX pin enable control bit"]
    #[inline(always)]
    pub fn rxpen(&self) -> RxpenR {
        RxpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCLK pin enable control bit"]
    #[inline(always)]
    pub fn clkpen(&self) -> ClkpenR {
        ClkpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX pin enable control bit"]
    #[inline(always)]
    pub fn txpen(&self) -> TxpenR {
        TxpenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CS pin enable control bit"]
    #[inline(always)]
    pub fn cspen(&mut self) -> CspenW<'_, Usart1RouteenSpec> {
        CspenW::new(self, 0)
    }
    #[doc = "Bit 1 - RTS pin enable control bit"]
    #[inline(always)]
    pub fn rtspen(&mut self) -> RtspenW<'_, Usart1RouteenSpec> {
        RtspenW::new(self, 1)
    }
    #[doc = "Bit 2 - RX pin enable control bit"]
    #[inline(always)]
    pub fn rxpen(&mut self) -> RxpenW<'_, Usart1RouteenSpec> {
        RxpenW::new(self, 2)
    }
    #[doc = "Bit 3 - SCLK pin enable control bit"]
    #[inline(always)]
    pub fn clkpen(&mut self) -> ClkpenW<'_, Usart1RouteenSpec> {
        ClkpenW::new(self, 3)
    }
    #[doc = "Bit 4 - TX pin enable control bit"]
    #[inline(always)]
    pub fn txpen(&mut self) -> TxpenW<'_, Usart1RouteenSpec> {
        TxpenW::new(self, 4)
    }
}
#[doc = "USART1 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`usart1_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart1_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usart1RouteenSpec;
impl crate::RegisterSpec for Usart1RouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart1_routeen::R`](R) reader structure"]
impl crate::Readable for Usart1RouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`usart1_routeen::W`](W) writer structure"]
impl crate::Writable for Usart1RouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USART1_ROUTEEN to value 0"]
impl crate::Resettable for Usart1RouteenSpec {}
