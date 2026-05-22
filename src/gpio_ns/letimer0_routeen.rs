#[doc = "Register `LETIMER0_ROUTEEN` reader"]
pub type R = crate::R<Letimer0RouteenSpec>;
#[doc = "Register `LETIMER0_ROUTEEN` writer"]
pub type W = crate::W<Letimer0RouteenSpec>;
#[doc = "Field `OUT0PEN` reader - OUT0 pin enable control bit"]
pub type Out0penR = crate::BitReader;
#[doc = "Field `OUT0PEN` writer - OUT0 pin enable control bit"]
pub type Out0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT1PEN` reader - OUT1 pin enable control bit"]
pub type Out1penR = crate::BitReader;
#[doc = "Field `OUT1PEN` writer - OUT1 pin enable control bit"]
pub type Out1penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OUT0 pin enable control bit"]
    #[inline(always)]
    pub fn out0pen(&self) -> Out0penR {
        Out0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUT1 pin enable control bit"]
    #[inline(always)]
    pub fn out1pen(&self) -> Out1penR {
        Out1penR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OUT0 pin enable control bit"]
    #[inline(always)]
    pub fn out0pen(&mut self) -> Out0penW<'_, Letimer0RouteenSpec> {
        Out0penW::new(self, 0)
    }
    #[doc = "Bit 1 - OUT1 pin enable control bit"]
    #[inline(always)]
    pub fn out1pen(&mut self) -> Out1penW<'_, Letimer0RouteenSpec> {
        Out1penW::new(self, 1)
    }
}
#[doc = "LETIMER pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`letimer0_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`letimer0_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Letimer0RouteenSpec;
impl crate::RegisterSpec for Letimer0RouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`letimer0_routeen::R`](R) reader structure"]
impl crate::Readable for Letimer0RouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`letimer0_routeen::W`](W) writer structure"]
impl crate::Writable for Letimer0RouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LETIMER0_ROUTEEN to value 0"]
impl crate::Resettable for Letimer0RouteenSpec {}
