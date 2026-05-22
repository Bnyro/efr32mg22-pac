#[doc = "Register `RNGCTRL` reader"]
pub type R = crate::R<RngctrlSpec>;
#[doc = "Register `RNGCTRL` writer"]
pub type W = crate::W<RngctrlSpec>;
#[doc = "TRNG Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Module disabled"]
    Disabled = 0,
    #[doc = "1: Module enabled"]
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - TRNG Module Enable"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disabled,
            true => Enable::Enabled,
        }
    }
    #[doc = "Module disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enable::Disabled
    }
    #[doc = "Module enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enable::Enabled
    }
}
#[doc = "Field `ENABLE` writer - TRNG Module Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Module disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disabled)
    }
    #[doc = "Module enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enabled)
    }
}
#[doc = "Test Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Testen {
    #[doc = "0: Non-determinsitc random number generation"]
    Noise = 0,
    #[doc = "1: Pseudo-random number generation"]
    Testdata = 1,
}
impl From<Testen> for bool {
    #[inline(always)]
    fn from(variant: Testen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TESTEN` reader - Test Enable"]
pub type TestenR = crate::BitReader<Testen>;
impl TestenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Testen {
        match self.bits {
            false => Testen::Noise,
            true => Testen::Testdata,
        }
    }
    #[doc = "Non-determinsitc random number generation"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == Testen::Noise
    }
    #[doc = "Pseudo-random number generation"]
    #[inline(always)]
    pub fn is_testdata(&self) -> bool {
        *self == Testen::Testdata
    }
}
#[doc = "Field `TESTEN` writer - Test Enable"]
pub type TestenW<'a, REG> = crate::BitWriter<'a, REG, Testen>;
impl<'a, REG> TestenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-determinsitc random number generation"]
    #[inline(always)]
    pub fn noise(self) -> &'a mut crate::W<REG> {
        self.variant(Testen::Noise)
    }
    #[doc = "Pseudo-random number generation"]
    #[inline(always)]
    pub fn testdata(self) -> &'a mut crate::W<REG> {
        self.variant(Testen::Testdata)
    }
}
#[doc = "Conditioning Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Condbypass {
    #[doc = "0: The conditionig function is used"]
    Normal = 0,
    #[doc = "1: The conditioning function is bypassed"]
    Bypass = 1,
}
impl From<Condbypass> for bool {
    #[inline(always)]
    fn from(variant: Condbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONDBYPASS` reader - Conditioning Bypass"]
pub type CondbypassR = crate::BitReader<Condbypass>;
impl CondbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Condbypass {
        match self.bits {
            false => Condbypass::Normal,
            true => Condbypass::Bypass,
        }
    }
    #[doc = "The conditionig function is used"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Condbypass::Normal
    }
    #[doc = "The conditioning function is bypassed"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Condbypass::Bypass
    }
}
#[doc = "Field `CONDBYPASS` writer - Conditioning Bypass"]
pub type CondbypassW<'a, REG> = crate::BitWriter<'a, REG, Condbypass>;
impl<'a, REG> CondbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The conditionig function is used"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Condbypass::Normal)
    }
    #[doc = "The conditioning function is bypassed"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Condbypass::Bypass)
    }
}
#[doc = "Field `REPCOUNTIEN` reader - IRQ enable for Repetition Count Test"]
pub type RepcountienR = crate::BitReader;
#[doc = "Field `REPCOUNTIEN` writer - IRQ enable for Repetition Count Test"]
pub type RepcountienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APT64IEN` reader - IRQ enable for APT64IF"]
pub type Apt64ienR = crate::BitReader;
#[doc = "Field `APT64IEN` writer - IRQ enable for APT64IF"]
pub type Apt64ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APT4096IEN` reader - IRQ enable for APT4096IF"]
pub type Apt4096ienR = crate::BitReader;
#[doc = "Field `APT4096IEN` writer - IRQ enable for APT4096IF"]
pub type Apt4096ienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULLIEN` reader - IRQ enable for FIFO full"]
pub type FullienR = crate::BitReader;
#[doc = "Field `FULLIEN` writer - IRQ enable for FIFO full"]
pub type FullienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Softreset {
    #[doc = "0: Module not in reset"]
    Normal = 0,
    #[doc = "1: The continuous test, the conditioning function and the FIFO are reset"]
    Reset = 1,
}
impl From<Softreset> for bool {
    #[inline(always)]
    fn from(variant: Softreset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTRESET` reader - Software Reset"]
