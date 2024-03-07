///Type enum for the [`byondapi_sys::ByondValueType`] field of [`byondapi_sys::CByondValue`], copied from auxtools
#[repr(u8)]
#[derive(Copy, Clone, Debug, num_enum::TryFromPrimitive)]
#[non_exhaustive]
pub enum ValueType {
    Null = 0x00,
    Turf = 0x01,
    Obj = 0x02,
    Mob = 0x03,
    Area = 0x04,
    Client = 0x05,
    String = 0x06,

    MobTypepath = 0x08,
    ObjTypepath = 0x09,
    TurfTypepath = 0x0A,
    AreaTypepath = 0x0B,
    Resource = 0x0C,
    Image = 0x0D,
    World = 0x0E,

    // Lists
    List = 0x0F,
    ArgList = 0x10,
    MobContents = 0x17,
    TurfContents = 0x18,
    AreaContents = 0x19,
    WorldContents = 0x1A,
    ObjContents = 0x1C,
    MobVars = 0x2C,
    ObjVars = 0x2D,
    TurfVars = 0x2E,
    AreaVars = 0x2F,
    ClientVars = 0x30,
    Vars = 0x31,
    MobOverlays = 0x32,
    MobUnderlays = 0x33,
    ObjOverlays = 0x34,
    ObjUnderlays = 0x35,
    TurfOverlays = 0x36,
    TurfUnderlays = 0x37,
    AreaOverlays = 0x38,
    AreaUnderlays = 0x39,
    ImageOverlays = 0x40,
    ImageUnderlays = 0x41,
    ImageVars = 0x42,
    TurfVisContents = 0x4B,
    ObjVisContents = 0x4C,
    MobVisContents = 0x4D,
    TurfVisLocs = 0x4E,
    ObjVisLocs = 0x4F,
    MobVisLocs = 0x50,
    WorldVars = 0x51,
    GlobalVars = 0x52,
    ImageVisContents = 0x54,

    Datum = 0x21,
    SaveFile = 0x23,

    Number = 0x2A,
    Appearance = 0x3A,
}