pub struct FieldDefinition {
    /// Full path of the trait that has the field (e.g. `::auth_core::FactorMetadata`)
    pub trait_path: &'static str,

    /// Name of the field, defined by the trait
    pub field_name: &'static str,

    /// Full path of the field type
    pub field_type: &'static str,

    /// Name of the method used to access the field given a reference to `self`
    pub method_name: &'static str,
}

pub static FLOW_TYPE: FieldDefinition = FieldDefinition {
    trait_path: "::auth_core::FactorMetadata",
    field_name: "FLOW_TYPE",
    field_type: "::auth_core::FlowType",
    method_name: "flow_type",
};

pub static SECURITY_LEVEL: FieldDefinition = FieldDefinition {
    trait_path: "::auth_core::FactorMetadata",
    field_name: "SECURITY_LEVEL",
    field_type: "::auth_core::SecurityLevel",
    method_name: "security_level",
};

pub static ROLE: FieldDefinition = FieldDefinition {
    trait_path: "::auth_core::FactorMetadata",
    field_name: "ROLE",
    field_type: "::auth_core::FactorRole",
    method_name: "role",
};

pub static SLUG: FieldDefinition = FieldDefinition {
    trait_path: "::auth_core::FactorSlug",
    field_name: "SLUG",
    field_type: "&'static str",
    method_name: "slug",
};

pub static FIELDS: &[&FieldDefinition] = &[&FLOW_TYPE, &SECURITY_LEVEL, &ROLE, &SLUG];
