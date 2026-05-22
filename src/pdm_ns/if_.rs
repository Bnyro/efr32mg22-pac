#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `DV` reader - Data Valid Interrupt Flag"]
pub type DvR = crate::BitReader;
#[doc = "Field `DV` writer - Data Valid Interrupt Flag"]
pub type DvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DVL` reader - Data Valid Level Interrupt Flag"]
pub type DvlR = crate::BitReader;
#[doc = "Field `DVL` writer - Data Valid Level Interrupt Flag"]
pub type DvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` reader - FIFO Overflow Interrupt Flag"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - FIFO Overflow Interrupt Flag"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` reader - FIFO Undeflow Interrupt Flag"]
pub type UfR = crate::BitReader;
#[doc = "Field `UF` writer - FIFO Undeflow Interrupt Flag"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn dv(&self) -> DvR {
        DvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Valid Level Interrupt Flag"]
    #[inline(always)]
    pub fn dvl(&self) -> DvlR {
        DvlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Undeflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn dv(&mut self) -> DvW<'_, IfSpec> {
        DvW::new(self, 0)
    }
    #[doc = "Bit 1 - Data Valid Level Interrupt Flag"]
    #[inline(always)]
    pub fn dvl(&mut self) -> DvlW<'_, IfSpec> {
        DvlW::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IfSpec> {
        OfW::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO Undeflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<'_, IfSpec> {
        UfW::new(self, 3)
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
