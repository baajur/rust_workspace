/* automatically generated by rust-bindgen */

#[allow(unused_imports)] use super::*;


#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
pub struct Field {
    pub Crit: *const CritterCl,
    pub DeadCrits: *const CrClVec,
    pub ScrX: ::std::os::raw::c_int,
    pub ScrY: ::std::os::raw::c_int,
    pub SimplyTile: [*const SpriteAnim; 2usize],
    pub Tiles: [*const Field_TileVec; 2usize],
    pub Items: *const ItemVec,
    pub RoofNum: int16,
    pub Flags: Field__bindgen_ty_1,
    pub Corner: uint8,
    pub LightValues: [uint8; 3usize],
}
#[repr(C)]
pub struct Field_Tile {
    pub Anim: *const SpriteAnim,
    pub OffsX: int16,
    pub OffsY: int16,
    pub Layer: uint8,
}
#[test]
fn bindgen_test_layout_Field_Tile() {
    assert_eq!(
        ::std::mem::size_of::<Field_Tile>(),
        12usize,
        concat!("Size of: ", stringify!(Field_Tile))
    );
    assert_eq!(
        ::std::mem::align_of::<Field_Tile>(),
        4usize,
        concat!("Alignment of ", stringify!(Field_Tile))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field_Tile>())).Anim as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Field_Tile),
            "::",
            stringify!(Anim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field_Tile>())).OffsX as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Field_Tile),
            "::",
            stringify!(OffsX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field_Tile>())).OffsY as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(Field_Tile),
            "::",
            stringify!(OffsY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field_Tile>())).Layer as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Field_Tile),
            "::",
            stringify!(Layer)
        )
    );
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Field__bindgen_ty_1 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
}
#[test]
fn bindgen_test_layout_Field__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<Field__bindgen_ty_1>(),
        2usize,
        concat!("Size of: ", stringify!(Field__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<Field__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(Field__bindgen_ty_1))
    );
}
impl Field__bindgen_ty_1 {
    #[inline]
    pub fn ScrollBlock(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_ScrollBlock(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn IsWall(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_IsWall(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn IsWallSAI(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_IsWallSAI(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn IsWallTransp(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_IsWallTransp(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn IsScen(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_IsScen(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn IsExitGrid(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_IsExitGrid(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn IsNotPassed(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_IsNotPassed(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn IsNotRaked(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_IsNotRaked(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn IsNoLight(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_IsNoLight(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn IsMultihex(&self) -> bool {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_IsMultihex(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        ScrollBlock: bool,
        IsWall: bool,
        IsWallSAI: bool,
        IsWallTransp: bool,
        IsScen: bool,
        IsExitGrid: bool,
        IsNotPassed: bool,
        IsNotRaked: bool,
        IsNoLight: bool,
        IsMultihex: bool,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let ScrollBlock: u8 = unsafe { ::std::mem::transmute(ScrollBlock) };
            ScrollBlock as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let IsWall: u8 = unsafe { ::std::mem::transmute(IsWall) };
            IsWall as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let IsWallSAI: u8 = unsafe { ::std::mem::transmute(IsWallSAI) };
            IsWallSAI as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let IsWallTransp: u8 = unsafe { ::std::mem::transmute(IsWallTransp) };
            IsWallTransp as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let IsScen: u8 = unsafe { ::std::mem::transmute(IsScen) };
            IsScen as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let IsExitGrid: u8 = unsafe { ::std::mem::transmute(IsExitGrid) };
            IsExitGrid as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let IsNotPassed: u8 = unsafe { ::std::mem::transmute(IsNotPassed) };
            IsNotPassed as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let IsNotRaked: u8 = unsafe { ::std::mem::transmute(IsNotRaked) };
            IsNotRaked as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let IsNoLight: u8 = unsafe { ::std::mem::transmute(IsNoLight) };
            IsNoLight as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let IsMultihex: u8 = unsafe { ::std::mem::transmute(IsMultihex) };
            IsMultihex as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_Field() {
    assert_eq!(
        ::std::mem::size_of::<Field>(),
        44usize,
        concat!("Size of: ", stringify!(Field))
    );
    assert_eq!(
        ::std::mem::align_of::<Field>(),
        4usize,
        concat!("Alignment of ", stringify!(Field))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).Crit as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(Crit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).DeadCrits as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(DeadCrits)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).ScrX as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(ScrX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).ScrY as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(ScrY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).SimplyTile as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(SimplyTile)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).Tiles as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(Tiles)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).Items as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(Items)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).RoofNum as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(RoofNum)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).Flags as *const _ as usize },
        38usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).Corner as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(Corner)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Field>())).LightValues as *const _ as usize },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(Field),
            "::",
            stringify!(LightValues)
        )
    );
}
