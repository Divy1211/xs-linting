# XS Maximal Munch

Generation of PA from XS.

## 1. Notation

- $M_e(E) \vdash ({\tt d}, {\tt lis})$ is a function which yields the PA labeled instructions ${\tt lis}$ which compute the value of the expression $E$ and stores the result in ${\tt d}$
- $M_s(S) \vdash {\tt lis}$ is a function which yields the PA labeled instructions ${\tt lis}$ for statement $S$
- ${\tt newId} \vdash {\tt m}$ is a generator which yields a new and unique identifier
- ${\tt newAddr} \vdash {\tt l}$ is a generator which yields a new and unique address for a PA instruction
- ${\tt inFn} \vdash {\tt fnName}$ is a function that remembers the name of the current function
- $$\begin{array}{rc}
    {\tt (xsMmCase)} & \begin{array}{c}
        \begin{array}{cc} C_1 & C_2 \end{array}
        \\ \hline
        S_1
    \end{array}
\end{array}
$$

    is read as $C_1 \land C_2 \implies S_1$

## 2. MM for Expressions

### 2.1. PA Operand

$$
\begin{array}{rc}
    {\tt (xsMmSrc)} & M_e({\tt s}) \vdash ({\tt s}, \texttt{[]})
\end{array}
$$

### 2.2. Parenthesis

$$
\begin{array}{rc}
    {\tt (xsMmParen)} & \begin{array}{c}
        M_e(E) \vdash ({\tt d}, {\tt lis})
        \\ \hline
        M_e((E)) \vdash ({\tt d}, {\tt lis})
    \end{array}
\end{array}
$$

### 2.2. Function Call (Expression)

$$
\begin{array}{rc}
    {\tt (xsMmFncExpr)} & \begin{array}{c}
        \begin{array}{c}
            M_e(E_i) \vdash ({\tt d_i}, {\tt lis_i})
            \\ {\tt newId} \vdash {\tt d_r}
            \\ {\tt newAddr} \vdash {\tt l_r}
        \end{array}
        \\ \hline
        M_e({\tt fnName(E_1, ..., E_n)}) \vdash \left({\tt d_r}, \begin{array}{c}
            {\tt lis_1\ +\ ...\ +\ lis_n}
            \\ {\tt +\ [push\ d_n, ..., push\ d_1]}
            \\ {\tt +\ [jmp\ fnName]}
            \\ {\tt +\ [dealloc\ n]}
            \\ {\tt +\ [ l_r : d_r \leftarrow r_{ret} ]}
        \end{array}\right)
    \end{array}
\end{array}
$$

### 2.4. Operations

$$
\begin{array}{rc}
    {\tt (xsMmOp)} & \begin{array}{c}
        \begin{array}{c}
            M_e(E_1) \vdash ({\tt d_1}, {\tt lis_1})
            \\ M_e(E_2) \vdash ({\tt d_2}, {\tt lis_2})
            \\ {\tt newId} \vdash {\tt d_3}
            \\ {\tt newAddr} \vdash {\tt l}
        \end{array}
        \\ \hline
        M_e(E_1\ {\tt op}\ E_2) \vdash ({\tt d_3}, {\tt lis_1\ +\ lis_2\ +\ } {\tt [ l : d_3 \leftarrow d_1\ op\ d_2 ]})
    \end{array}
\end{array}
$$
