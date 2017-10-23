//! Abstract syntax tree
//!
//! Types in this module represent various pieces a C program can
//! contain after preprocessing phase. They mostly follow C11 grammar
//! naming conventions.
//!
//! References to C11 standard given in parenthesis refer to the
//! [ISO/IEC 9899:201x
//! draft](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf)
//! published on April 12, 2011.
//!
//! A number of GNU extensions to the standard C are included
//! here. Where appropriate, types, struct fields or enum variants
//! specific to GNU are marked as "GNU extension". Supported
//! extensions are:
//!
//! - attributes in various positions
//! - inline asembly statements and asm labels
//! - extensions to the initializer list syntax
//! - statement expressions
//! - `typeof` type specifiers

use span::Node;

// From 6.4 Lexical elements

/// Variable, function and other names that are not types
///
/// (C11 6.4.2)
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}

/// Constant literals
///
/// C11 places string literals under primary expressions, thus they
/// are not included here.
///
/// (C11 6.4.4)
#[derive(Debug, PartialEq, Clone)]
pub enum Constant {
    Integer(Integer),
    Float(Float),
    Character(String),
}

/// Integer number literal
///
/// (C11 6.4.4.1)
#[derive(Debug, PartialEq, Clone)]
pub enum Integer {
    Decimal(String),
    Octal(String),
    Hexademical(String),
}

/// Floating point number literal
///
/// (C11 6.4.4.2)
#[derive(Debug, PartialEq, Clone)]
pub enum Float {
    Decimal(String),
    Hexademical(String),
}

/// String literal
///
/// (C11 6.4.5)
pub type StringLiteral = Vec<String>;

// From 6.5 Expressions

/// Expressions
///
/// (C11 6.5)
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// Identifier
    ///
    /// May be a variable, function name or enumerator. The latter is
    /// different from the standard, where enumerators are classified
    /// as constants.
    ///
    /// (C11 6.5.1)
    Identifier(Node<Identifier>),
    /// Numeric and character constants
    ///
    /// Enumerator constants, being valid identifiers, are reprented
    /// as `Identifier` in this enum.
    ///
    /// (C11 6.5.1)
    Constant(Node<Constant>),

    /// String literal
    ///
    /// (C11 6.5.1)
    StringLiteral(Node<StringLiteral>),

    /// Generic selection
    ///
    /// (C11 6.5.1.1)
    GenericSelection {
        expression: Box<Node<Expression>>,
        associations: Vec<Node<GenericAssociation>>,
    },

    /// Structure and union members
    ///
    /// Both direct (`.`) and indirect (`->`) access.
    ///
    /// (C11 6.5.2)
    Member {
        operator: Node<MemberOperator>,
        expression: Box<Node<Expression>>,
        identifier: Node<Identifier>,
    },

    /// Function call expression
    ///
    /// (C11 6.5.2)
    Call {
        callee: Box<Node<Expression>>,
        arguments: Vec<Node<Expression>>,
    },

    /// Compound literal
    ///
    /// (C11 6.5.2)
    CompoundLiteral {
        type_name: Node<TypeName>,
        initializer_list: Vec<Node<Initializer>>,
    },

    /// Size of a type
    ///
    /// Note: size of an expression is represented with `UnaryOperator::SizeOf`.
    ///
    /// (C11 6.5.3)
    SizeOf(Node<TypeName>),

    /// Alignment of a type
    ///
    /// (C11 6.5.3)
    AlignOf(Node<TypeName>),

    /// Unary operators
    ///
    /// This represents both postfix and prefix unary oprators. Postfix expressions that take
    /// additional operands are represented by a separate entry in this enum.
    ///
    /// (C11 6.5.2, c11 6.5.3)
    UnaryOperator {
        operator: Node<UnaryOperator>,
        operand: Box<Node<Expression>>,
    },

    /// Cast expression
    ///
    /// `(type) expr`
    ///
    /// (C11 6.5.4)
    Cast {
        type_name: Node<TypeName>,
        expression: Box<Node<Expression>>,
    },

    /// Binary operators
    ///
    /// All of C binary operators that can be applied to two expressions.
    ///
    /// (C11 6.5.5 -- 6.5.16)
    BinaryOperator {
        operator: Node<BinaryOperator>,
        lhs: Box<Node<Expression>>,
        rhs: Box<Node<Expression>>,
    },

    /// Conditional operator
    ///
    /// (C11 6.5.15)
    Conditional {
        condition: Box<Node<Expression>>,
        then_expression: Box<Node<Expression>>,
        else_expression: Box<Node<Expression>>,
    },

    /// Comma operator
    ///
    /// (C11 6.5.17)
    Comma(Vec<Node<Expression>>),

    /// Member offset expression
    ///
    /// Result of expansion of `offsetof` macro.
    ///
    /// (C11 7.19 §3).
    OffsetOf {
        type_name: Node<TypeName>,
        designator: Node<OffsetDesignator>,
    },

    /// Variable argument list access
    ///
    /// Result of expansion of `va_arg` macro.
    ///
    /// (C11 7.16.1.1).
    VaArg {
        va_list: Box<Node<Expression>>,
        type_name: Node<TypeName>,
    },

    /// Statement expression
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Statement-Exprs.html)
    Statement(Node<Statement>),
}

