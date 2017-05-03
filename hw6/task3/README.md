# README hw6-t3

In dieser Aufgabe erweitern Sie die Shell von task2

Implementieren Sie dazu die neuen Commands:

- cd
- history
- '|' (pipe)

Mit **history** werden die letzten 1000 Befehle inkl NR ausgegeben. Einer dieser Befehle kann dann mit !NR nochmal in die Eingabe kopiert werden. Somit ist dies die gleiche Funktionalität (history und ! ) wir Ihre normale Shell bietet.

Die Pipe (**|**) ist hier als Command angegeben, da sie eine neue Art des Programmaufrufs implementiert. Beachten Sie dazu auch die Informationen aus der Vorlesung, insbesondere die Reihenfolge, wie die Kommandos rechts und links der jeweiligen Pipe aufzurufen sind. Scannen Sie somit die Eingabe nach Pipes und teilen Sie den String entsprechend in Substrings auf. Die Substrings entsprechen den einzelnen Programmen, die miteinander per Pipe verbunden werden.

In der Prompt Methode werden zum Ausgeben des Prompts und Einlesen der Nutzereingabe *unwrap()* Aufrufe benutzt. Mögliche Fehler in diesem Teil der Funktion brauchen Sie nicht auszuwerten. Fehler die hierbei auftreten können sind systembedingt und werden von uns in dieser Aufgabe nicht weiter verfolgt.

Alle anderen möglichen Fehler müssen jedoch unbedingt ausgewertet werden, da Tippfehler bei der Eingabe eines Nutzers nicht zum 'Absturz' der Shell führen dürfen.

Ihre Lösung kann mit Methoden der 'Klasse' Command und Shell arbeiten, jedoch von Ihnen in Ihrem eigenen Stiel modifiziert und erweitert werden.

## Externe Crates

Benutzen Sie für Ihre Implementierung nur die externe Crate `nix`.

## Module und Tests

Ob und wie Sie den Code in weitere Module aufteilen wollen ist Ihnen überlassen. Schreiben Sie jedoch Ihre Unit-Tests in der Datei `unit_test_shell_with_pipe.rs` oder als eigenständigen Test, der von 'cargo test' aufgerufen wird, siehe auch [Testing][]. Einfache Tests können auch direkt in die Dokumentation 'codiert' werden, siehe [Documentation Tests][]

## Dokumentation

Bei dieser Aufgabe ist Ihre Dokumentation wichtig, um Ihren Programmablauf nachvollziehen zu können. Bitte dokumentieren Sie Ihre Funktionen entsprechend umfangreicher und kommentieren Sie spezielle Kniffe im Code, die Sie verwendet haben.

[Testing]: https://doc.rust-lang.org/book/testing.html
[Documentation Tests]: https://doc.rust-lang.org/book/testing.html#documentation-tests
