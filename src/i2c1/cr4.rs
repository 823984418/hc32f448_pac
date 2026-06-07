#[doc = "Register `CR4` reader"]
pub type R = crate::R<Cr4Spec>;
#[doc = "Register `CR4` writer"]
pub type W = crate::W<Cr4Spec>;
#[doc = "Field `BUSWAIT` reader - desc BUSWAIT"]
pub type BuswaitR = crate::BitReader;
#[doc = "Field `BUSWAIT` writer - desc BUSWAIT"]
pub type BuswaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSFREE_CLREN` reader - desc BUSFREE_CLREN"]
pub type BusfreeClrenR = crate::BitReader;
#[doc = "Field `BUSFREE_CLREN` writer - desc BUSFREE_CLREN"]
pub type BusfreeClrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDADLY` reader - desc SDADLY"]
pub type SdadlyR = crate::FieldReader;
#[doc = "Field `SDADLY` writer - desc SDADLY"]
pub type SdadlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 10 - desc BUSWAIT"]
    #[inline(always)]
    pub fn buswait(&self) -> BuswaitR {
        BuswaitR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - desc BUSFREE_CLREN"]
    #[inline(always)]
    pub fn busfree_clren(&self) -> BusfreeClrenR {
        BusfreeClrenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - desc SDADLY"]
    #[inline(always)]
    pub fn sdadly(&self) -> SdadlyR {
        SdadlyR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR4")
            .field("buswait", &self.buswait())
            .field("busfree_clren", &self.busfree_clren())
            .field("sdadly", &self.sdadly())
            .finish()
    }
}
impl W {
    #[doc = "Bit 10 - desc BUSWAIT"]
    #[inline(always)]
    pub fn buswait(&mut self) -> BuswaitW<'_, Cr4Spec> {
        BuswaitW::new(self, 10)
    }
    #[doc = "Bit 12 - desc BUSFREE_CLREN"]
    #[inline(always)]
    pub fn busfree_clren(&mut self) -> BusfreeClrenW<'_, Cr4Spec> {
        BusfreeClrenW::new(self, 12)
    }
    #[doc = "Bits 16:19 - desc SDADLY"]
    #[inline(always)]
    pub fn sdadly(&mut self) -> SdadlyW<'_, Cr4Spec> {
        SdadlyW::new(self, 16)
    }
}
#[doc = "desc CR4\n\nYou can [`read`](crate::Reg::read) this register and get [`cr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr4Spec;
impl crate::RegisterSpec for Cr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr4::R`](R) reader structure"]
impl crate::Readable for Cr4Spec {}
#[doc = "`write(|w| ..)` method takes [`cr4::W`](W) writer structure"]
impl crate::Writable for Cr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR4 to value 0x0030_1b07"]
impl crate::Resettable for Cr4Spec {
    const RESET_VALUE: u32 = 0x0030_1b07;
}
