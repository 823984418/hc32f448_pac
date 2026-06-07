#[doc = "Register `SRAMB_ECCERRADDR` reader"]
pub type R = crate::R<SrambEccerraddrSpec>;
#[doc = "Field `ECCERRADDR` reader - desc ECCERRADDR"]
pub type EccerraddrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - desc ECCERRADDR"]
    #[inline(always)]
    pub fn eccerraddr(&self) -> EccerraddrR {
        EccerraddrR::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAMB_ECCERRADDR")
            .field("eccerraddr", &self.eccerraddr())
            .finish()
    }
}
#[doc = "desc SRAMB_ECCERRADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`sramb_eccerraddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrambEccerraddrSpec;
impl crate::RegisterSpec for SrambEccerraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramb_eccerraddr::R`](R) reader structure"]
impl crate::Readable for SrambEccerraddrSpec {}
#[doc = "`reset()` method sets SRAMB_ECCERRADDR to value 0"]
impl crate::Resettable for SrambEccerraddrSpec {}
