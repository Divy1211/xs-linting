# XS Operational Semantics

This is a formal description of how a [well typed XS program](../static/xs_type_chk.md#11-well-typed-programs) runs.

## 1. Notation

- $\Delta$ is a memory environment mapping XS identifiers to values
- $\Delta \vdash E_1 \Downarrow E_2$ means that $E_1$ reduces to $E_2$ in $\Delta$ (read as $\Delta$ yields $E_1$ reduced to $E_2$)
- $(\Delta, S) \Downarrow \Delta'$ means that running statement $S$ changes the memory environment to $\Delta'$. A reduction may optionally yield more statements/values along with the modified memory environment.
- $$\begin{array}{rc}
    {\tt (xsBssCase)} & \begin{array}{c}
    \begin{array}{cc} C_1 & C_2 \end{array}
    \\ \hline
    S_1
    \end{array}
    \end{array}
    $$

    is read as $C_1 \land C_2 \implies S_1$

## 2. Big Step Semantics For Expressions

### 2.1. Literals

let $L$ denote a literal

$$\begin{matrix}{\tt (xsBssLit)} & \Delta \vdash L \Downarrow L \end{matrix}$$

### 2.2. Identifiers

let $X$ be an identifier

$$\begin{matrix}{\tt (xsBssId)} & \Delta \vdash X \Downarrow \Delta(X) \end{matrix}$$

### 2.3. Parenthesis

$$
\begin{array}{rc}
    {\tt (xsBssParen)} & \begin{array}{c}
        \Delta \vdash E \Downarrow L
        \\ \hline
        \Delta \vdash (E) \Downarrow L
    \end{array}
\end{array}
$$

### 2.4. Function Call (Expression)

$$
\begin{array}{rc}
    {\tt (xsBssFncExpr)} & \begin{array}{c}
        \begin{array}{cccc}
            \Delta \vdash {\tt fnName} \Downarrow (\{{\tt (id_1, L_1), ..., (id_n, L_n)}\}, \bar{S})
            & \Delta \vdash E_j \Downarrow V_j
            & (\Delta \oplus ({\tt id_1}, V_1) \oplus ... \oplus ({\tt id_j}, V_j) \oplus ... \oplus ({\tt id_n}, L_n), \bar{S}) \Downarrow (\Delta', {\tt return\ (E_r)}; \bar{S}')
            & \Delta \vdash E_r \Downarrow L_r
        \end{array}
        \\ \hline
        \Delta \vdash {\tt fnName(E_1, ..., E_j)} \Downarrow (\Delta', L_r)
    \end{array}
\end{array}
$$

See [§ 3.9. Function Definition](#39-function-definition)

### 2.5. Operations

$$
\begin{array}{rc}
    {\tt (xsBssOp)} & \begin{array}{c}
        \begin{array}{ccc}
            \Delta \vdash E_1 \Downarrow L_1
            & \Delta \vdash E_2 \Downarrow L_2
            & \Delta \vdash L_1\ {\tt op}\ L_2 \Downarrow L_3
        \end{array}
        \\ \hline
        \Delta \vdash E_1\ {\tt op}\ E_2 \Downarrow L_3
    \end{array}
\end{array}
$$

## 3. Big Step Semantics For Statements

### 3.1. Sequence

$$
\begin{array}{rc}
    {\tt (xsBssSeq)} & \begin{array}{c}
        \begin{array}{cc}
            (\Delta, S) \Downarrow \Delta'
            & (\Delta', \bar{S}) \Downarrow \Delta''
        \end{array}
        \\ \hline
        (\Delta, S \bar{S}) \Downarrow \Delta''
    \end{array}
\end{array}
$$

### 3.2. Include

let $X$ be a named, well typed XS program

$$
\begin{array}{rc}
    {\tt (xsBssInc)} & \begin{array}{c}
        \begin{array}{cc}
            X := \bar{S}
            & (\{\}, \bar{S}) \Downarrow \Delta_X
        \end{array}
        \\ \hline
        (\Delta, {\tt include}\ X;) \Downarrow \Delta \oplus \Delta_X
    \end{array}
\end{array}
$$

### 3.3. Var Def/Assign

let $X$ be an identifier

$$
\begin{array}{rc}
    {\tt (xsBssAssign)} & \begin{array}{c}
        \begin{array}{c}
            \Delta \vdash E \Downarrow L
        \end{array}
        \\ \hline
        (\Delta, X\ =\ E;) \Downarrow \Delta \oplus (X, L)
    \end{array}
\end{array}
$$

### 3.4. If Else

$$
\begin{array}{rc}
    {\tt (xsBssIfT)} & \begin{array}{c}
        \begin{array}{cc}
            \Delta \vdash E_c \Downarrow {\tt true}
            & (\Delta, S_1) \Downarrow \Delta_t
        \end{array}
        \\ \hline
        (\Delta, {\tt if\ (} E_c {\tt)\ \{\ } \bar{S_1} {\tt\ \}\ else\ \{\ } \bar{S_2} {\tt\ \}}) \Downarrow \Delta_t
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsBssIfF)} & \begin{array}{c}
        \begin{array}{cc}
            \Delta \vdash E_c \Downarrow {\tt false}
            & (\Delta, S_2) \Downarrow \Delta_f
        \end{array}
        \\ \hline
        (\Delta, {\tt if\ (} E_c {\tt)\ \{\ } \bar{S_1} {\tt\ \}\ else\ \{\ } \bar{S_2} {\tt\ \}}) \Downarrow \Delta_f
    \end{array}
\end{array}
$$

### 3.5. While

$$
\begin{array}{rc}
    {\tt (xsBssWhileT)} & \begin{array}{c}
        \begin{array}{cc}
            \Delta \vdash E_c \Downarrow {\tt true}
            & (\Delta, \bar{S}; {\tt while\ (} E_c {\tt)\ \{\ } \bar{S} {\tt\ \}}) \Downarrow \Delta'
        \end{array}
        \\ \hline
        (\Delta, {\tt while\ (} E_c {\tt)\ \{\ } \bar{S} {\tt\ \}}) \Downarrow \Delta'
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsBssWhileTBr)} & \begin{array}{c}
        \begin{array}{cc}
            \Delta \vdash E_c \Downarrow {\tt true}
            & (\Delta, \bar{S}) \Downarrow (\Delta', {\tt break}; \bar{S}')
        \end{array}
        \\ \hline
        (\Delta, {\tt while\ (} E_c {\tt)\ \{\ } \bar{S} {\tt\ \}}) \Downarrow \Delta'
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsBssWhileTCo)} & \begin{array}{c}
        \begin{array}{ccc}
            \Delta \vdash E_c \Downarrow {\tt true}
            & (\Delta, \bar{S}) \Downarrow (\Delta', {\tt coninue}; \bar{S}')
            & (\Delta', {\tt while\ (} E_c {\tt)\ \{\ } \bar{S} {\tt\ \}}) \Downarrow \Delta''
        \end{array}
        \\ \hline
        (\Delta, {\tt while\ (} E_c {\tt)\ \{\ } \bar{S} {\tt\ \}}) \Downarrow \Delta''
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsBssWhileF)} & \begin{array}{c}
        \begin{array}{cc}
            \Delta \vdash E_c \Downarrow {\tt false}
        \end{array}
        \\ \hline
        (\Delta, {\tt while\ (} E_c {\tt)\ \{\ } \bar{S} {\tt\ \}}) \Downarrow \Delta
    \end{array}
