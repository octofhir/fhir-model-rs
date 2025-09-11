//! Type reflection system for FHIRPath type operations

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::type_system::{ConversionInfo, PolymorphicContext};

/// Type reflection information following FHIRPath specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeReflectionInfo {
    /// Simple primitive type information
    SimpleType {
        /// Type namespace (e.g., "System", "FHIR")
        namespace: String,
        /// Type name (e.g., "String", "Patient")
        name: String,
        /// Base type if this type inherits from another
        base_type: Option<String>,
    },

    /// Class/complex type information with elements
    ClassInfo {
        /// Type namespace
        namespace: String,
        /// Type name
        name: String,
        /// Base type if this type inherits from another
        base_type: Option<String>,
        /// Element definitions for this class
        elements: Vec<ElementInfo>,
    },

    /// Collection/list type information
    ListType {
        /// Element type information
        element_type: Box<TypeReflectionInfo>,
    },

    /// Tuple type information for anonymous types
    TupleType {
        /// Element definitions for this tuple
        elements: Vec<TupleElementInfo>,
    },
}

/// Element information for class types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElementInfo {
    /// Element name
    pub name: String,
    /// Element type information
    pub type_info: TypeReflectionInfo,
    /// Minimum cardinality
    pub min_cardinality: u32,
    /// Maximum cardinality (None for unbounded)
    pub max_cardinality: Option<u32>,
    /// Whether this is a modifier element
    pub is_modifier: bool,
    /// Whether this element appears in summaries
    pub is_summary: bool,
    /// Whether this element is one-based indexed (FHIRPath ClassInfoElement compliance)
    pub is_one_based: bool,
    /// Element documentation
    pub documentation: Option<String>,
}

/// Element information for tuple types
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TupleElementInfo {
    /// Element name
    pub name: String,
    /// Element type information
    pub type_info: TypeReflectionInfo,
    /// Whether this element is one-based indexed
    pub is_one_based: bool,
}

/// Type hierarchy information for inheritance relationships
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeHierarchy {
    /// The root type name
    pub root_type: String,
    /// Parent types (inheritance chain)
    pub parents: Vec<String>,
    /// Direct child types
    pub children: Vec<String>,
    /// All descendant types (transitive children)
    pub descendants: Vec<String>,
}

/// Type suggestion for autocomplete and search
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeSuggestion {
    /// Suggested type name
    pub type_name: String,
    /// Type kind (resource, complex-type, primitive-type)
    pub kind: String,
    /// Description of the type
    pub description: Option<String>,
    /// Relevance score (0.0 to 1.0)
    pub relevance_score: f64,
    /// Namespace of the type
    pub namespace: String,
}

impl TypeReflectionInfo {
    /// Get the type name
    pub fn name(&self) -> &str {
        match self {
            TypeReflectionInfo::SimpleType { name, .. } => name,
            TypeReflectionInfo::ClassInfo { name, .. } => name,
            TypeReflectionInfo::ListType { element_type: _ } => {
                // For list types, we use a generic name
                "List"
            }
            TypeReflectionInfo::TupleType { .. } => "Tuple",
        }
    }

    /// Get the namespace
    pub fn namespace(&self) -> Option<&str> {
        match self {
            TypeReflectionInfo::SimpleType { namespace, .. } => Some(namespace),
            TypeReflectionInfo::ClassInfo { namespace, .. } => Some(namespace),
            TypeReflectionInfo::ListType { .. } => None,
            TypeReflectionInfo::TupleType { .. } => None,
        }
    }

    /// Get the base type if available
    pub fn base_type(&self) -> Option<&str> {
        match self {
            TypeReflectionInfo::SimpleType { base_type, .. } => base_type.as_deref(),
            TypeReflectionInfo::ClassInfo { base_type, .. } => base_type.as_deref(),
            TypeReflectionInfo::ListType { .. } => None,
            TypeReflectionInfo::TupleType { .. } => None,
        }
    }

    /// Check if this is a primitive type
    pub fn is_primitive(&self) -> bool {
        match self {
            TypeReflectionInfo::SimpleType { namespace, .. } => namespace == "System",
            _ => false,
        }
    }

    /// Check if this is a FHIR type
    pub fn is_fhir_type(&self) -> bool {
        match self {
            TypeReflectionInfo::SimpleType { namespace, .. }
            | TypeReflectionInfo::ClassInfo { namespace, .. } => namespace == "FHIR",
            _ => false,
        }
    }

    /// Check if this is a collection type
    pub fn is_collection(&self) -> bool {
        matches!(self, TypeReflectionInfo::ListType { .. })
    }

    /// Get the element type if this is a collection
    pub fn element_type(&self) -> Option<&TypeReflectionInfo> {
        match self {
            TypeReflectionInfo::ListType { element_type } => Some(element_type),
            _ => None,
        }
    }

    /// Get elements if this is a class or tuple type
    pub fn elements(&self) -> Vec<&ElementInfo> {
        match self {
            TypeReflectionInfo::ClassInfo { elements, .. } => elements.iter().collect(),
            _ => Vec::new(),
        }
    }

