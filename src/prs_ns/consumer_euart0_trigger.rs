#[doc = "Register `CONSUMER_EUART0_TRIGGER` reader"]
pub type R = crate::R<ConsumerEuart0TriggerSpec>;
#[doc = "Register `CONSUMER_EUART0_TRIGGER` writer"]
pub type W = crate::W<ConsumerEuart0TriggerSpec>;
#[doc = "Field `PRSSEL` reader - TRIGGER async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - TRIGGER async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TRIGGER async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TRIGGER async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, ConsumerEuart0TriggerSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "TRIGGER Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_euart0_trigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_euart0_trigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerEuart0TriggerSpec;
impl crate::RegisterSpec for ConsumerEuart0TriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_euart0_trigger::R`](R) reader structure"]
impl crate::Readable for ConsumerEuart0TriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_euart0_trigger::W`](W) writer structure"]
impl crate::Writable for ConsumerEuart0TriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_EUART0_TRIGGER to value 0"]
impl crate::Resettable for ConsumerEuart0TriggerSpec {}