\end{array}
$$

### 3.6. For

$$
\begin{array}{rc}
    {\tt (xsBssForInc)} & \begin{array}{c}
        \begin{array}{ccc}
            \Delta \vdash E_1 \Downarrow L_1
            & {\tt op} \in \{{\tt <, <=}\}
            & (\Delta \oplus (X, L_1), {\tt while\ (} X {\tt op}\ E_2 {\tt)\ \{\ } \bar{S}; {\tt\ X++; \}}) \Downarrow \Delta'
        \end{array}
        \\ \hline
        (\Delta, {\tt for\ (}X\ =\ E_1{\tt ;}\ {\tt op}\ E_2 {\tt)\ \{\ } \bar{S} {\tt\ \}}) \Downarrow \Delta' \ominus X
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsBssForDec)} & \begin{array}{c}
        \begin{array}{ccc}
            \Delta \vdash E_1 \Downarrow L_1
            & {\tt op} \in \{{\tt >, >=}\}
            & (\Delta \oplus (X, L_1), {\tt while\ (} X {\tt op}\ E_2 {\tt)\ \{\ } \bar{S}; {\tt\ X--; \}}) \Downarrow \Delta'
        \end{array}
        \\ \hline
        (\Delta, {\tt for\ (}X\ =\ E_1{\tt ;}\ {\tt op}\ E_2 {\tt)\ \{\ } \bar{S} {\tt\ \}}) \Downarrow \Delta' \ominus X
    \end{array}
\end{array}
$$

### 3.7. Switch Case

$$
\begin{array}{rc}
    {\tt (xsBssSwitchC)} & \begin{array}{c}
        \begin{array}{cccc}
            \Delta \vdash E_c \Downarrow L_c
            & \Delta \vdash E_i \Downarrow L_i
            & \min\limits_j L_j = L_c
            & (\Delta, \bar{S}_j) \Downarrow \Delta_j
        \end{array}
        \\ \hline
        (\Delta, {\tt switch\ (} E_c {\tt)\ \{\ } {\tt case\ } E_1 {\tt\ :\ \{\ } \bar{S_1} {\tt\ \}} {\tt\ ...\ case\ } E_n {\tt\ :\ \{\ } \bar{S_n} {\tt\ \}} {\tt\ default\ :\ \{\ } \bar{S_d} {\tt\ \}} {\tt\ \}}) \Downarrow \Delta_j
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsBssSwitchD)} & \begin{array}{c}
        \begin{array}{cccc}
            \Delta \vdash E_c \Downarrow L_c
            & \Delta \vdash E_i \Downarrow L_i
            & L_i \ne L_c
            & (\Delta, \bar{S}_d) \Downarrow \Delta_d
        \end{array}
        \\ \hline
        (\Delta, {\tt switch\ (} E_c {\tt)\ \{\ } {\tt case\ } E_1 {\tt\ :\ \{\ } \bar{S_1} {\tt\ \}} {\tt\ ...\ case\ } E_n {\tt\ :\ \{\ } \bar{S_n} {\tt\ \}} {\tt\ default\ :\ \{\ } \bar{S_d} {\tt\ \}} {\tt\ \}}) \Downarrow \Delta_d
    \end{array}
