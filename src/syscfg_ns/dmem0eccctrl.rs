#[doc = "Register `DMEM0ECCCTRL` reader"]
pub type R = crate::R<Dmem0eccctrlSpec>;
#[doc = "Register `DMEM0ECCCTRL` writer"]
pub type W = crate::W<Dmem0eccctrlSpec>;
#[doc = "Field `RAMECCEN` reader - RAM ECC Enable"]
pub type RameccenR = crate::BitReader;
#[doc = "Field `RAMECCEN` writer - RAM ECC Enable"]
pub type RameccenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMECCEWEN` reader - RAM ECC Error Writeback Enable"]
pub type RameccewenR = crate::BitReader;
#[doc = "Field `RAMECCEWEN` writer - RAM ECC Error Writeback Enable"]
pub type RameccewenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM ECC Enable"]
    #[inline(always)]
    pub fn rameccen(&self) -> RameccenR {
        RameccenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM ECC Error Writeback Enable"]
    #[inline(always)]
    pub fn rameccewen(&self) -> RameccewenR {
        RameccewenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM ECC Enable"]
    #[inline(always)]
    pub fn rameccen(&mut self) -> RameccenW<'_, Dmem0eccctrlSpec> {
        RameccenW::new(self, 0)
    }
    #[doc = "Bit 1 - RAM ECC Error Writeback Enable"]
    #[inline(always)]
    pub fn rameccewen(&mut self) -> RameccewenW<'_, Dmem0eccctrlSpec> {
        RameccewenW::new(self, 1)
    }
}
#[doc = "Configure to set RAM ECC control.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmem0eccctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmem0eccctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmem0eccctrlSpec;
impl crate::RegisterSpec for Dmem0eccctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmem0eccctrl::R`](R) reader structure"]
impl crate::Readable for Dmem0eccctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmem0eccctrl::W`](W) writer structure"]
impl crate::Writable for Dmem0eccctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMEM0ECCCTRL to value 0"]
impl crate::Resettable for Dmem0eccctrlSpec {}
