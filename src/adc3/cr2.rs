#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `OVSS` reader - desc OVSS"]
pub type OvssR = crate::FieldReader;
#[doc = "Field `OVSS` writer - desc OVSS"]
pub type OvssW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OVSMOD` reader - desc OVSMOD"]
pub type OvsmodR = crate::BitReader;
#[doc = "Field `OVSMOD` writer - desc OVSMOD"]
pub type OvsmodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:11 - desc OVSS"]
    #[inline(always)]
    pub fn ovss(&self) -> OvssR {
        OvssR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - desc OVSMOD"]
    #[inline(always)]
    pub fn ovsmod(&self) -> OvsmodR {
        OvsmodR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("ovss", &self.ovss())
            .field("ovsmod", &self.ovsmod())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:11 - desc OVSS"]
    #[inline(always)]
    pub fn ovss(&mut self) -> OvssW<'_, Cr2Spec> {
        OvssW::new(self, 8)
    }
    #[doc = "Bit 12 - desc OVSMOD"]
    #[inline(always)]
    pub fn ovsmod(&mut self) -> OvsmodW<'_, Cr2Spec> {
        OvsmodW::new(self, 12)
    }
}
#[doc = "desc CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