pub type SoftresetR = crate::BitReader<Softreset>;
impl SoftresetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Softreset {
        match self.bits {
            false => Softreset::Normal,
            true => Softreset::Reset,
        }
    }
    #[doc = "Module not in reset"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Softreset::Normal
    }
    #[doc = "The continuous test, the conditioning function and the FIFO are reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Softreset::Reset
    }
}
#[doc = "Field `SOFTRESET` writer - Software Reset"]
pub type SoftresetW<'a, REG> = crate::BitWriter<'a, REG, Softreset>;
impl<'a, REG> SoftresetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Module not in reset"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Softreset::Normal)
    }
    #[doc = "The continuous test, the conditioning function and the FIFO are reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Softreset::Reset)
    }
}
#[doc = "Field `PREIEN` reader - IRQ enable for AIS31 prelim. noise alarm"]
pub type PreienR = crate::BitReader;
#[doc = "Field `PREIEN` writer - IRQ enable for AIS31 prelim. noise alarm"]
pub type PreienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMIEN` reader - IRQ enable for AIS31 noise alarm"]
pub type AlmienR = crate::BitReader;
#[doc = "Field `ALMIEN` writer - IRQ enable for AIS31 noise alarm"]
pub type AlmienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Oscillator Force Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forcerun {
    #[doc = "0: Oscillators will shut down when FIFO is full"]
    Normal = 0,
    #[doc = "1: Oscillators will continue to run even after FIFO is full"]
    Run = 1,
}
impl From<Forcerun> for bool {
    #[inline(always)]
    fn from(variant: Forcerun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCERUN` reader - Oscillator Force Run"]
