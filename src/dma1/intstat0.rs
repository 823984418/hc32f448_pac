#[doc = "Register `INTSTAT0` reader"]
pub type R = crate::R<Intstat0Spec>;
#[doc = "Field `TRNERR` reader - desc TRNERR"]
pub type TrnerrR = crate::FieldReader;
#[doc = "Field `REQERR` reader - desc REQERR"]
pub type ReqerrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - desc TRNERR"]
    #[inline(always)]
    pub fn trnerr(&self) -> TrnerrR {
        TrnerrR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - desc REQERR"]
    #[inline(always)]
    pub fn reqerr(&self) -> ReqerrR {
        ReqerrR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT0")
            .field("trnerr", &self.trnerr())
            .field("reqerr", &self.reqerr())
            .finish()
    }
}
#[doc = "desc INTSTAT0\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intstat0Spec;
impl crate::RegisterSpec for Intstat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat0::R`](R) reader structure"]
impl crate::Readable for Intstat0Spec {}
#[doc = "`reset()` method sets INTSTAT0 to value 0"]
impl crate::Resettable for Intstat0Spec {}
