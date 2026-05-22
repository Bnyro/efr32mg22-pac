#[doc = "Register `TRACEROUTEPEN` reader"]
pub type R = crate::R<TraceroutepenSpec>;
#[doc = "Register `TRACEROUTEPEN` writer"]
pub type W = crate::W<TraceroutepenSpec>;
#[doc = "Field `SWVPEN` reader - Serial Wire Viewer Output Pin Enable"]
pub type SwvpenR = crate::BitReader;
#[doc = "Field `SWVPEN` writer - Serial Wire Viewer Output Pin Enable"]
pub type SwvpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACECLKPEN` reader - Trace Clk Pin Enable"]
pub type TraceclkpenR = crate::BitReader;
#[doc = "Field `TRACECLKPEN` writer - Trace Clk Pin Enable"]
pub type TraceclkpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACEDATA0PEN` reader - Trace Data0 Pin Enable"]
pub type Tracedata0penR = crate::BitReader;
#[doc = "Field `TRACEDATA0PEN` writer - Trace Data0 Pin Enable"]
pub type Tracedata0penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&self) -> SwvpenR {
        SwvpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trace Clk Pin Enable"]
    #[inline(always)]
    pub fn traceclkpen(&self) -> TraceclkpenR {
        TraceclkpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trace Data0 Pin Enable"]
    #[inline(always)]
    pub fn tracedata0pen(&self) -> Tracedata0penR {
        Tracedata0penR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&mut self) -> SwvpenW<'_, TraceroutepenSpec> {
        SwvpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Trace Clk Pin Enable"]
    #[inline(always)]
    pub fn traceclkpen(&mut self) -> TraceclkpenW<'_, TraceroutepenSpec> {
        TraceclkpenW::new(self, 1)
    }
    #[doc = "Bit 2 - Trace Data0 Pin Enable"]
    #[inline(always)]
    pub fn tracedata0pen(&mut self) -> Tracedata0penW<'_, TraceroutepenSpec> {
        Tracedata0penW::new(self, 2)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`traceroutepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceroutepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TraceroutepenSpec;
impl crate::RegisterSpec for TraceroutepenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`traceroutepen::R`](R) reader structure"]
impl crate::Readable for TraceroutepenSpec {}
#[doc = "`write(|w| ..)` method takes [`traceroutepen::W`](W) writer structure"]
impl crate::Writable for TraceroutepenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRACEROUTEPEN to value 0"]
impl crate::Resettable for TraceroutepenSpec {}
