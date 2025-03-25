use std::fmt;

macro_rules! create_imm_struct {
    ($name:ident, $from_instr:expr, $safety:expr, $safety_doc:meta) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $name(u32);
        impl $name {
            pub fn new(imm: u32) -> Option<Self> {
                if ($safety)(imm) {
                    // SAFETY: Since we do the safety check here, this is always safe
                    Some(unsafe { Self::new_unchecked(imm) })
                } else {
                    None
                }
            }

            pub fn from_instr(instr: u32) -> Self {
                // SAFETY: The provided from_instr function is assumed to be safe
                let val = $from_instr(instr);
                debug_assert!(($safety)(val));
                unsafe { Self::new_unchecked(($from_instr)(instr)) }
            }

            #[$safety_doc]
            pub unsafe fn new_unchecked(imm: u32) -> Self {
                debug_assert!(
                    ($safety)(imm),
                    "new_unchecked called with invalid immediate"
                );
                if !($safety)(imm) {
                    // SAFETY: The caller guarantees the safety of the immediate, so this is unreachable
                    std::hint::unreachable_unchecked();
                }
                Self(imm)
            }

            pub fn as_u32(&self) -> u32 {
                self.0
            }
        }

        impl PartialEq<u32> for $name {
            fn eq(&self, other: &u32) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<$name> for u32 {
            fn eq(&self, other: &$name) -> bool {
                *self == other.0
            }
        }

        impl From<$name> for u32 {
            fn from(imm: $name) -> u32 {
                imm.as_u32()
            }
        }

        impl TryFrom<u32> for $name {
            type Error = ();

            fn try_from(imm: u32) -> Result<Self, Self::Error> {
                Self::new(imm).ok_or(())
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }
    };
}

/// Helper function to check if a value was signed extended within a given range
fn helper_signed_ext_within_range(val: u32, bit_mask: u32) -> bool {
    let bit_mask = !(bit_mask >> 1);
    if val & 0x8000_0000 != 0 {
        // The value was signed exeted
        val & bit_mask == bit_mask
    } else {
        val & bit_mask == 0
    }
}

create_imm_struct!(
    UTypeImm,
    |instr:u32| { instr & 0xFFFF_F000 },
    |imm| { imm & 0xFFF == 0 },
    doc = "Creates a new UTypeImm without checking if the given imm is valid\n\n# Safety\n\nThe value requires the lower 12 bits to be zero"
);

create_imm_struct!(
    JTypeImm,
    |instr: u32| {
        ((((instr as i32) >> 31) << 20) as u32)
        | (((instr >> 21) & 0x3FF) << 1)
        | (((instr >> 20) & 0x1) << 11)
        | (((instr >> 12) & 0xFF) << 12)
    },
    |imm: u32| {
        // 21 first bits are included in jump
        helper_signed_ext_within_range(imm, 0x1F_FFFF) && (imm & 0x1) == 0
    },
    doc = "Creates a new JTypeImm without checking if the given imm is valid\n\n# Safety\n\nThe value requires that the LSB is 0 and the upper 11 bits are either all 0 or all 1"
);

create_imm_struct!(
    ITypeImm,
    |instr: u32| { ((instr as i32) >> 20) as u32 },
    |imm| { 
        helper_signed_ext_within_range(imm, 0xFFF)
    },
    doc = "Creates a new ITypeImm without checking if the given imm is valid\n\n# Safety\n\nThe value requires that that upper 20 bits are either 0 or 1"
);

create_imm_struct!(
    ShamtImm,
    |instr: u32| { (instr >> 20) & 0x1F },
    |imm| { 
        imm < 32
    },
    doc = "Creates a new ShamtImm without checking if the given imm is valid\n\n# Safety\n\nThe value requires to be between 0..=32"
);

create_imm_struct!(
    STypeImm,
    |instr: u32| {  (((instr as i32) >> 25) << 5) as u32 | ((instr >> 7) & 0x1F) },
    |imm| { helper_signed_ext_within_range(imm, 0xFFF)  },
    doc = "Creates a new STypeImm without checking if the given imm is valid\n\n# Safety\n\nThe value requires that that upper 20 bits are either 0 or 1"
);

create_imm_struct!(
    BTypeImm,
    |instr: u32| {  
        (((instr as i32) >> 31) << 12) as u32
        | (((instr >> 7) & 0x1) << 11)
        | (((instr >> 25) & 0x3F) << 5)
        | (((instr >> 8) & 0xF) << 1)
    },
    |imm| { helper_signed_ext_within_range(imm, 0x1FFF) && imm & 0x1 == 0 },
    doc = "Creates a new STypeImm without checking if the given imm is valid\n\n# Safety\n\nThe value requires that LSB is 0 that upper 19 bits are either 0 or 1"
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_utype_imm() {
        let cases = vec![
            (0, true),
            (17, false),
            (-18, false),
            (0xFFFFE, false),
            (0x100000, true),
            (-0x100000, true),
            (-0x100002, false),
            (0xFF000, true),
            (0x1a19000, true),
        ];

        for (i, case) in cases.into_iter().enumerate() {
            let imm = UTypeImm::new(case.0 as u32);
            assert_eq!(imm.is_some(), case.1, "Case {} failed: {:x}", i, case.0);
        }
    }

    #[test]
    fn test_jtype_imm() {
        let cases = vec![
            (0, true),
            (17, false),
            (-18, true),
            (0xFFFFE, true),
            (0x100000, false),
            (-0x100000, true),
            (-0x100002, false),
        ];

        for (i, case) in cases.into_iter().enumerate() {
            let imm = JTypeImm::new(case.0 as u32);
            assert_eq!(imm.is_some(), case.1, "Case {} failed: {:x}", i, case.0);
        }
    }

    #[test]
    fn test_itype_imm() {
        let cases = vec![
            (0, true),
            (17, true),
            (-18, true),
            (0xFFF, false),
            (0x7FF, true),
            (-2048, true),
            (-2049, false),
        ];

        for (i, case) in cases.into_iter().enumerate() {
            let imm = ITypeImm::new(case.0 as u32);
            assert_eq!(imm.is_some(), case.1, "Case {} failed: {:x}", i, case.0);
        }
    }

    #[test]
    fn test_stype_imm() {
        let cases = vec![
            (0, true),
            (17, true),
            (-18, true),
            (0xFFF, false),
            (0x7FF, true),
            (0x800, false),
            (-2048, true),
            (-2049, false),
        ];

        for (i, case) in cases.into_iter().enumerate() {
            let imm = STypeImm::new(case.0 as u32);
            assert_eq!(imm.is_some(), case.1, "Case {} failed: {:x}", i, case.0);
        }
    }

    #[test]
    fn test_btype_imm() {
        let cases = vec![
            (0, true),
            (17, false),
            (-18, true),
            (0xFFF, false),
            (0x7FF, false),
            (4094, true),
            (4096, false),
            (-2048, true),
            (-2049, false),
            (-4096, true),
            (-4097, false),
            (-4098, false),
            (0x7FE, true),
            (0x7FF, false),
        ];

        for (i, case) in cases.into_iter().enumerate() {
            let imm = BTypeImm::new(case.0 as u32);
            assert_eq!(imm.is_some(), case.1, "Case {} failed: 0x{:x}", i, case.0);
        }
    }
}
