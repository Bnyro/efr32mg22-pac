#[doc = "Register `CLKEN1` reader"]
pub type R = crate::R<Clken1Spec>;
#[doc = "Register `CLKEN1` writer"]
pub type W = crate::W<Clken1Spec>;
#[doc = "Field `AGC` reader - Enable Bus Clock"]
pub type AgcR = crate::BitReader;
#[doc = "Field `AGC` writer - Enable Bus Clock"]
pub type AgcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM` reader - Enable Bus Clock"]
pub type ModemR = crate::BitReader;
#[doc = "Field `MODEM` writer - Enable Bus Clock"]
pub type ModemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFCRC` reader - Enable Bus Clock"]
pub type RfcrcR = crate::BitReader;
#[doc = "Field `RFCRC` writer - Enable Bus Clock"]
pub type RfcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRC` reader - Enable Bus Clock"]
pub type FrcR = crate::BitReader;
#[doc = "Field `FRC` writer - Enable Bus Clock"]
pub type FrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROTIMER` reader - Enable Bus Clock"]
pub type ProtimerR = crate::BitReader;
#[doc = "Field `PROTIMER` writer - Enable Bus Clock"]
pub type ProtimerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAC` reader - Enable Bus Clock"]
pub type RacR = crate::BitReader;
#[doc = "Field `RAC` writer - Enable Bus Clock"]
pub type RacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNTH` reader - Enable Bus Clock"]
pub type SynthR = crate::BitReader;
#[doc = "Field `SYNTH` writer - Enable Bus Clock"]
pub type SynthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDSCRATCHPAD` reader - Enable Bus Clock"]
pub type RdscratchpadR = crate::BitReader;
#[doc = "Field `RDSCRATCHPAD` writer - Enable Bus Clock"]
pub type RdscratchpadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDMAILBOX0` reader - Enable Bus Clock"]
pub type Rdmailbox0R = crate::BitReader;
#[doc = "Field `RDMAILBOX0` writer - Enable Bus Clock"]
pub type Rdmailbox0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDMAILBOX1` reader - Enable Bus Clock"]
pub type Rdmailbox1R = crate::BitReader;
#[doc = "Field `RDMAILBOX1` writer - Enable Bus Clock"]
pub type Rdmailbox1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRORTC` reader - Enable Bus Clock"]
pub type PrortcR = crate::BitReader;
#[doc = "Field `PRORTC` writer - Enable Bus Clock"]
pub type PrortcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFC` reader - Enable Bus Clock"]
pub type BufcR = crate::BitReader;
#[doc = "Field `BUFC` writer - Enable Bus Clock"]
pub type BufcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFADCDEBUG` reader - Enable Bus Clock"]
pub type IfadcdebugR = crate::BitReader;
#[doc = "Field `IFADCDEBUG` writer - Enable Bus Clock"]
pub type IfadcdebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTOACC` reader - Enable Bus Clock"]
pub type CryptoaccR = crate::BitReader;
#[doc = "Field `CRYPTOACC` writer - Enable Bus Clock"]
pub type CryptoaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFSENSE` reader - Enable Bus Clock"]
pub type RfsenseR = crate::BitReader;
#[doc = "Field `RFSENSE` writer - Enable Bus Clock"]
pub type RfsenseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMU` reader - Enable Bus Clock"]
pub type SmuR = crate::BitReader;
#[doc = "Field `SMU` writer - Enable Bus Clock"]
pub type SmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE0` reader - Enable Bus Clock"]
pub type Icache0R = crate::BitReader;
#[doc = "Field `ICACHE0` writer - Enable Bus Clock"]
pub type Icache0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSC` reader - Enable Bus Clock"]
pub type MscR = crate::BitReader;
#[doc = "Field `MSC` writer - Enable Bus Clock"]
pub type MscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4` reader - Enable Bus Clock"]
pub type Timer4R = crate::BitReader;
#[doc = "Field `TIMER4` writer - Enable Bus Clock"]
pub type Timer4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Bus Clock"]
    #[inline(always)]
    pub fn agc(&self) -> AgcR {
        AgcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Bus Clock"]
    #[inline(always)]
    pub fn modem(&self) -> ModemR {
        ModemR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rfcrc(&self) -> RfcrcR {
        RfcrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Bus Clock"]
    #[inline(always)]
    pub fn frc(&self) -> FrcR {
        FrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Bus Clock"]
    #[inline(always)]
    pub fn protimer(&self) -> ProtimerR {
        ProtimerR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rac(&self) -> RacR {
        RacR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Bus Clock"]
    #[inline(always)]
    pub fn synth(&self) -> SynthR {
        SynthR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rdscratchpad(&self) -> RdscratchpadR {
        RdscratchpadR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rdmailbox0(&self) -> Rdmailbox0R {
        Rdmailbox0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rdmailbox1(&self) -> Rdmailbox1R {
        Rdmailbox1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Bus Clock"]
    #[inline(always)]
    pub fn prortc(&self) -> PrortcR {
        PrortcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Bus Clock"]
    #[inline(always)]
    pub fn bufc(&self) -> BufcR {
        BufcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Bus Clock"]
    #[inline(always)]
    pub fn ifadcdebug(&self) -> IfadcdebugR {
        IfadcdebugR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Bus Clock"]
    #[inline(always)]
    pub fn cryptoacc(&self) -> CryptoaccR {
        CryptoaccR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rfsense(&self) -> RfsenseR {
        RfsenseR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Bus Clock"]
    #[inline(always)]
    pub fn smu(&self) -> SmuR {
        SmuR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Bus Clock"]
    #[inline(always)]
    pub fn icache0(&self) -> Icache0R {
        Icache0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Bus Clock"]
    #[inline(always)]
    pub fn msc(&self) -> MscR {
        MscR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer4(&self) -> Timer4R {
        Timer4R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Bus Clock"]
    #[inline(always)]
    pub fn agc(&mut self) -> AgcW<'_, Clken1Spec> {
        AgcW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Bus Clock"]
    #[inline(always)]
    pub fn modem(&mut self) -> ModemW<'_, Clken1Spec> {
        ModemW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rfcrc(&mut self) -> RfcrcW<'_, Clken1Spec> {
        RfcrcW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Bus Clock"]
    #[inline(always)]
    pub fn frc(&mut self) -> FrcW<'_, Clken1Spec> {
        FrcW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Bus Clock"]
    #[inline(always)]
    pub fn protimer(&mut self) -> ProtimerW<'_, Clken1Spec> {
        ProtimerW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rac(&mut self) -> RacW<'_, Clken1Spec> {
        RacW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Bus Clock"]
    #[inline(always)]
    pub fn synth(&mut self) -> SynthW<'_, Clken1Spec> {
        SynthW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rdscratchpad(&mut self) -> RdscratchpadW<'_, Clken1Spec> {
        RdscratchpadW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rdmailbox0(&mut self) -> Rdmailbox0W<'_, Clken1Spec> {
        Rdmailbox0W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rdmailbox1(&mut self) -> Rdmailbox1W<'_, Clken1Spec> {
        Rdmailbox1W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Bus Clock"]
    #[inline(always)]
    pub fn prortc(&mut self) -> PrortcW<'_, Clken1Spec> {
        PrortcW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Bus Clock"]
    #[inline(always)]
    pub fn bufc(&mut self) -> BufcW<'_, Clken1Spec> {
        BufcW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Bus Clock"]
    #[inline(always)]
    pub fn ifadcdebug(&mut self) -> IfadcdebugW<'_, Clken1Spec> {
        IfadcdebugW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Bus Clock"]
    #[inline(always)]
    pub fn cryptoacc(&mut self) -> CryptoaccW<'_, Clken1Spec> {
        CryptoaccW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Bus Clock"]
    #[inline(always)]
    pub fn rfsense(&mut self) -> RfsenseW<'_, Clken1Spec> {
        RfsenseW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Bus Clock"]
    #[inline(always)]
    pub fn smu(&mut self) -> SmuW<'_, Clken1Spec> {
        SmuW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Bus Clock"]
    #[inline(always)]
    pub fn icache0(&mut self) -> Icache0W<'_, Clken1Spec> {
        Icache0W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Bus Clock"]
    #[inline(always)]
    pub fn msc(&mut self) -> MscW<'_, Clken1Spec> {
        MscW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer4(&mut self) -> Timer4W<'_, Clken1Spec> {
        Timer4W::new(self, 18)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`clken1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clken1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clken1Spec;
impl crate::RegisterSpec for Clken1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clken1::R`](R) reader structure"]
impl crate::Readable for Clken1Spec {}
#[doc = "`write(|w| ..)` method takes [`clken1::W`](W) writer structure"]
impl crate::Writable for Clken1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKEN1 to value 0"]
impl crate::Resettable for Clken1Spec {}
