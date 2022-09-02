use core::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub enum ReportCode {
    AssertWrongType,
    ParseFail,
    CompilerVersionError,
    WrongTypesInAssignOperation,
    WrongNumberOfArguments(usize, usize),
    UndefinedFunction,
    UndefinedTemplate,
    UninitializedSymbolInExpression,
    UnableToTypeFunction,
    UnreachableConstraints,
    UnknownIndex,
    UnknownDimension,
    SameFunctionDeclaredTwice,
    SameTemplateDeclaredTwice,
    SameSymbolDeclaredTwice,
    StaticInfoWasOverwritten,
    SignalInLineInitialization,
    SignalOutsideOriginalScope,
    FunctionWrongNumberOfArguments,
    FunctionInconsistentTyping,
    FunctionPathWithoutReturn,
    FunctionReturnError,
    ForbiddenDeclarationInFunction,
    NonHomogeneousArray,
    NonBooleanCondition,
    NonCompatibleBranchTypes,
    NonEqualTypesInExpression,
    NonExistentSymbol,
    NoMainFoundInProject,
    NoCompilerVersionWarning,
    MultipleMainInComponent,
    TemplateCallAsArgument,
    TemplateWrongNumberOfArguments,
    TemplateWithReturnStatement,
    TypeCantBeUseAsCondition,
    EmptyArrayInlineDeclaration,
    PrefixOperatorWithWrongTypes,
    ParallelOperatorWithWrongTypes,
    InfixOperatorWithWrongTypes,
    InvalidArgumentInCall,
    InconsistentReturnTypesInBlock,
    InconsistentStaticInformation,
    InvalidArrayAccess,
    InvalidSignalAccess,
    InvalidArraySize,
    InvalidArrayType,
    ForStatementIllConstructed,
    BadArrayAccess,
    AssigningAComponentTwice,
    AssigningASignalTwice,
    NotAllowedOperation,
    ConstraintGeneratorInFunction,
    WrongSignalTags,
    InvalidPartialArray,
    MustBeSingleArithmetic,
    ExpectedDimDiffGotDim(usize, usize),
    RuntimeError,
    RuntimeWarning,
    UnknownTemplate,
    NonQuadratic,
    NonConstantArrayLength,
    NonComputableExpression,
    // Constraint analysis codes
    UnconstrainedSignal,
    OneConstraintIntermediate,
    NoOutputInInstance,
    ErrorWat2Wasm,
    CustomGateIntermediateSignalWarning,
    CustomGateConstraintError,
    CustomGateSubComponentError,
    CustomGatesPragmaError,
    CustomGatesVersionError,
}
impl fmt::Display for ReportCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use self::ReportCode::*;
        let string_format = match self {
            ParseFail => "P1000",
            NoMainFoundInProject => "P1001",
            MultipleMainInComponent => "P1002",
            CompilerVersionError => "P1003",
            NoCompilerVersionWarning => "P1004",
            WrongTypesInAssignOperation => "T2000",
            UndefinedFunction => "T2001",
            UndefinedTemplate => "T2002",
            UninitializedSymbolInExpression => "T2003",
            UnableToTypeFunction => "T2004",
            UnreachableConstraints => "T2005",
            SameFunctionDeclaredTwice => "T2006",
            SameTemplateDeclaredTwice => "T2007",
            SameSymbolDeclaredTwice => "T2008",
            StaticInfoWasOverwritten => "T2009",
            SignalInLineInitialization => "T2010",
            SignalOutsideOriginalScope => "T2011",
            FunctionWrongNumberOfArguments => "T2012",
            FunctionInconsistentTyping => "T2013",
            FunctionPathWithoutReturn => "T2014",
            FunctionReturnError => "T2015",
            ForbiddenDeclarationInFunction => "T2016",
            NonHomogeneousArray => "T2017",
            NonBooleanCondition => "T2018",
            NonCompatibleBranchTypes => "T2019",
            NonEqualTypesInExpression => "T2020",
            NonExistentSymbol => "T2021",
            TemplateCallAsArgument => "T2022",
            TemplateWrongNumberOfArguments => "T2023",
            TemplateWithReturnStatement => "T2024",
            TypeCantBeUseAsCondition => "T2025",
            EmptyArrayInlineDeclaration => "T2026",
            PrefixOperatorWithWrongTypes => "T2027",
            ParallelOperatorWithWrongTypes => "T2047",
            InfixOperatorWithWrongTypes => "T2028",
            InvalidArgumentInCall => "T2029",
            InconsistentReturnTypesInBlock => "T2030",
            InconsistentStaticInformation => "T2031",
            InvalidArrayAccess => "T2032",
            InvalidSignalAccess => "T2046",
            InvalidArraySize => "T2033",
            InvalidArrayType => "T2034",
            ForStatementIllConstructed => "T2035",
            BadArrayAccess => "T2035",
            AssigningAComponentTwice => "T2036",
            AssigningASignalTwice => "T2037",
            NotAllowedOperation => "T2038",
            ConstraintGeneratorInFunction => "T2039",
            WrongSignalTags => "T2040",
            AssertWrongType => "T2041",
            UnknownIndex => "T2042",
            InvalidPartialArray => "T2043",
            MustBeSingleArithmetic => "T2044",
            ExpectedDimDiffGotDim(..) => "T2045",
            RuntimeError => "T3001",
            RuntimeWarning => "T3002",
            UnknownDimension => "T20460",
            UnknownTemplate => "T20461",
            NonQuadratic => "T20462",
            NonConstantArrayLength => "T20463",
            NonComputableExpression => "T20464",
            WrongNumberOfArguments(..) => "T20465",
            // Constraint analysis codes
            UnconstrainedSignal => "CA01",
            OneConstraintIntermediate => "CA02",
            NoOutputInInstance => "CA03",
            ErrorWat2Wasm => "W01",
            CustomGateIntermediateSignalWarning => "CG01",
            CustomGateConstraintError => "CG02",
            CustomGateSubComponentError => "CG03",
            CustomGatesPragmaError => "CG04",
            CustomGatesVersionError => "CG05",
        };
        f.write_str(string_format)
    }
}