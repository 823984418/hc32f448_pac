#[doc = "Register `CHIPID` reader"]
pub type R = crate::R<ChipidSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc CHIPID\n\nYou can [`read`](crate::Reg::read) this register and get [`chipid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChipidSpec;
impl crate::RegisterSpec for ChipidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid::R`](R) reader structure"]
impl crate::Readable for ChipidSpec {}
#[doc = "`reset()` method sets CHIPID to value 0x5848_0448"]
impl crate::Resettable for ChipidSpec {
    const RESET_VALUE: u32 = 0x5848_0448;
}
