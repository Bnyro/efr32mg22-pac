#[doc = "Register `BMPUNSPATD0` reader"]
pub type R = crate::R<Bmpunspatd0Spec>;
#[doc = "Register `BMPUNSPATD0` writer"]
pub type W = crate::W<Bmpunspatd0Spec>;
#[doc = "Field `RADIOAES` reader - RADIO AES DMA privileged mode"]
pub type RadioaesR = crate::BitReader;
#[doc = "Field `RADIOAES` writer - RADIO AES DMA privileged mode"]
pub type RadioaesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTOACC` reader - CRYPTOACC DMA privileged mode"]
pub type CryptoaccR = crate::BitReader;
#[doc = "Field `CRYPTOACC` writer - CRYPTOACC DMA privileged mode"]
pub type CryptoaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RADIOSUBSYSTEM` reader - RADIO subsystem manager privileged mode"]
pub type RadiosubsystemR = crate::BitReader;
#[doc = "Field `RADIOSUBSYSTEM` writer - RADIO subsystem manager privileged mode"]
pub type RadiosubsystemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RADIOIFADCDEBUG` reader - RADIO IFADC debug privileged mode"]
pub type RadioifadcdebugR = crate::BitReader;
#[doc = "Field `RADIOIFADCDEBUG` writer - RADIO IFADC debug privileged mode"]
pub type RadioifadcdebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA` reader - MCU LDMA privileged mode"]
pub type LdmaR = crate::BitReader;
#[doc = "Field `LDMA` writer - MCU LDMA privileged mode"]
pub type LdmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RADIO AES DMA privileged mode"]
    #[inline(always)]
    pub fn radioaes(&self) -> RadioaesR {
        RadioaesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRYPTOACC DMA privileged mode"]
    #[inline(always)]
    pub fn cryptoacc(&self) -> CryptoaccR {
        CryptoaccR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RADIO subsystem manager privileged mode"]
    #[inline(always)]
    pub fn radiosubsystem(&self) -> RadiosubsystemR {
        RadiosubsystemR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RADIO IFADC debug privileged mode"]
    #[inline(always)]
    pub fn radioifadcdebug(&self) -> RadioifadcdebugR {
        RadioifadcdebugR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MCU LDMA privileged mode"]
    #[inline(always)]
    pub fn ldma(&self) -> LdmaR {
        LdmaR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RADIO AES DMA privileged mode"]
    #[inline(always)]
    pub fn radioaes(&mut self) -> RadioaesW<'_, Bmpunspatd0Spec> {
        RadioaesW::new(self, 0)
    }
    #[doc = "Bit 1 - CRYPTOACC DMA privileged mode"]
    #[inline(always)]
    pub fn cryptoacc(&mut self) -> CryptoaccW<'_, Bmpunspatd0Spec> {
        CryptoaccW::new(self, 1)
    }
    #[doc = "Bit 2 - RADIO subsystem manager privileged mode"]
    #[inline(always)]
    pub fn radiosubsystem(&mut self) -> RadiosubsystemW<'_, Bmpunspatd0Spec> {
        RadiosubsystemW::new(self, 2)
    }
    #[doc = "Bit 3 - RADIO IFADC debug privileged mode"]
    #[inline(always)]
    pub fn radioifadcdebug(&mut self) -> RadioifadcdebugW<'_, Bmpunspatd0Spec> {
        RadioifadcdebugW::new(self, 3)
    }
    #[doc = "Bit 4 - MCU LDMA privileged mode"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LdmaW<'_, Bmpunspatd0Spec> {
        LdmaW::new(self, 4)
    }
}
#[doc = "Write to set BMPU priveledged attributes.\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpunspatd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmpunspatd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bmpunspatd0Spec;
impl crate::RegisterSpec for Bmpunspatd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmpunspatd0::R`](R) reader structure"]
impl crate::Readable for Bmpunspatd0Spec {}
#[doc = "`write(|w| ..)` method takes [`bmpunspatd0::W`](W) writer structure"]
impl crate::Writable for Bmpunspatd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMPUNSPATD0 to value 0"]
impl crate::Resettable for Bmpunspatd0Spec {}
