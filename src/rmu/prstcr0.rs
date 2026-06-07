#[doc = "Register `PRSTCR0` reader"]
pub type R = crate::R<Prstcr0Spec>;
#[doc = "Register `PRSTCR0` writer"]
pub type W = crate::W<Prstcr0Spec>;
#[doc = "Field `LKUPREN` reader - desc LKUPREN"]
pub type LkuprenR = crate::BitReader;
#[doc = "Field `LKUPREN` writer - desc LKUPREN"]
pub type LkuprenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - desc LKUPREN"]
    #[inline(always)]
    pub fn lkupren(&self) -> LkuprenR {
        LkuprenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRSTCR0")
            .field("lkupren", &self.lkupren())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - desc LKUPREN"]
    #[inline(always)]
    pub fn lkupren(&mut self) -> LkuprenW<'_, Prstcr0Spec> {
        LkuprenW::new(self, 5)
    }
}
#[doc = "desc PRSTCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`prstcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prstcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prstcr0Spec;
impl crate::RegisterSpec for Prstcr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prstcr0::R`](R) reader structure"]
impl crate::Readable for Prstcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`prstcr0::W`](W) writer structure"]
impl crate::Writable for Prstcr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRSTCR0 to value 0x40"]
impl crate::Resettable for Prstcr0Spec {
    const RESET_VALUE: u8 = 0x40;
}
