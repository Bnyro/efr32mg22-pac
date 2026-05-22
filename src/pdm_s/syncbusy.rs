#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SYNCBUSY` reader - sync busy"]
pub type SyncbusyR = crate::BitReader;
#[doc = "Field `FIFOFLBUSY` reader - FIFO Flush Sync busy"]
pub type FifoflbusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - sync busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SyncbusyR {
        SyncbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Flush Sync busy"]
    #[inline(always)]
    pub fn fifoflbusy(&self) -> FifoflbusyR {
        FifoflbusyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
