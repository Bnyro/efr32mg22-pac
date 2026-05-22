#[doc = "Register `KEY0` reader"]
pub type R = crate::R<Key0Spec>;
#[doc = "Register `KEY0` writer"]
pub type W = crate::W<Key0Spec>;
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
    pub fn key(&mut self) -> KeyW<'_, Key0Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "This set of registers bits form the 128-bit AES key used for conditioning function. The first byte (MSB of 128-bit word) is at address 0x0010, the second byte at address 0x0011...\n\nYou can [`read`](crate::Reg::read) this register and get [`key0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Key0Spec;
impl crate::RegisterSpec for Key0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key0::R`](R) reader structure"]
impl crate::Readable for Key0Spec {}
#[doc = "`write(|w| ..)` method takes [`key0::W`](W) writer structure"]
impl crate::Writable for Key0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY0 to value 0"]
impl crate::Resettable for Key0Spec {}
