#[doc = "Register `CRYPTOACCCLKCTRL` reader"]
pub type R = crate::R<CryptoaccclkctrlSpec>;
#[doc = "Register `CRYPTOACCCLKCTRL` writer"]
pub type W = crate::W<CryptoaccclkctrlSpec>;
#[doc = "Field `PKEN` reader - PK Enable"]
pub type PkenR = crate::BitReader;
#[doc = "Field `PKEN` writer - PK Enable"]
pub type PkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESEN` reader - AES Enable"]
pub type AesenR = crate::BitReader;
#[doc = "Field `AESEN` writer - AES Enable"]
pub type AesenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PK Enable"]
    #[inline(always)]
    pub fn pken(&self) -> PkenR {
        PkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES Enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AesenR {
        AesenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PK Enable"]
    #[inline(always)]
    pub fn pken(&mut self) -> PkenW<'_, CryptoaccclkctrlSpec> {
        PkenW::new(self, 0)
    }
    #[doc = "Bit 1 - AES Enable"]
    #[inline(always)]
    pub fn aesen(&mut self) -> AesenW<'_, CryptoaccclkctrlSpec> {
        AesenW::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cryptoaccclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryptoaccclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoaccclkctrlSpec;
impl crate::RegisterSpec for CryptoaccclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryptoaccclkctrl::R`](R) reader structure"]
impl crate::Readable for CryptoaccclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cryptoaccclkctrl::W`](W) writer structure"]
impl crate::Writable for CryptoaccclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRYPTOACCCLKCTRL to value 0"]
impl crate::Resettable for CryptoaccclkctrlSpec {}
