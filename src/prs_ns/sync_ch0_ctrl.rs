#[doc = "Register `SYNC_CH0_CTRL` reader"]
pub type R = crate::R<SyncCh0CtrlSpec>;
#[doc = "Register `SYNC_CH0_CTRL` writer"]
pub type W = crate::W<SyncCh0CtrlSpec>;
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SigselR = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SourceselR = crate::FieldReader;
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SourceselW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SourceselR {
        SourceselR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&mut self) -> SigselW<'_, SyncCh0CtrlSpec> {
        SigselW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&mut self) -> SourceselW<'_, SyncCh0CtrlSpec> {
        SourceselW::new(self, 8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ch0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ch0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncCh0CtrlSpec;
impl crate::RegisterSpec for SyncCh0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_ch0_ctrl::R`](R) reader structure"]
impl crate::Readable for SyncCh0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sync_ch0_ctrl::W`](W) writer structure"]
impl crate::Writable for SyncCh0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNC_CH0_CTRL to value 0"]
impl crate::Resettable for SyncCh0CtrlSpec {}
