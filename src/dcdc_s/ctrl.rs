#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "DCDC/Bypass Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: DCDC is OFF, bypass switch is enabled"]
    Bypass = 0,
    #[doc = "1: Request DCDC regulation, bypass switch disabled"]
    Dcdcregulation = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - DCDC/Bypass Mode Control"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Bypass,
            true => Mode::Dcdcregulation,
        }
    }
    #[doc = "DCDC is OFF, bypass switch is enabled"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Mode::Bypass
    }
    #[doc = "Request DCDC regulation, bypass switch disabled"]
    #[inline(always)]
    pub fn is_dcdcregulation(&self) -> bool {
        *self == Mode::Dcdcregulation
    }
}
#[doc = "Field `MODE` writer - DCDC/Bypass Mode Control"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DCDC is OFF, bypass switch is enabled"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Bypass)
    }
    #[doc = "Request DCDC regulation, bypass switch disabled"]
    #[inline(always)]
    pub fn dcdcregulation(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Dcdcregulation)
    }
}
#[doc = "DCDC DCM Only Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcmonlyen {
    #[doc = "0: Support higher load current at lower battery voltage by working in CCM mode"]
    Dualmode = 0,
    #[doc = "1: DCM only mode for normal operation, this is the default setting"]
    Dcmonlyen = 1,
}
impl From<Dcmonlyen> for bool {
    #[inline(always)]
    fn from(variant: Dcmonlyen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMONLYEN` reader - DCDC DCM Only Enable"]
pub type DcmonlyenR = crate::BitReader<Dcmonlyen>;
impl DcmonlyenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcmonlyen {
        match self.bits {
            false => Dcmonlyen::Dualmode,
            true => Dcmonlyen::Dcmonlyen,
        }
    }
    #[doc = "Support higher load current at lower battery voltage by working in CCM mode"]
    #[inline(always)]
    pub fn is_dualmode(&self) -> bool {
        *self == Dcmonlyen::Dualmode
    }
    #[doc = "DCM only mode for normal operation, this is the default setting"]
    #[inline(always)]
    pub fn is_dcmonlyen(&self) -> bool {
        *self == Dcmonlyen::Dcmonlyen
    }
}
#[doc = "Field `DCMONLYEN` writer - DCDC DCM Only Enable"]
pub type DcmonlyenW<'a, REG> = crate::BitWriter<'a, REG, Dcmonlyen>;
impl<'a, REG> DcmonlyenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Support higher load current at lower battery voltage by working in CCM mode"]
    #[inline(always)]
    pub fn dualmode(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmonlyen::Dualmode)
    }
    #[doc = "DCM only mode for normal operation, this is the default setting"]
    #[inline(always)]
    pub fn dcmonlyen(self) -> &'a mut crate::W<REG> {
        self.variant(Dcmonlyen::Dcmonlyen)
    }
}
#[doc = "Peak Current Timeout Control\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipktmaxctrl {
    #[doc = "0: Ton_max disabled"]
    Off = 0,
    #[doc = "1: 0.35us"]
    Tmax0p35us = 1,
    #[doc = "2: 0.63us"]
    Tmax0p63us = 2,
    #[doc = "3: 0.91us"]
    Tmax0p91us = 3,
    #[doc = "4: 1.19us"]
    Tmax1p19us = 4,
    #[doc = "5: 1.47us"]
    Tmax1p47us = 5,
    #[doc = "6: 1.75us"]
    Tmax1p75us = 6,
    #[doc = "7: 2.03us"]
    Tmax2p03us = 7,
}
impl From<Ipktmaxctrl> for u8 {
    #[inline(always)]
    fn from(variant: Ipktmaxctrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ipktmaxctrl {
    type Ux = u8;
}
impl crate::IsEnum for Ipktmaxctrl {}
#[doc = "Field `IPKTMAXCTRL` reader - Peak Current Timeout Control"]
pub type IpktmaxctrlR = crate::FieldReader<Ipktmaxctrl>;
impl IpktmaxctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipktmaxctrl {
        match self.bits {
            0 => Ipktmaxctrl::Off,
            1 => Ipktmaxctrl::Tmax0p35us,
            2 => Ipktmaxctrl::Tmax0p63us,
            3 => Ipktmaxctrl::Tmax0p91us,
            4 => Ipktmaxctrl::Tmax1p19us,
            5 => Ipktmaxctrl::Tmax1p47us,
            6 => Ipktmaxctrl::Tmax1p75us,
            7 => Ipktmaxctrl::Tmax2p03us,
            _ => unreachable!(),
        }
    }
    #[doc = "Ton_max disabled"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ipktmaxctrl::Off
    }
    #[doc = "0.35us"]
    #[inline(always)]
    pub fn is_tmax_0p35us(&self) -> bool {
        *self == Ipktmaxctrl::Tmax0p35us
    }
    #[doc = "0.63us"]
    #[inline(always)]
    pub fn is_tmax_0p63us(&self) -> bool {
        *self == Ipktmaxctrl::Tmax0p63us
    }
    #[doc = "0.91us"]
    #[inline(always)]
    pub fn is_tmax_0p91us(&self) -> bool {
        *self == Ipktmaxctrl::Tmax0p91us
    }
    #[doc = "1.19us"]
    #[inline(always)]
    pub fn is_tmax_1p19us(&self) -> bool {
        *self == Ipktmaxctrl::Tmax1p19us
    }
    #[doc = "1.47us"]
    #[inline(always)]
    pub fn is_tmax_1p47us(&self) -> bool {
        *self == Ipktmaxctrl::Tmax1p47us
    }
    #[doc = "1.75us"]
    #[inline(always)]
    pub fn is_tmax_1p75us(&self) -> bool {
        *self == Ipktmaxctrl::Tmax1p75us
    }
    #[doc = "2.03us"]
    #[inline(always)]
    pub fn is_tmax_2p03us(&self) -> bool {
        *self == Ipktmaxctrl::Tmax2p03us
    }
}
#[doc = "Field `IPKTMAXCTRL` writer - Peak Current Timeout Control"]
pub type IpktmaxctrlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ipktmaxctrl, crate::Safe>;
impl<'a, REG> IpktmaxctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ton_max disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Ipktmaxctrl::Off)
    }
    #[doc = "0.35us"]
    #[inline(always)]
    pub fn tmax_0p35us(self) -> &'a mut crate::W<REG> {
        self.variant(Ipktmaxctrl::Tmax0p35us)
    }
    #[doc = "0.63us"]
    #[inline(always)]
    pub fn tmax_0p63us(self) -> &'a mut crate::W<REG> {
        self.variant(Ipktmaxctrl::Tmax0p63us)
    }
    #[doc = "0.91us"]
    #[inline(always)]
    pub fn tmax_0p91us(self) -> &'a mut crate::W<REG> {
        self.variant(Ipktmaxctrl::Tmax0p91us)
    }
    #[doc = "1.19us"]
    #[inline(always)]
    pub fn tmax_1p19us(self) -> &'a mut crate::W<REG> {
        self.variant(Ipktmaxctrl::Tmax1p19us)
    }
    #[doc = "1.47us"]
    #[inline(always)]
    pub fn tmax_1p47us(self) -> &'a mut crate::W<REG> {
        self.variant(Ipktmaxctrl::Tmax1p47us)
    }
    #[doc = "1.75us"]
    #[inline(always)]
    pub fn tmax_1p75us(self) -> &'a mut crate::W<REG> {
        self.variant(Ipktmaxctrl::Tmax1p75us)
    }
    #[doc = "2.03us"]
    #[inline(always)]
    pub fn tmax_2p03us(self) -> &'a mut crate::W<REG> {
        self.variant(Ipktmaxctrl::Tmax2p03us)
    }
}
impl R {
    #[doc = "Bit 0 - DCDC/Bypass Mode Control"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - DCDC DCM Only Enable"]
    #[inline(always)]
    pub fn dcmonlyen(&self) -> DcmonlyenR {
        DcmonlyenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Peak Current Timeout Control"]
    #[inline(always)]
    pub fn ipktmaxctrl(&self) -> IpktmaxctrlR {
        IpktmaxctrlR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DCDC/Bypass Mode Control"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CtrlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 2 - DCDC DCM Only Enable"]
    #[inline(always)]
    pub fn dcmonlyen(&mut self) -> DcmonlyenW<'_, CtrlSpec> {
        DcmonlyenW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Peak Current Timeout Control"]
    #[inline(always)]
    pub fn ipktmaxctrl(&mut self) -> IpktmaxctrlW<'_, CtrlSpec> {
        IpktmaxctrlW::new(self, 4)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x44"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x44;
}
