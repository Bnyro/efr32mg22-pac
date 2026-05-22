#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `GAIN` reader - Selects Gain factor of DCF"]
pub type GainR = crate::FieldReader;
#[doc = "Field `GAIN` writer - Selects Gain factor of DCF"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DSR` reader - Down sampling rate of Decimation filter"]
pub type DsrR = crate::FieldReader<u16>;
#[doc = "Field `DSR` writer - Down sampling rate of Decimation filter"]
pub type DsrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:4 - Selects Gain factor of DCF"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:19 - Down sampling rate of Decimation filter"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects Gain factor of DCF"]
    #[inline(always)]
    pub fn gain(&mut self) -> GainW<'_, CtrlSpec> {
        GainW::new(self, 0)
    }
    #[doc = "Bits 8:19 - Down sampling rate of Decimation filter"]
    #[inline(always)]
    pub fn dsr(&mut self) -> DsrW<'_, CtrlSpec> {
        DsrW::new(self, 8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