    /// Get tuple elements if this is a tuple type
    pub fn tuple_elements(&self) -> Vec<&TupleElementInfo> {
        match self {
            TypeReflectionInfo::TupleType { elements } => elements.iter().collect(),
            _ => Vec::new(),
        }
    }

    /// Find an element by name
    pub fn find_element(&self, name: &str) -> Option<&ElementInfo> {
        match self {
            TypeReflectionInfo::ClassInfo { elements, .. } => {
                elements.iter().find(|e| e.name == name)
            }
            _ => None,
        }
    }

    /// Get the fully qualified type name
    pub fn qualified_name(&self) -> String {
        match self {
            TypeReflectionInfo::SimpleType {
                namespace, name, ..
            }
            | TypeReflectionInfo::ClassInfo {
                namespace, name, ..
            } => {
                if namespace.is_empty() {
                    name.clone()
                } else {
                    format!("{namespace}.{name}")
                }
            }
            TypeReflectionInfo::ListType { element_type } => {
                format!("List<{}>", element_type.qualified_name())
            }
            TypeReflectionInfo::TupleType { elements } => {
                let element_names: Vec<String> = elements
                    .iter()
                    .map(|e| format!("{}: {}", e.name, e.type_info.qualified_name()))
                    .collect();
                format!("{{ {} }}", element_names.join(", "))
            }
        }
    }

    /// Create a simple type
    pub fn simple_type(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        TypeReflectionInfo::SimpleType {
            namespace: namespace.into(),
            name: name.into(),
            base_type: None,
        }
    }

    /// Create a simple type with base type
    pub fn simple_type_with_base(
        namespace: impl Into<String>,
        name: impl Into<String>,
        base_type: impl Into<String>,
    ) -> Self {
        TypeReflectionInfo::SimpleType {
            namespace: namespace.into(),
            name: name.into(),
            base_type: Some(base_type.into()),
        }
    }

    /// Create a class type
    pub fn class_type(
        namespace: impl Into<String>,
        name: impl Into<String>,
        elements: Vec<ElementInfo>,
    ) -> Self {
        TypeReflectionInfo::ClassInfo {
            namespace: namespace.into(),
            name: name.into(),
            base_type: None,
            elements,
        }
    }

    /// Create a list type
    pub fn list_type(element_type: TypeReflectionInfo) -> Self {
        TypeReflectionInfo::ListType {
            element_type: Box::new(element_type),
        }
    }

    /// Create a tuple type
    pub fn tuple_type(elements: Vec<TupleElementInfo>) -> Self {
        TypeReflectionInfo::TupleType { elements }
    }
}

impl ElementInfo {
    /// Create a new element info
    pub fn new(name: impl Into<String>, type_info: TypeReflectionInfo) -> Self {
        Self {
            name: name.into(),
            type_info,
            min_cardinality: 0,
            max_cardinality: Some(1),
            is_modifier: false,
            is_summary: false,
            is_one_based: false,
            documentation: None,
        }
    }

    /// Set cardinality
    pub fn with_cardinality(mut self, min: u32, max: Option<u32>) -> Self {
        self.min_cardinality = min;
        self.max_cardinality = max;
        self
    }

    /// Mark as modifier element
    pub fn with_modifier(mut self) -> Self {
        self.is_modifier = true;
        self
    }

    /// Mark as summary element
    pub fn with_summary(mut self) -> Self {
        self.is_summary = true;
        self
    }

    /// Add documentation
    pub fn with_documentation(mut self, doc: impl Into<String>) -> Self {
        self.documentation = Some(doc.into());
        self
    }

    /// Mark as one-based indexed
    pub fn with_one_based(mut self) -> Self {
        self.is_one_based = true;
        self
    }

    /// Check if this element is required (min > 0)
    pub fn is_required(&self) -> bool {
        self.min_cardinality > 0
    }

    /// Check if this element can have multiple values
    pub fn is_multiple(&self) -> bool {
        self.max_cardinality.is_none_or(|max| max > 1)
    }
}

impl TupleElementInfo {
    /// Create a new tuple element info
    pub fn new(name: impl Into<String>, type_info: TypeReflectionInfo) -> Self {
        Self {
            name: name.into(),
            type_info,
            is_one_based: false,
        }
    }

    /// Mark as one-based indexed
    pub fn with_one_based(mut self) -> Self {
        self.is_one_based = true;
        self
    }
}

impl TypeHierarchy {
    /// Create a new type hierarchy
    pub fn new(root_type: impl Into<String>) -> Self {
        Self {
            root_type: root_type.into(),
            parents: Vec::new(),
            children: Vec::new(),
            descendants: Vec::new(),
        }
    }

    /// Add a parent type
    pub fn add_parent(&mut self, parent: String) {
        if !self.parents.contains(&parent) {
            self.parents.push(parent);
        }
    }

    /// Add a child type
    pub fn add_child(&mut self, child: String) {
        if !self.children.contains(&child) {
            self.children.push(child.clone());
            if !self.descendants.contains(&child) {
                self.descendants.push(child);
            }
        }
    }

