# Ejercicio individual | (75.42) Taller de programación I 

## Alumno: Testa Santiago Tomas ( 108301 ) 

### Detalles de entrega

El trabajo no se encuentra completo. 

Pendientes:

1. Funcionalidad: Logica booleana en WHERE

2. Funcionalidad: SELECT c/ ORDER BY condicionado ( con WHERE, se implementará primero la logica booleana y luego este punto )

3. Testing (mas test unitarios e integración)

4. Documentación de funciones

5. Refactor de funciones para que tengan menos de 30 lineas. Por una cuestión de tiempos se deja para el final. 

Funcionalidades probadas OK

1. SELECT * FROM 

2. SELECT columnas FROM 

3. SELECT c/ WHERE (basico)

4. INSERT INTO 

5. UPDATE 

6. DELETE  FROM WHERE (basico)

7. SELECT c/ ORDER BY incondicional

### Detalles de implementación

Para la lógica booleana el objetivo era crear un arbol de condiciones formado por las relaciones de precedencia. 

Dicho objetivo no pudo ser implementado. Se implementó lógica de condiciones simples ( no compuestas ) 

Para en ORDER BY no cargar las tablas completas en memoria se eligió "Full external sort" (https://cs186berkeley.net/notes/note8/)

