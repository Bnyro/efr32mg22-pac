#[doc = "Register `NSSTATUS` reader"]
pub type R = crate::R<NsstatusSpec>;
#[doc = "SMUNS Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smunslock {
    #[doc = "0: SMUNSLOCK Unlocked"]
    Unlocked = 0,
    #[doc = "1: SMUNSLOCK Locked"]
    Locked = 1,
}
impl From<Smunslock> for bool {
    #[inline(always)]
    fn from(variant: Smunslock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMUNSLOCK` reader - SMUNS Lock Status"]
pub type SmunslockR = crate::BitReader<Smunslock>;
impl SmunslockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smunslock {
        match self.bits {
            false => Smunslock::Unlocked,
            true => Smunslock::Locked,
        }
    }
    #[doc = "SMUNSLOCK Unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Smunslock::Unlocked
    }
    #[doc = "SMUNSLOCK Locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Smunslock::Locked
    }
}
impl R {
    #[doc = "Bit 0 - SMUNS Lock Status"]
    #[inline(always)]
    pub fn smunslock(&self) -> SmunslockR {
        SmunslockR::new((self.bits & 1) != 0)
    }
}
#[doc = "Register for status flags.\n\nYou can [`read`](crate::Reg::read) this register and get [`nsstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NsstatusSpec;
impl crate::RegisterSpec for NsstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsstatus::R`](R) reader structure"]
impl crate::Readable for NsstatusSpec {}
#[doc = "`reset()` method sets NSSTATUS to value 0"]
impl crate::Resettable for NsstatusSpec {}
