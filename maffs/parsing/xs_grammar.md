# XS Grammar

Syntax for production rules is:

$A \to B\alpha$

The following represent non terminal symbols:
- uppercase letters

The following represent terminal symbols:
- lowercase words/letters
- greek symbols
- [keywords](./xs_keywords.md)

## 1. Program

### 1.1. Literals

$\text{NUM} \rightarrow \text{INT | FLT}$

$\text{VCT} \rightarrow \texttt{vector(}\text{NUM}\texttt{, }\text{NUM}\texttt{, }\text{NUM}\texttt{);}$

### 1.2. Top Level Statements

$\text{X} \rightarrow \text{I X | P}$

$\text{P} \rightarrow \text{RL P | FN P | V}_\text{top}\text{ P | }\epsilon$

where

$\color{gray} \text{X} := \text{XS script}$

$\color{gray} \text{I} := \text{Include statement}$

$\color{gray} \text{P} := \text{Program}$

$\color{gray} \text{RL} := \text{Rule def}$

$\color{gray} \text{FN} := \text{Function def}$

$\color{gray} \text{V}_\text{top} := \text{Top level variable definition}$

### 1.3. Prelude

This consists of all the constants and functions described in

1. [XS Constant Reference](https://ugc.aoe2.rocks/general/xs/constants/)
2. [XS Function Reference](https://ugc.aoe2.rocks/general/xs/functions/)

## 2. Statement
$\text{S} \rightarrow \text{V}_\text{decl}\text{ | V}_\text{def}\text{ | V}_\text{asgn}\text{ | IE | W | F | SC | R | Br | Co}$

$\bar{\text{S}} \rightarrow \text{S }\bar{\text{S}}\text{ | }\epsilon$

$\text{B} \rightarrow \texttt{\{}\bar{\text{ S }}\texttt{\}}$

$\text{BS} \rightarrow \text{B | S}$

where

$\color{gray} \text{S} := \text{Statement}$

$\color{gray} \text{V}_\text{decl} := \text{Variable declaration}$

$\color{gray} \text{V}_\text{def} := \text{Variable definition}$

$\color{gray} \text{V}_\text{asgn} := \text{Variable Assignment}$

$\color{gray} \text{IE} := \text{If (Else) statement}$

$\color{gray} \text{W} := \text{While loop}$

$\color{gray} \text{F} := \text{For loop}$

$\color{gray} \text{SC} := \text{Switch case}$

$\color{gray} \text{R} := \text{Return statement}$

$\color{gray} \text{Br} := \text{Break statement}$

$\color{gray} \text{Co} := \text{Continue statement}$

$\color{gray} \bar{\text{S}} := \text{Statements}$

$\color{gray} \text{B} := \text{Body}$

$\color{gray} \text{BS} := \text{Body or statement}$

### 2.1. Top Level Var Def

$\text{V}_\text{top} \rightarrow \texttt{extern } \text{V}_\text{top}\text{ | }\texttt{const }\text{V}_\text{top}\text{ | }\texttt{static }\text{V}_\text{top}$

$\text{V}_\text{top} \rightarrow \text{DTYPE ID}\texttt{ = }\text{LIT}\texttt{;}$

<!-- todo: double check if top level var defs can have exprs -->

$\text{DTYPE} \rightarrow \texttt{int | float | bool | string | vect}$

$\text{LIT} \rightarrow \text{INT | FLT | STR | VCT | BOOL}$

Note: XS currently has bugs with defining top level strings and vectors

where

$\color{gray}\text{DTYPE} := \text{Datatype}$

$\color{gray}\text{ID} := \text{Identifier}$

$\color{gray}\text{LIT} := \text{Literal}$

### 2.2. Var Decl

$\text{V}_\text{decl} \rightarrow \texttt{static }\text{V}_\text{decl}$

$\text{V}_\text{decl} \rightarrow \text{DTYPE ID}\texttt{;}$

### 2.3. Var Def

$\text{V}_\text{def} \rightarrow \texttt{const }\text{V}_\text{def}$

$\text{V}_\text{def} \rightarrow \text{DTYPE ID}\texttt{ = }\text{E}\texttt{;}$

### 2.4. Var Assign

$\text{V}_\text{asgn} \rightarrow \text{ID}\texttt{ = }\text{E}\texttt{;}$

where

$\color{gray}\text{E} := \text{Expression}$

### 2.5. If Else

$\text{IE} \rightarrow \texttt{if ( }\text{E}\texttt{ ) }\text{BS ELSE}$

$\text{ELSE} \rightarrow \texttt{else }\text{BS | } \epsilon$

where

$\color{gray}\text{ELSE} := \text{Else branch}$

### 2.6. While

$\text{W} \rightarrow \texttt{while ( }\text{E}\texttt{ ) }\text{BS}$

### 2.7. For

$\text{F} \rightarrow \texttt{for ( }\text{V}_\text{asgn} \text{ OP}_\text{rel} \text{ INT} \texttt{ ) }\text{BS}$

where

$\color{gray}\text{OP}_\text{rel} := \text{Relational operators}$

<!-- todo: check for all rel op use -->
<!-- todo: check for exp in for asgn -->

### 2.8. Switch

$\text{SC} \rightarrow \texttt{switch ( E ) \{ } \text{CASES} \texttt{ \}}$

$\text{CASES} \rightarrow \text{CASE CASES | DEFAULT CASES}_\text{no default}\text{ | } \epsilon$

$\text{CASES}_\text{no default} \rightarrow \text{CASE CASES}_\text{no default}\text{ | } \epsilon$

$\text{CASE} \rightarrow \texttt{case} \text{ LIT } \texttt{:} \text{ B}$

$\text{DEFAULT} \rightarrow \texttt{default :} \text{ B}$

where

$\color{gray}\text{CASES} := \text{Optional case statements with one optional default case}$

$\color{gray}\text{CASES}_\text{no default} := \text{Optional case statements only}$

$\color{gray}\text{CASE} := \text{case statement}$

$\color{gray}\text{DEFAULT} := \text{default statement}$

### 2.9. Functions

$\text{FN} \rightarrow \texttt{extern}\text{ FN | }\texttt{mutable}\text{ FN}$

$\text{FN} \rightarrow \text{RTYPE ID ( ARGS ) } \text{B}$

$\text{RTYPE} \rightarrow \texttt{void}\text{ | DTYPE}$

<!-- use the interleave trick, this is still LL(1) -->
$\text{ARGS} \rightarrow \text{ARG | ARG, ARGS}$

$\text{ARG} \rightarrow\text{DTYPE ID = LIT | } \epsilon$

### 2.10. Return

$\text{R} \rightarrow\texttt{return ( } \text{E} \texttt{ );}$

$\text{R} \rightarrow\texttt{return;}$

### 2.11. Rules



### 2.12. Include

$\text{I} \rightarrow \texttt{include }\text{STR}\texttt{;}$

### 2.13 Break

$\text{Br} \rightarrow \texttt{break;}$

### 2.14 Continue

$\text{Co} \rightarrow \texttt{continue;}$

### 2.15. Single Comment
### 2.16. Multiple Line Comment
### 2.17. Docstring

## 3. Expression

### 3.1. Expression
### 3.2. Parenthesis
### 3.3. Identifier
### 3.4. Operators

#### 3.4.1. Postfix Double Plus
#### 3.4.2. Postfix Double Minus

#### 3.4.3. Plus
#### 3.4.4. Minus
#### 3.4.5. Asterisk
#### 3.4.6. Forward Slash
#### 3.4.7. Percent

#### 3.4.8. Equals
#### 3.4.9. Less Than
#### 3.4.10. Greater Than
#### 3.4.11. Less Than or Equal
#### 3.4.12. Greater Than or Equal
#### 3.4.13. Not Equals

#### 3.4.14 And
#### 3.4.15 Or

## 4. The Full Grammar