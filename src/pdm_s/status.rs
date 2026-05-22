#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `ACT` reader - PDM is active"]
pub type ActR = crate::BitReader;
#[doc = "Field `FULL` reader - FIFO FULL Status"]
pub type FullR = crate::BitReader;
#[doc = "Field `EMPTY` reader - FIFO EMPTY Status"]
pub type EmptyR = crate::BitReader;
#[doc = "Field `FIFOCNT` reader - FIFO CNT"]
pub type FifocntR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - PDM is active"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO FULL Status"]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO EMPTY Status"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:10 - FIFO CNT"]
    #[inline(always)]
    pub fn fifocnt(&self) -> FifocntR {
        FifocntR::new(((self.bits >> 8) & 7) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x20"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x20;
}