    /// Check if a type is an ancestor
    pub fn is_ancestor(&self, type_name: &str) -> bool {
        self.parents.contains(&type_name.to_string())
    }

    /// Check if a type is a descendant
    pub fn is_descendant(&self, type_name: &str) -> bool {
        self.descendants.contains(&type_name.to_string())
    }
}

impl TypeSuggestion {
    /// Create a new type suggestion
    pub fn new(
        type_name: impl Into<String>,
        kind: impl Into<String>,
        namespace: impl Into<String>,
        relevance_score: f64,
    ) -> Self {
        Self {
            type_name: type_name.into(),
            kind: kind.into(),
            description: None,
            relevance_score,
            namespace: namespace.into(),
        }
    }

    /// Add description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// Enhanced type utility operations for FHIRPath compliance
impl TypeReflectionInfo {
    /// Check if this type is compatible with another type
    pub fn is_compatible_with(&self, other: &TypeReflectionInfo) -> bool {
        match (self, other) {
            // Same types are always compatible
            (a, b) if a == b => true,

            // Check base type relationships
            (
                TypeReflectionInfo::SimpleType {
                    base_type: Some(base),
                    ..
                },
                other_type,
            ) => {
                if base == other_type.name() {
                    return true;
                }
                false
            }
            (
                TypeReflectionInfo::ClassInfo {
                    base_type: Some(base),
                    ..
                },
                other_type,
            ) => {
                if base == other_type.name() {
                    return true;
                }
                false
            }

            // Collection compatibility - check element types
            (
                TypeReflectionInfo::ListType { element_type: e1 },
                TypeReflectionInfo::ListType { element_type: e2 },
            ) => e1.is_compatible_with(e2),

            // System types have special compatibility rules
            (
                TypeReflectionInfo::SimpleType {
                    namespace: ns1,
                    name: n1,
                    ..
                },
                TypeReflectionInfo::SimpleType {
                    namespace: ns2,
                    name: n2,
                    ..
                },
            ) => {
                if ns1 == "System" && ns2 == "System" {
                    // Check system type compatibility (e.g., Integer -> Decimal)
                    return self.is_system_type_compatible(n1, n2);
                }
                false
            }

            _ => false,
        }
    }

    /// Check system type compatibility rules
    fn is_system_type_compatible(&self, from_type: &str, to_type: &str) -> bool {
        match (from_type, to_type) {
            // Numeric type promotions
            ("Integer", "Decimal") => true,
            ("Integer", "String") => true,
            ("Decimal", "String") => true,
            ("Boolean", "String") => true,

            // Date/time conversions
            ("Date", "DateTime") => true,
            ("DateTime", "String") => true,
            ("Date", "String") => true,
            ("Time", "String") => true,

            _ => false,
        }
    }

    /// Get all ancestor types (inheritance chain)
    pub fn get_ancestors(&self) -> Vec<String> {
        let mut ancestors = Vec::new();
        if let Some(base) = self.base_type() {
            ancestors.push(base.to_string());
            // Note: In a full implementation, we'd recursively get ancestors
            // For now, we just return the immediate base type
        }
        ancestors
    }

    /// Check if this type is a subtype of another
    pub fn is_subtype_of(&self, parent_type: &str) -> bool {
        if self.name() == parent_type {
            return true;
        }

        if let Some(base) = self.base_type() {
            if base == parent_type {
                return true;
            }
            // In a full implementation, we'd recursively check ancestors
        }

        false
    }

    /// Get the most specific common type between two types
    pub fn common_supertype(&self, other: &TypeReflectionInfo) -> Option<TypeReflectionInfo> {
        // If types are the same, return one of them
        if self == other {
            return Some(self.clone());
        }

        // Check if one is a subtype of the other
        if self.is_subtype_of(other.name()) {
            return Some(other.clone());
        }
        if other.is_subtype_of(self.name()) {
            return Some(self.clone());
        }

        // For system types, find common supertype
        if self.is_primitive() && other.is_primitive() {
            // Most primitive types can be converted to String
            return Some(TypeReflectionInfo::simple_type("System", "String"));
        }

        // For FHIR types, check for Resource base type
        if self.is_fhir_type() && other.is_fhir_type() {
            // In a full implementation, we'd walk up the inheritance tree
            return Some(TypeReflectionInfo::simple_type("FHIR", "Element"));
        }

        None
    }

    /// Check if this type can be used as a collection element
    pub fn is_valid_collection_element(&self) -> bool {
        match self {
            TypeReflectionInfo::SimpleType { .. } => true,
            TypeReflectionInfo::ClassInfo { .. } => true,
            TypeReflectionInfo::ListType { .. } => false, // No nested collections in FHIRPath
            TypeReflectionInfo::TupleType { .. } => false, // Tuples are not collection elements
        }
    }