/// Struct or union member access
#[derive(Debug, PartialEq, Clone)]
pub enum MemberOperator {
    /// `expression.identifier`
    Direct,
    /// `expression->identifier`
    Indirect,
}

/// Single element of a generic selection expression
///
/// (C11 6.5.1.1)
#[derive(Debug, PartialEq, Clone)]
pub enum GenericAssociation {
    Type {
        type_name: Node<TypeName>,
        expression: Box<Node<Expression>>,
    },
    Default(Box<Node<Expression>>),
}

/// All operators with one operand
///
/// (C11 6.5)
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    /// `operand++`
    PostIncrement,
    /// `operand--`
    PostDecrement,
    /// `++operand`
    PreIncrement,
    /// `--operand`
    PreDecrement,
    /// `&operand`
    Address,
    /// `*operand`
    Indirection,
    /// `+operand`
    Plus,
    /// `-operand`
    Minus,
    /// `~operand`
    Complement,
    /// `!operand`
    Negate,
    /// `sizeof operand`
    SizeOf,
}

/// All operators with two operands
///
/// (C11 6.5)
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    /// `lhs[rhs]`
    Index,
    /// `lhs * rhs`
    Multiply,
    /// `lhs / rhs`
    Divide,
    /// `lhs % rhs`
    Modulo,
    /// `lhs + rhs`
    Plus,
    /// `lhs - rhs`
    Minus,
    /// `lhs << rhs`
    ShiftLeft,
    /// `lhs >> rhs`
    ShiftRight,
    /// `lhs < rhs`
    Less,
    /// `lhs > rhs`
    Greater,
    /// `lhs <= rhs`
    LessOrEqual,
    /// `lhs >= rhs`
    GreaterOrEqual,
    /// `lhs == rhs`
    Equals,
    /// `lhs != rhs`
    NotEquals,
    /// `lhs & rhs`
    BitwiseAnd,
    /// `lhs ^ rhs`
    BitwiseXor,
    /// `lhs | rhs`
    BitwiseOr,
    /// `lhs && rhs`
    LogicalAnd,
    /// `lhs || rhs`
    LogicalOr,
    /// `lhs = rhs`
    Assign,
    /// `lhs *= rhs`
    AssignMultiply,
    /// `lhs /= rhs`
    AssignDivide,
    /// `lhs %= rhs`
    AssignModulo,
    /// `lhs += rhs`
    AssignPlus,
    /// `lhs -= rhs`
    AssignMinus,
    /// `lhs <<= rhs`
    AssignShiftLeft,
    /// `lhs >>= rhs`
    AssignShiftRight,
    /// `lhs &= rhs`
    AssignBitwiseAnd,
    /// `lhs ^= rhs`
    AssignBitwiseXor,
    /// `lhs |= rhs`
    AssignBitwiseOr,
}

