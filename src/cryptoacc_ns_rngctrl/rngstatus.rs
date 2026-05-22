#[doc = "Register `RNGSTATUS` reader"]
pub type R = crate::R<RngstatusSpec>;
#[doc = "Register `RNGSTATUS` writer"]
pub type W = crate::W<RngstatusSpec>;
#[doc = "Test Data Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Testdatabusy {
    #[doc = "0: TESTDATA write is finished processing or no test in progress."]
    Idle = 0,
    #[doc = "1: TESTDATA write is still being processed."]
    Busy = 1,
}
impl From<Testdatabusy> for bool {
    #[inline(always)]
    fn from(variant: Testdatabusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TESTDATABUSY` reader - Test Data Busy"]
pub type TestdatabusyR = crate::BitReader<Testdatabusy>;
impl TestdatabusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Testdatabusy {
        match self.bits {
            false => Testdatabusy::Idle,
            true => Testdatabusy::Busy,
        }
    }
    #[doc = "TESTDATA write is finished processing or no test in progress."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Testdatabusy::Idle
    }
    #[doc = "TESTDATA write is still being processed."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Testdatabusy::Busy
    }
}
#[doc = "State of the control FSM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "0: RESET State"]
    Reset = 0,
    #[doc = "1: STARTUP State"]
    Startup = 1,
    #[doc = "2: FIFOFULLON State"]
    Fifofullon = 2,
    #[doc = "3: FIFOFULLOFF State"]
    Fifofulloff = 3,
    #[doc = "4: RUNNING State"]
    Running = 4,
    #[doc = "5: ERROR State"]
    Error = 5,
    #[doc = "6: UNUSED"]
    Unused6 = 6,
    #[doc = "7: UNUSED"]
    Unused7 = 7,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - State of the control FSM"]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> State {
        match self.bits {
            0 => State::Reset,
            1 => State::Startup,
            2 => State::Fifofullon,
            3 => State::Fifofulloff,
            4 => State::Running,
            5 => State::Error,
            6 => State::Unused6,
            7 => State::Unused7,
            _ => unreachable!(),
        }
    }
    #[doc = "RESET State"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == State::Reset
    }
    #[doc = "STARTUP State"]
    #[inline(always)]
    pub fn is_startup(&self) -> bool {
        *self == State::Startup
    }
    #[doc = "FIFOFULLON State"]
    #[inline(always)]
    pub fn is_fifofullon(&self) -> bool {
        *self == State::Fifofullon
    }
    #[doc = "FIFOFULLOFF State"]
    #[inline(always)]
    pub fn is_fifofulloff(&self) -> bool {
        *self == State::Fifofulloff
    }
    #[doc = "RUNNING State"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == State::Running
    }
    #[doc = "ERROR State"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == State::Error
    }
    #[doc = "UNUSED"]
    #[inline(always)]
    pub fn is_unused_6(&self) -> bool {
        *self == State::Unused6
    }
    #[doc = "UNUSED"]
    #[inline(always)]
    pub fn is_unused_7(&self) -> bool {
        *self == State::Unused7
    }
}
#[doc = "Field `REPCOUNTIF` reader - Repetition Count Test interrupt status"]
pub type RepcountifR = crate::BitReader;
#[doc = "Field `APT64IF` reader - 64-sample window Adaptive Proportion IF"]
pub type Apt64ifR = crate::BitReader;
#[doc = "Field `APT4096IF` reader - 4096-sample window Adaptive Prop. IF"]
pub type Apt4096ifR = crate::BitReader;
#[doc = "Field `FULLIF` reader - FIFO full interrupt status"]
pub type FullifR = crate::BitReader;
#[doc = "Field `PREIF` reader - AIS31 Preliminary Noise Alarm IF"]
pub type PreifR = crate::BitReader;
#[doc = "Field `PREIF` writer - AIS31 Preliminary Noise Alarm IF"]
pub type PreifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALMIF` reader - AIS31 Noise Alarm interrupt status"]
pub type AlmifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Test Data Busy"]
    #[inline(always)]
    pub fn testdatabusy(&self) -> TestdatabusyR {
        TestdatabusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - State of the control FSM"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Repetition Count Test interrupt status"]
    #[inline(always)]
    pub fn repcountif(&self) -> RepcountifR {
        RepcountifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 64-sample window Adaptive Proportion IF"]
    #[inline(always)]
    pub fn apt64if(&self) -> Apt64ifR {
        Apt64ifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 4096-sample window Adaptive Prop. IF"]
    #[inline(always)]
    pub fn apt4096if(&self) -> Apt4096ifR {
        Apt4096ifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO full interrupt status"]
    #[inline(always)]
    pub fn fullif(&self) -> FullifR {
        FullifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AIS31 Preliminary Noise Alarm IF"]
    #[inline(always)]
    pub fn preif(&self) -> PreifR {
        PreifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AIS31 Noise Alarm interrupt status"]
    #[inline(always)]
    pub fn almif(&self) -> AlmifR {
        AlmifR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - AIS31 Preliminary Noise Alarm IF"]
    #[inline(always)]
    pub fn preif(&mut self) -> PreifW<'_, RngstatusSpec> {
        PreifW::new(self, 8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rngstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rngstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngstatusSpec;
impl crate::RegisterSpec for RngstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rngstatus::R`](R) reader structure"]
impl crate::Readable for RngstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rngstatus::W`](W) writer structure"]
impl crate::Writable for RngstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNGSTATUS to value 0"]
impl crate::Resettable for RngstatusSpec {}
