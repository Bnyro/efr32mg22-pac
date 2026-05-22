#[doc = "Register `USART1_CTSROUTE` reader"]
pub type R = crate::R<Usart1CtsrouteSpec>;
#[doc = "Register `USART1_CTSROUTE` writer"]
pub type W = crate::W<Usart1CtsrouteSpec>;
#[doc = "Field `PORT` reader - CTS port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - CTS port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - CTS pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - CTS pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CTS port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CTS pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CTS port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, Usart1CtsrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - CTS pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<'_, Usart1CtsrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "CTS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`usart1_ctsroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart1_ctsroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usart1CtsrouteSpec;
impl crate::RegisterSpec for Usart1CtsrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart1_ctsroute::R`](R) reader structure"]
impl crate::Readable for Usart1CtsrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`usart1_ctsroute::W`](W) writer structure"]
impl crate::Writable for Usart1CtsrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USART1_CTSROUTE to value 0"]
impl crate::Resettable for Usart1CtsrouteSpec {}
