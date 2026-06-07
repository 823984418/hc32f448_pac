#[doc = "Register `PFSRE2` reader"]
pub type R = crate::R<Pfsre2Spec>;
#[doc = "Register `PFSRE2` writer"]
pub type W = crate::W<Pfsre2Spec>;
#[doc = "Field `FSEL` reader - desc FSEL"]
pub type FselR = crate::FieldReader;
#[doc = "Field `FSEL` writer - desc FSEL"]
pub type FselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BFE` reader - desc BFE"]
pub type BfeR = crate::BitReader;
#[doc = "Field `BFE` writer - desc BFE"]
pub type BfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - desc FSEL"]
    #[inline(always)]
    pub fn fsel(&self) -> FselR {
        FselR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - desc BFE"]
    #[inline(always)]
    pub fn bfe(&self) -> BfeR {
        BfeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PFSRE2")
            .field("fsel", &self.fsel())
            .field("bfe", &self.bfe())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - desc FSEL"]
    #[inline(always)]
    pub fn fsel(&mut self) -> FselW<'_, Pfsre2Spec> {
        FselW::new(self, 0)
    }
    #[doc = "Bit 8 - desc BFE"]
    #[inline(always)]
    pub fn bfe(&mut self) -> BfeW<'_, Pfsre2Spec> {
        BfeW::new(self, 8)
    }
}
#[doc = "desc PFSRE2\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsre2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsre2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pfsre2Spec;
impl crate::RegisterSpec for Pfsre2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pfsre2::R`](R) reader structure"]
impl crate::Readable for Pfsre2Spec {}
#[doc = "`write(|w| ..)` method takes [`pfsre2::W`](W) writer structure"]
impl crate::Writable for Pfsre2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFSRE2 to value 0"]
impl crate::Resettable for Pfsre2Spec {}
