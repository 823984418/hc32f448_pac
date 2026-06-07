#[doc = "Register `DRR` reader"]
pub type R = crate::R<DrrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc DRR\n\nYou can [`read`](crate::Reg::read) this register and get [`drr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrrSpec;
impl crate::RegisterSpec for DrrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`drr::R`](R) reader structure"]
impl crate::Readable for DrrSpec {}
#[doc = "`reset()` method sets DRR to value 0"]
impl crate::Resettable for DrrSpec {}
