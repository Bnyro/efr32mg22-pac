#[doc = "Register `AIS31CONF0` reader"]
pub type R = crate::R<Ais31conf0Spec>;
#[doc = "Register `AIS31CONF0` writer"]
pub type W = crate::W<Ais31conf0Spec>;
#[doc = "Field `STARTUPTHRES` reader - Start-up Threshold"]
pub type StartupthresR = crate::FieldReader<u16>;
#[doc = "Field `STARTUPTHRES` writer - Start-up Threshold"]
pub type StartupthresW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `ONLINETHRESH` reader - Online Threshold"]
pub type OnlinethreshR = crate::FieldReader<u16>;
#[doc = "Field `ONLINETHRESH` writer - Online Threshold"]
pub type OnlinethreshW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Start-up Threshold"]
    #[inline(always)]
    pub fn startupthres(&self) -> StartupthresR {
        StartupthresR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Online Threshold"]
    #[inline(always)]
    pub fn onlinethresh(&self) -> OnlinethreshR {
        OnlinethreshR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Start-up Threshold"]
    #[inline(always)]
    pub fn startupthres(&mut self) -> StartupthresW<'_, Ais31conf0Spec> {
        StartupthresW::new(self, 0)
    }
    #[doc = "Bits 16:30 - Online Threshold"]
    #[inline(always)]
    pub fn onlinethresh(&mut self) -> OnlinethreshW<'_, Ais31conf0Spec> {
        OnlinethreshW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ais31conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ais31conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ais31conf0Spec;
impl crate::RegisterSpec for Ais31conf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ais31conf0::R`](R) reader structure"]
impl crate::Readable for Ais31conf0Spec {}
#[doc = "`write(|w| ..)` method takes [`ais31conf0::W`](W) writer structure"]
impl crate::Writable for Ais31conf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AIS31CONF0 to value 0x4340_1040"]
impl crate::Resettable for Ais31conf0Spec {
    const RESET_VALUE: u32 = 0x4340_1040;
}
