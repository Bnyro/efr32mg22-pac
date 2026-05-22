#[doc = "Register `PDM_CLKROUTE` reader"]
pub type R = crate::R<PdmClkrouteSpec>;
#[doc = "Register `PDM_CLKROUTE` writer"]
pub type W = crate::W<PdmClkrouteSpec>;
#[doc = "Field `PORT` reader - CLK port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - CLK port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - CLK pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - CLK pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CLK port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CLK pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLK port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<'_, PdmClkrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - CLK pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<'_, PdmClkrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "CLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`pdm_clkroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdm_clkroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdmClkrouteSpec;
impl crate::RegisterSpec for PdmClkrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdm_clkroute::R`](R) reader structure"]
impl crate::Readable for PdmClkrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`pdm_clkroute::W`](W) writer structure"]
impl crate::Writable for PdmClkrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDM_CLKROUTE to value 0"]
impl crate::Resettable for PdmClkrouteSpec {}
