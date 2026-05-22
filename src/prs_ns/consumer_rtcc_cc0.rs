#[doc = "Register `CONSUMER_RTCC_CC0` reader"]
pub type R = crate::R<ConsumerRtccCc0Spec>;
#[doc = "Register `CONSUMER_RTCC_CC0` writer"]
pub type W = crate::W<ConsumerRtccCc0Spec>;
#[doc = "Field `PRSSEL` reader - CC0 async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - CC0 async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CC0 async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CC0 async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, ConsumerRtccCc0Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "CC0 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rtcc_cc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rtcc_cc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerRtccCc0Spec;
impl crate::RegisterSpec for ConsumerRtccCc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_rtcc_cc0::R`](R) reader structure"]
impl crate::Readable for ConsumerRtccCc0Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_rtcc_cc0::W`](W) writer structure"]
impl crate::Writable for ConsumerRtccCc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_RTCC_CC0 to value 0"]
impl crate::Resettable for ConsumerRtccCc0Spec {}
