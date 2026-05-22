#[doc = "Register `USART1_CLKROUTE` reader"]
pub type R = crate::R<Usart1ClkrouteSpec>;
#[doc = "Register `USART1_CLKROUTE` writer"]
pub type W = crate::W<Usart1ClkrouteSpec>;
#[doc = "Field `PORT` reader - SCLK port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - SCLK port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - SCLK pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - SCLK pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - SCLK port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - SCLK pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SCLK port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, Usart1ClkrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - SCLK pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<'_, Usart1ClkrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`usart1_clkroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart1_clkroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usart1ClkrouteSpec;
impl crate::RegisterSpec for Usart1ClkrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart1_clkroute::R`](R) reader structure"]
impl crate::Readable for Usart1ClkrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`usart1_clkroute::W`](W) writer structure"]
impl crate::Writable for Usart1ClkrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USART1_CLKROUTE to value 0"]
impl crate::Resettable for Usart1ClkrouteSpec {}
