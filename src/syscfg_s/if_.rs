#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `SW0` reader - Software Interrupt 0"]
pub type Sw0R = crate::BitReader;
#[doc = "Field `SW0` writer - Software Interrupt 0"]
pub type Sw0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW1` reader - Software Interrupt 1"]
pub type Sw1R = crate::BitReader;
#[doc = "Field `SW1` writer - Software Interrupt 1"]
pub type Sw1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW2` reader - Software Interrupt 2"]
pub type Sw2R = crate::BitReader;
#[doc = "Field `SW2` writer - Software Interrupt 2"]
pub type Sw2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW3` reader - Software Interrupt 3"]
pub type Sw3R = crate::BitReader;
#[doc = "Field `SW3` writer - Software Interrupt 3"]
pub type Sw3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR1B` reader - RAM 1-Bit Error Interrupt Flag"]
pub type Ramerr1bR = crate::BitReader;
#[doc = "Field `RAMERR1B` writer - RAM 1-Bit Error Interrupt Flag"]
pub type Ramerr1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERR2B` reader - RAM 2-Bit Error Interrupt Flag"]
pub type Ramerr2bR = crate::BitReader;
#[doc = "Field `RAMERR2B` writer - RAM 2-Bit Error Interrupt Flag"]
pub type Ramerr2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQRAMERR1B` reader - SEQRAM 1-Bit Error Interrupt Flag"]
pub type Seqramerr1bR = crate::BitReader;
#[doc = "Field `SEQRAMERR1B` writer - SEQRAM 1-Bit Error Interrupt Flag"]
pub type Seqramerr1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQRAMERR2B` reader - SEQRAM 2-Bit Error Interrupt Flag"]
pub type Seqramerr2bR = crate::BitReader;
#[doc = "Field `SEQRAMERR2B` writer - SEQRAM 2-Bit Error Interrupt Flag"]
pub type Seqramerr2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCRAMERR1B` reader - FRCRAM 1-Bit Error Interrupt Flag"]
pub type Frcramerr1bR = crate::BitReader;
#[doc = "Field `FRCRAMERR1B` writer - FRCRAM 1-Bit Error Interrupt Flag"]
pub type Frcramerr1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCRAMERR2B` reader - FRCRAM 2-Bit Error Interrupt Flag"]
pub type Frcramerr2bR = crate::BitReader;
#[doc = "Field `FRCRAMERR2B` writer - FRCRAM 2-Bit Error Interrupt Flag"]
pub type Frcramerr2bW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Interrupt 0"]
    #[inline(always)]
    pub fn sw0(&self) -> Sw0R {
        Sw0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Interrupt 1"]
    #[inline(always)]
    pub fn sw1(&self) -> Sw1R {
        Sw1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Interrupt 2"]
    #[inline(always)]
    pub fn sw2(&self) -> Sw2R {
        Sw2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Interrupt 3"]
    #[inline(always)]
    pub fn sw3(&self) -> Sw3R {
        Sw3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - RAM 1-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr1b(&self) -> Ramerr1bR {
        Ramerr1bR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAM 2-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr2b(&self) -> Ramerr2bR {
        Ramerr2bR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - SEQRAM 1-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr1b(&self) -> Seqramerr1bR {
        Seqramerr1bR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SEQRAM 2-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr2b(&self) -> Seqramerr2bR {
        Seqramerr2bR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - FRCRAM 1-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr1b(&self) -> Frcramerr1bR {
        Frcramerr1bR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FRCRAM 2-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr2b(&self) -> Frcramerr2bR {
        Frcramerr2bR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Interrupt 0"]
    #[inline(always)]
    pub fn sw0(&mut self) -> Sw0W<'_, IfSpec> {
        Sw0W::new(self, 0)
    }
    #[doc = "Bit 1 - Software Interrupt 1"]
    #[inline(always)]
    pub fn sw1(&mut self) -> Sw1W<'_, IfSpec> {
        Sw1W::new(self, 1)
    }
    #[doc = "Bit 2 - Software Interrupt 2"]
    #[inline(always)]
    pub fn sw2(&mut self) -> Sw2W<'_, IfSpec> {
        Sw2W::new(self, 2)
    }
    #[doc = "Bit 3 - Software Interrupt 3"]
    #[inline(always)]
    pub fn sw3(&mut self) -> Sw3W<'_, IfSpec> {
        Sw3W::new(self, 3)
    }
    #[doc = "Bit 16 - RAM 1-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr1b(&mut self) -> Ramerr1bW<'_, IfSpec> {
        Ramerr1bW::new(self, 16)
    }
    #[doc = "Bit 17 - RAM 2-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr2b(&mut self) -> Ramerr2bW<'_, IfSpec> {
        Ramerr2bW::new(self, 17)
    }
    #[doc = "Bit 24 - SEQRAM 1-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr1b(&mut self) -> Seqramerr1bW<'_, IfSpec> {
        Seqramerr1bW::new(self, 24)
    }
    #[doc = "Bit 25 - SEQRAM 2-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr2b(&mut self) -> Seqramerr2bW<'_, IfSpec> {
        Seqramerr2bW::new(self, 25)
    }
    #[doc = "Bit 28 - FRCRAM 1-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr1b(&mut self) -> Frcramerr1bW<'_, IfSpec> {
        Frcramerr1bW::new(self, 28)
    }
    #[doc = "Bit 29 - FRCRAM 2-Bit Error Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr2b(&mut self) -> Frcramerr2bW<'_, IfSpec> {
        Frcramerr2bW::new(self, 29)
    }
}
#[doc = "Read to get system status.\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`write(|w| ..)` method takes [`if_::W`](W) writer structure"]
impl crate::Writable for IfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
