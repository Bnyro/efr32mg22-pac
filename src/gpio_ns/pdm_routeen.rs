#[doc = "Register `PDM_ROUTEEN` reader"]
pub type R = crate::R<PdmRouteenSpec>;
#[doc = "Register `PDM_ROUTEEN` writer"]
pub type W = crate::W<PdmRouteenSpec>;
#[doc = "Field `CLKPEN` reader - CLK pin enable control bit"]
pub type ClkpenR = crate::BitReader;
#[doc = "Field `CLKPEN` writer - CLK pin enable control bit"]
pub type ClkpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CLK pin enable control bit"]
    #[inline(always)]
    pub fn clkpen(&self) -> ClkpenR {
        ClkpenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLK pin enable control bit"]
    #[inline(always)]
    pub fn clkpen(&mut self) -> ClkpenW<'_, PdmRouteenSpec> {
        ClkpenW::new(self, 0)
    }
}
#[doc = "PDM pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pdm_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdm_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdmRouteenSpec;
impl crate::RegisterSpec for PdmRouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdm_routeen::R`](R) reader structure"]
impl crate::Readable for PdmRouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`pdm_routeen::W`](W) writer structure"]
impl crate::Writable for PdmRouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDM_ROUTEEN to value 0"]
impl crate::Resettable for PdmRouteenSpec {}
