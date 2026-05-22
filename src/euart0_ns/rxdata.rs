#[doc = "Register `RXDATA` reader"]
pub type R = crate::R<RxdataSpec>;
#[doc = "Field `RXDATA` reader - RX Data"]
pub type RxdataR = crate::FieldReader<u16>;
#[doc = "Field `PERR` reader - Parity Error"]
pub type PerrR = crate::BitReader;
#[doc = "Field `FERR` reader - Framing Error"]
pub type FerrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - RX Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Framing Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdataSpec;
impl crate::RegisterSpec for RxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RxdataSpec {}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RxdataSpec {}