/// Offset designator in a `offsetof` macro expansion
///
/// (C11 7.19 §3).
#[derive(Debug, PartialEq, Clone)]
pub struct OffsetDesignator {
    pub base: Node<Identifier>,
    pub members: Vec<Node<OffsetMember>>,
}

/// Single element of an offset designator
///
/// (C11 7.19 §3).
#[derive(Debug, PartialEq, Clone)]
pub enum OffsetMember {
    Member(Node<Identifier>),
    IndirectMember(Node<Identifier>),
    Index(Node<Expression>),
}

// From 6.7 Declarations

/// Variable, function or type declaration
///
/// (C11 6.7)
#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Declaration {
        specifiers: Vec<Node<DeclarationSpecifier>>,
        declarators: Vec<Node<InitDeclarator>>,
    },
    StaticAssert(Node<StaticAssert>),
}

/// Common part of a declaration
///
/// These apply to all declarators in a declaration.
///
/// (C11 6.7)
#[derive(Debug, PartialEq, Clone)]
pub enum DeclarationSpecifier {
    StorageClass(Node<StorageClassSpecifier>),
    TypeSpecifier(Node<TypeSpecifier>),
    TypeQualifier(Node<TypeQualifier>),
    Function(Node<FunctionSpecifier>),
    Alignment(Node<AlignmentSpecifier>),
    /// Vendor-specific declaration extensions that can be mixed with standard specifiers
    Extension(Vec<Node<Extension>>),
}

/// Defines a single name in a declaration
///
/// (C11 6.7.6)
#[derive(Debug, PartialEq, Clone)]
pub struct InitDeclarator {
    pub declarator: Node<Declarator>,
    pub initializer: Option<Node<Initializer>>,
}

// From 6.7.1

/// Storage class
///
/// (C11 6.7.1)
#[derive(Debug, PartialEq, Clone)]
pub enum StorageClassSpecifier {
    /// `typedef`
    Typedef,
    /// `extern`
    Extern,
    /// `static`
    Static,
    /// `_Thread_local`
    ThreadLocal,
    /// `auto`
    Auto,
    /// `register`
    Register,
}

// From 6.7.2

/// Type specifier
///
/// (C11 6.7.2)
#[derive(Debug, PartialEq, Clone)]
pub enum TypeSpecifier {
    /// `void`
    Void,
    /// `char`
    Char,
    /// `short`
    Short,
    /// `int`
    Int,
    /// `long`
    Long,
    /// `float`
    Float,
    /// `double`
    Double,
    /// `signed`
    ///
    /// `__signed`, `__signed__` (GNU extension)
    Signed,
    /// `unsigned`
    Unsigned,
    /// `_Bool`
    Bool,
    /// `_Complex`
    ///
    /// `__complex`, `__complex__` (GNU extension)
    Complex,
    /// `_Atomic(typename)`
    Atomic(Node<TypeName>),
    /// `struct identifier { … }`
    ///
    /// `union identifier { … }`
    Struct {
        kind: Node<StructType>,
        identifier: Option<Node<Identifier>>,
        declarations: Vec<Node<StructDeclaration>>,
    },
    /// `enum identifier { … }`
    Enum {
        identifier: Option<Node<Identifier>>,
        enumerators: Vec<Node<Enumerator>>,
    },
    /// Name of a previously defined type
    TypedefName(Node<Identifier>),
    /// Specifies type of another type or expression
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Typeof.html)
    TypeOf(Node<TypeOf>),
}

// From 6.7.2.1

/// The only difference between a `struct` and a `union`
///
/// (C11 6.7.2.1)
#[derive(Debug, PartialEq, Clone)]
pub enum StructType {
    Struct,
    Union,
}

/// Single declaration in a struct or a union
///
/// (C11 6.7.2.1)
#[derive(Debug, PartialEq, Clone)]
pub enum StructDeclaration {
    Field {
        specifiers: Vec<Node<SpecifierQualifier>>,
        declarators: Vec<Node<StructDeclarator>>,
    },
    StaticAssert(Node<StaticAssert>),
}

