#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `OFSVAL` reader - desc OFSVAL"]
pub type OfsvalR = crate::FieldReader;
#[doc = "Field `OFSVAL` writer - desc OFSVAL"]
pub type OfsvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RLDVAL` reader - desc RLDVAL"]
pub type RldvalR = crate::FieldReader<u16>;
#[doc = "Field `RLDVAL` writer - desc RLDVAL"]
pub type RldvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - desc OFSVAL"]
    #[inline(always)]
    pub fn ofsval(&self) -> OfsvalR {
        OfsvalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - desc RLDVAL"]
    #[inline(always)]
    pub fn rldval(&self) -> RldvalR {
        RldvalR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("ofsval", &self.ofsval())
            .field("rldval", &self.rldval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - desc OFSVAL"]
    #[inline(always)]
    pub fn ofsval(&mut self) -> OfsvalW<'_, Cr2Spec> {
        OfsvalW::new(self, 0)
    }
    #[doc = "Bits 16:31 - desc RLDVAL"]
    #[inline(always)]
    pub fn rldval(&mut self) -> RldvalW<'_, Cr2Spec> {
        RldvalW::new(self, 16)
    }
}
#[doc = "desc CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
