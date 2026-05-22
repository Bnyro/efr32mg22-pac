#[doc = "Register `CONSUMER_RAC_SEQ` reader"]
pub type R = crate::R<ConsumerRacSeqSpec>;
#[doc = "Register `CONSUMER_RAC_SEQ` writer"]
pub type W = crate::W<ConsumerRacSeqSpec>;
#[doc = "Field `PRSSEL` reader - SEQ async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - SEQ async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SEQ async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SEQ async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, ConsumerRacSeqSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "SEQ Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_rac_seq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_rac_seq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerRacSeqSpec;
impl crate::RegisterSpec for ConsumerRacSeqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_rac_seq::R`](R) reader structure"]
impl crate::Readable for ConsumerRacSeqSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_rac_seq::W`](W) writer structure"]
impl crate::Writable for ConsumerRacSeqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_RAC_SEQ to value 0"]
impl crate::Resettable for ConsumerRacSeqSpec {}