/// Type and qualifiers for a struct declaration
///
/// C11 also uses this type in a few other places.
///
/// (C11 6.7.2.1)
#[derive(Debug, PartialEq, Clone)]
pub enum SpecifierQualifier {
    TypeSpecifier(Node<TypeSpecifier>),
    TypeQualifier(Node<TypeQualifier>),
}

/// Field declarator for a struct or a union
///
/// (C11 6.7.2.1)
#[derive(Debug, PartialEq, Clone)]
pub struct StructDeclarator {
    pub declarator: Option<Node<Declarator>>,
    pub bit_width: Option<Box<Node<Expression>>>,
}

// From 6.7.2.2

/// Single constant inside a `enum` definition
///
/// (C11 6.7.2.2)
#[derive(Debug, PartialEq, Clone)]
pub struct Enumerator {
    pub identifier: Node<Identifier>,
    pub expression: Option<Box<Node<Expression>>>,
}

// From 6.7.3

/// Type qualifier
///
/// (C11 6.7.3)
#[derive(Debug, PartialEq, Clone)]
pub enum TypeQualifier {
    /// `const`
    ///
    /// `__const` (GNU extension)
    Const,
    /// `restrict`
    ///
    /// `__restrict`, `__restrict__` (GNU extension)
    Restrict,
    /// `volatile`
    ///
    /// `__volatile`, `__volatile__` (GNU extension)
    Volatile,
    /// `_Atomic`
    Atomic,
}

// From 6.7.4

/// Function specifier
///
/// (C11 6.7.4)
#[derive(Debug, PartialEq, Clone)]
pub enum FunctionSpecifier {
    /// `inline`
    ///
    /// `__inline`, `__inline__` (GNU extension)
    Inline,
    /// `_Noreturn`
    Noreturn,
}

// From 6.7.5

/// Alignment specifier
///
/// (C11 6.7.5)
#[derive(Debug, PartialEq, Clone)]
pub enum AlignmentSpecifier {
    /// `_Alignas(typename)`
    Type(Node<TypeName>),
    /// `_Alignas(expression)`
    Constant(Box<Node<Expression>>),
}

// From 6.7.6 Declarators

/// Single item in a declaration
///
/// Represents both normal and abstract declarators.
///
/// (C11 6.7.6, 6.7.7)
#[derive(Debug, PartialEq, Clone)]
pub struct Declarator {
    /// What is being declared
    pub kind: Node<DeclaratorKind>,
    /// Contains pointer, array and function declarator elements
    pub derived: Vec<Node<DerivedDeclarator>>,
    /// Vendor-specific extensions
    pub extensions: Vec<Node<Extension>>,
}

/// Name of a declarator
///
/// (C11 6.7.6, 6.7.7)
#[derive(Debug, PartialEq, Clone)]
pub enum DeclaratorKind {
    /// Unnamed declarator
    ///
    /// E.g. part of a function prototype without parameter names.
    Abstract,
    /// Named declarator
    ///
    /// E.g. a variable or a named function parameter.
    Identifier(Node<Identifier>),
    /// Nested declarator
    ///
    /// Any group of parenthesis inside a declarator. E.g. pointer to
    /// a function.
    Declarator(Box<Node<Declarator>>),
}

/// Modifies declarator type
///
/// (C11 6.7.6)
#[derive(Debug, PartialEq, Clone)]
pub enum DerivedDeclarator {
    /// `* qualifiers …`
    Pointer(Vec<Node<PointerQualifier>>),
    /// `… []`
    Array {
        qualifiers: Vec<Node<TypeQualifier>>,
        size: ArraySize,
    },
    /// `… ( parameters )`
    Function {
        parameters: Vec<Node<ParameterDeclaration>>,
        ellipsis: Ellipsis,
    },
    /// `… ( identifiers )`
    KRFunction(Vec<Node<Identifier>>),
}

