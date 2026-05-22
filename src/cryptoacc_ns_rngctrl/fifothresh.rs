#[doc = "Register `FIFOTHRESH` reader"]
pub type R = crate::R<FifothreshSpec>;
#[doc = "Field `FIFOTHRESH` reader - FIFO threshold level"]
pub type FifothreshR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO threshold level"]
    #[inline(always)]
    pub fn fifothresh(&self) -> FifothreshR {
        FifothreshR::new(self.bits)
    }
}
#[doc = "FIFO level at which the rings are restarted when in the FIFOFull_Off state, expressed in number of 128bit blocks\n\nYou can [`read`](crate::Reg::read) this register and get [`fifothresh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifothreshSpec;
impl crate::RegisterSpec for FifothreshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifothresh::R`](R) reader structure"]
impl crate::Readable for FifothreshSpec {}
#[doc = "`reset()` method sets FIFOTHRESH to value 0x3f"]
impl crate::Resettable for FifothreshSpec {
    const RESET_VALUE: u32 = 0x3f;
}
