#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<ClkdivSpec>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<ClkdivSpec>;
#[doc = "Field `VALUE` reader - Sample clock divider"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Sample clock divider"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sample clock divider"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sample clock divider"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, ClkdivSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Sample clock divider. The frequency at which the outputs of the rings are sampled is given by Fs = Fpclk/(ClkDiv + 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkdivSpec;
impl crate::RegisterSpec for ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for ClkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for ClkdivSpec {}