pub type ForcerunR = crate::BitReader<Forcerun>;
impl ForcerunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcerun {
        match self.bits {
            false => Forcerun::Normal,
            true => Forcerun::Run,
        }
    }
    #[doc = "Oscillators will shut down when FIFO is full"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Forcerun::Normal
    }
    #[doc = "Oscillators will continue to run even after FIFO is full"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Forcerun::Run
    }
}
#[doc = "Field `FORCERUN` writer - Oscillator Force Run"]
pub type ForcerunW<'a, REG> = crate::BitWriter<'a, REG, Forcerun>;
impl<'a, REG> ForcerunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oscillators will shut down when FIFO is full"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Forcerun::Normal)
    }
    #[doc = "Oscillators will continue to run even after FIFO is full"]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Forcerun::Run)
    }
}
#[doc = "NIST Start-up Test Bypass.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypnist {
    #[doc = "0: NIST-800-90B startup test is applied. No data will be written to the FIFO until the test passes."]
    Normal = 0,
    #[doc = "1: NIST-800-90B startup test is bypassed."]
    Bypass = 1,
}
impl From<Bypnist> for bool {
    #[inline(always)]
    fn from(variant: Bypnist) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPNIST` reader - NIST Start-up Test Bypass."]
pub type BypnistR = crate::BitReader<Bypnist>;
impl BypnistR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypnist {
        match self.bits {
            false => Bypnist::Normal,
            true => Bypnist::Bypass,
        }
    }
    #[doc = "NIST-800-90B startup test is applied. No data will be written to the FIFO until the test passes."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Bypnist::Normal
    }
    #[doc = "NIST-800-90B startup test is bypassed."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Bypnist::Bypass
    }
}
#[doc = "Field `BYPNIST` writer - NIST Start-up Test Bypass."]
pub type BypnistW<'a, REG> = crate::BitWriter<'a, REG, Bypnist>;
impl<'a, REG> BypnistW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NIST-800-90B startup test is applied. No data will be written to the FIFO until the test passes."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Bypnist::Normal)
    }
    #[doc = "NIST-800-90B startup test is bypassed."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Bypnist::Bypass)
    }
}
#[doc = "AIS31 Start-up Test Bypass.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypais31 {
    #[doc = "0: AIS31 startup test is applied. No data will be written to the FIFO until the test passes."]
    Normal = 0,
    #[doc = "1: AIS31 startup test is bypassed."]
    Bypass = 1,
}
impl From<Bypais31> for bool {
    #[inline(always)]
    fn from(variant: Bypais31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPAIS31` reader - AIS31 Start-up Test Bypass."]
pub type Bypais31R = crate::BitReader<Bypais31>;
impl Bypais31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypais31 {
        match self.bits {
            false => Bypais31::Normal,
            true => Bypais31::Bypass,
        }
    }
    #[doc = "AIS31 startup test is applied. No data will be written to the FIFO until the test passes."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Bypais31::Normal
    }
    #[doc = "AIS31 startup test is bypassed."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Bypais31::Bypass
    }
}
#[doc = "Field `BYPAIS31` writer - AIS31 Start-up Test Bypass."]
pub type Bypais31W<'a, REG> = crate::BitWriter<'a, REG, Bypais31>;
impl<'a, REG> Bypais31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AIS31 startup test is applied. No data will be written to the FIFO until the test passes."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Bypais31::Normal)
    }
    #[doc = "AIS31 startup test is bypassed."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Bypais31::Bypass)
    }
}
#[doc = "Health test input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Healthtestsel {
    #[doc = "0: Before conditioning"]
    Before = 0,
    #[doc = "1: After conditioning"]
    After = 1,
}
impl From<Healthtestsel> for bool {
    #[inline(always)]
    fn from(variant: Healthtestsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEALTHTESTSEL` reader - Health test input select"]
pub type HealthtestselR = crate::BitReader<Healthtestsel>;
impl HealthtestselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Healthtestsel {
        match self.bits {
            false => Healthtestsel::Before,
            true => Healthtestsel::After,
        }
    }
    #[doc = "Before conditioning"]
    #[inline(always)]
    pub fn is_before(&self) -> bool {
        *self == Healthtestsel::Before
    }
    #[doc = "After conditioning"]
    #[inline(always)]
    pub fn is_after(&self) -> bool {
        *self == Healthtestsel::After
    }
}
#[doc = "Field `HEALTHTESTSEL` writer - Health test input select"]
pub type HealthtestselW<'a, REG> = crate::BitWriter<'a, REG, Healthtestsel>;
impl<'a, REG> HealthtestselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Before conditioning"]
    #[inline(always)]
    pub fn before(self) -> &'a mut crate::W<REG> {
        self.variant(Healthtestsel::Before)
    }
    #[doc = "After conditioning"]
    #[inline(always)]
    pub fn after(self) -> &'a mut crate::W<REG> {
        self.variant(Healthtestsel::After)
    }
}
#[doc = "AIS31 test input select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ais31testsel {
    #[doc = "0: Before conditioning"]
    Before = 0,
    #[doc = "1: After conditioning"]
    After = 1,
}
impl From<Ais31testsel> for bool {
    #[inline(always)]
    fn from(variant: Ais31testsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIS31TESTSEL` reader - AIS31 test input select"]
pub type Ais31testselR = crate::BitReader<Ais31testsel>;
impl Ais31testselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ais31testsel {
        match self.bits {
            false => Ais31testsel::Before,
            true => Ais31testsel::After,
        }
    }
    #[doc = "Before conditioning"]
    #[inline(always)]
    pub fn is_before(&self) -> bool {
        *self == Ais31testsel::Before
    }
    #[doc = "After conditioning"]
    #[inline(always)]
    pub fn is_after(&self) -> bool {
        *self == Ais31testsel::After
    }
}
#[doc = "Field `AIS31TESTSEL` writer - AIS31 test input select"]
pub type Ais31testselW<'a, REG> = crate::BitWriter<'a, REG, Ais31testsel>;
impl<'a, REG> Ais31testselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Before conditioning"]
    #[inline(always)]
    pub fn before(self) -> &'a mut crate::W<REG> {
        self.variant(Ais31testsel::Before)
    }
    #[doc = "After conditioning"]
    #[inline(always)]
    pub fn after(self) -> &'a mut crate::W<REG> {
        self.variant(Ais31testsel::After)
    }
}
#[doc = "Field `NB128BITBLOCKS` reader - Number of 128b blocks in AES-CBCMAC"]
pub type Nb128bitblocksR = crate::FieldReader;
#[doc = "Field `NB128BITBLOCKS` writer - Number of 128b blocks in AES-CBCMAC"]
pub type Nb128bitblocksW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIFOWRSTARTUP` reader - Fifo Write Start Up"]
pub type FifowrstartupR = crate::BitReader;
#[doc = "Field `FIFOWRSTARTUP` writer - Fifo Write Start Up"]
pub type FifowrstartupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    pub fn testen(&self) -> TestenR {
        TestenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    pub fn condbypass(&self) -> CondbypassR {
        CondbypassR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IRQ enable for Repetition Count Test"]
    #[inline(always)]
    pub fn repcountien(&self) -> RepcountienR {
        RepcountienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRQ enable for APT64IF"]
    #[inline(always)]
    pub fn apt64ien(&self) -> Apt64ienR {
        Apt64ienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ enable for APT4096IF"]
    #[inline(always)]
    pub fn apt4096ien(&self) -> Apt4096ienR {
        Apt4096ienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRQ enable for FIFO full"]
    #[inline(always)]
    pub fn fullien(&self) -> FullienR {
        FullienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IRQ enable for AIS31 prelim. noise alarm"]
    #[inline(always)]
    pub fn preien(&self) -> PreienR {
        PreienR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IRQ enable for AIS31 noise alarm"]
    #[inline(always)]
    pub fn almien(&self) -> AlmienR {
        AlmienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    pub fn forcerun(&self) -> ForcerunR {
        ForcerunR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypnist(&self) -> BypnistR {
        BypnistR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypais31(&self) -> Bypais31R {
        Bypais31R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Health test input select"]
    #[inline(always)]
    pub fn healthtestsel(&self) -> HealthtestselR {
        HealthtestselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AIS31 test input select"]
    #[inline(always)]
    pub fn ais31testsel(&self) -> Ais31testselR {
        Ais31testselR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Number of 128b blocks in AES-CBCMAC"]
    #[inline(always)]
    pub fn nb128bitblocks(&self) -> Nb128bitblocksR {
        Nb128bitblocksR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Fifo Write Start Up"]
    #[inline(always)]
    pub fn fifowrstartup(&self) -> FifowrstartupR {
        FifowrstartupR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, RngctrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    pub fn testen(&mut self) -> TestenW<'_, RngctrlSpec> {
        TestenW::new(self, 2)
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    pub fn condbypass(&mut self) -> CondbypassW<'_, RngctrlSpec> {
        CondbypassW::new(self, 3)
    }
    #[doc = "Bit 4 - IRQ enable for Repetition Count Test"]
    #[inline(always)]
    pub fn repcountien(&mut self) -> RepcountienW<'_, RngctrlSpec> {
        RepcountienW::new(self, 4)
    }
    #[doc = "Bit 5 - IRQ enable for APT64IF"]
    #[inline(always)]
    pub fn apt64ien(&mut self) -> Apt64ienW<'_, RngctrlSpec> {
        Apt64ienW::new(self, 5)
    }
    #[doc = "Bit 6 - IRQ enable for APT4096IF"]
    #[inline(always)]
    pub fn apt4096ien(&mut self) -> Apt4096ienW<'_, RngctrlSpec> {
        Apt4096ienW::new(self, 6)
    }
    #[doc = "Bit 7 - IRQ enable for FIFO full"]
    #[inline(always)]
    pub fn fullien(&mut self) -> FullienW<'_, RngctrlSpec> {
        FullienW::new(self, 7)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    pub fn softreset(&mut self) -> SoftresetW<'_, RngctrlSpec> {
        SoftresetW::new(self, 8)
    }
    #[doc = "Bit 9 - IRQ enable for AIS31 prelim. noise alarm"]
    #[inline(always)]
    pub fn preien(&mut self) -> PreienW<'_, RngctrlSpec> {
        PreienW::new(self, 9)
    }
    #[doc = "Bit 10 - IRQ enable for AIS31 noise alarm"]
    #[inline(always)]
    pub fn almien(&mut self) -> AlmienW<'_, RngctrlSpec> {
        AlmienW::new(self, 10)
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    pub fn forcerun(&mut self) -> ForcerunW<'_, RngctrlSpec> {
        ForcerunW::new(self, 11)
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypnist(&mut self) -> BypnistW<'_, RngctrlSpec> {
        BypnistW::new(self, 12)
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypais31(&mut self) -> Bypais31W<'_, RngctrlSpec> {
        Bypais31W::new(self, 13)
    }
    #[doc = "Bit 14 - Health test input select"]
    #[inline(always)]
    pub fn healthtestsel(&mut self) -> HealthtestselW<'_, RngctrlSpec> {
        HealthtestselW::new(self, 14)
    }
    #[doc = "Bit 15 - AIS31 test input select"]
    #[inline(always)]
    pub fn ais31testsel(&mut self) -> Ais31testselW<'_, RngctrlSpec> {
        Ais31testselW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Number of 128b blocks in AES-CBCMAC"]
    #[inline(always)]
    pub fn nb128bitblocks(&mut self) -> Nb128bitblocksW<'_, RngctrlSpec> {
        Nb128bitblocksW::new(self, 16)
    }
    #[doc = "Bit 20 - Fifo Write Start Up"]
    #[inline(always)]
    pub fn fifowrstartup(&mut self) -> FifowrstartupW<'_, RngctrlSpec> {
        FifowrstartupW::new(self, 20)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rngctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngctrlSpec;
impl crate::RegisterSpec for RngctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngctrl::R`](R) reader structure"]
impl crate::Readable for RngctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rngctrl::W`](W) writer structure"]
impl crate::Writable for RngctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNGCTRL to value 0x0004_0000"]
impl crate::Resettable for RngctrlSpec {
    const RESET_VALUE: u32 = 0x0004_0000;
}
