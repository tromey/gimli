use std::collections::hash_map;

/// Abbreviation tag types, aka `DW_TAG_whatever` in the standard.
///
/// DWARF standard 4, section 7.5.4, page 154
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum AbbreviationTag {
    ArrayType = 0x01,
    ClassType = 0x02,
    EntryPoint = 0x03,
    EnumerationType = 0x04,
    FormalParameter = 0x05,
    ImportedDeclaration = 0x08,
    Label = 0x0a,
    LexicalBlock = 0x0b,
    Member = 0x0d,
    PointerType = 0x0f,
    ReferenceType = 0x10,
    CompileUnit = 0x11,
    StringType = 0x12,
    StructureType = 0x13,
    SubroutineType = 0x15,
    Typedef = 0x16,
    UnionType = 0x17,
    UnspecifiedParameters = 0x18,
    Variant = 0x19,
    CommonBlock = 0x1a,
    CommonInclusion = 0x1b,
    Inheritance = 0x1c,
    InlinedSubroutine = 0x1d,
    Module = 0x1e,
    PtrToMemberType = 0x1f,
    SetType = 0x20,
    SubrangeType = 0x21,
    WithStmt = 0x22,
    AccessDeclaration = 0x23,
    BaseType = 0x24,
    CatchBlock = 0x25,
    ConstType = 0x26,
    Constant = 0x27,
    Enumerator = 0x28,
    FileType = 0x29,
    Friend = 0x2a,
    Namelist = 0x2b,
    NamelistItem = 0x2c,
    PackedType = 0x2d,
    Subprogram = 0x2e,
    TemplateTypeParameter = 0x2f,
    TemplateValueParameter = 0x30,
    ThrownType = 0x31,
    TryBlock = 0x32,
    VariantPart = 0x33,
    Variable = 0x34,
    VolatileType = 0x35,
    DwarfProcedure = 0x36,
    RestrictType = 0x37,
    InterfaceType = 0x38,
    Namespace = 0x39,
    ImportedModule = 0x3a,
    UnspecifiedType = 0x3b,
    PartialUnit = 0x3c,
    ImportedUnit = 0x3d,
    Condition = 0x3f,
    SharedType = 0x40,
    TypeUnit = 0x41,
    RvalueReferenceType = 0x42,
    TemplateAlias = 0x43,
    LoUser = 0x4080,
    HiUser = 0xffff,
}

/// Whether an abbreviation's type has children or not, aka
/// `DW_CHILDREN_{yes,no}` in the standard.
///
/// DWARF standard 4, section 7.5.4, page 154
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbbreviationHasChildren {
    /// The type has children.
    Yes = 0x0,

    /// The type does not have children.
    No = 0x1,
}

/// The set of possible attribute names, aka `DW_AT_whatever` in the standard.
///
/// DWARF standard 4, section 7.5.4, page 155
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum AttributeName {
    Sibling = 0x1,
    Location = 0x2,
    Name = 0x3,
    Ordering = 0x9,
    ByteSize = 0xb,
    BitOffset = 0xc,
    BitSize = 0x0d,
    StmtList = 0x10,
    LowPc = 0x11,
    HighPc = 0x12,
    Language = 0x13,
    Discr = 0x15,
    DiscrValue = 0x16,
    Visibility = 0x17,
    Import = 0x18,
    StringLength = 0x19,
    CommonReference = 0x1a,
    CompDir = 0x1b,
    ConstValue = 0x1c,
    ContainingType = 0x1d,
    DefaultValue = 0x1e,
    Inline = 0x20,
    IsOptional = 0x21,
    LowerBound = 0x22,
    Producer = 0x25,
    Prototyped = 0x27,
    ReturnAddr = 0x2a,
    StartScope = 0x2c,
    BitStride = 0x2e,
    UpperBound = 0x2f,
    AbstractOrigin = 0x31,
    Accessibility = 0x32,
    AddressClass = 0x33,
    Artificial = 0x34,
    BaseTypes = 0x35,
    CallingConvention = 0x36,
    Count = 0x37,
    DataMemberLocation = 0x38,
    DeclColumn = 0x39,
    DeclFile = 0x3a,
    DeclLine = 0x3b,
    Declaration = 0x3c,
    DiscrList = 0x3d,
    Encoding = 0x3e,
    External = 0x3f,
    FrameBase = 0x40,
    Friend = 0x41,
    IdentifierCase = 0x42,
    MacroInfo = 0x43,
    NamelistItem = 0x44,
    Priority = 0x45,
    Segment = 0x46,
    Specification = 0x47,
    StaticLink = 0x48,
    Type = 0x49,
    UseLocation = 0x4a,
    VariableParameter = 0x4b,
    Virtuality = 0x4c,
    VtableElemLocation = 0x4d,
    Allocated = 0x4e,
    Associated = 0x4f,
    DataLocation = 0x50,
    ByteStride = 0x51,
    EntryPc = 0x52,
    UseUtf8 = 0x53,
    Extension = 0x54,
    Ranges = 0x55,
    Trampoline = 0x56,
    CallColumn = 0x57,
    CallFile = 0x58,
    CallLine = 0x59,
    Description = 0x5a,
    BinaryScale = 0x5b,
    DecimalScale = 0x5c,
    Small = 0x5d,
    DecimalSign = 0x5e,
    DigitCount = 0x5f,
    PictureString = 0x60,
    Mutable = 0x61,
    ThreadsScaled = 0x62,
    Explicit = 0x63,
    ObjectPointer = 0x64,
    Endianity = 0x65,
    Elemental = 0x66,
    Pure = 0x67,
    Recursive = 0x68,
    Signature = 0x69,
    MainSubprogram = 0x6a,
    DataBitOffset = 0x6b,
    ConstExpr = 0x6c,
    EnumClass = 0x6d,
    LinkageName = 0x6e,
    LoUser = 0x2000,
    HiUser = 0x3fff,
}