    /// Get default value for this type (if applicable)
    pub fn default_value(&self) -> Option<String> {
        if let TypeReflectionInfo::SimpleType {
            namespace, name, ..
        } = self
        {
            if namespace == "System" {
                return match name.as_str() {
                    "Boolean" => Some("false".to_string()),
                    "Integer" => Some("0".to_string()),
                    "Decimal" => Some("0.0".to_string()),
                    "String" => Some("".to_string()),
                    _ => None,
                };
            }
        }
        None
    }

    /// Check if this type requires special handling in FHIRPath
    pub fn requires_special_handling(&self) -> bool {
        match self {
            TypeReflectionInfo::SimpleType {
                namespace, name, ..
            } => {
                namespace == "FHIR"
                    && matches!(
                        name.as_str(),
                        "Quantity"
                            | "Coding"
                            | "CodeableConcept"
                            | "Reference"
                            | "DateTime"
                            | "Date"
                            | "Time"
                    )
            }
            _ => false,
        }
    }

    /// Get validation rules for this type
    pub fn validation_rules(&self) -> Vec<String> {
        let mut rules = Vec::new();

        match self {
            TypeReflectionInfo::SimpleType {
                namespace, name, ..
            } => {
                if namespace == "System" {
                    match name.as_str() {
                        "Integer" => rules.push("Must be a valid integer".to_string()),
                        "Decimal" => rules.push("Must be a valid decimal number".to_string()),
                        "Boolean" => rules.push("Must be true or false".to_string()),
                        "String" => rules.push("Must be a valid string".to_string()),
                        _ => {}
                    }
                } else if namespace == "FHIR" {
                    match name.as_str() {
                        "id" => {
                            rules.push("Must match pattern [A-Za-z0-9\\-\\.]{1,64}".to_string())
                        }
                        "uri" => rules.push("Must be a valid URI".to_string()),
                        "url" => rules.push("Must be a valid URL".to_string()),
                        "code" => rules.push("Must be a valid code".to_string()),
                        _ => {}
                    }
                }
            }
            TypeReflectionInfo::ClassInfo { elements, .. } => {
                for element in elements {
                    if element.is_required() {
                        rules.push(format!("Element '{}' is required", element.name));
                    }
                }
            }
            _ => {}
        }

        rules
    }

    // === NEW POLYMORPHIC METHODS FOR A1.5 ===

    /// Check if this type supports a specific operation with given operand types
    pub fn supports_operation(&self, operation: &str, operand_types: &[String]) -> bool {
        match operation {
            // FHIRPath comparison operations
            "=" | "!=" | "~" | "!~" => {
                // Most types support equality/inequality
                operand_types.len() == 1
            }
            "<" | "<=" | ">" | ">=" => {
                // Only ordered types support comparison
                self.is_ordered_type() && operand_types.len() == 1
            }
            "+" | "-" | "*" | "/" => {
                // Only numeric types support arithmetic
                self.is_numeric_type() && operand_types.len() == 1
            }
            "and" | "or" | "xor" => {
                // Boolean operations
                self.is_boolean_type() && operand_types.len() == 1
            }
            "in" | "contains" => {
                // Collection operations
                operand_types.len() == 1
            }
            "is" | "as" => {
                // Type checking operations - always supported
                operand_types.len() == 1
            }
            _ => false,
        }
    }

    /// Get all types that this type is compatible with
    pub fn get_compatible_types(&self) -> Vec<String> {
        let mut compatible = vec![self.qualified_name()];

        // Add base type compatibility
        if let Some(base) = self.base_type() {
            compatible.push(base.to_string());
        }

        // Add system type conversions
        if self.is_primitive() {
            match self.name() {
                "Integer" => {
                    compatible.push("System.Decimal".to_string());
                    compatible.push("System.String".to_string());
                }
                "Decimal" => {
                    compatible.push("System.String".to_string());
                }
                "Boolean" => {
                    compatible.push("System.String".to_string());
                }
                "Date" => {
                    compatible.push("System.DateTime".to_string());
                    compatible.push("System.String".to_string());
                }
                "DateTime" => {
                    compatible.push("System.String".to_string());
                }
                "Time" => {
                    compatible.push("System.String".to_string());
                }
                _ => {}
            }
        }

        // All types can be converted to string in some way
        if !compatible.contains(&"System.String".to_string()) {
            compatible.push("System.String".to_string());
        }

        compatible
    }

    /// Check if this type can be converted to target type and return conversion info
    pub fn can_convert_to(&self, target_type: &str) -> ConversionInfo {
        // Check if target type is in our compatible types
        let compatible_types = self.get_compatible_types();

        if compatible_types.contains(&target_type.to_string()) {
            // Determine conversion type
            let conversion_type = if self.qualified_name() == target_type {
                crate::type_system::ConversionType::Implicit // Same type
            } else if self.is_primitive() && target_type == "System.String" {
                crate::type_system::ConversionType::Function // toString()
            } else if self.is_system_type_promotion(target_type) {
                crate::type_system::ConversionType::Implicit // Safe promotion
            } else {
                crate::type_system::ConversionType::Explicit // Requires casting
            };

            ConversionInfo {
                conversion_type,
                conversion_function: self.get_conversion_function(target_type),
                data_loss_possible: self.conversion_may_lose_data(target_type),
                validation_rules: self.get_conversion_validation_rules(target_type),
                performance_cost: self.get_conversion_cost(target_type),
            }
        } else {
            // Conversion not supported
            ConversionInfo {
                conversion_type: crate::type_system::ConversionType::Forbidden,
                conversion_function: None,
                data_loss_possible: false,
                validation_rules: vec![],
                performance_cost: 0.0,
            }
        }
    }

