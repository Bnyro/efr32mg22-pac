#[doc = "Register `TIMINGCFG` reader"]
pub type R = crate::R<TimingcfgSpec>;
#[doc = "Register `TIMINGCFG` writer"]
pub type W = crate::W<TimingcfgSpec>;
#[doc = "TX Delay Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txdelay {
    #[doc = "0: Frames are transmitted immediately."]
    None = 0,
    #[doc = "1: Transmission of new frames is delayed by a single bit period."]
    Single = 1,
    #[doc = "2: Transmission of new frames is delayed by a two bit periods."]
    Double = 2,
    #[doc = "3: Transmission of new frames is delayed by a three bit periods."]
    Tripple = 3,
}
impl From<Txdelay> for u8 {
    #[inline(always)]
    fn from(variant: Txdelay) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txdelay {
    type Ux = u8;
}
impl crate::IsEnum for Txdelay {}
#[doc = "Field `TXDELAY` reader - TX Delay Transmission"]
pub type TxdelayR = crate::FieldReader<Txdelay>;
impl TxdelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdelay {
        match self.bits {
            0 => Txdelay::None,
            1 => Txdelay::Single,
            2 => Txdelay::Double,
            3 => Txdelay::Tripple,
            _ => unreachable!(),
        }
    }
    #[doc = "Frames are transmitted immediately."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Txdelay::None
    }
    #[doc = "Transmission of new frames is delayed by a single bit period."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Txdelay::Single
    }
    #[doc = "Transmission of new frames is delayed by a two bit periods."]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == Txdelay::Double
    }
    #[doc = "Transmission of new frames is delayed by a three bit periods."]
    #[inline(always)]
    pub fn is_tripple(&self) -> bool {
        *self == Txdelay::Tripple
    }
}
#[doc = "Field `TXDELAY` writer - TX Delay Transmission"]
pub type TxdelayW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txdelay, crate::Safe>;
impl<'a, REG> TxdelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frames are transmitted immediately."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::None)
    }
    #[doc = "Transmission of new frames is delayed by a single bit period."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Single)
    }
    #[doc = "Transmission of new frames is delayed by a two bit periods."]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Double)
    }
    #[doc = "Transmission of new frames is delayed by a three bit periods."]
    #[inline(always)]
    pub fn tripple(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Tripple)
    }
}
impl R {
    #[doc = "Bits 0:1 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&self) -> TxdelayR {
        TxdelayR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&mut self) -> TxdelayW<'_, TimingcfgSpec> {
        TxdelayW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timingcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimingcfgSpec;
impl crate::RegisterSpec for TimingcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timingcfg::R`](R) reader structure"]
impl crate::Readable for TimingcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`timingcfg::W`](W) writer structure"]
impl crate::Writable for TimingcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMINGCFG to value 0"]
impl crate::Resettable for TimingcfgSpec {}