/// List of qualifiers that can follow a `*` in a declaration
///
/// (C11 6.7.6.1)
#[derive(Debug, PartialEq, Clone)]
pub enum PointerQualifier {
    TypeQualifier(Node<TypeQualifier>),
    Extension(Vec<Node<Extension>>),
}

/// Size of an array in a declaration
///
/// (C11 6.7.6.2)
#[derive(Debug, PartialEq, Clone)]
pub enum ArraySize {
    /// `[]`
    Unknown,
    /// `[*]`
    VariableUnknown,
    /// `[10]`
    VariableExpression(Box<Node<Expression>>),
    /// `[static 10]`
    StaticExpression(Box<Node<Expression>>),
}

/// Complete parameter declaration in a function prototype or declaration
///
/// This is so called "new-style" or "C89" parameter declaration that
/// follows in parenthesis after a function name. "Old-style" or "K&R"
/// function parameter declaration are collected in the
/// `FunctionDefinition::declarations` field.
///
/// (C11 6.7.6.3)
#[derive(Debug, PartialEq, Clone)]
pub struct ParameterDeclaration {
    pub specifiers: Vec<Node<DeclarationSpecifier>>,
    pub declarator: Option<Node<Declarator>>,
    pub extensions: Vec<Node<Extension>>,
}

/// Whether function signature ends with a `...`
#[derive(Debug, PartialEq, Clone)]
pub enum Ellipsis {
    Some,
    None,
}

// From 6.7.7 Type names

/// References to types outside of declarations
///
/// Type names contain only abstract declarators.
///
/// (C11 6.7.7)
#[derive(Debug, PartialEq, Clone)]
pub struct TypeName {
    pub specifiers: Vec<Node<SpecifierQualifier>>,
    pub declarator: Option<Node<Declarator>>,
}

// From 6.7.9 Initialization

/// Value that is assigned immediately in a declaration
///
/// (C11 6.7.9)
#[derive(Debug, PartialEq, Clone)]
pub enum Initializer {
    Expression(Box<Node<Expression>>),
    List(Vec<Node<InitializerListItem>>),
}

/// Initializes one field or array element in a initializer list
///
/// (C11 6.7.9)
#[derive(Debug, PartialEq, Clone)]
pub struct InitializerListItem {
    pub designation: Vec<Node<Designator>>,
    pub initializer: Box<Node<Initializer>>,
}

/// Single element of an designation in an initializer
#[derive(Debug, PartialEq, Clone)]
pub enum Designator {
    /// Array element
    ///
    /// `{ [expression] = … }`
    ///
    /// `{ [expression] … }` (obsolete GNU extension)
    Index(Node<Expression>),

    /// Struct or union member
    ///
    /// `{ .identifier = … }`
    ///
    /// `{ identifier: … }` (obsolete GNU extension)
    Member(Node<Identifier>),

    /// Range of array elements
    ///
    /// `{ [from ... to] … }`
    /// ([GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Designated-Inits.html#Designated-Inits))
    Range {
        from: Node<Expression>,
        to: Node<Expression>,
    },
}

// From 6.7.10 Static assertions


/// Static assertion
///
/// (C11 6.7.10)
#[derive(Debug, PartialEq, Clone)]
pub struct StaticAssert {
    pub expression: Box<Node<Expression>>,
    pub message: Node<StringLiteral>,
}

// From 6.8 Statement

/// Element of a function body
///
/// (C11 6.8)
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Labeled {
        label: Node<Label>,
        statement: Box<Node<Statement>>,
    },
    Compound(Vec<Node<BlockItem>>),
    Expression(Option<Box<Node<Expression>>>),
    If {
        condition: Box<Node<Expression>>,
        then_statement: Box<Node<Statement>>,
        else_statement: Option<Box<Node<Statement>>>,
    },
    Switch {
        expression: Box<Node<Expression>>,
        statement: Box<Node<Statement>>,
    },
    While {
        expression: Box<Node<Expression>>,
        statement: Box<Node<Statement>>,
    },
    DoWhile {
        statement: Box<Node<Statement>>,
        expression: Box<Node<Expression>>,
    },
    For {
        initializer: Node<ForInitializer>,
        condition: Option<Box<Node<Expression>>>,
        step: Option<Box<Node<Expression>>>,
        statement: Box<Node<Statement>>,
    },
    Goto(Node<Identifier>),
    Continue,
    Break,
    Return(Option<Box<Node<Expression>>>),
    /// Vendor specific inline assembly extensions
    Asm(Node<AsmStatement>),
}

