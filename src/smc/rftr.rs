#[doc = "Register `RFTR` reader"]
pub type R = crate::R<RftrSpec>;
#[doc = "Register `RFTR` writer"]
pub type W = crate::W<RftrSpec>;
#[doc = "Field `REFPRD` reader - desc REFPRD"]
pub type RefprdR = crate::FieldReader;
#[doc = "Field `REFPRD` writer - desc REFPRD"]
pub type RefprdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc REFPRD"]
    #[inline(always)]
    pub fn refprd(&self) -> RefprdR {
        RefprdR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFTR")
            .field("refprd", &self.refprd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - desc REFPRD"]
    #[inline(always)]
    pub fn refprd(&mut self) -> RefprdW<'_, RftrSpec> {
        RefprdW::new(self, 0)
    }
}
#[doc = "desc RFTR\n\nYou can [`read`](crate::Reg::read) this register and get [`rftr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rftr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RftrSpec;
impl crate::RegisterSpec for RftrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rftr::R`](R) reader structure"]
impl crate::Readable for RftrSpec {}
#[doc = "`write(|w| ..)` method takes [`rftr::W`](W) writer structure"]
impl crate::Writable for RftrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFTR to value 0"]
impl crate::Resettable for RftrSpec {}
