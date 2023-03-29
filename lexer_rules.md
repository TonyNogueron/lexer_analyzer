# Reglas Léxicas

### Autor: Antonio Noguerón - A01423759

### Fecha: 03/27/2023

## Lenguajes de Programación escogidos

* Rust
* Python
* JavaScript

### Las categorías léxicas que tienen en común los lenguajes son:

* Identificadores
* Palabras reservadas
* Operadores Aritméticos
* Operadores Lógicos
* Separadores
* Literales numéricos
* Enteros
* Decimales
* Literales de texto
* Comentarios
* Tipo de dato

## Identificadores

Los identificadores son cadenas de caracteres que se utilizan para nombrar variables, funciones, clases, etc. En los
lenguajes de programación, los identificadores son sensibles a mayúsculas y minúsculas. Los identificadores pueden
comenzar con una letra, un guión bajo o un signo de dólar. Los identificadores pueden contener letras, números, guiones
bajos y signos de dólar. Los identificadores no pueden contener espacios y no pueden ser palabras reservadas.

## Palabras reservadas

Las palabras reservadas son palabras o cadenas de texto que tienen un significado especial para el lenguaje de
programación. No se pueden utilizar como identificadores. Las palabras reservadas son sensibles a mayúsculas y
minúsculas.

Tablas con algunas de las palabras reservadas de los lenguajes escogidos:

### Rust

| Palabra reservada | Descripción                     |
|-------------------|---------------------------------|
| as                | Convierte un valor en otro tipo |
| break             | Rompe un ciclo                  |
| if                | Condicional                     |
| else              | Condicional                     |
| enum              | Enumeración                     |
| for               | Ciclo                           |
| while             | Ciclo                           |
| fn                | Función                         |
| impl              | Implementación                  |
| let               | Declaración de variable         |

Expression regular para las palabras reservadas de Rust:

```javascript
 let RegEx = String::From("(as|break|if|else|enum|for|while|fn|impl|let)");
```

### Python

| Palabra reservada | Descripción                     |
|-------------------|---------------------------------|
| and               | Operador lógico                 |
| as                | Convierte un valor en otro tipo |
| break             | Rompe un ciclo                  |
| if                | Condicional                     |
| else              | Condicional                     |
| elif              | Condicional                     |
| for               | Ciclo                           |
| while             | Ciclo                           |
| def               | Función                         |

Expression regular para las palabras reservadas de Python:

```javascript
let RegEx = String::From("(and|as|break|if|else|elif|for|while|def)");
```

### JavaScript

| Palabra reservada | Descripción                           |
|-------------------|---------------------------------------|
| as                | Convierte un valor en otro tipo       |
| break             | Rompe un ciclo                        |
| if                | Condicional                           |
| else              | Condicional                           |
| for               | Ciclo                                 |
| while             | Ciclo                                 |
| function          | Función                               |
| let               | Declaración de una variable           |
| var               | Declaración de una variable global    |
| const             | Declaración de una variable constante |

Expression regular para las palabras reservadas de JavaScript:

```javascript
RegEx = String::From("(as|break|if|else|for|while|function|let|var|const)");
```

Expression regular para las palabras reservadas de los tres lenguajes:

```javascript
let RegEx = String::From("(as|break|if|else|enum|for|while|fn|impl|let|and|as|break|if|else|elif|for|while|def|as|break|if|else|for|while|function|let|var|const)");
```

## Operadores Aritméticos

Los operadores aritméticos son símbolos que se utilizan para realizar operaciones aritméticas. Los operadores
aritméticos son sensibles a mayúsculas y minúsculas.

Tabla con los operadores aritméticos de los lenguajes escogidos:

| Lenguaje   | Operador | Descripción    |
|------------|----------|----------------|
| Rust       | +        | Suma           |
| Rust       | -        | Resta          |
| Rust       | *        | Multiplicación |
| Rust       | /        | División       |
| Rust       | %        | Módulo         |
| Python     | +        | Suma           |
| Python     | -        | Resta          |
| Python     | *        | Multiplicación |
| Python     | /        | División       |
| Python     | %        | Módulo         |
| JavaScript | +        | Suma           |
| JavaScript | -        | Resta          |
| JavaScript | *        | Multiplicación |
| JavaScript | /        | División       |
| JavaScript | %        | Módulo         |

