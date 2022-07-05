# Übungsblatt 5
## Allgemeine Hinweise
Für diese und alle folgenden Praktikumsaufgaben gilt, dass Einsendungen, die in der jeweils mitgegebenen Testumgebung nicht laufen, mit null Punkten bewertet werden!
Das beinhaltet insbesondere alle Programme, die sich nicht fehlerfrei kompilieren lassen.
Da Cargo für die Ausführung verantwortlich ist, sollte das Projekt bei Ihnen am Ende mit `cargo test` ohne Fehler und Warnungen durchlaufen.


## Abgabemodus
Die Lösung ist in einem eigenen Git-Repository abzugeben.
Sie können in ihrer Lösung beliebige Hilfstypen und Module selbst definieren, jedoch dürfen die vorhandenen Testfälle nicht abgeändert werden.

Zur Lösung der Aufgaben steht für Sie dieses Repository mit
- einer Symboltabelle in [symbol_table](src/parser/structures/symbol_table.rs)
- einem Syntaxbaum in [syntax_tree](src/parser/structures/syntax_tree.rs)
- Baumknoten und Hilfsfunktionen für C1 in [syntax_c1](src/parser/syntax_c1.rs)
- einem Parser-Template in [minako-syntax](src/parser/minako_syntax.y)

zur Verfügung.

## Aufgabenstellung (100 Punkte)
Für Rust bietet die [rust-bison-skeleton](https://crates.io/crates/rust-bison-skeleton) crate ein Bison frontend an, welches Parser in Rust generiert. Nähere Informationen finden Sie in der README der crate, und in der Vorbesprechung des Übungsblatts.

Implementieren Sie die semantische Analyse in den gegebenen Parser unter Zuhilfenahme der zur Verfügung gestellten Symboltabelle. 
Die zu prüfende Programmsemantik finden Sie unten. 

Um Ihnen das Testen zu erleichtern, geben wir Ihnen eine Sammlung von Testfällen mit, die viele semantischen Fehlerquellen abdeckt.
Ein Teil der semantischen Regeln wird bereits (teilweise) geprüft.
Sie finden verschiedene `// TODO` Kommentare im Projekt, welche Ihnen Stellen aufzeigen, an denen Code ergänzt werden muss.

Im Falle eines Syntaxfehlers soll der Parser durch folgenden Aufruf unterbrochen werden:
```rust
return self.report_semantic_error("FEHLER");
```
wobei `"FEHLER"` durch eine passende Fehlermeldung ersetzt werden soll. Die aufgerufene Methode ist für _Parser_ definiert und befindet sich in [minako-syntax](src/parser/minako_syntax.y).

### Semantik
Die Grammatik von C1 finden Sie [online](https://amor.cms.hu-berlin.de/~kunert/lehre/material/c1-grammar.php).

Ein Sichtbarkeitsbereich ist ein Abschnitt des Quellcodes, der zu folgenden Metasymbolen reduziert wird:

- block
- forstatement
- functiondefinition, wobei der Funktionsname Teil des äußeren Sichtbarkeitsbereiches ist

Die Grammatik erlaubt eine Schichtung von Sichtbarkeitsbereichen. 
Die äußere Schicht wird als globale Schicht bezeichnet. 
C1 unterstützt die implizite Konvertierung von int nach float für Zuweisungen, bei Rückgabewerten und in Ausdrücken. 
Ein Typ wird als kompatibel zu einem zweiten Typ bezeichnet, wenn er in den anderen konvertierbar ist.

Folgende semantische Regeln gelten in C1:

- Es muss eine parameterlose main()-Funktion mit dem Rückgabetyp void geben.
- Alle Bezeichner müssen vor ihrer Benutzung deklariert werden.
- Eine Schicht eines Sichtbarkeitsbereiches darf keinen Bezeichner doppelt enthalten.
  - Eine innere Schicht kann einen Bezeichner einer äußeren Schicht jedoch überdecken (aka. shadowing). 
- Bei der Namensauflösung von Bezeichnern, werden die Sichtbarkeitsbereiche von innen nach außen durchsucht und der erste Treffer ausgewählt.
- Es gibt keine Variablen des Typs void.
- Ausdrücke des Typs void lassen sich nicht mit printf() ausgeben.
- Die Bedingungen für while, do while, for und if sind boolesche Ausdrücke.
- Funktionsaufrufe sind parameterkonform.
  - Parameter- und Argumentlisten haben identische Länge
  - Parameter- und Argumenttypen sind paarweise identisch (und nicht nur kompatibel!)
- Der Typ des Rückgabewertes ist kompatibel zum Rückgabetyp der aktuell definierten Funktion.
- Zuweisungen erfolgen nur in zuweisungsfähige Strukturen und der Typ der rechten Seite ist kompatibel zum Typ der linken Seite.
- Alle hier nicht betrachteten Fälle werden entsprechend des [C-Standards](https://web.archive.org/web/20181230041359if_/http://www.open-std.org/jtc1/sc22/wg14/www/abq/c17_updated_proposed_fdis.pdf) behandelt.