/// The type and encoding of an attribute, aka `DW_FORM_whatever` in the
/// standard.
///
/// DWARF standard 4, section 7.5.4, page 160
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum AttributeForm {
    Addr = 0x01,
    Block2 = 0x03,
    Block4 = 0x04,
    Data2 = 0x05,
    Data4 = 0x06,
    Data8 = 0x07,
    String = 0x08,
    Block = 0x09,
    Block1 = 0x0a,
    Data1 = 0x0b,
    Flag = 0x0c,
    Sdata = 0x0d,
    Strp = 0x0e,
    Udata = 0x0f,
    RefAddr = 0x10,
    Ref1 = 0x11,
    Ref2 = 0x12,
    Ref4 = 0x13,
    Ref8 = 0x14,
    RefUdata = 0x15,
    Indirect = 0x16,
    SecOffset = 0x17,
    Exprloc = 0x18,
    FlagPresent = 0x19,
    RefSig8 = 0x20,
}

/// The description of an attribute in an abbreviated type. It is a pair of name
/// and form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributeSpecification {
    name: AttributeName,
    form: AttributeForm,
}

impl AttributeSpecification {
    /// Construct a new `AttributeSpecification` from the given name and form.
    pub fn new(name: AttributeName, form: AttributeForm) -> AttributeSpecification {
        AttributeSpecification {
            name: name,
            form: form,
        }
    }

    /// Get the attribute's name.
    pub fn name(&self) -> AttributeName {
        self.name
    }

    /// Get the attribute's form.
    pub fn form(&self) -> AttributeForm {
        self.form
    }
}

/// An abbreviation describes the shape of a DIE type: its code, tag type,
/// whether it has children, and its set of attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abbreviation {
    code: u64,
    tag: AbbreviationTag,
    has_children: AbbreviationHasChildren,
    attributes: Vec<AttributeSpecification>,
}

impl Abbreviation {
    /// Construct a new `Abbreviation`.
    ///
    /// ### Panics
    ///
    /// Panics if `code` is `0`.
    pub fn new(code: u64,
               tag: AbbreviationTag,
               has_children: AbbreviationHasChildren,
               attributes: Vec<AttributeSpecification>) -> Abbreviation {
        assert!(code != 0);
        Abbreviation {
            code: code,
            tag: tag,
            has_children: has_children,
            attributes: attributes,
        }
    }

    /// Get this abbreviation's code.
    pub fn code(&self) -> u64 {
        self.code
    }

    /// Get this abbreviation's tag.
    pub fn tag(&self) -> AbbreviationTag {
        self.tag
    }

    /// Return true if this abbreviation's type has children, false otherwise.
    pub fn has_children(&self) -> bool {
        match self.has_children {
            AbbreviationHasChildren::Yes => true,
            AbbreviationHasChildren::No => false,
        }
    }

    /// Get this abbreviation's attributes.
    pub fn attributes(&self) -> &[AttributeSpecification] {
        &self.attributes[..]
    }
}

/// A set of type abbreviations.
#[derive(Debug, Clone)]
pub struct Abbreviations {
    abbrevs: hash_map::HashMap<u64, Abbreviation>,
}

impl Abbreviations {
    /// Construct a new, empty set of abbreviations.
    pub fn new() -> Abbreviations {
        Abbreviations {
            abbrevs: hash_map::HashMap::new(),
        }
    }

    /// Insert an abbreviation into the set.
    ///
    /// Returns `Ok` if it is the first abbreviation in the set with its code,
    /// `Err` if the code is a duplicate and there already exists an
    /// abbreviation in the set with the given abbreviation's code.
    pub fn insert(&mut self, abbrev: Abbreviation) -> Result<(), ()> {
        match self.abbrevs.entry(abbrev.code) {
            hash_map::Entry::Occupied(_) =>
                Err(()),
            hash_map::Entry::Vacant(entry) => {
                entry.insert(abbrev);
                Ok(())
            },
        }
    }
}

/// The header of a compilation unit's debugging information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompilationUnitHeader {
    unit_length: u64,
    version: u16,
    debug_abbrev_offset: u64,
    address_size: u8,
}

impl CompilationUnitHeader {
    /// Construct a new `CompilationUnitHeader`.
    pub fn new(unit_length: u64,
               version: u16,
               debug_abbrev_offset: u64,
               address_size: u8) -> CompilationUnitHeader {
        CompilationUnitHeader {
            unit_length: unit_length,
            version: version,
            debug_abbrev_offset: debug_abbrev_offset,
            address_size: address_size
        }
    }

    /// Get the length of the debugging info for this compilation unit.
    pub fn unit_length(&self) -> u64 {
        self.unit_length
    }

    /// Get the DWARF version of the debugging info for this compilation unit.
    pub fn version(&self) -> u16 {
        self.version
    }

    /// The offset into the `.debug_abbrev` section for this compilation unit's
    /// debugging information entries.
    pub fn debug_abbrev_offset(&self) -> u64 {
        self.debug_abbrev_offset
    }

    /// The size of addresses (in bytes) in this compilation unit.
    pub fn address_size(&self) -> u8 {
        self.address_size
    }
}
