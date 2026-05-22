#[doc = "Register `PKSTATUS` reader"]
pub type R = crate::R<PkstatusSpec>;
#[doc = "Field `FAILADDR` reader - Fail Address"]
pub type FailaddrR = crate::FieldReader;
#[doc = "Field `NOTONCURVE` reader - Point Px not on curve"]
pub type NotoncurveR = crate::BitReader;
#[doc = "Field `ATINFINITY` reader - Point Px at infinity"]
pub type AtinfinityR = crate::BitReader;
#[doc = "Field `COUPLENOTVALID` reader - Couple not valid"]
pub type CouplenotvalidR = crate::BitReader;
#[doc = "Field `PARAMNNOTVALID` reader - Param n not valid"]
pub type ParamnnotvalidR = crate::BitReader;
#[doc = "Field `NOTIMPLEMENTED` reader - Not implemented"]
pub type NotimplementedR = crate::BitReader;
#[doc = "Field `SIGNOTVALID` reader - Signature not valid"]
pub type SignotvalidR = crate::BitReader;
#[doc = "Field `PARAMABNOTVALID` reader - Param AB not valid"]
pub type ParamabnotvalidR = crate::BitReader;
#[doc = "Field `NOTINVERTIBLE` reader - Not invertible"]
pub type NotinvertibleR = crate::BitReader;
#[doc = "Composite\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Composite {
    #[doc = "0: random number under test is probably prime"]
    False = 0,
    #[doc = "1: random number under test is composite"]
    True = 1,
}
impl From<Composite> for bool {
    #[inline(always)]
    fn from(variant: Composite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPOSITE` reader - Composite"]
pub type CompositeR = crate::BitReader<Composite>;
impl CompositeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Composite {
        match self.bits {
            false => Composite::False,
            true => Composite::True,
        }
    }
    #[doc = "random number under test is probably prime"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Composite::False
    }
    #[doc = "random number under test is composite"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Composite::True
    }
}
#[doc = "Field `NOTQUAD` reader - Not quadratic residue"]
pub type NotquadR = crate::BitReader;
#[doc = "Field `PKBUSY` reader - PK busy"]
pub type PkbusyR = crate::BitReader;
#[doc = "Field `PKIF` reader - Interrupt status"]
pub type PkifR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Fail Address"]
    #[inline(always)]
    pub fn failaddr(&self) -> FailaddrR {
        FailaddrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Point Px not on curve"]
    #[inline(always)]
    pub fn notoncurve(&self) -> NotoncurveR {
        NotoncurveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Point Px at infinity"]
    #[inline(always)]
    pub fn atinfinity(&self) -> AtinfinityR {
        AtinfinityR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Couple not valid"]
    #[inline(always)]
    pub fn couplenotvalid(&self) -> CouplenotvalidR {
        CouplenotvalidR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Param n not valid"]
    #[inline(always)]
    pub fn paramnnotvalid(&self) -> ParamnnotvalidR {
        ParamnnotvalidR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Not implemented"]
    #[inline(always)]
    pub fn notimplemented(&self) -> NotimplementedR {
        NotimplementedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Signature not valid"]
    #[inline(always)]
    pub fn signotvalid(&self) -> SignotvalidR {
        SignotvalidR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Param AB not valid"]
    #[inline(always)]
    pub fn paramabnotvalid(&self) -> ParamabnotvalidR {
        ParamabnotvalidR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Not invertible"]
    #[inline(always)]
    pub fn notinvertible(&self) -> NotinvertibleR {
        NotinvertibleR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Composite"]
    #[inline(always)]
    pub fn composite(&self) -> CompositeR {
        CompositeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Not quadratic residue"]
    #[inline(always)]
    pub fn notquad(&self) -> NotquadR {
        NotquadR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - PK busy"]
    #[inline(always)]
    pub fn pkbusy(&self) -> PkbusyR {
        PkbusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt status"]
    #[inline(always)]
    pub fn pkif(&self) -> PkifR {
        PkifR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pkstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkstatusSpec;
impl crate::RegisterSpec for PkstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkstatus::R`](R) reader structure"]
impl crate::Readable for PkstatusSpec {}
#[doc = "`reset()` method sets PKSTATUS to value 0"]
impl crate::Resettable for PkstatusSpec {}