/// Statement labels for `goto` and `switch`
#[derive(Debug, PartialEq, Clone)]
pub enum Label {
    /// Goto label
    ///
    /// `ident: ...`
    Identifier(Node<Identifier>),
    /// Case in a `switch` statement
    ///
    /// `case 'a': ...`
    Case(Box<Node<Expression>>),
    /// Default case in a `switch` statement
    ///
    /// `default: ...`
    Default,
}

/// First element of a `for` statement
#[derive(Debug, PartialEq, Clone)]
pub enum ForInitializer {
    /// `for(; ...)`
    Empty,
    /// `for(a = 1; ...)`
    Expression(Box<Node<Expression>>),
    /// `for(int a = 1; ...)`
    Declaration(Node<Declaration>),
}

// From 6.8.2

/// Element of a compound statement
#[derive(Debug, PartialEq, Clone)]
pub enum BlockItem {
    Declaration(Node<Declaration>),
    Statement(Node<Statement>),
}

// From 6.9 External definitions

/// Entire C source file after preprocessing
///
/// (C11 6.9)
#[derive(Debug, PartialEq, Clone)]
pub struct TranslationUnit(pub Vec<Node<ExternalDeclaration>>);

/// Top-level elements of a C program
///
/// (C11 6.9)
#[derive(Debug, PartialEq, Clone)]
pub enum ExternalDeclaration {
    Declaration(Node<Declaration>),
    FunctionDefinition(Node<FunctionDefinition>),
}

/// Function definition
///
/// (C11 6.9.1)
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDefinition {
    /// Return type of the function, possibly mixed with other specifiers
    pub specifiers: Vec<Node<DeclarationSpecifier>>,
    /// Contains function name and parameter list
    pub declarator: Node<Declarator>,
    /// K&R style parameter type definitions (C11 6.9.1 §6)
    pub declarations: Vec<Node<Declaration>>,
    /// Body of the function.
    pub statement: Node<Statement>,
}

// Syntax extensions

/// Extended vendor-specific syntax that does not fit elsewhere
#[derive(Debug, PartialEq, Clone)]
pub enum Extension {
    /// Attributes
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Attribute-Syntax.html)
    Attribute {
        name: Node<String>,
        arguments: Vec<Node<Expression>>,
    },
    /// Assembler name for an object
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Asm-Labels.html)
    AsmLabel(Node<StringLiteral>),
}

/// Inline assembler
#[derive(Debug, PartialEq, Clone)]
pub enum AsmStatement {
    /// Basic asm statement with just source code
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Basic-Asm.html)
    GnuBasic(Node<StringLiteral>),

    /// Extended statement that has access to C variables
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Extended-Asm.html)
    GnuExtended {
        qualifier: Option<Node<TypeQualifier>>,
        template: Node<StringLiteral>,
        outputs: Vec<Node<GnuAsmOperand>>,
        inputs: Vec<Node<GnuAsmOperand>>,
        clobbers: Vec<Node<StringLiteral>>,
    },
}

/// Single input or output operand specifier for GNU extended asm statement
///
/// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Extended-Asm.html#Output-Operands)
#[derive(Debug, PartialEq, Clone)]
pub struct GnuAsmOperand {
    pub symbolic_name: Option<Node<Identifier>>,
    pub constraints: Node<StringLiteral>,
    pub variable_name: Node<Expression>,
}

/// Type of an expression or type
///
/// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Typeof.html)
#[derive(Debug, PartialEq, Clone)]
pub enum TypeOf {
    Expression(Node<Expression>),
    Type(Node<TypeName>),
}