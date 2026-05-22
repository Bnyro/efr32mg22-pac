#[doc = "Register `FIFOLEVEL` reader"]
pub type R = crate::R<FifolevelSpec>;
#[doc = "Field `FIFOLEVEL` reader - FIFO Level"]
pub type FifolevelR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO Level"]
    #[inline(always)]
    pub fn fifolevel(&self) -> FifolevelR {
        FifolevelR::new(self.bits)
    }
}
#[doc = "Number of 32 bits words of random available in the FIFO. Writing to this register clears the FIFO full interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolevel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifolevelSpec;
impl crate::RegisterSpec for FifolevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifolevel::R`](R) reader structure"]
impl crate::Readable for FifolevelSpec {}
#[doc = "`reset()` method sets FIFOLEVEL to value 0"]
impl crate::Resettable for FifolevelSpec {}
