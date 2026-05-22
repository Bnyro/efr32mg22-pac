#[doc = "Register `PKCTRL` writer"]
pub type W = crate::W<PkctrlSpec>;
#[doc = "Field `PKSTART` writer - PK Start"]
pub type PkstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFC` writer - ClearIRQ"]
pub type IfcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - PK Start"]
    #[inline(always)]
    pub fn pkstart(&mut self) -> PkstartW<'_, PkctrlSpec> {
        PkstartW::new(self, 0)
    }
    #[doc = "Bit 1 - ClearIRQ"]
    #[inline(always)]
    pub fn ifc(&mut self) -> IfcW<'_, PkctrlSpec> {
        IfcW::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkctrlSpec;
impl crate::RegisterSpec for PkctrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pkctrl::W`](W) writer structure"]
impl crate::Writable for PkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKCTRL to value 0"]
impl crate::Resettable for PkctrlSpec {}
