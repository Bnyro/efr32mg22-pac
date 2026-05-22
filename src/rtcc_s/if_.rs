#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `OF` reader - Overflow Interrupt Flag"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - Overflow Interrupt Flag"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTTICK` reader - Main counter tick"]
pub type CnttickR = crate::BitReader;
#[doc = "Field `CNTTICK` writer - Main counter tick"]
pub type CnttickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` reader - CC Channel n Interrupt Flag"]
pub type Cc0R = crate::BitReader;
#[doc = "Field `CC0` writer - CC Channel n Interrupt Flag"]
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` reader - CC Channel n Interrupt Flag"]
pub type Cc1R = crate::BitReader;
#[doc = "Field `CC1` writer - CC Channel n Interrupt Flag"]
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` reader - CC Channel n Interrupt Flag"]
pub type Cc2R = crate::BitReader;
#[doc = "Field `CC2` writer - CC Channel n Interrupt Flag"]
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main counter tick"]
    #[inline(always)]
    pub fn cnttick(&self) -> CnttickR {
        CnttickR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CC Channel n Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CC Channel n Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CC Channel n Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IfSpec> {
        OfW::new(self, 0)
    }
    #[doc = "Bit 1 - Main counter tick"]
    #[inline(always)]
    pub fn cnttick(&mut self) -> CnttickW<'_, IfSpec> {
        CnttickW::new(self, 1)
    }
    #[doc = "Bit 4 - CC Channel n Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&mut self) -> Cc0W<'_, IfSpec> {
        Cc0W::new(self, 4)
    }
    #[doc = "Bit 6 - CC Channel n Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&mut self) -> Cc1W<'_, IfSpec> {
        Cc1W::new(self, 6)
    }
    #[doc = "Bit 8 - CC Channel n Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&mut self) -> Cc2W<'_, IfSpec> {
        Cc2W::new(self, 8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
