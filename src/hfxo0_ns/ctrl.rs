#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `FORCEEN` reader - Force Enable"]
pub type ForceenR = crate::BitReader;
#[doc = "Field `FORCEEN` writer - Force Enable"]
pub type ForceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISONDEMAND` reader - Disable On-demand Mode"]
pub type DisondemandR = crate::BitReader;
#[doc = "Field `DISONDEMAND` writer - Disable On-demand Mode"]
pub type DisondemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEPWARM` reader - Keep Warm"]
pub type KeepwarmR = crate::BitReader;
#[doc = "Field `KEEPWARM` writer - Keep Warm"]
pub type KeepwarmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Force XI Pin to Ground\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcexi2gndana {
    #[doc = "0: Disabled (not pulled)"]
    Disable = 0,
    #[doc = "1: Enabled (pulled)"]
    Enable = 1,
}
impl From<Forcexi2gndana> for bool {
    #[inline(always)]
    fn from(variant: Forcexi2gndana) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEXI2GNDANA` reader - Force XI Pin to Ground"]
pub type Forcexi2gndanaR = crate::BitReader<Forcexi2gndana>;
impl Forcexi2gndanaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcexi2gndana {
        match self.bits {
            false => Forcexi2gndana::Disable,
            true => Forcexi2gndana::Enable,
        }
    }
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Forcexi2gndana::Disable
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Forcexi2gndana::Enable
    }
}
#[doc = "Field `FORCEXI2GNDANA` writer - Force XI Pin to Ground"]
pub type Forcexi2gndanaW<'a, REG> = crate::BitWriter<'a, REG, Forcexi2gndana>;
impl<'a, REG> Forcexi2gndanaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Forcexi2gndana::Disable)
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Forcexi2gndana::Enable)
    }
}
#[doc = "Force XO Pin to Ground\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcexo2gndana {
    #[doc = "0: Disabled (not pulled)"]
    Disable = 0,
    #[doc = "1: Enabled (pulled)"]
    Enable = 1,
}
impl From<Forcexo2gndana> for bool {
    #[inline(always)]
    fn from(variant: Forcexo2gndana) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEXO2GNDANA` reader - Force XO Pin to Ground"]
pub type Forcexo2gndanaR = crate::BitReader<Forcexo2gndana>;
impl Forcexo2gndanaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcexo2gndana {
        match self.bits {
            false => Forcexo2gndana::Disable,
            true => Forcexo2gndana::Enable,
        }
    }
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Forcexo2gndana::Disable
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Forcexo2gndana::Enable
    }
}
#[doc = "Field `FORCEXO2GNDANA` writer - Force XO Pin to Ground"]
pub type Forcexo2gndanaW<'a, REG> = crate::BitWriter<'a, REG, Forcexo2gndana>;
impl<'a, REG> Forcexo2gndanaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Forcexo2gndana::Disable)
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Forcexo2gndana::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Force Enable"]
    #[inline(always)]
    pub fn forceen(&self) -> ForceenR {
        ForceenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable On-demand Mode"]
    #[inline(always)]
    pub fn disondemand(&self) -> DisondemandR {
        DisondemandR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Keep Warm"]
    #[inline(always)]
    pub fn keepwarm(&self) -> KeepwarmR {
        KeepwarmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Force XI Pin to Ground"]
    #[inline(always)]
    pub fn forcexi2gndana(&self) -> Forcexi2gndanaR {
        Forcexi2gndanaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force XO Pin to Ground"]
    #[inline(always)]
    pub fn forcexo2gndana(&self) -> Forcexo2gndanaR {
        Forcexo2gndanaR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force Enable"]
    #[inline(always)]
    pub fn forceen(&mut self) -> ForceenW<'_, CtrlSpec> {
        ForceenW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable On-demand Mode"]
    #[inline(always)]
    pub fn disondemand(&mut self) -> DisondemandW<'_, CtrlSpec> {
        DisondemandW::new(self, 1)
    }
    #[doc = "Bit 2 - Keep Warm"]
    #[inline(always)]
    pub fn keepwarm(&mut self) -> KeepwarmW<'_, CtrlSpec> {
        KeepwarmW::new(self, 2)
    }
    #[doc = "Bit 4 - Force XI Pin to Ground"]
    #[inline(always)]
    pub fn forcexi2gndana(&mut self) -> Forcexi2gndanaW<'_, CtrlSpec> {
        Forcexi2gndanaW::new(self, 4)
    }
    #[doc = "Bit 5 - Force XO Pin to Ground"]
    #[inline(always)]
    pub fn forcexo2gndana(&mut self) -> Forcexo2gndanaW<'_, CtrlSpec> {
        Forcexo2gndanaW::new(self, 5)
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
#[doc = "`reset()` method sets CTRL to value 0x02"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x02;
}
