#[doc = "Register `DMEM0ECCADDR` reader"]
pub type R = crate::R<Dmem0eccaddrSpec>;
#[doc = "Field `DMEM0ECCADDR` reader - DMEM0 RAM ECC Error Address"]
pub type Dmem0eccaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DMEM0 RAM ECC Error Address"]
    #[inline(always)]
    pub fn dmem0eccaddr(&self) -> Dmem0eccaddrR {
        Dmem0eccaddrR::new(self.bits)
    }
}
#[doc = "Read to get status of the DMEM0 ECC error address.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmem0eccaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmem0eccaddrSpec;
impl crate::RegisterSpec for Dmem0eccaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmem0eccaddr::R`](R) reader structure"]
impl crate::Readable for Dmem0eccaddrSpec {}
#[doc = "`reset()` method sets DMEM0ECCADDR to value 0"]
impl crate::Resettable for Dmem0eccaddrSpec {}
