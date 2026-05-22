#[doc = "Register `KEY2` reader"]
pub type R = crate::R<Key2Spec>;
#[doc = "Register `KEY2` writer"]
pub type W = crate::W<Key2Spec>;
#[doc = "Field `KEY` reader - Key"]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, Key2Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011...\n\nYou can [`read`](crate::Reg::read) this register and get [`key2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key2Spec;
impl crate::RegisterSpec for Key2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key2::R`](R) reader structure"]
impl crate::Readable for Key2Spec {}
#[doc = "`write(|w| ..)` method takes [`key2::W`](W) writer structure"]
impl crate::Writable for Key2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY2 to value 0"]
impl crate::Resettable for Key2Spec {}
