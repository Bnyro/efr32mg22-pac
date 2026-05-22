#[doc = "Register `CC1_CTRL` reader"]
pub type R = crate::R<Cc1CtrlSpec>;
#[doc = "Register `CC1_CTRL` writer"]
pub type W = crate::W<Cc1CtrlSpec>;
#[doc = "CC Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Compare/Capture channel turned off"]
    Off = 0,
    #[doc = "1: Input capture"]
    Inputcapture = 1,
    #[doc = "2: Output compare"]
    Outputcompare = 2,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - CC Channel Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Off),
            1 => Some(Mode::Inputcapture),
            2 => Some(Mode::Outputcompare),
            _ => None,
        }
    }
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Mode::Off
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn is_inputcapture(&self) -> bool {
        *self == Mode::Inputcapture
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn is_outputcompare(&self) -> bool {
        *self == Mode::Outputcompare
    }
}
#[doc = "Field `MODE` writer - CC Channel Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare/Capture channel turned off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Off)
    }
    #[doc = "Input capture"]
    #[inline(always)]
    pub fn inputcapture(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Inputcapture)
    }
    #[doc = "Output compare"]
    #[inline(always)]
    pub fn outputcompare(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Outputcompare)
    }
}
#[doc = "Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmoa {
    #[doc = "0: A single clock cycle pulse is generated on output"]
    Pulse = 0,
    #[doc = "1: Toggle output on compare match"]
    Toggle = 1,
    #[doc = "2: Clear output on compare match"]
    Clear = 2,
    #[doc = "3: Set output on compare match"]
    Set = 3,
}
impl From<Cmoa> for u8 {
    #[inline(always)]
    fn from(variant: Cmoa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmoa {
    type Ux = u8;
}
impl crate::IsEnum for Cmoa {}
#[doc = "Field `CMOA` reader - Compare Match Output Action"]
pub type CmoaR = crate::FieldReader<Cmoa>;
impl CmoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmoa {
        match self.bits {
            0 => Cmoa::Pulse,
            1 => Cmoa::Toggle,
            2 => Cmoa::Clear,
            3 => Cmoa::Set,
            _ => unreachable!(),
        }
    }
    #[doc = "A single clock cycle pulse is generated on output"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == Cmoa::Pulse
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cmoa::Toggle
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cmoa::Clear
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cmoa::Set
    }
}
#[doc = "Field `CMOA` writer - Compare Match Output Action"]
pub type CmoaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmoa, crate::Safe>;
impl<'a, REG> CmoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A single clock cycle pulse is generated on output"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::Pulse)
    }
    #[doc = "Toggle output on compare match"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::Toggle)
    }
    #[doc = "Clear output on compare match"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::Clear)
    }
    #[doc = "Set output on compare match"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmoa::Set)
    }
}
#[doc = "Capture compare channel comparison base.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compbase {
    #[doc = "0: RTCC_CCx_ICVALUE/OCVALUE is compared with CNT register."]
    Cnt = 0,
    #[doc = "1: Least significant bits of RTCC_CCx_ICVALUE/OCVALUE are compared with COMBCNT."]
    Precnt = 1,
}
impl From<Compbase> for bool {
    #[inline(always)]
    fn from(variant: Compbase) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPBASE` reader - Capture compare channel comparison base."]
pub type CompbaseR = crate::BitReader<Compbase>;
impl CompbaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compbase {
        match self.bits {
            false => Compbase::Cnt,
            true => Compbase::Precnt,
        }
    }
    #[doc = "RTCC_CCx_ICVALUE/OCVALUE is compared with CNT register."]
    #[inline(always)]
    pub fn is_cnt(&self) -> bool {
        *self == Compbase::Cnt
    }
    #[doc = "Least significant bits of RTCC_CCx_ICVALUE/OCVALUE are compared with COMBCNT."]
    #[inline(always)]
    pub fn is_precnt(&self) -> bool {
        *self == Compbase::Precnt
    }
}
#[doc = "Field `COMPBASE` writer - Capture compare channel comparison base."]
pub type CompbaseW<'a, REG> = crate::BitWriter<'a, REG, Compbase>;
impl<'a, REG> CompbaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTCC_CCx_ICVALUE/OCVALUE is compared with CNT register."]
    #[inline(always)]
    pub fn cnt(self) -> &'a mut crate::W<REG> {
        self.variant(Compbase::Cnt)
    }
    #[doc = "Least significant bits of RTCC_CCx_ICVALUE/OCVALUE are compared with COMBCNT."]
    #[inline(always)]
    pub fn precnt(self) -> &'a mut crate::W<REG> {
        self.variant(Compbase::Precnt)
    }
}
#[doc = "Input Capture Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Icedge {
    #[doc = "0: Rising edges detected"]
    Rising = 0,
    #[doc = "1: Falling edges detected"]
    Falling = 1,
    #[doc = "2: Both edges detected"]
    Both = 2,
    #[doc = "3: No edge detection, signal is left as it is"]
    None = 3,
}
impl From<Icedge> for u8 {
    #[inline(always)]
    fn from(variant: Icedge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Icedge {
    type Ux = u8;
}
impl crate::IsEnum for Icedge {}
#[doc = "Field `ICEDGE` reader - Input Capture Edge Select"]
pub type IcedgeR = crate::FieldReader<Icedge>;
impl IcedgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icedge {
        match self.bits {
            0 => Icedge::Rising,
            1 => Icedge::Falling,
            2 => Icedge::Both,
            3 => Icedge::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Icedge::Rising
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Icedge::Falling
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Icedge::Both
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Icedge::None
    }
}
#[doc = "Field `ICEDGE` writer - Input Capture Edge Select"]
pub type IcedgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Icedge, crate::Safe>;
impl<'a, REG> IcedgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::Rising)
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::Falling)
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::Both)
    }
    #[doc = "No edge detection, signal is left as it is"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Icedge::None)
    }
}
impl R {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&self) -> CmoaR {
        CmoaR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Capture compare channel comparison base."]
    #[inline(always)]
    pub fn compbase(&self) -> CompbaseR {
        CompbaseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&self) -> IcedgeR {
        IcedgeR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC Channel Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Cc1CtrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Compare Match Output Action"]
    #[inline(always)]
    pub fn cmoa(&mut self) -> CmoaW<'_, Cc1CtrlSpec> {
        CmoaW::new(self, 2)
    }
    #[doc = "Bit 4 - Capture compare channel comparison base."]
    #[inline(always)]
    pub fn compbase(&mut self) -> CompbaseW<'_, Cc1CtrlSpec> {
        CompbaseW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Input Capture Edge Select"]
    #[inline(always)]
    pub fn icedge(&mut self) -> IcedgeW<'_, Cc1CtrlSpec> {
        IcedgeW::new(self, 5)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc1CtrlSpec;
impl crate::RegisterSpec for Cc1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_ctrl::R`](R) reader structure"]
impl crate::Readable for Cc1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cc1_ctrl::W`](W) writer structure"]
impl crate::Writable for Cc1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC1_CTRL to value 0"]
impl crate::Resettable for Cc1CtrlSpec {}
