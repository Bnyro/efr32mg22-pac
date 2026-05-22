#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<CommandSpec>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<CommandSpec>;
#[doc = "Field `OPERATION` reader - Type of Operation"]
pub type OperationR = crate::FieldReader;
#[doc = "Field `OPERATION` writer - Type of Operation"]
pub type OperationW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field {
    #[doc = "0: Field is GF(p)"]
    Gfp = 0,
    #[doc = "1: Field is GF(2^m)"]
    Gf2m = 1,
}
impl From<Field> for bool {
    #[inline(always)]
    fn from(variant: Field) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELD` reader - Field"]
pub type FieldR = crate::BitReader<Field>;
impl FieldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Field {
        match self.bits {
            false => Field::Gfp,
            true => Field::Gf2m,
        }
    }
    #[doc = "Field is GF(p)"]
    #[inline(always)]
    pub fn is_gfp(&self) -> bool {
        *self == Field::Gfp
    }
    #[doc = "Field is GF(2^m)"]
    #[inline(always)]
    pub fn is_gf2m(&self) -> bool {
        *self == Field::Gf2m
    }
}
#[doc = "Field `FIELD` writer - Field"]
pub type FieldW<'a, REG> = crate::BitWriter<'a, REG, Field>;
impl<'a, REG> FieldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Field is GF(p)"]
    #[inline(always)]
    pub fn gfp(self) -> &'a mut crate::W<REG> {
        self.variant(Field::Gfp)
    }
    #[doc = "Field is GF(2^m)"]
    #[inline(always)]
    pub fn gf2m(self) -> &'a mut crate::W<REG> {
        self.variant(Field::Gf2m)
    }
}
#[doc = "Field `SIZE` reader - Size of Operands in data memory"]
pub type SizeR = crate::FieldReader<u16>;
#[doc = "Field `SIZE` writer - Size of Operands in data memory"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Select Curve\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selcurve {
    #[doc = "0: No acceleration"]
    None = 0,
    #[doc = "1: P256"]
    P256 = 1,
    #[doc = "4: P192"]
    P192 = 4,
}
impl From<Selcurve> for u8 {
    #[inline(always)]
    fn from(variant: Selcurve) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selcurve {
    type Ux = u8;
}
impl crate::IsEnum for Selcurve {}
#[doc = "Field `SELCURVE` reader - Select Curve"]
pub type SelcurveR = crate::FieldReader<Selcurve>;
impl SelcurveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selcurve> {
        match self.bits {
            0 => Some(Selcurve::None),
            1 => Some(Selcurve::P256),
            4 => Some(Selcurve::P192),
            _ => None,
        }
    }
    #[doc = "No acceleration"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Selcurve::None
    }
    #[doc = "P256"]
    #[inline(always)]
    pub fn is_p256(&self) -> bool {
        *self == Selcurve::P256
    }
    #[doc = "P192"]
    #[inline(always)]
    pub fn is_p192(&self) -> bool {
        *self == Selcurve::P192
    }
}
#[doc = "Field `SELCURVE` writer - Select Curve"]
pub type SelcurveW<'a, REG> = crate::FieldWriter<'a, REG, 3, Selcurve>;
impl<'a, REG> SelcurveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No acceleration"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Selcurve::None)
    }
    #[doc = "P256"]
    #[inline(always)]
    pub fn p256(self) -> &'a mut crate::W<REG> {
        self.variant(Selcurve::P256)
    }
    #[doc = "P192"]
    #[inline(always)]
    pub fn p192(self) -> &'a mut crate::W<REG> {
        self.variant(Selcurve::P192)
    }
}
#[doc = "Field `EDWARDS` reader - Edwards Curve Enable"]
pub type EdwardsR = crate::BitReader;
#[doc = "Field `EDWARDS` writer - Edwards Curve Enable"]
pub type EdwardsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Buffer Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufsel {
    #[doc = "0: use data in data memory 0"]
    Mem0 = 0,
}
impl From<Bufsel> for bool {
    #[inline(always)]
    fn from(variant: Bufsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFSEL` reader - Buffer Select"]
pub type BufselR = crate::BitReader<Bufsel>;
impl BufselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bufsel> {
        match self.bits {
            false => Some(Bufsel::Mem0),
            _ => None,
        }
    }
    #[doc = "use data in data memory 0"]
    #[inline(always)]
    pub fn is_mem0(&self) -> bool {
        *self == Bufsel::Mem0
    }
}
#[doc = "Field `BUFSEL` writer - Buffer Select"]
pub type BufselW<'a, REG> = crate::BitWriter<'a, REG, Bufsel>;
impl<'a, REG> BufselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use data in data memory 0"]
    #[inline(always)]
    pub fn mem0(self) -> &'a mut crate::W<REG> {
        self.variant(Bufsel::Mem0)
    }
}
#[doc = "Swap bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swapbytes {
    #[doc = "0: Native format (little endian)"]
    Native = 0,
    #[doc = "1: Byte swapped (big endian)"]
    Swapped = 1,
}
impl From<Swapbytes> for bool {
    #[inline(always)]
    fn from(variant: Swapbytes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAPBYTES` reader - Swap bytes"]
pub type SwapbytesR = crate::BitReader<Swapbytes>;
impl SwapbytesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swapbytes {
        match self.bits {
            false => Swapbytes::Native,
            true => Swapbytes::Swapped,
        }
    }
    #[doc = "Native format (little endian)"]
    #[inline(always)]
    pub fn is_native(&self) -> bool {
        *self == Swapbytes::Native
    }
    #[doc = "Byte swapped (big endian)"]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == Swapbytes::Swapped
    }
}
#[doc = "Field `SWAPBYTES` writer - Swap bytes"]
pub type SwapbytesW<'a, REG> = crate::BitWriter<'a, REG, Swapbytes>;
impl<'a, REG> SwapbytesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Native format (little endian)"]
    #[inline(always)]
    pub fn native(self) -> &'a mut crate::W<REG> {
        self.variant(Swapbytes::Native)
    }
    #[doc = "Byte swapped (big endian)"]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(Swapbytes::Swapped)
    }
}
#[doc = "Field `FLAGA` reader - Flag A"]
pub type FlagaR = crate::BitReader;
#[doc = "Field `FLAGA` writer - Flag A"]
pub type FlagaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAGB` reader - Flag B"]
pub type FlagbR = crate::BitReader;
#[doc = "Field `FLAGB` writer - Flag B"]
pub type FlagbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Calculate R2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calcr2 {
    #[doc = "0: don't recalculate R² mod N"]
    False = 0,
    #[doc = "1: re-calculate R² mod N"]
    True = 1,
}
impl From<Calcr2> for bool {
    #[inline(always)]
    fn from(variant: Calcr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALCR2` reader - Calculate R2"]
pub type Calcr2R = crate::BitReader<Calcr2>;
impl Calcr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calcr2 {
        match self.bits {
            false => Calcr2::False,
            true => Calcr2::True,
        }
    }
    #[doc = "don't recalculate R² mod N"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == Calcr2::False
    }
    #[doc = "re-calculate R² mod N"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == Calcr2::True
    }
}
#[doc = "Field `CALCR2` writer - Calculate R2"]
pub type Calcr2W<'a, REG> = crate::BitWriter<'a, REG, Calcr2>;
impl<'a, REG> Calcr2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "don't recalculate R² mod N"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut crate::W<REG> {
        self.variant(Calcr2::False)
    }
    #[doc = "re-calculate R² mod N"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut crate::W<REG> {
        self.variant(Calcr2::True)
    }
}
impl R {
    #[doc = "Bits 0:6 - Type of Operation"]
    #[inline(always)]
    pub fn operation(&self) -> OperationR {
        OperationR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Field"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:18 - Size of Operands in data memory"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bits 20:22 - Select Curve"]
    #[inline(always)]
    pub fn selcurve(&self) -> SelcurveR {
        SelcurveR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 26 - Edwards Curve Enable"]
    #[inline(always)]
    pub fn edwards(&self) -> EdwardsR {
        EdwardsR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Buffer Select"]
    #[inline(always)]
    pub fn bufsel(&self) -> BufselR {
        BufselR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Swap bytes"]
    #[inline(always)]
    pub fn swapbytes(&self) -> SwapbytesR {
        SwapbytesR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Flag A"]
    #[inline(always)]
    pub fn flaga(&self) -> FlagaR {
        FlagaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Flag B"]
    #[inline(always)]
    pub fn flagb(&self) -> FlagbR {
        FlagbR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Calculate R2"]
    #[inline(always)]
    pub fn calcr2(&self) -> Calcr2R {
        Calcr2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Type of Operation"]
    #[inline(always)]
    pub fn operation(&mut self) -> OperationW<'_, CommandSpec> {
        OperationW::new(self, 0)
    }
    #[doc = "Bit 7 - Field"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<'_, CommandSpec> {
        FieldW::new(self, 7)
    }
    #[doc = "Bits 8:18 - Size of Operands in data memory"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, CommandSpec> {
        SizeW::new(self, 8)
    }
    #[doc = "Bits 20:22 - Select Curve"]
    #[inline(always)]
    pub fn selcurve(&mut self) -> SelcurveW<'_, CommandSpec> {
        SelcurveW::new(self, 20)
    }
    #[doc = "Bit 26 - Edwards Curve Enable"]
    #[inline(always)]
    pub fn edwards(&mut self) -> EdwardsW<'_, CommandSpec> {
        EdwardsW::new(self, 26)
    }
    #[doc = "Bit 27 - Buffer Select"]
    #[inline(always)]
    pub fn bufsel(&mut self) -> BufselW<'_, CommandSpec> {
        BufselW::new(self, 27)
    }
    #[doc = "Bit 28 - Swap bytes"]
    #[inline(always)]
    pub fn swapbytes(&mut self) -> SwapbytesW<'_, CommandSpec> {
        SwapbytesW::new(self, 28)
    }
    #[doc = "Bit 29 - Flag A"]
    #[inline(always)]
    pub fn flaga(&mut self) -> FlagaW<'_, CommandSpec> {
        FlagaW::new(self, 29)
    }
    #[doc = "Bit 30 - Flag B"]
    #[inline(always)]
    pub fn flagb(&mut self) -> FlagbW<'_, CommandSpec> {
        FlagbW::new(self, 30)
    }
    #[doc = "Bit 31 - Calculate R2"]
    #[inline(always)]
    pub fn calcr2(&mut self) -> Calcr2W<'_, CommandSpec> {
        Calcr2W::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandSpec;
impl crate::RegisterSpec for CommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for CommandSpec {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for CommandSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for CommandSpec {}
