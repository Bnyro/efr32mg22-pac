#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RUNNING` reader - RTCC running status"]
pub type RunningR = crate::BitReader;
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcclockstatus {
    #[doc = "0: RTCC registers are unlocked"]
    Unlocked = 0,
    #[doc = "1: RTCC registers are locked"]
    Locked = 1,
}
impl From<Rtcclockstatus> for bool {
    #[inline(always)]
    fn from(variant: Rtcclockstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCCLOCKSTATUS` reader - Lock Status"]
pub type RtcclockstatusR = crate::BitReader<Rtcclockstatus>;
impl RtcclockstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcclockstatus {
        match self.bits {
            false => Rtcclockstatus::Unlocked,
            true => Rtcclockstatus::Locked,
        }
    }
    #[doc = "RTCC registers are unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Rtcclockstatus::Unlocked
    }
    #[doc = "RTCC registers are locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Rtcclockstatus::Locked
    }
}
impl R {
    #[doc = "Bit 0 - RTCC running status"]
    #[inline(always)]
    pub fn running(&self) -> RunningR {
        RunningR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Status"]
    #[inline(always)]
    pub fn rtcclockstatus(&self) -> RtcclockstatusR {
        RtcclockstatusR::new(((self.bits >> 1) & 1) != 0)
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