    /// Get all polymorphic variants of this type
    pub fn get_polymorphic_variants(&self) -> Vec<TypeReflectionInfo> {
        let mut variants = vec![self.clone()];

        // Add base type variants
        if let Some(base) = self.base_type() {
            variants.push(TypeReflectionInfo::simple_type(
                self.namespace().unwrap_or(""),
                base,
            ));
        }

        // Add derived types (this would be populated from type registry in real implementation)
        // For now, we'll add common FHIR type hierarchies
        if self.is_fhir_type() {
            match self.name() {
                "Resource" => {
                    variants.push(TypeReflectionInfo::simple_type("FHIR", "DomainResource"));
                }
                "DomainResource" => {
                    variants.push(TypeReflectionInfo::simple_type("FHIR", "Patient"));
                    variants.push(TypeReflectionInfo::simple_type("FHIR", "Observation"));
                    variants.push(TypeReflectionInfo::simple_type("FHIR", "Practitioner"));
                }
                "Element" => {
                    variants.push(TypeReflectionInfo::simple_type("FHIR", "BackboneElement"));
                }
                _ => {}
            }
        }

        variants
    }

    /// Resolve choice type based on polymorphic context
    pub fn resolve_choice_type(&self, context: &PolymorphicContext) -> Option<String> {
        // If this type matches available types in context, return it
        let type_name = self.qualified_name();
        if context.available_types.contains(&type_name) {
            return Some(type_name);
        }

        // Check if any compatible types are available
        let compatible = self.get_compatible_types();
        for available in &context.available_types {
            if compatible.contains(available) {
                return Some(available.clone());
            }
        }

        // Use resolution strategy to pick from available types
        match context.resolution_strategy {
            crate::type_system::ResolutionStrategy::FirstMatch => {
                context.available_types.first().cloned()
            }
            crate::type_system::ResolutionStrategy::MostSpecific => {
                // Find most specific type (lowest in inheritance hierarchy)
                self.find_most_specific_type(&context.available_types)
            }
            crate::type_system::ResolutionStrategy::MostCommon => {
                // Use inference hints to find most common
                self.find_most_common_type(context)
            }
            crate::type_system::ResolutionStrategy::ContextInferred => {
                // Use context clues to infer type
                self.infer_from_context(context)
            }
            crate::type_system::ResolutionStrategy::ExplicitOnly => {
                // Don't resolve - require explicit specification
                None
            }
            crate::type_system::ResolutionStrategy::ConfidenceBased => {
                // Use inference hints with confidence scores
                self.find_highest_confidence_type(context)
            }
        }
    }

    // === HELPER METHODS FOR POLYMORPHIC OPERATIONS ===

    /// Check if this is an ordered type (supports <, >, etc.)
    fn is_ordered_type(&self) -> bool {
        self.is_primitive()
            && matches!(
                self.name(),
                "Integer" | "Decimal" | "Date" | "DateTime" | "Time" | "String"
            )
    }

    /// Check if this is a numeric type
    fn is_numeric_type(&self) -> bool {
        self.is_primitive() && matches!(self.name(), "Integer" | "Decimal")
    }

    /// Check if this is a boolean type
    fn is_boolean_type(&self) -> bool {
        self.is_primitive() && self.name() == "Boolean"
    }

    /// Check if conversion to target is a safe system type promotion
    fn is_system_type_promotion(&self, target_type: &str) -> bool {
        if !self.is_primitive() {
            return false;
        }

        matches!(
            (self.name(), target_type),
            ("Integer", "System.Decimal") | ("Date", "System.DateTime")
        )
    }

    /// Get conversion function name if needed
    fn get_conversion_function(&self, target_type: &str) -> Option<String> {
        if self.is_primitive() && target_type == "System.String" {
            Some("toString".to_string())
        } else if target_type.contains("Integer") {
            Some("toInteger".to_string())
        } else if target_type.contains("Decimal") {
            Some("toDecimal".to_string())
        } else if target_type.contains("Boolean") {
            Some("toBoolean".to_string())
        } else {
            None
        }
    }

    /// Check if conversion may lose data
    fn conversion_may_lose_data(&self, target_type: &str) -> bool {
        match (self.name(), target_type) {
            ("Decimal", "System.Integer") => true, // May lose fractional part
            ("DateTime", "System.Date") => true,   // May lose time component
            ("String", _) if target_type != "System.String" => true, // Parsing may fail
            _ => false,
        }
    }

