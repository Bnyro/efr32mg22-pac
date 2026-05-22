#[doc = "Register `TESTDATA` writer"]
pub type W = crate::W<TestdataSpec>;
#[doc = "Field `VALUE` writer - Test data input to conditioning tests"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Test data input to conditioning tests"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, TestdataSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "This register is used to feed known data to the conditioning function or to the continuous tests. See manual\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestdataSpec;
impl crate::RegisterSpec for TestdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`testdata::W`](W) writer structure"]
impl crate::Writable for TestdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TESTDATA to value 0"]
impl crate::Resettable for TestdataSpec {}
