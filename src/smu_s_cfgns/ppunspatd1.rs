#[doc = "Register `PPUNSPATD1` reader"]
pub type R = crate::R<Ppunspatd1Spec>;
#[doc = "Register `PPUNSPATD1` writer"]
pub type W = crate::W<Ppunspatd1Spec>;
#[doc = "Field `DCDC` reader - DCDC Privileged Access"]
pub type DcdcR = crate::BitReader;
#[doc = "Field `DCDC` writer - DCDC Privileged Access"]
pub type DcdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDM` reader - PDM Privileged Access"]
pub type PdmR = crate::BitReader;
#[doc = "Field `PDM` writer - PDM Privileged Access"]
pub type PdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFSENSE` reader - RFSENSE Privileged Access"]
pub type RfsenseR = crate::BitReader;
#[doc = "Field `RFSENSE` writer - RFSENSE Privileged Access"]
pub type RfsenseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RADIOAES` reader - RADIOAES Privileged Access"]
pub type RadioaesR = crate::BitReader;
#[doc = "Field `RADIOAES` writer - RADIOAES Privileged Access"]
pub type RadioaesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMU` reader - SMU Privileged Access"]
pub type SmuR = crate::BitReader;
#[doc = "Field `SMU` writer - SMU Privileged Access"]
pub type SmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMUCFGNS` reader - SMUCFGNS Privileged Access"]
pub type SmucfgnsR = crate::BitReader;
#[doc = "Field `SMUCFGNS` writer - SMUCFGNS Privileged Access"]
pub type SmucfgnsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCC` reader - RTCC Privileged Access"]
pub type RtccR = crate::BitReader;
#[doc = "Field `RTCC` writer - RTCC Privileged Access"]
pub type RtccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER0` reader - LETIMER0 Privileged Access"]
pub type Letimer0R = crate::BitReader;
#[doc = "Field `LETIMER0` writer - LETIMER0 Privileged Access"]
pub type Letimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IADC0` reader - IADC0 Privileged Access"]
pub type Iadc0R = crate::BitReader;
#[doc = "Field `IADC0` writer - IADC0 Privileged Access"]
pub type Iadc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - I2C0 Privileged Access"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C0` writer - I2C0 Privileged Access"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG0` reader - WDOG0 Privileged Access"]
pub type Wdog0R = crate::BitReader;
#[doc = "Field `WDOG0` writer - WDOG0 Privileged Access"]
pub type Wdog0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMUXCP0` reader - AMUXCP0 Privileged Access"]
pub type Amuxcp0R = crate::BitReader;
#[doc = "Field `AMUXCP0` writer - AMUXCP0 Privileged Access"]
pub type Amuxcp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EUART0` reader - EUART0 Privileged Access"]
pub type Euart0R = crate::BitReader;
#[doc = "Field `EUART0` writer - EUART0 Privileged Access"]
pub type Euart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTOACC` reader - CRYPTOACC Privileged Access"]
pub type CryptoaccR = crate::BitReader;
#[doc = "Field `CRYPTOACC` writer - CRYPTOACC Privileged Access"]
pub type CryptoaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBRADIO` reader - AHBRADIO Privileged Access"]
pub type AhbradioR = crate::BitReader;
#[doc = "Field `AHBRADIO` writer - AHBRADIO Privileged Access"]
pub type AhbradioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - DCDC Privileged Access"]
    #[inline(always)]
    pub fn dcdc(&self) -> DcdcR {
        DcdcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDM Privileged Access"]
    #[inline(always)]
    pub fn pdm(&self) -> PdmR {
        PdmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RFSENSE Privileged Access"]
    #[inline(always)]
    pub fn rfsense(&self) -> RfsenseR {
        RfsenseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RADIOAES Privileged Access"]
    #[inline(always)]
    pub fn radioaes(&self) -> RadioaesR {
        RadioaesR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMU Privileged Access"]
    #[inline(always)]
    pub fn smu(&self) -> SmuR {
        SmuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMUCFGNS Privileged Access"]
    #[inline(always)]
    pub fn smucfgns(&self) -> SmucfgnsR {
        SmucfgnsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTCC Privileged Access"]
    #[inline(always)]
    pub fn rtcc(&self) -> RtccR {
        RtccR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LETIMER0 Privileged Access"]
    #[inline(always)]
    pub fn letimer0(&self) -> Letimer0R {
        Letimer0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IADC0 Privileged Access"]
    #[inline(always)]
    pub fn iadc0(&self) -> Iadc0R {
        Iadc0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C0 Privileged Access"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WDOG0 Privileged Access"]
    #[inline(always)]
    pub fn wdog0(&self) -> Wdog0R {
        Wdog0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AMUXCP0 Privileged Access"]
    #[inline(always)]
    pub fn amuxcp0(&self) -> Amuxcp0R {
        Amuxcp0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EUART0 Privileged Access"]
    #[inline(always)]
    pub fn euart0(&self) -> Euart0R {
        Euart0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRYPTOACC Privileged Access"]
    #[inline(always)]
    pub fn cryptoacc(&self) -> CryptoaccR {
        CryptoaccR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AHBRADIO Privileged Access"]
    #[inline(always)]
    pub fn ahbradio(&self) -> AhbradioR {
        AhbradioR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DCDC Privileged Access"]
    #[inline(always)]
    pub fn dcdc(&mut self) -> DcdcW<'_, Ppunspatd1Spec> {
        DcdcW::new(self, 1)
    }
    #[doc = "Bit 2 - PDM Privileged Access"]
    #[inline(always)]
    pub fn pdm(&mut self) -> PdmW<'_, Ppunspatd1Spec> {
        PdmW::new(self, 2)
    }
    #[doc = "Bit 3 - RFSENSE Privileged Access"]
    #[inline(always)]
    pub fn rfsense(&mut self) -> RfsenseW<'_, Ppunspatd1Spec> {
        RfsenseW::new(self, 3)
    }
    #[doc = "Bit 4 - RADIOAES Privileged Access"]
    #[inline(always)]
    pub fn radioaes(&mut self) -> RadioaesW<'_, Ppunspatd1Spec> {
        RadioaesW::new(self, 4)
    }
    #[doc = "Bit 5 - SMU Privileged Access"]
    #[inline(always)]
    pub fn smu(&mut self) -> SmuW<'_, Ppunspatd1Spec> {
        SmuW::new(self, 5)
    }
    #[doc = "Bit 6 - SMUCFGNS Privileged Access"]
    #[inline(always)]
    pub fn smucfgns(&mut self) -> SmucfgnsW<'_, Ppunspatd1Spec> {
        SmucfgnsW::new(self, 6)
    }
    #[doc = "Bit 7 - RTCC Privileged Access"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RtccW<'_, Ppunspatd1Spec> {
        RtccW::new(self, 7)
    }
    #[doc = "Bit 8 - LETIMER0 Privileged Access"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> Letimer0W<'_, Ppunspatd1Spec> {
        Letimer0W::new(self, 8)
    }
    #[doc = "Bit 9 - IADC0 Privileged Access"]
    #[inline(always)]
    pub fn iadc0(&mut self) -> Iadc0W<'_, Ppunspatd1Spec> {
        Iadc0W::new(self, 9)
    }
    #[doc = "Bit 10 - I2C0 Privileged Access"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2c0W<'_, Ppunspatd1Spec> {
        I2c0W::new(self, 10)
    }
    #[doc = "Bit 11 - WDOG0 Privileged Access"]
    #[inline(always)]
    pub fn wdog0(&mut self) -> Wdog0W<'_, Ppunspatd1Spec> {
        Wdog0W::new(self, 11)
    }
    #[doc = "Bit 12 - AMUXCP0 Privileged Access"]
    #[inline(always)]
    pub fn amuxcp0(&mut self) -> Amuxcp0W<'_, Ppunspatd1Spec> {
        Amuxcp0W::new(self, 12)
    }
    #[doc = "Bit 13 - EUART0 Privileged Access"]
    #[inline(always)]
    pub fn euart0(&mut self) -> Euart0W<'_, Ppunspatd1Spec> {
        Euart0W::new(self, 13)
    }
    #[doc = "Bit 14 - CRYPTOACC Privileged Access"]
    #[inline(always)]
    pub fn cryptoacc(&mut self) -> CryptoaccW<'_, Ppunspatd1Spec> {
        CryptoaccW::new(self, 14)
    }
    #[doc = "Bit 15 - AHBRADIO Privileged Access"]
    #[inline(always)]
    pub fn ahbradio(&mut self) -> AhbradioW<'_, Ppunspatd1Spec> {
        AhbradioW::new(self, 15)
    }
}
#[doc = "Set peripheral bits to 1 to mark as privileged access only.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppunspatd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppunspatd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppunspatd1Spec;
impl crate::RegisterSpec for Ppunspatd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppunspatd1::R`](R) reader structure"]
impl crate::Readable for Ppunspatd1Spec {}
#[doc = "`write(|w| ..)` method takes [`ppunspatd1::W`](W) writer structure"]
impl crate::Writable for Ppunspatd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPUNSPATD1 to value 0"]
impl crate::Resettable for Ppunspatd1Spec {}
