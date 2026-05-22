#[doc = "Register `DCDC_ROUTEEN` reader"]
pub type R = crate::R<DcdcRouteenSpec>;
#[doc = "Register `DCDC_ROUTEEN` writer"]
pub type W = crate::W<DcdcRouteenSpec>;
#[doc = "Field `DCDCCOREHIDDENPEN` reader - DCDCCOREHIDDEN pin enable control bit"]
pub type DcdccorehiddenpenR = crate::BitReader;
#[doc = "Field `DCDCCOREHIDDENPEN` writer - DCDCCOREHIDDEN pin enable control bit"]
pub type DcdccorehiddenpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DCDCCOREHIDDEN pin enable control bit"]
    #[inline(always)]
    pub fn dcdccorehiddenpen(&self) -> DcdccorehiddenpenR {
        DcdccorehiddenpenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCDCCOREHIDDEN pin enable control bit"]
    #[inline(always)]
    pub fn dcdccorehiddenpen(&mut self) -> DcdccorehiddenpenW<'_, DcdcRouteenSpec> {
        DcdccorehiddenpenW::new(self, 0)
    }
}
#[doc = "DCDC pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdcRouteenSpec;
impl crate::RegisterSpec for DcdcRouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_routeen::R`](R) reader structure"]
impl crate::Readable for DcdcRouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`dcdc_routeen::W`](W) writer structure"]
impl crate::Writable for DcdcRouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDC_ROUTEEN to value 0"]
impl crate::Resettable for DcdcRouteenSpec {}
