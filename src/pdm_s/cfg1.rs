#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `PRESC` reader - Prescalar Setting for PDM sample"]
pub type PrescR = crate::FieldReader<u16>;
#[doc = "Field `PRESC` writer - Prescalar Setting for PDM sample"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DLYMUXSEL` reader - Data delay buffer mux selection"]
pub type DlymuxselR = crate::FieldReader;
#[doc = "Field `DLYMUXSEL` writer - Data delay buffer mux selection"]
pub type DlymuxselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - Prescalar Setting for PDM sample"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 24:25 - Data delay buffer mux selection"]
    #[inline(always)]
    pub fn dlymuxsel(&self) -> DlymuxselR {
        DlymuxselR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Prescalar Setting for PDM sample"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<'_, Cfg1Spec> {
        PrescW::new(self, 0)
    }
    #[doc = "Bits 24:25 - Data delay buffer mux selection"]
    #[inline(always)]
    pub fn dlymuxsel(&mut self) -> DlymuxselW<'_, Cfg1Spec> {
        DlymuxselW::new(self, 24)
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
