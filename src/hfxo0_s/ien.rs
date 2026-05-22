#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `RDY` reader - Ready Interrupt"]
pub type RdyR = crate::BitReader;
#[doc = "Field `RDY` writer - Ready Interrupt"]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREBIASOPTRDY` reader - Core Bias Optimization Ready Interrupt"]
pub type CorebiasoptrdyR = crate::BitReader;
#[doc = "Field `COREBIASOPTRDY` writer - Core Bias Optimization Ready Interrupt"]
pub type CorebiasoptrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNSERR` reader - Did Not Start Error Interrupt"]
pub type DnserrR = crate::BitReader;
#[doc = "Field `DNSERR` writer - Did Not Start Error Interrupt"]
pub type DnserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREBIASOPTERR` reader - Core Bias Optimization Error Interrupt"]
pub type CorebiasopterrR = crate::BitReader;
#[doc = "Field `COREBIASOPTERR` writer - Core Bias Optimization Error Interrupt"]
pub type CorebiasopterrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ready Interrupt"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core Bias Optimization Ready Interrupt"]
    #[inline(always)]
    pub fn corebiasoptrdy(&self) -> CorebiasoptrdyR {
        CorebiasoptrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 29 - Did Not Start Error Interrupt"]
    #[inline(always)]
    pub fn dnserr(&self) -> DnserrR {
        DnserrR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Core Bias Optimization Error Interrupt"]
    #[inline(always)]
    pub fn corebiasopterr(&self) -> CorebiasopterrR {
        CorebiasopterrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ready Interrupt"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<'_, IenSpec> {
        RdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Core Bias Optimization Ready Interrupt"]
    #[inline(always)]
    pub fn corebiasoptrdy(&mut self) -> CorebiasoptrdyW<'_, IenSpec> {
        CorebiasoptrdyW::new(self, 1)
    }
    #[doc = "Bit 29 - Did Not Start Error Interrupt"]
    #[inline(always)]
    pub fn dnserr(&mut self) -> DnserrW<'_, IenSpec> {
        DnserrW::new(self, 29)
    }
    #[doc = "Bit 31 - Core Bias Optimization Error Interrupt"]
    #[inline(always)]
    pub fn corebiasopterr(&mut self) -> CorebiasopterrW<'_, IenSpec> {
        CorebiasopterrW::new(self, 31)
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
