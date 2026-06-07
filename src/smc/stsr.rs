#[doc = "Register `STSR` reader"]
pub type R = crate::R<StsrSpec>;
#[doc = "Field `STATUS` reader - desc STATUS"]
pub type StatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc STATUS"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STSR")
            .field("status", &self.status())
            .finish()
    }
}
#[doc = "desc STSR\n\nYou can [`read`](crate::Reg::read) this register and get [`stsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsrSpec;
impl crate::RegisterSpec for StsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stsr::R`](R) reader structure"]
impl crate::Readable for StsrSpec {}
#[doc = "`reset()` method sets STSR to value 0x01"]
impl crate::Resettable for StsrSpec {
    const RESET_VALUE: u32 = 0x01;
}
