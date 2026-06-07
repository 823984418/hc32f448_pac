#[doc = "Register `MONSNSEQCTL2` reader"]
pub type R = crate::R<Monsnseqctl2Spec>;
#[doc = "Field `SOFFSET` reader - desc SOFFSET"]
pub type SoffsetR = crate::FieldReader<u32>;
#[doc = "Field `SNSCNT` reader - desc SNSCNT"]
pub type SnscntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:19 - desc SOFFSET"]
    #[inline(always)]
    pub fn soffset(&self) -> SoffsetR {
        SoffsetR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - desc SNSCNT"]
    #[inline(always)]
    pub fn snscnt(&self) -> SnscntR {
        SnscntR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MONSNSEQCTL2")
            .field("soffset", &self.soffset())
            .field("snscnt", &self.snscnt())
            .finish()
    }
}
#[doc = "desc MONSNSEQCTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`monsnseqctl2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Monsnseqctl2Spec;
impl crate::RegisterSpec for Monsnseqctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monsnseqctl2::R`](R) reader structure"]
impl crate::Readable for Monsnseqctl2Spec {}
#[doc = "`reset()` method sets MONSNSEQCTL2 to value 0"]
impl crate::Resettable for Monsnseqctl2Spec {}
