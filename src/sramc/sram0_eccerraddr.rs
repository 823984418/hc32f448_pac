#[doc = "Register `SRAM0_ECCERRADDR` reader"]
pub type R = crate::R<Sram0EccerraddrSpec>;
#[doc = "Field `ECCERRADDR` reader - desc ECCERRADDR"]
pub type EccerraddrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - desc ECCERRADDR"]
    #[inline(always)]
    pub fn eccerraddr(&self) -> EccerraddrR {
        EccerraddrR::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM0_ECCERRADDR")
            .field("eccerraddr", &self.eccerraddr())
            .finish()
    }
}
#[doc = "desc SRAM0_ECCERRADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`sram0_eccerraddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sram0EccerraddrSpec;
impl crate::RegisterSpec for Sram0EccerraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram0_eccerraddr::R`](R) reader structure"]
impl crate::Readable for Sram0EccerraddrSpec {}
#[doc = "`reset()` method sets SRAM0_ECCERRADDR to value 0"]
impl crate::Resettable for Sram0EccerraddrSpec {}
