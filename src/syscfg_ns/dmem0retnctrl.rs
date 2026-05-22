#[doc = "Register `DMEM0RETNCTRL` reader"]
pub type R = crate::R<Dmem0retnctrlSpec>;
#[doc = "Register `DMEM0RETNCTRL` writer"]
pub type W = crate::W<Dmem0retnctrlSpec>;
#[doc = "DMEM0 blockset retention control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ramretnctrl {
    #[doc = "0: None of the RAM blocks powered down"]
    Allon = 0,
    #[doc = "1: Power down RAM block 0"]
    Blk0 = 1,
    #[doc = "2: Power down RAM block 1"]
    Blk1 = 2,
}
impl From<Ramretnctrl> for u8 {
    #[inline(always)]
    fn from(variant: Ramretnctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ramretnctrl {
    type Ux = u8;
}
impl crate::IsEnum for Ramretnctrl {}
#[doc = "Field `RAMRETNCTRL` reader - DMEM0 blockset retention control"]
pub type RamretnctrlR = crate::FieldReader<Ramretnctrl>;
impl RamretnctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ramretnctrl> {
        match self.bits {
            0 => Some(Ramretnctrl::Allon),
            1 => Some(Ramretnctrl::Blk0),
            2 => Some(Ramretnctrl::Blk1),
            _ => None,
        }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn is_allon(&self) -> bool {
        *self == Ramretnctrl::Allon
    }
    #[doc = "Power down RAM block 0"]
    #[inline(always)]
    pub fn is_blk0(&self) -> bool {
        *self == Ramretnctrl::Blk0
    }
    #[doc = "Power down RAM block 1"]
    #[inline(always)]
    pub fn is_blk1(&self) -> bool {
        *self == Ramretnctrl::Blk1
    }
}
#[doc = "Field `RAMRETNCTRL` writer - DMEM0 blockset retention control"]
pub type RamretnctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ramretnctrl>;
impl<'a, REG> RamretnctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn allon(self) -> &'a mut crate::W<REG> {
        self.variant(Ramretnctrl::Allon)
    }
    #[doc = "Power down RAM block 0"]
    #[inline(always)]
    pub fn blk0(self) -> &'a mut crate::W<REG> {
        self.variant(Ramretnctrl::Blk0)
    }
    #[doc = "Power down RAM block 1"]
    #[inline(always)]
    pub fn blk1(self) -> &'a mut crate::W<REG> {
        self.variant(Ramretnctrl::Blk1)
    }
}
impl R {
    #[doc = "Bits 0:1 - DMEM0 blockset retention control"]
    #[inline(always)]
    pub fn ramretnctrl(&self) -> RamretnctrlR {
        RamretnctrlR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DMEM0 blockset retention control"]
    #[inline(always)]
    pub fn ramretnctrl(&mut self) -> RamretnctrlW<'_, Dmem0retnctrlSpec> {
        RamretnctrlW::new(self, 0)
    }
}
#[doc = "Configure to provide general RAM retention configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmem0retnctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmem0retnctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmem0retnctrlSpec;
impl crate::RegisterSpec for Dmem0retnctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmem0retnctrl::R`](R) reader structure"]
impl crate::Readable for Dmem0retnctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmem0retnctrl::W`](W) writer structure"]
impl crate::Writable for Dmem0retnctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMEM0RETNCTRL to value 0"]
impl crate::Resettable for Dmem0retnctrlSpec {}
