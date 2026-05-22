#[doc = "Register `EUART0_ROUTEEN` reader"]
pub type R = crate::R<Euart0RouteenSpec>;
#[doc = "Register `EUART0_ROUTEEN` writer"]
pub type W = crate::W<Euart0RouteenSpec>;
#[doc = "Field `RTSPEN` reader - RTS pin enable control bit"]
pub type RtspenR = crate::BitReader;
#[doc = "Field `RTSPEN` writer - RTS pin enable control bit"]
pub type RtspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPEN` reader - TX pin enable control bit"]
pub type TxpenR = crate::BitReader;
#[doc = "Field `TXPEN` writer - TX pin enable control bit"]
pub type TxpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTS pin enable control bit"]
    #[inline(always)]
    pub fn rtspen(&self) -> RtspenR {
        RtspenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX pin enable control bit"]
    #[inline(always)]
    pub fn txpen(&self) -> TxpenR {
        TxpenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTS pin enable control bit"]
    #[inline(always)]
    pub fn rtspen(&mut self) -> RtspenW<'_, Euart0RouteenSpec> {
        RtspenW::new(self, 0)
    }
    #[doc = "Bit 1 - TX pin enable control bit"]
    #[inline(always)]
    pub fn txpen(&mut self) -> TxpenW<'_, Euart0RouteenSpec> {
        TxpenW::new(self, 1)
    }
}
#[doc = "EUART pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`euart0_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`euart0_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Euart0RouteenSpec;
impl crate::RegisterSpec for Euart0RouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`euart0_routeen::R`](R) reader structure"]
impl crate::Readable for Euart0RouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`euart0_routeen::W`](W) writer structure"]
impl crate::Writable for Euart0RouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EUART0_ROUTEEN to value 0"]
impl crate::Resettable for Euart0RouteenSpec {}
