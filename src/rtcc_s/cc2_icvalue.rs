#[doc = "Register `CC2_ICVALUE` reader"]
pub type R = crate::R<Cc2IcvalueSpec>;
#[doc = "Field `IC` reader - Input Capture Value"]
pub type IcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Input Capture Value"]
    #[inline(always)]
    pub fn ic(&self) -> IcR {
        IcR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_icvalue::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc2IcvalueSpec;
impl crate::RegisterSpec for Cc2IcvalueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_icvalue::R`](R) reader structure"]
impl crate::Readable for Cc2IcvalueSpec {}
#[doc = "`reset()` method sets CC2_ICVALUE to value 0"]
impl crate::Resettable for Cc2IcvalueSpec {}