    /// Get validation rules for conversion
    fn get_conversion_validation_rules(
        &self,
        target_type: &str,
    ) -> Vec<crate::type_system::ValidationRule> {
        let mut rules = vec![];

        if target_type.contains("Integer") && self.name() == "String" {
            rules.push(crate::type_system::ValidationRule {
                rule_id: "string-to-integer".to_string(),
                description: "String must contain valid integer".to_string(),
                validation_expression: Some("matches('^-?\\\\d+$')".to_string()),
                error_message: "Invalid integer format".to_string(),
            });
        }

        rules
    }

    /// Get performance cost of conversion (0.0 = free, 1.0 = expensive)
    fn get_conversion_cost(&self, target_type: &str) -> f32 {
        match (self.name(), target_type) {
            (a, b) if a == b => 0.0,              // Same type = free
            ("Integer", "System.Decimal") => 0.1, // Simple promotion
            (_, "System.String") => 0.2,          // String conversion
            ("String", _) => 0.5,                 // Parsing is more expensive
            _ => 0.3,
        }
    }

    /// Find most specific type from available types
    fn find_most_specific_type(&self, available_types: &[String]) -> Option<String> {
        // In real implementation, would use type hierarchy
        // For now, prefer non-base types
        for type_name in available_types {
            if !type_name.contains("Element") && !type_name.contains("Resource") {
                return Some(type_name.clone());
            }
        }
        available_types.first().cloned()
    }

    /// Find most common type based on usage statistics
    fn find_most_common_type(&self, context: &PolymorphicContext) -> Option<String> {
        // Use inference hints with statistical type
        for hint in &context.inference_hints {
            if hint.hint_type == crate::type_system::InferenceHintType::Statistical
                && context.available_types.contains(&hint.suggested_type)
            {
                return Some(hint.suggested_type.clone());
            }
        }
        None
    }

    /// Infer type from context clues
    fn infer_from_context(&self, context: &PolymorphicContext) -> Option<String> {
        // Use contextual hints
        for hint in &context.inference_hints {
            if hint.hint_type == crate::type_system::InferenceHintType::Contextual
                && context.available_types.contains(&hint.suggested_type)
            {
                return Some(hint.suggested_type.clone());
            }
        }
        None
    }

    /// Find type with highest confidence score
    fn find_highest_confidence_type(&self, context: &PolymorphicContext) -> Option<String> {
        let mut best_type: Option<String> = None;
        let mut best_confidence = 0.0;

        for hint in &context.inference_hints {
            if context.available_types.contains(&hint.suggested_type)
                && hint.confidence > best_confidence
            {
                best_type = Some(hint.suggested_type.clone());
                best_confidence = hint.confidence;
            }
        }

        best_type
    }
}

impl fmt::Display for TypeReflectionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.qualified_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_type() {
        let string_type = TypeReflectionInfo::simple_type("System", "String");
        assert_eq!(string_type.name(), "String");
        assert_eq!(string_type.namespace(), Some("System"));
        assert_eq!(string_type.qualified_name(), "System.String");
        assert!(string_type.is_primitive());
        assert!(!string_type.is_fhir_type());
    }

    #[test]
    fn test_class_type() {
        let elements = vec![
            ElementInfo::new("id", TypeReflectionInfo::simple_type("System", "String"))
                .with_cardinality(0, Some(1)),
            ElementInfo::new("name", TypeReflectionInfo::simple_type("System", "String"))
                .with_cardinality(0, None)
                .with_summary(),
        ];

        let patient_type = TypeReflectionInfo::class_type("FHIR", "Patient", elements);
        assert_eq!(patient_type.name(), "Patient");
        assert_eq!(patient_type.namespace(), Some("FHIR"));
        assert!(patient_type.is_fhir_type());
        assert!(!patient_type.is_primitive());

        let patient_elements = patient_type.elements();
        assert_eq!(patient_elements.len(), 2);
        assert_eq!(patient_elements[0].name, "id");
        assert_eq!(patient_elements[1].name, "name");
        assert!(patient_elements[1].is_summary);
    }

    #[test]
    fn test_list_type() {
        let string_type = TypeReflectionInfo::simple_type("System", "String");
        let list_type = TypeReflectionInfo::list_type(string_type);

        assert!(list_type.is_collection());
        assert_eq!(list_type.name(), "List");
        assert_eq!(list_type.qualified_name(), "List<System.String>");

        let element_type = list_type.element_type().unwrap();
        assert_eq!(element_type.name(), "String");
    }

    #[test]
    fn test_tuple_type() {
        let elements = vec![
            TupleElementInfo::new("x", TypeReflectionInfo::simple_type("System", "Integer")),
            TupleElementInfo::new("y", TypeReflectionInfo::simple_type("System", "Integer")),
        ];

        let tuple_type = TypeReflectionInfo::tuple_type(elements);
        assert_eq!(tuple_type.name(), "Tuple");

        let tuple_elements = tuple_type.tuple_elements();
        assert_eq!(tuple_elements.len(), 2);
        assert_eq!(tuple_elements[0].name, "x");
        assert_eq!(tuple_elements[1].name, "y");
    }

