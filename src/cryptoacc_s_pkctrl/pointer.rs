#[doc = "Register `POINTER` reader"]
pub type R = crate::R<PointerSpec>;
#[doc = "Register `POINTER` writer"]
pub type W = crate::W<PointerSpec>;
#[doc = "Field `OPPTRA` reader - OpPtrA"]
pub type OpptraR = crate::FieldReader;
#[doc = "Field `OPPTRA` writer - OpPtrA"]
pub type OpptraW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OPPTRB` reader - OpPtrB"]
pub type OpptrbR = crate::FieldReader;
#[doc = "Field `OPPTRB` writer - OpPtrB"]
pub type OpptrbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OPPTRC` reader - OpPtrC"]
pub type OpptrcR = crate::FieldReader;
#[doc = "Field `OPPTRC` writer - OpPtrC"]
pub type OpptrcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OPPTRN` reader - OpPtrN"]
pub type OpptrnR = crate::FieldReader;
#[doc = "Field `OPPTRN` writer - OpPtrN"]
pub type OpptrnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - OpPtrA"]
    #[inline(always)]
    pub fn opptra(&self) -> OpptraR {
        OpptraR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - OpPtrB"]
    #[inline(always)]
    pub fn opptrb(&self) -> OpptrbR {
        OpptrbR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - OpPtrC"]
    #[inline(always)]
    pub fn opptrc(&self) -> OpptrcR {
        OpptrcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - OpPtrN"]
    #[inline(always)]
    pub fn opptrn(&self) -> OpptrnR {
        OpptrnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - OpPtrA"]
    #[inline(always)]
    pub fn opptra(&mut self) -> OpptraW<'_, PointerSpec> {
        OpptraW::new(self, 0)
    }
    #[doc = "Bits 8:11 - OpPtrB"]
    #[inline(always)]
    pub fn opptrb(&mut self) -> OpptrbW<'_, PointerSpec> {
        OpptrbW::new(self, 8)
    }
    #[doc = "Bits 16:19 - OpPtrC"]
    #[inline(always)]
    pub fn opptrc(&mut self) -> OpptrcW<'_, PointerSpec> {
        OpptrcW::new(self, 16)
    }
    #[doc = "Bits 24:27 - OpPtrN"]
    #[inline(always)]
    pub fn opptrn(&mut self) -> OpptrnW<'_, PointerSpec> {
        OpptrnW::new(self, 24)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pointer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pointer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PointerSpec;
impl crate::RegisterSpec for PointerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pointer::R`](R) reader structure"]
impl crate::Readable for PointerSpec {}
#[doc = "`write(|w| ..)` method takes [`pointer::W`](W) writer structure"]
impl crate::Writable for PointerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POINTER to value 0"]
impl crate::Resettable for PointerSpec {}
