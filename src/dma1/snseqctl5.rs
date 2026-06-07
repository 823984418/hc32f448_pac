#[doc = "Register `SNSEQCTL5` reader"]
pub type R = crate::R<Snseqctl5Spec>;
#[doc = "Register `SNSEQCTL5` writer"]
pub type W = crate::W<Snseqctl5Spec>;
#[doc = "Field `SOFFSET` reader - desc SOFFSET"]
pub type SoffsetR = crate::FieldReader<u32>;
#[doc = "Field `SOFFSET` writer - desc SOFFSET"]
pub type SoffsetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `SNSCNT` reader - desc SNSCNT"]
pub type SnscntR = crate::FieldReader<u16>;
#[doc = "Field `SNSCNT` writer - desc SNSCNT"]
pub type SnscntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
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
        f.debug_struct("SNSEQCTL5")
            .field("soffset", &self.soffset())
            .field("snscnt", &self.snscnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - desc SOFFSET"]
    #[inline(always)]
    pub fn soffset(&mut self) -> SoffsetW<'_, Snseqctl5Spec> {
        SoffsetW::new(self, 0)
    }
    #[doc = "Bits 20:31 - desc SNSCNT"]
    #[inline(always)]
    pub fn snscnt(&mut self) -> SnscntW<'_, Snseqctl5Spec> {
        SnscntW::new(self, 20)
    }
}
#[doc = "desc SNSEQCTL5\n\nYou can [`read`](crate::Reg::read) this register and get [`snseqctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snseqctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Snseqctl5Spec;
impl crate::RegisterSpec for Snseqctl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snseqctl5::R`](R) reader structure"]
impl crate::Readable for Snseqctl5Spec {}
#[doc = "`write(|w| ..)` method takes [`snseqctl5::W`](W) writer structure"]
impl crate::Writable for Snseqctl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SNSEQCTL5 to value 0"]
impl crate::Resettable for Snseqctl5Spec {}
