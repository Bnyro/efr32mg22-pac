#[doc = "Register `CONSUMER_EUART0_RX` reader"]
pub type R = crate::R<ConsumerEuart0RxSpec>;
#[doc = "Register `CONSUMER_EUART0_RX` writer"]
pub type W = crate::W<ConsumerEuart0RxSpec>;
#[doc = "Field `PRSSEL` reader - RX async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - RX async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RX async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RX async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<'_, ConsumerEuart0RxSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "RX Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_euart0_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_euart0_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerEuart0RxSpec;
impl crate::RegisterSpec for ConsumerEuart0RxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_euart0_rx::R`](R) reader structure"]
impl crate::Readable for ConsumerEuart0RxSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_euart0_rx::W`](W) writer structure"]
impl crate::Writable for ConsumerEuart0RxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_EUART0_RX to value 0"]
impl crate::Resettable for ConsumerEuart0RxSpec {}
