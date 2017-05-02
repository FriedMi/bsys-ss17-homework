# README hw6-t2

In dieser Aufgabe werden Sie eine einfache Shell erstellen. Folgende Art der Umsetzung soll dabei verfolgt werden:

Für die Shell gibt es 2 Objekte (Structs):

- Shell
- Command

In der *main()* Funktion wird eine Instanz der Shell erstellt und gestartet. Die Shell startet eine Loop, in der sie auf Eingaben wartet. Nach dem Return wird die Zeile eingelesen und analysiert. Die Eingaben, die es zu interpretieren gilt, sind im Modul `command.rs` bereits enthalten:

```Rust
#[derive(Debug)]
pub enum Command {
    /// Execute a command with arguments.
    Exec { prog: CString, argv: Vec<CString> },
    /// Empty command.
    Empty,
    /// Exit.
    Exit,
}
```

- 'Exec': Die Eingabe wird als Aufruf eines Programms, inkl. Parameter, ausgewertet und es wird versucht, das zugehörige Programm zu starten (mit *execvp()*). Wird das Programm nicht gefunden, so wird eine Fehlermeldung ausgegeben.
- 'Empty': Keine Eingabe, z.B. hat der Benutzer nur Return, Leerzeichen oder andere Whitespaces getippt.
- 'Exit': Dieses Kommando bedeutet, dass sich die Shell vor dem Start der nächsten Loop beendet.

Mehr Kommandos kommen in der optionalen Aufgabe Task3 dazu.

Im Modul `command.rs` wird die Funktion `FromStr` bereit gestellt, so dass das Objekt Command dieses Trait unterstützt. Diese Funktion können Sie benutzen, um sich den Input des Users in der Shell analysieren zu lassen. Als Rückgabewert erhalten Sie den entsprechenden `Command`.

Die Funktion `parse_exec` enthält den generischen Typen I mit dem Trait Bound Iterator. Das bedeutet, die Funktion verlangt von dem generischen Typen I, dass er vom Typ Iterator mit Items auf `&str` ist. Somit können Sie in der Funktion über den Parameter `args` iterieren. Bei diesen Arten von Funktionen mit Iteratoren müssen typischerweise zusätzlich Lifetime Parameter (`a) mit angegeben werden. Auf dieses Thema kommen wir noch in den nächsten Wochen zu sprechen.

Für das Shell Objekt entwerfen Sie geeignete Methoden. Die Loop der Shell hat folgenden Ablauf:

- StarteLoop
    - Gib Prompt aus und warte auf Eingabe
    - Werte Eingabe aus
        - Starte entspr. Programm
            - Wird Programm nicht gefunden oder ist Programm beendet, gehe zu 'StarteLoop'
            - Wird exit erkannt, beende Instanz und beende das Programm korrekt mit exit(0).

Wie jede Shell gibt auch Ihre Shell bei korrektem Beenden ein 'exit' aus. Eventuelle nicht behebbare Fehler beendet die Shell mit dem Exitcode 1.

In der Prompt Methode werden zum Ausgeben des Prompts und Einlesen der Nutzereingabe *unwrap()* Aufrufe benutzt. Mögliche Fehler in diesem Teil der Funktion brauchen Sie nicht auszuwerten. Fehler die hierbei auftreten können sind systembedingt und werden von uns in dieser Aufgabe nicht weiter verfolgt.

Alle anderen möglichen Fehler müssen jedoch unbedingt ausgewertet werden, da Tippfehler bei der Eingabe eines Nutzers nicht zum 'Absturz' der Shell führen dürfen.

Da Sie alle Zeicheneingaben, außer Whitespaces (Leerzeichen, TAB usw.) und `exit` als Kommandoaufruf interpretieren, wird sich Ihre Shell beim Aufruf Ihrer Programme genauso verhalten wie die normale Shell. Die zur Prozessverwaltung verwendeten Sonderzeichen wie &, | usw. werden in dieser Aufgabe nicht behandelt. Somit wird immer nur ein Programm inklusive aller angegebenen Parameter aufgerufen, auf dessen Ende die Shell wartet, um dann wieder den Prompt auszugeben und auf neue Eingaben zu warten.

```text
bsys-shell /Users/maechtel$ ps 1
  PID   TT  STAT      TIME COMMAND
    1   ??  Ss    28:42.16 /sbin/launchd
bsys-shell /Users/maechtel$ cat blabla.txt
cat: blabla.txt: No such file or directory
bsys-shell /Users/maechtel$ cat testdatei.txt
Hello you snoopy user ....
bsys-shell /Users/maechtel$ file testdatei.txt
testdatei.txt: ASCII text
bsys-shell /Users/maechtel$ exit
exit
```

Ihre Lösung sollte soweit möglich mit Methoden der 'Klasse' Command und Shell arbeiten, diese entsprechend verwenden und geeignet erweitern.

## Externe Crates

Benutzen Sie für Ihre Implementierung nur die externe Crate `nix`.

## Module und Tests

Ob und wie Sie den Code in weitere Module aufteilen wollen ist Ihnen überlassen. Schreiben Sie jedoch Ihre Unit-Tests in der Datei `unit_test_shell.rs` oder als eigenständigen Test, der von 'cargo test' aufgerufen wird, siehe auch [Testing][]. Einfache Tests können auch direkt in die Dokumentation 'codiert' werden, siehe [Documentation Tests][]

## Dokumentation

Bei dieser Aufgabe ist Ihre Dokumentation wichtig, um Ihren Programmablauf nachvollziehen zu können. Bitte dokumentieren Sie Ihre Funktionen entsprechend umfangreicher und kommentieren Sie spezielle Kniffe im Code, die Sie verwendet haben.

[Testing]: https://doc.rust-lang.org/book/testing.html
[Documentation Tests]: https://doc.rust-lang.org/book/testing.html#documentation-tests
