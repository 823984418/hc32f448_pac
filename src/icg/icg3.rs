#[doc = "Register `ICG3` reader"]
pub type R = crate::R<Icg3Spec>;
#[doc = "Field `DBUSPRT` reader - desc DBUSPRT"]
pub type DbusprtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - desc DBUSPRT"]
    #[inline(always)]
    pub fn dbusprt(&self) -> DbusprtR {
        DbusprtR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICG3")
            .field("dbusprt", &self.dbusprt())
            .finish()
    }
}
#[doc = "desc ICG3\n\nYou can [`read`](crate::Reg::read) this register and get [`icg3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icg3Spec;
impl crate::RegisterSpec for Icg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icg3::R`](R) reader structure"]
impl crate::Readable for Icg3Spec {}
#[doc = "`reset()` method sets ICG3 to value 0xffff_ffff"]
impl crate::Resettable for Icg3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
