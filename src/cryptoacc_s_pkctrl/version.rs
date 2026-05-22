#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Field `SW` reader - Software version number"]
pub type SwR = crate::FieldReader;
#[doc = "Field `HW` reader - Hardware version number"]
pub type HwR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Software version number"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Hardware version number"]
    #[inline(always)]
    pub fn hw(&self) -> HwR {
        HwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
#[doc = "`reset()` method sets VERSION to value 0"]
impl crate::Resettable for VersionSpec {}
