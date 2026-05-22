#[doc = "Register `CFG0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<Cfg0Spec>;
#[doc = "Filter order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Forder {
    #[doc = "0: Second order filter."]
    Second = 0,
    #[doc = "1: Third order filter."]
    Third = 1,
    #[doc = "2: Fourth order filter."]
    Fourth = 2,
    #[doc = "3: Fifth order filter."]
    Fifth = 3,
}
impl From<Forder> for u8 {
    #[inline(always)]
    fn from(variant: Forder) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Forder {
    type Ux = u8;
}
impl crate::IsEnum for Forder {}
#[doc = "Field `FORDER` reader - Filter order"]
pub type ForderR = crate::FieldReader<Forder>;
impl ForderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forder {
        match self.bits {
            0 => Forder::Second,
            1 => Forder::Third,
            2 => Forder::Fourth,
            3 => Forder::Fifth,
            _ => unreachable!(),
        }
    }
    #[doc = "Second order filter."]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == Forder::Second
    }
    #[doc = "Third order filter."]
    #[inline(always)]
    pub fn is_third(&self) -> bool {
        *self == Forder::Third
    }
    #[doc = "Fourth order filter."]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == Forder::Fourth
    }
    #[doc = "Fifth order filter."]
    #[inline(always)]
    pub fn is_fifth(&self) -> bool {
        *self == Forder::Fifth
    }
}
#[doc = "Field `FORDER` writer - Filter order"]
pub type ForderW<'a, REG> = crate::FieldWriter<'a, REG, 2, Forder, crate::Safe>;
impl<'a, REG> ForderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Second order filter."]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(Forder::Second)
    }
    #[doc = "Third order filter."]
    #[inline(always)]
    pub fn third(self) -> &'a mut crate::W<REG> {
        self.variant(Forder::Third)
    }
    #[doc = "Fourth order filter."]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut crate::W<REG> {
        self.variant(Forder::Fourth)
    }
    #[doc = "Fifth order filter."]
    #[inline(always)]
    pub fn fifth(self) -> &'a mut crate::W<REG> {
        self.variant(Forder::Fifth)
    }
}
#[doc = "Number of Channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Numch {
    #[doc = "0: One channel."]
    One = 0,
    #[doc = "1: Two channels."]
    Two = 1,
}
impl From<Numch> for bool {
    #[inline(always)]
    fn from(variant: Numch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NUMCH` reader - Number of Channels"]
pub type NumchR = crate::BitReader<Numch>;
impl NumchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Numch {
        match self.bits {
            false => Numch::One,
            true => Numch::Two,
        }
    }
    #[doc = "One channel."]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Numch::One
    }
    #[doc = "Two channels."]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Numch::Two
    }
}
#[doc = "Field `NUMCH` writer - Number of Channels"]
pub type NumchW<'a, REG> = crate::BitWriter<'a, REG, Numch>;
impl<'a, REG> NumchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One channel."]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Numch::One)
    }
    #[doc = "Two channels."]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Numch::Two)
    }
}
#[doc = "Filter output format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dataformat {
    #[doc = "0: Right aligned 16-bit, left bits are sign extended."]
    Right16 = 0,
    #[doc = "1: Pack two 16-bit samples into one 32-bit word."]
    Double16 = 1,
    #[doc = "2: Right aligned 24bit, left bits are sign extended."]
    Right24 = 2,
    #[doc = "3: 32 bit data."]
    Full32bit = 3,
    #[doc = "4: Left aligned 16-bit, right bits are zeros."]
    Left16 = 4,
    #[doc = "5: Left aligned 24-bit, right bits are zeros."]
    Left24 = 5,
    #[doc = "6: RAW 32 bit data from Integrator."]
    Raw32bit = 6,
}
impl From<Dataformat> for u8 {
    #[inline(always)]
    fn from(variant: Dataformat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dataformat {
    type Ux = u8;
}
impl crate::IsEnum for Dataformat {}
#[doc = "Field `DATAFORMAT` reader - Filter output format"]
pub type DataformatR = crate::FieldReader<Dataformat>;
impl DataformatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dataformat> {
        match self.bits {
            0 => Some(Dataformat::Right16),
            1 => Some(Dataformat::Double16),
            2 => Some(Dataformat::Right24),
            3 => Some(Dataformat::Full32bit),
            4 => Some(Dataformat::Left16),
            5 => Some(Dataformat::Left24),
            6 => Some(Dataformat::Raw32bit),
            _ => None,
        }
    }
    #[doc = "Right aligned 16-bit, left bits are sign extended."]
    #[inline(always)]
    pub fn is_right16(&self) -> bool {
        *self == Dataformat::Right16
    }
    #[doc = "Pack two 16-bit samples into one 32-bit word."]
    #[inline(always)]
    pub fn is_double16(&self) -> bool {
        *self == Dataformat::Double16
    }
    #[doc = "Right aligned 24bit, left bits are sign extended."]
    #[inline(always)]
    pub fn is_right24(&self) -> bool {
        *self == Dataformat::Right24
    }
    #[doc = "32 bit data."]
    #[inline(always)]
    pub fn is_full32bit(&self) -> bool {
        *self == Dataformat::Full32bit
    }
    #[doc = "Left aligned 16-bit, right bits are zeros."]
    #[inline(always)]
    pub fn is_left16(&self) -> bool {
        *self == Dataformat::Left16
    }
    #[doc = "Left aligned 24-bit, right bits are zeros."]
    #[inline(always)]
    pub fn is_left24(&self) -> bool {
        *self == Dataformat::Left24
    }
    #[doc = "RAW 32 bit data from Integrator."]
    #[inline(always)]
    pub fn is_raw32bit(&self) -> bool {
        *self == Dataformat::Raw32bit
    }
}
#[doc = "Field `DATAFORMAT` writer - Filter output format"]
pub type DataformatW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dataformat>;
impl<'a, REG> DataformatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Right aligned 16-bit, left bits are sign extended."]
    #[inline(always)]
    pub fn right16(self) -> &'a mut crate::W<REG> {
        self.variant(Dataformat::Right16)
    }
    #[doc = "Pack two 16-bit samples into one 32-bit word."]
    #[inline(always)]
    pub fn double16(self) -> &'a mut crate::W<REG> {
        self.variant(Dataformat::Double16)
    }
    #[doc = "Right aligned 24bit, left bits are sign extended."]
    #[inline(always)]
    pub fn right24(self) -> &'a mut crate::W<REG> {
        self.variant(Dataformat::Right24)
    }
    #[doc = "32 bit data."]
    #[inline(always)]
    pub fn full32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dataformat::Full32bit)
    }
    #[doc = "Left aligned 16-bit, right bits are zeros."]
    #[inline(always)]
    pub fn left16(self) -> &'a mut crate::W<REG> {
        self.variant(Dataformat::Left16)
    }
    #[doc = "Left aligned 24-bit, right bits are zeros."]
    #[inline(always)]
    pub fn left24(self) -> &'a mut crate::W<REG> {
        self.variant(Dataformat::Left24)
    }
    #[doc = "RAW 32 bit data from Integrator."]
    #[inline(always)]
    pub fn raw32bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dataformat::Raw32bit)
    }
}
#[doc = "Data Valid level in FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fifodvl {
    #[doc = "0: Atleast one word."]
    One = 0,
    #[doc = "1: Two words."]
    Two = 1,
    #[doc = "2: Three words."]
    Three = 2,
    #[doc = "3: Four words."]
    Four = 3,
}
impl From<Fifodvl> for u8 {
    #[inline(always)]
    fn from(variant: Fifodvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fifodvl {
    type Ux = u8;
}
impl crate::IsEnum for Fifodvl {}
#[doc = "Field `FIFODVL` reader - Data Valid level in FIFO"]
pub type FifodvlR = crate::FieldReader<Fifodvl>;
impl FifodvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifodvl {
        match self.bits {
            0 => Fifodvl::One,
            1 => Fifodvl::Two,
            2 => Fifodvl::Three,
            3 => Fifodvl::Four,
            _ => unreachable!(),
        }
    }
    #[doc = "Atleast one word."]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Fifodvl::One
    }
    #[doc = "Two words."]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Fifodvl::Two
    }
    #[doc = "Three words."]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Fifodvl::Three
    }
    #[doc = "Four words."]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Fifodvl::Four
    }
}
#[doc = "Field `FIFODVL` writer - Data Valid level in FIFO"]
pub type FifodvlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fifodvl, crate::Safe>;
impl<'a, REG> FifodvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Atleast one word."]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Fifodvl::One)
    }
    #[doc = "Two words."]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Fifodvl::Two)
    }
    #[doc = "Three words."]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Fifodvl::Three)
    }
    #[doc = "Four words."]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Fifodvl::Four)
    }
}
#[doc = "Stereo mode CH01\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stereomodech01 {
    #[doc = "0: No Stereo mode."]
    Disable = 0,
    #[doc = "1: CH0 and CH1 in Stereo mode."]
    Ch01enable = 1,
}
impl From<Stereomodech01> for bool {
    #[inline(always)]
    fn from(variant: Stereomodech01) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STEREOMODECH01` reader - Stereo mode CH01"]
pub type Stereomodech01R = crate::BitReader<Stereomodech01>;
impl Stereomodech01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stereomodech01 {
        match self.bits {
            false => Stereomodech01::Disable,
            true => Stereomodech01::Ch01enable,
        }
    }
    #[doc = "No Stereo mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stereomodech01::Disable
    }
    #[doc = "CH0 and CH1 in Stereo mode."]
    #[inline(always)]
    pub fn is_ch01enable(&self) -> bool {
        *self == Stereomodech01::Ch01enable
    }
}
#[doc = "Field `STEREOMODECH01` writer - Stereo mode CH01"]
pub type Stereomodech01W<'a, REG> = crate::BitWriter<'a, REG, Stereomodech01>;
impl<'a, REG> Stereomodech01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Stereo mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stereomodech01::Disable)
    }
    #[doc = "CH0 and CH1 in Stereo mode."]
    #[inline(always)]
    pub fn ch01enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stereomodech01::Ch01enable)
    }
}
#[doc = "CH0 CLK Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0clkpol {
    #[doc = "0: Input data clocked on rising clock edge."]
    Normal = 0,
    #[doc = "1: Input data clocked on falling clock edge."]
    Invert = 1,
}
impl From<Ch0clkpol> for bool {
    #[inline(always)]
    fn from(variant: Ch0clkpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0CLKPOL` reader - CH0 CLK Polarity"]
pub type Ch0clkpolR = crate::BitReader<Ch0clkpol>;
impl Ch0clkpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0clkpol {
        match self.bits {
            false => Ch0clkpol::Normal,
            true => Ch0clkpol::Invert,
        }
    }
    #[doc = "Input data clocked on rising clock edge."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Ch0clkpol::Normal
    }
    #[doc = "Input data clocked on falling clock edge."]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == Ch0clkpol::Invert
    }
}
#[doc = "Field `CH0CLKPOL` writer - CH0 CLK Polarity"]
pub type Ch0clkpolW<'a, REG> = crate::BitWriter<'a, REG, Ch0clkpol>;
impl<'a, REG> Ch0clkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input data clocked on rising clock edge."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0clkpol::Normal)
    }
    #[doc = "Input data clocked on falling clock edge."]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0clkpol::Invert)
    }
}
#[doc = "CH1 CLK Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1clkpol {
    #[doc = "0: Input data clocked on rising clock edge."]
    Normal = 0,
    #[doc = "1: Input data clocked on falling clock edge."]
    Invert = 1,
}
impl From<Ch1clkpol> for bool {
    #[inline(always)]
    fn from(variant: Ch1clkpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1CLKPOL` reader - CH1 CLK Polarity"]
pub type Ch1clkpolR = crate::BitReader<Ch1clkpol>;
impl Ch1clkpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1clkpol {
        match self.bits {
            false => Ch1clkpol::Normal,
            true => Ch1clkpol::Invert,
        }
    }
    #[doc = "Input data clocked on rising clock edge."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Ch1clkpol::Normal
    }
    #[doc = "Input data clocked on falling clock edge."]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == Ch1clkpol::Invert
    }
}
#[doc = "Field `CH1CLKPOL` writer - CH1 CLK Polarity"]
pub type Ch1clkpolW<'a, REG> = crate::BitWriter<'a, REG, Ch1clkpol>;
impl<'a, REG> Ch1clkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input data clocked on rising clock edge."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1clkpol::Normal)
    }
    #[doc = "Input data clocked on falling clock edge."]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1clkpol::Invert)
    }
}
impl R {
    #[doc = "Bits 0:1 - Filter order"]
    #[inline(always)]
    pub fn forder(&self) -> ForderR {
        ForderR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Number of Channels"]
    #[inline(always)]
    pub fn numch(&self) -> NumchR {
        NumchR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Filter output format"]
    #[inline(always)]
    pub fn dataformat(&self) -> DataformatR {
        DataformatR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Data Valid level in FIFO"]
    #[inline(always)]
    pub fn fifodvl(&self) -> FifodvlR {
        FifodvlR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Stereo mode CH01"]
    #[inline(always)]
    pub fn stereomodech01(&self) -> Stereomodech01R {
        Stereomodech01R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - CH0 CLK Polarity"]
    #[inline(always)]
    pub fn ch0clkpol(&self) -> Ch0clkpolR {
        Ch0clkpolR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CH1 CLK Polarity"]
    #[inline(always)]
    pub fn ch1clkpol(&self) -> Ch1clkpolR {
        Ch1clkpolR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Filter order"]
    #[inline(always)]
    pub fn forder(&mut self) -> ForderW<'_, Cfg0Spec> {
        ForderW::new(self, 0)
    }
    #[doc = "Bit 4 - Number of Channels"]
    #[inline(always)]
    pub fn numch(&mut self) -> NumchW<'_, Cfg0Spec> {
        NumchW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Filter output format"]
    #[inline(always)]
    pub fn dataformat(&mut self) -> DataformatW<'_, Cfg0Spec> {
        DataformatW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Data Valid level in FIFO"]
    #[inline(always)]
    pub fn fifodvl(&mut self) -> FifodvlW<'_, Cfg0Spec> {
        FifodvlW::new(self, 12)
    }
    #[doc = "Bit 16 - Stereo mode CH01"]
    #[inline(always)]
    pub fn stereomodech01(&mut self) -> Stereomodech01W<'_, Cfg0Spec> {
        Stereomodech01W::new(self, 16)
    }
    #[doc = "Bit 24 - CH0 CLK Polarity"]
    #[inline(always)]
    pub fn ch0clkpol(&mut self) -> Ch0clkpolW<'_, Cfg0Spec> {
        Ch0clkpolW::new(self, 24)
    }
    #[doc = "Bit 25 - CH1 CLK Polarity"]
    #[inline(always)]
    pub fn ch1clkpol(&mut self) -> Ch1clkpolW<'_, Cfg0Spec> {
        Ch1clkpolW::new(self, 25)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for Cfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for Cfg0Spec {}
