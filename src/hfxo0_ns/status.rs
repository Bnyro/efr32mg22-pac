#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RDY` reader - Ready Status"]
pub type RdyR = crate::BitReader;
#[doc = "Field `COREBIASOPTRDY` reader - Core Bias Optimization Ready"]
pub type CorebiasoptrdyR = crate::BitReader;
#[doc = "Field `ENS` reader - Enabled Status"]
pub type EnsR = crate::BitReader;
#[doc = "Field `HWREQ` reader - Oscillator Requested by Hardware"]
pub type HwreqR = crate::BitReader;
#[doc = "Field `ISWARM` reader - Oscillator Is Kept Warm"]
pub type IswarmR = crate::BitReader;
#[doc = "FSM Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsmlock {
    #[doc = "0: FSM lock is unlocked"]
    Unlocked = 0,
    #[doc = "1: FSM lock is locked"]
    Locked = 1,
}
impl From<Fsmlock> for bool {
    #[inline(always)]
    fn from(variant: Fsmlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSMLOCK` reader - FSM Lock Status"]
pub type FsmlockR = crate::BitReader<Fsmlock>;
impl FsmlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsmlock {
        match self.bits {
            false => Fsmlock::Unlocked,
            true => Fsmlock::Locked,
        }
    }
    #[doc = "FSM lock is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Fsmlock::Unlocked
    }
    #[doc = "FSM lock is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Fsmlock::Locked
    }
}
#[doc = "Configuration Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Configuration lock is unlocked"]
    Unlocked = 0,
    #[doc = "1: Configuration lock is locked"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Configuration Lock Status"]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlocked,
            true => Lock::Locked,
        }
    }
    #[doc = "Configuration lock is unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "Configuration lock is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
impl R {
    #[doc = "Bit 0 - Ready Status"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core Bias Optimization Ready"]
    #[inline(always)]
    pub fn corebiasoptrdy(&self) -> CorebiasoptrdyR {
        CorebiasoptrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Enabled Status"]
    #[inline(always)]
    pub fn ens(&self) -> EnsR {
        EnsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Oscillator Requested by Hardware"]
    #[inline(always)]
    pub fn hwreq(&self) -> HwreqR {
        HwreqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Oscillator Is Kept Warm"]
    #[inline(always)]
    pub fn iswarm(&self) -> IswarmR {
        IswarmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 30 - FSM Lock Status"]
    #[inline(always)]
    pub fn fsmlock(&self) -> FsmlockR {
        FsmlockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configuration Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
