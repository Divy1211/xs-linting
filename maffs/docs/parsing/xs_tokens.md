# XS Tokens

The following is a list of XS tokens

## 1. Operators

### 1.1. Arithmetic Operators

$\text{PLUS} :=\ \texttt{+}$

$\text{MINUS} :=\ \texttt{-}$

$\text{STAR} :=\ \texttt{*}$

$\text{FSLASH} :=\ \texttt{/}$

$\text{PCENT} :=\ \texttt{\%}$


### 1.2. Prefix/Postfix Operators

$\text{DPLUS} :=\ \texttt{++}$

$\text{DMINUS} :=\ \texttt{--}$



### 1.3. Relational Operators

$\text{LT} :=\ \texttt{<}$

$\text{GT} :=\ \texttt{>}$

$\text{LE} :=\ \texttt{<=}$

$\text{GE} :=\ \texttt{>=}$

$\text{DEQ} :=\ \texttt{==}$

$\text{NEQ} :=\ \texttt{!=}$


### 1.4. Boolean Operators

$\text{AND} :=\ \texttt{\&\&}$

$\text{OR} :=\ \texttt{||}$


## 2. Punctuations

$\text{EQ} :=\ \texttt{=}$

$\text{LBRACE} :=\ \texttt{\{}$

$\text{RBRACE} :=\ \texttt{\}}$

$\text{LPAREN} :=\ \texttt{(}$

$\text{RPAREN} :=\ \texttt{)}$

$\text{SEMICOLON} :=\ \texttt{;}$

$\text{COLON} :=\ \texttt{:}$

$\text{COMMA} :=\ \texttt{,}$

$\text{DOT} :=\ \texttt{.}$


## 3. Literals

$\text{INT} :=\ \texttt{-999999999} \text{ to } \texttt{999999999}$

$\text{FLT} :=\ \texttt{-?\textbackslash d+.\textbackslash d+} \text{ where } \texttt{\textbackslash d} \text{ is a digit}$

$\text{STR} :=\ \texttt{"\textbackslash w*"}  \text{ where } \texttt{\textbackslash w} \text{ is a unicode character}$

$\text{BOOL} :=\ \texttt{false} \text{ or } \texttt{true}$

$\text{ID} :=\ \texttt{[a-zA-Z\_][a-zA-Z\_0-9]*}$

Note: Vectors are part of the grammar and are not treated as literals

## 4. Comments

### 4.1. Single Comment

$\text{CMT} := \texttt{//[\^ \textbackslash n]*}$

### 4.2. Multiple Line Comment

$\text{CMT} := /* .* */$

## XS Keywords

every XS keyword is a token

[XS Keywords](./xs_keywords.md)