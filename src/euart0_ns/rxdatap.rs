#[doc = "Register `RXDATAP` reader"]
pub type R = crate::R<RxdatapSpec>;
#[doc = "Field `RXDATAP` reader - RX Data Peek"]
pub type RxdatapR = crate::FieldReader<u16>;
#[doc = "Field `PERRP` reader - Parity Error Peek"]
pub type PerrpR = crate::BitReader;
#[doc = "Field `FERRP` reader - Framing Error Peek"]
pub type FerrpR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - RX Data Peek"]
    #[inline(always)]
    pub fn rxdatap(&self) -> RxdatapR {
        RxdatapR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Parity Error Peek"]
    #[inline(always)]
    pub fn perrp(&self) -> PerrpR {
        PerrpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Framing Error Peek"]
    #[inline(always)]
    pub fn ferrp(&self) -> FerrpR {
        FerrpR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdatap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdatapSpec;
impl crate::RegisterSpec for RxdatapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdatap::R`](R) reader structure"]
impl crate::Readable for RxdatapSpec {}
#[doc = "`reset()` method sets RXDATAP to value 0"]
impl crate::Resettable for RxdatapSpec {}
