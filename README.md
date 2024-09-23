# Ejercicio individual | (75.42) Taller de programación I 

## Alumno: Testa Santiago Tomas ( 108301 ) 

### Detalles de entrega

Pendientes:

1. Testing (mas test unitarios e integración)

2. Documentación de funciones

3. Refactor de funciones para que tengan menos de 30 lineas. Por una cuestión de tiempos se deja para el final. 

Funcionalidades probadas OK

1. SELECT 

2. INSERT INTO

3. UPDATE 

4. DELETE FROM 

### Detalles de implementación

Logica booleana: Se implemento por medio de un vector de vectores. Por cada OR se pushea un nuevo vector, donde en este vector se pushearán las expresiones simples y las negaciones ( los ANDs no son pusheados, ya que sería redundante ).

Esta implementación tiene como problema que un query con el condicional WHERE A AND AND AND {...} B resulta en WHERE A AND B. Como solución simple se planteó un chequeo en parseo que si se encuentra un condicional AND, se chequeará que el largo sea tal que el condicional sea válido, y que el proximo elemento no sea AND u OR.

Para la lógica booleana tambien se considero un arbol binario de expresiones booleanas (https://en.wikipedia.org/wiki/Binary_expression_tree). No se logró implementar debido a problemas con el manejo de memoria en Rust, aunque en mi opinión creo que es la mejor implementación posible.  

Para en ORDER BY no cargar las tablas completas en memoria se eligió "Full external sort" (https://cs186berkeley.net/notes/note8/)

