// This file is @generated by syn-serde-internal-codegen.
// It is not intended for manual editing.

use crate::*;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttrStyle {
    Outer,
    Inner,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expr {
    Array(ExprArray),
    Assign(ExprAssign),
    AssignOp(ExprAssignOp),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(ExprBlock),
    Box(ExprBox),
    Break(ExprBreak),
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Continue(ExprContinue),
    Field(ExprField),
    ForLoop(ExprForLoop),
    Group(ExprGroup),
    If(ExprIf),
    Index(ExprIndex),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    Reference(ExprReference),
    Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    Try(ExprTry),
    TryBlock(ExprTryBlock),
    Tuple(ExprTuple),
    Type(ExprType),
    Unary(ExprUnary),
    Unsafe(ExprUnsafe),
    Verbatim(TokenStream),
    While(ExprWhile),
    Yield(ExprYield),
    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Fields {
    Named(FieldsNamed),
    Unnamed(FieldsUnnamed),
    Unit,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FnArg {
    Receiver(Receiver),
    Typed(PatType),
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForeignItem {
    Fn(ForeignItemFn),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
    Verbatim(TokenStream),
    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Binding(Binding),
    Constraint(Constraint),
    Const(Expr),
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenericMethodArgument {
    Type(Type),
    Const(Expr),
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenericParam {
    Type(TypeParam),
    Lifetime(LifetimeDef),
    Const(ConstParam),
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImplItem {
    Const(ImplItemConst),
    Method(ImplItemMethod),
    Type(ImplItemType),
    Macro(ImplItemMacro),
    Verbatim(TokenStream),
    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Item {
    Const(ItemConst),
    Enum(ItemEnum),
    ExternCrate(ItemExternCrate),
    Fn(ItemFn),
    ForeignMod(ItemForeignMod),
    Impl(ItemImpl),
    Macro(ItemMacro),
    Macro2(ItemMacro2),
    Mod(ItemMod),
    Static(ItemStatic),
    Struct(ItemStruct),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Type(ItemType),
    Union(ItemUnion),
    Use(ItemUse),
    Verbatim(TokenStream),
    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Lit {
    Str(LitStr),
    ByteStr(LitByteStr),
    Byte(LitByte),
    Char(LitChar),
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
    Verbatim(Literal),
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MacroDelimiter {
    Paren,
    Brace,
    Bracket,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Meta {
    Path(Path),
    List(MetaList),
    NameValue(MetaNameValue),
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NestedMeta {
    Meta(Meta),
    Lit(Lit),
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathArguments {
    None,
    AngleBracketed(AngleBracketedGenericArguments),
    Parenthesized(ParenthesizedGenericArguments),
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraitBoundModifier {
    None,
    Maybe,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraitItem {
    Const(TraitItemConst),
    Method(TraitItemMethod),
    Type(TraitItemType),
    Macro(TraitItemMacro),
    Verbatim(TokenStream),
    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeParamBound {
    Trait(TraitBound),
    Lifetime(Lifetime),
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WherePredicate {
    Type(PredicateType),
    Lifetime(PredicateLifetime),
    Eq(PredicateEq),
}