\end{array}
$$

### 3.8 Break, Continue, Break Point, Debug

$$\begin{matrix}{\tt (xsBssBr)} & (\Delta, {\tt break;}) \Downarrow \Delta \end{matrix}$$

$$\begin{matrix}{\tt (xsBssCo)} & (\Delta, {\tt continue;}) \Downarrow \Delta \end{matrix}$$

Note: ${\tt break}$ or ${\tt continue}$ outside a looping construct is not allowed. ${\tt break}$ may be used in switch case blocks but is unnecessary.

$$\begin{matrix}{\tt (xsBssBrPt)} & (\Delta, {\tt breakpoint;}) \Downarrow \Delta \end{matrix}$$

$$\begin{matrix}{\tt (xsBssBrPt)} & (\Delta, {\tt dbg\ id;}) \Downarrow \Delta \end{matrix}$$

Note: ${\tt breakpoint}$ will pause XS execution with no known way of resumption. ${\tt debug}$ operational semantics are unknown

### 3.9. Function Definition

$$
\begin{array}{rc}
    {\tt (xsBssFn)} & \begin{array}{c}
        \begin{array}{cccccc}
            \Delta \vdash E_i \Downarrow L_i
        \end{array}
        \\ \hline
        (\Delta, T_r\ {\tt fnName(T_1\ id_1\ =\ E_1,\ ...,\ T_n\ id_n\ =\ E_n )\ \{\ } \bar{S} {\tt\ \}}) \Downarrow \Delta \oplus ({\tt fnName}, (\{{\tt (id_1, L_1), ..., (id_n, L_n)}\}, \bar{S}))
    \end{array}
\end{array}
$$

### 3.10. Rule Definitions

$$
\begin{array}{rc}
    {\tt (xsBssRule)} & (\Delta, {\tt rule\ ruleName\ ruleOpts\ \ \{\ } \bar{S} {\tt\ \}}) \Downarrow \Delta \oplus ({\tt ruleName}, \bar{S})
\end{array}
$$

Note: Running a rule has the same semantics as a void function with no arguments.

### 3.11. Postfix

$$
\begin{array}{rc}
    {\tt (xsBssPostInc)} & (\Delta, X{\tt ++};) \Downarrow \Delta \oplus (X, \Delta(X) + 1)
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsBssPostDec)} & (\Delta, X{\tt --};) \Downarrow \Delta \oplus (X, \Delta(X) - 1)
\end{array}
$$

### 3.12. Label, Goto

$$
\begin{array}{rc}
    {\tt (xsBssLabel)} & \begin{array}{c}
        \begin{array}{cc}
            (\Delta, \bar{S}) \Downarrow \Delta'
        \end{array}
        \\ \hline
        (\Delta, {\tt label\ id}; \bar{S}) \Downarrow \Delta'
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsBssGoto)} & \begin{array}{c}
        \begin{array}{cc}
            (\Delta, \bar{S}) \Downarrow (\Delta', {\tt goto\ id}; \bar{S}')
            & (\Delta', {\tt label\ id}; \bar{S}) \Downarrow \Delta''
        \end{array}
        \\ \hline
        (\Delta, {\tt label\ id}; \bar{S}) \Downarrow \Delta''
    \end{array}
\end{array}
$$

### 3.13. Function Call (Statement)

$({\tt xsBssFncStmt})$ same as [2.4. Function Call (Expression)](#24-function-call-expression) with a terminating semicolon.

### 3.14. Class Definition

$$
\begin{array}{rc}
    {\tt (xsBssClsDef)} & \begin{array}{c}
        \begin{array}{c}
            \Delta \vdash E_i \Downarrow L_i
        \end{array}
        \\ \hline
        (\Delta, {\tt class\ clsName\ \{}\ T_1\ id_1\ =\ E_1;\ ...\ T_n\ id_n\ =\ E_n; {\tt\ \};}) \Downarrow \Delta \oplus ({\tt clsName}, \{ ({\tt id_1}, L_1), ..., ({\tt id_n}, L_n) \})
    \end{array}
\end{array}
$$

Classes are unused in XS and can't be instantiated afaik. This exists purely for completeness' sake.