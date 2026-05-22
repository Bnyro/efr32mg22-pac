#[doc = "Register `CC1_ICVALUE` reader"]
pub type R = crate::R<Cc1IcvalueSpec>;
#[doc = "Field `IC` reader - Input Capture Value"]
pub type IcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Input Capture Value"]
    #[inline(always)]
    pub fn ic(&self) -> IcR {
        IcR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_icvalue::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc1IcvalueSpec;
impl crate::RegisterSpec for Cc1IcvalueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_icvalue::R`](R) reader structure"]
impl crate::Readable for Cc1IcvalueSpec {}
#[doc = "`reset()` method sets CC1_ICVALUE to value 0"]
impl crate::Resettable for Cc1IcvalueSpec {}
