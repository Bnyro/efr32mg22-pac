#[doc = "Register `CC0_OCVALUE` reader"]
pub type R = crate::R<Cc0OcvalueSpec>;
#[doc = "Register `CC0_OCVALUE` writer"]
pub type W = crate::W<Cc0OcvalueSpec>;
#[doc = "Field `OC` reader - Output Compare Value"]
pub type OcR = crate::FieldReader<u32>;
#[doc = "Field `OC` writer - Output Compare Value"]
pub type OcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Compare Value"]
    #[inline(always)]
    pub fn oc(&self) -> OcR {
        OcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output Compare Value"]
    #[inline(always)]
    pub fn oc(&mut self) -> OcW<'_, Cc0OcvalueSpec> {
        OcW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ocvalue::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ocvalue::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc0OcvalueSpec;
impl crate::RegisterSpec for Cc0OcvalueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_ocvalue::R`](R) reader structure"]
impl crate::Readable for Cc0OcvalueSpec {}
#[doc = "`write(|w| ..)` method takes [`cc0_ocvalue::W`](W) writer structure"]
impl crate::Writable for Cc0OcvalueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC0_OCVALUE to value 0"]
impl crate::Resettable for Cc0OcvalueSpec {}
