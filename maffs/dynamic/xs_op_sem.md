# XS Operational Semantics

This is a formal description of how a [well typed XS program](../static/xs_type_chk.md#11-well-typed-programs) runs.

## 1. Notation

- $\Delta$ is a memory environment mapping XS identifiers to values
- $\Delta \vdash E_1 \Downarrow E_2$ means that $E_1$ reduces to $E_2$ in $\Delta$ (read as $\Delta$ yields $E_1$ reduced to $E_2$)
- $(\Delta, S) \Downarrow \Delta'$ means that running statement $S$ changes the memory environment to $\Delta'$
- $$\begin{array}{rc}
    {\tt (xsBssCase)} & \begin{array}{c}
    \begin{array}{cc} C_1 & C_2 \end{array}
    \\ \hline
    S_1
    \end{array}
    \end{array}
    $$ is read as $C_1 \land C_2 \implies S_1$

## 2. Big Step Semantics For Expressions

### 2.1. Literals

let $L$ denote a literal

$$
\Delta \vdash L \Downarrow L
$$

### 2.2. Identifiers

let $X$ be an identifier, $V$ be a value

$$
\begin{array}{rc}
    {\tt (xsBssId)} & \begin{array}{c}
        (X, V) \in \Delta
        \\ \hline
        \Delta \vdash X \Downarrow V
    \end{array}
\end{array}
$$

### 2.3. Parenthesis

$$
\begin{array}{rc}
    {\tt (xsBssParen)} & \begin{array}{c}
        \Delta \vdash E \Downarrow V
        \\ \hline
        \Delta \vdash (E) \Downarrow V
    \end{array}
\end{array}
$$

### 2.4. Function Call (Expression)

let $L$ be a source location (`file::line_no`)

$$
\begin{array}{rc}
    {\tt (xsBssFncExpr)} & \begin{array}{c}
        \begin{array}{ccc}
            ({\tt fnName}, L) \in \Delta
            & \Delta \vdash E_i \Downarrow V_i
            & \Delta \vdash {\tt fnName(V_1, ..., V_i)} \Downarrow V_{r}
        \end{array}
        \\ \hline
        \Delta \vdash {\tt fnName(E_1, ..., E_n)} \Downarrow V_r
    \end{array}
\end{array}
$$

### 2.5. Operations

$$
\begin{array}{rc}
    {\tt (xsBssOp)} & \begin{array}{c}
        \begin{array}{ccc}
            \Delta \vdash E_1 \Downarrow V_1
            & \Delta \vdash E_2 \Downarrow V_2
            & \Delta \vdash V_1\ {\tt op}\ V_2 \Downarrow V_3
        \end{array}
        \\ \hline
        \Delta \vdash E_1\ {\tt op}\ E_2 \Downarrow V_3
    \end{array}
\end{array}
$$

## 3. Big Step Semantics For Expressions

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

let $X$ be a named XS program

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
            \Delta \vdash E \Downarrow V
        \end{array}
        \\ \hline
        (\Delta, X\ =\ E;) \Downarrow \Delta \oplus (X, V)
    \end{array}
\end{array}
$$

### 3.4. If Else

$$
\begin{array}{rc}
    {\tt (xsBssIfT)} & \begin{array}{c}
        \begin{array}{cc}
            \Delta \vdash E_c \Downarrow {\tt true}
            & (\Delta, S) \Downarrow \Delta_t
        \end{array}
        \\ \hline
        (\Delta, {\tt if\ (} E_c {\tt)\ \{\ } \bar{S} {\tt\ \}} \Downarrow \Delta_t
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsBssIfF)} & \begin{array}{c}
        \begin{array}{cc}
            \Delta \vdash E_c \Downarrow {\tt false}
        \end{array}
        \\ \hline
        (\Delta, {\tt if\ (} E_c {\tt)\ \{\ } \bar{S} {\tt\ \}} \Downarrow \Delta
    \end{array}
\end{array}
$$

$$
\begin{array}{rc}
    {\tt (xsBssIfElseT)} & \begin{array}{c}
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
    {\tt (xsBssIfElseF)} & \begin{array}{c}
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