Expression regular para los operadores aritméticos de los tres lenguajes:

```javascript
let RegEx = String::From("(\+|\-|\*|\/|\%)");
```

## Operadores Lógicos

Los operadores lógicos son símbolos que se utilizan para realizar operaciones lógicas. L
os operadores lógicos son sensibles a mayúsculas y minúsculas.

Tabla con los operadores lógicos de los lenguajes escogidos:

| Lenguaje   | Operador | Descripción |
|------------|--------|-----------|
| Rust       | &&     | Y lógico  |
| Rust       |        | O lógico   |      
| Rust       | !      | No lógico |
| Python     | and    | Y lógico  |
| Python     | or     | O lógico  |
| Python     | not    | No lógico |
| JavaScript | &&     | Y lógico  |
| JavaScript |        | O lógico    |
| JavaScript | !      | No lógico |

Expression regular para los operadores lógicos de los tres lenguajes:

```javascript
let RegEx = String::From("(&&|\|\||!|and|or|not)");
```

## Separadores

Los separadores son símbolos que se utilizan para separar los elementos de un programa. Como lo es el espacio,
tabulador, salto de línea, y punto y coma. " ", "\n", "\t", ";".

Tabla con los separadores de los lenguajes escogidos:

| Lenguaje   | Separador |
|------------|-----------|
| Rust       | " "       |
| Rust       | "\n"      |
| Rust       | "\t"      |
| Rust       | ";"       |
| Python     | " "       |
| Python     | "\n"      |
| Python     | "\t"      |
| Python     | ";"       |
| JavaScript | " "       |
| JavaScript | "\n"      |
| JavaScript | "\t"      |
| JavaScript | ";"       |

Expresión regular para los separadores de los tres lenguajes:

```javascript
let RegEx = String::From("( |\\n|\\t|;)");
```

## Literales numéricos

Los literales numéricos son números que se utilizan en un programa. Digitos del 0 al 9. Los literales numéricos pueden ser enteros o
decimales. Los literales numéricos pueden ser positivos o negativos.

Expresión regular para los literales numéricos de los tres lenguajes:

```javascript
let RegEx = String::From("(-?[0-9]+)");
```

## Literales de texto

Los literales de texto son cadenas de caracteres que se utilizan en un programa. Los literales de texto pueden ser
simples o dobles. Los literales de texto pueden contener caracteres especiales.

Expresión regular para los literales de texto de los tres lenguajes:

```javascript
let RegEx = String::From("(\".*\")");
```

## Comentarios

Los comentarios son líneas de texto que se utilizan para documentar un programa. Los comentarios no se ejecutan. Los
comentarios pueden ser de una sola línea o de varias líneas.

Tabla con los comentarios de los lenguajes escogidos:

| Lenguaje   | Comentario de una línea | Comentario de varias líneas |
|------------|-------------------------|-----------------------------|
| Rust       | //                      | /* */                       |
| Python     | #                       | """ """                     |
| JavaScript | //                      | /* */                       |

Expresión regular para los comentarios de los tres lenguajes:

```javascript
let RegEx = String::From("(//|#|/\*|\*/|\"\"\")");
```

## Tipo de dato

Los tipos de datos son los tipos de datos que se utilizan en un programa. Los tipos de datos pueden ser numéricos, de
texto o booleanos.

Tabla con los tipos de datos más comúnes de los lenguajes escogidos:

| Lenguaje   | Tipo de dato numérico | Tipo de dato de texto | Tipo de dato booleano |
|------------|-----------------------|-----------------------|-----------------------|
| Rust       | i32                   | String                | bool                  |
| Python     | int                   | str                   | bool                  |
| JavaScript | number                | string                | boolean               |


Expresión regular para los tipos de datos de los tres lenguajes:

```javascript
let RegEx = String::From("(i32|int|number|String|str|string|bool|boolean)");
```