    #[test]
    fn test_element_info() {
        let element = ElementInfo::new("name", TypeReflectionInfo::simple_type("System", "String"))
            .with_cardinality(1, None)
            .with_modifier()
            .with_summary()
            .with_documentation("Patient name");

        assert_eq!(element.name, "name");
        assert!(element.is_required());
        assert!(element.is_multiple());
        assert!(element.is_modifier);
        assert!(element.is_summary);
        assert_eq!(element.documentation.as_deref(), Some("Patient name"));
    }

    #[test]
    fn test_type_hierarchy() {
        let mut hierarchy = TypeHierarchy::new("Patient");
        hierarchy.add_parent("DomainResource".to_string());
        hierarchy.add_parent("Resource".to_string());
        hierarchy.add_child("USCorePatient".to_string());

        assert!(hierarchy.is_ancestor("Resource"));
        assert!(hierarchy.is_descendant("USCorePatient"));
        assert!(!hierarchy.is_ancestor("Observation"));
        assert_eq!(hierarchy.parents.len(), 2);
        assert_eq!(hierarchy.children.len(), 1);
    }

    #[test]
    fn test_type_suggestion() {
        let suggestion = TypeSuggestion::new("Patient", "resource", "FHIR", 0.95)
            .with_description("Patient demographic and administrative information");

        assert_eq!(suggestion.type_name, "Patient");
        assert_eq!(suggestion.kind, "resource");
        assert_eq!(suggestion.relevance_score, 0.95);
        assert!(suggestion.description.is_some());
    }

    #[test]
    fn test_type_compatibility() {
        let integer_type = TypeReflectionInfo::simple_type("System", "Integer");
        let decimal_type = TypeReflectionInfo::simple_type("System", "Decimal");
        let string_type = TypeReflectionInfo::simple_type("System", "String");

        // Integer can be promoted to Decimal
        assert!(integer_type.is_compatible_with(&decimal_type));
        // Both can be converted to String
        assert!(integer_type.is_compatible_with(&string_type));
        assert!(decimal_type.is_compatible_with(&string_type));

        // But not the reverse
        assert!(!decimal_type.is_compatible_with(&integer_type));
    }

    #[test]
    fn test_inheritance_relationships() {
        let base_type = TypeReflectionInfo::simple_type("FHIR", "Element");
        let derived_type =
            TypeReflectionInfo::simple_type_with_base("FHIR", "BackboneElement", "Element");

        assert!(derived_type.is_subtype_of("Element"));
        assert!(!base_type.is_subtype_of("BackboneElement"));

        let ancestors = derived_type.get_ancestors();
        assert_eq!(ancestors, vec!["Element"]);
    }

    #[test]
    fn test_common_supertype() {
        let integer_type = TypeReflectionInfo::simple_type("System", "Integer");
        let decimal_type = TypeReflectionInfo::simple_type("System", "Decimal");

        let common = integer_type.common_supertype(&decimal_type);
        assert!(common.is_some());
        assert_eq!(common.unwrap().name(), "String");
    }

    #[test]
    fn test_collection_element_validation() {
        let simple_type = TypeReflectionInfo::simple_type("System", "String");
        let list_type = TypeReflectionInfo::list_type(simple_type.clone());
        let nested_list = TypeReflectionInfo::list_type(list_type.clone());

        assert!(simple_type.is_valid_collection_element());
        assert!(!list_type.is_valid_collection_element()); // No nested collections
        assert!(!nested_list.is_valid_collection_element());
    }

    #[test]
    fn test_default_values() {
        let boolean_type = TypeReflectionInfo::simple_type("System", "Boolean");
        let integer_type = TypeReflectionInfo::simple_type("System", "Integer");
        let string_type = TypeReflectionInfo::simple_type("System", "String");
        let custom_type = TypeReflectionInfo::simple_type("FHIR", "Patient");

        assert_eq!(boolean_type.default_value(), Some("false".to_string()));
        assert_eq!(integer_type.default_value(), Some("0".to_string()));
        assert_eq!(string_type.default_value(), Some("".to_string()));
        assert_eq!(custom_type.default_value(), None);
    }

    #[test]
    fn test_special_handling_types() {
        let quantity_type = TypeReflectionInfo::simple_type("FHIR", "Quantity");
        let string_type = TypeReflectionInfo::simple_type("System", "String");
        let reference_type = TypeReflectionInfo::simple_type("FHIR", "Reference");

        assert!(quantity_type.requires_special_handling());
        assert!(!string_type.requires_special_handling());
        assert!(reference_type.requires_special_handling());
    }

    #[test]
    fn test_validation_rules() {
        let integer_type = TypeReflectionInfo::simple_type("System", "Integer");
        let id_type = TypeReflectionInfo::simple_type("FHIR", "id");

        let integer_rules = integer_type.validation_rules();
        assert!(!integer_rules.is_empty());
        assert!(integer_rules[0].contains("valid integer"));

        let id_rules = id_type.validation_rules();
        assert!(!id_rules.is_empty());
        assert!(id_rules[0].contains("pattern"));
    }

