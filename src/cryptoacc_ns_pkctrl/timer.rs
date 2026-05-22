#[doc = "Register `TIMER` reader"]
pub type R = crate::R<TimerSpec>;
#[doc = "Field `TIMER` reader - Timer"]
pub type TimerR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timer::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerSpec;
impl crate::RegisterSpec for TimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer::R`](R) reader structure"]
impl crate::Readable for TimerSpec {}
#[doc = "`reset()` method sets TIMER to value 0"]
impl crate::Resettable for TimerSpec {}