    #[test]
    fn test_class_type_validation_rules() {
        let elements = vec![
            ElementInfo::new("id", TypeReflectionInfo::simple_type("System", "String"))
                .with_cardinality(0, Some(1)),
            ElementInfo::new("name", TypeReflectionInfo::simple_type("System", "String"))
                .with_cardinality(1, None), // Required element
        ];

        let patient_type = TypeReflectionInfo::class_type("FHIR", "Patient", elements);
        let rules = patient_type.validation_rules();

        assert!(!rules.is_empty());
        assert!(
            rules
                .iter()
                .any(|rule| rule.contains("name") && rule.contains("required"))
        );
    }

    #[test]
    fn test_operation_support() {
        let integer_type = TypeReflectionInfo::simple_type("System", "Integer");
        let string_type = TypeReflectionInfo::simple_type("System", "String");
        let boolean_type = TypeReflectionInfo::simple_type("System", "Boolean");

        // Integer supports arithmetic operations
        assert!(integer_type.supports_operation("+", &["System.Integer".to_string()]));
        assert!(integer_type.supports_operation("<", &["System.Integer".to_string()]));
        assert!(!integer_type.supports_operation("and", &["System.Boolean".to_string()]));

        // Boolean supports logical operations
        assert!(boolean_type.supports_operation("and", &["System.Boolean".to_string()]));
        assert!(!boolean_type.supports_operation("+", &["System.Integer".to_string()]));

        // All types support type checking operations
        assert!(string_type.supports_operation("is", &["System.String".to_string()]));
        assert!(integer_type.supports_operation("as", &["System.Decimal".to_string()]));
    }

    #[test]
    fn test_compatible_types() {
        let integer_type = TypeReflectionInfo::simple_type("System", "Integer");
        let compatible = integer_type.get_compatible_types();

        assert!(compatible.contains(&"System.Integer".to_string()));
        assert!(compatible.contains(&"System.Decimal".to_string()));
        assert!(compatible.contains(&"System.String".to_string()));
    }

    #[test]
    fn test_conversion_info() {
        let integer_type = TypeReflectionInfo::simple_type("System", "Integer");

        // Conversion to Decimal should be implicit
        let decimal_conversion = integer_type.can_convert_to("System.Decimal");
        assert_eq!(
            decimal_conversion.conversion_type,
            crate::type_system::ConversionType::Implicit
        );
        assert!(!decimal_conversion.data_loss_possible);

        // Conversion to String should use toString function
        let string_conversion = integer_type.can_convert_to("System.String");
        assert_eq!(
            string_conversion.conversion_type,
            crate::type_system::ConversionType::Function
        );
        assert_eq!(
            string_conversion.conversion_function,
            Some("toString".to_string())
        );

        // Forbidden conversion
        let forbidden = integer_type.can_convert_to("System.Date");
        assert_eq!(
            forbidden.conversion_type,
            crate::type_system::ConversionType::Forbidden
        );
    }

    #[test]
    fn test_polymorphic_variants() {
        let derived_type =
            TypeReflectionInfo::simple_type_with_base("FHIR", "Patient", "DomainResource");
        let variants = derived_type.get_polymorphic_variants();

        assert!(variants.len() >= 2);
        assert!(variants.iter().any(|v| v.name() == "Patient"));
        assert!(variants.iter().any(|v| v.name() == "DomainResource"));
    }

    #[test]
    fn test_choice_type_resolution() {
        use crate::type_system::{
            InferenceHint, InferenceHintType, PolymorphicContext, ResolutionStrategy,
        };

        let string_type = TypeReflectionInfo::simple_type("System", "String");

        // Create context with available types
        let context = PolymorphicContext {
            current_path: "Patient.name".to_string(),
            base_type: "Patient".to_string(),
            available_types: vec!["System.String".to_string(), "System.Integer".to_string()],
            constraints: vec![],
            inference_hints: vec![InferenceHint {
                hint_type: InferenceHintType::Statistical,
                suggested_type: "System.String".to_string(),
                confidence: 0.8,
                reasoning: "Most common for name fields".to_string(),
            }],
            resolution_strategy: ResolutionStrategy::MostCommon,
            metadata: std::collections::HashMap::new(),
        };

        let resolved = string_type.resolve_choice_type(&context);
        assert_eq!(resolved, Some("System.String".to_string()));
    }

    #[test]
    fn test_type_helper_methods() {
        let integer_type = TypeReflectionInfo::simple_type("System", "Integer");
        let string_type = TypeReflectionInfo::simple_type("System", "String");
        let boolean_type = TypeReflectionInfo::simple_type("System", "Boolean");

        assert!(integer_type.is_numeric_type());
        assert!(!integer_type.is_boolean_type());
        assert!(integer_type.is_ordered_type());

        assert!(!string_type.is_numeric_type());
        assert!(!string_type.is_boolean_type());
        assert!(string_type.is_ordered_type()); // Strings are ordered

        assert!(!boolean_type.is_numeric_type());
        assert!(boolean_type.is_boolean_type());
        assert!(!boolean_type.is_ordered_type()); // Booleans are not ordered
    }
}
