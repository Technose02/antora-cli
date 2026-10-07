use antora_fs::Resource;
use relative_path::RelativeFile;

// tutorial/pages/diagrams.adoc
pub fn resource_file() -> (RelativeFile, Resource) {
    (
        "diagrams.adoc"
            .try_into()
            .expect("diagrams.adoc is a valid filename"),
        Resource::TextBased(String::from(
            r#"= Diagramme in AsciiDoc

Neben der Einbindung von Abbildungen als statische Dateien (``svg``,``png``, etc.) unterstützt AsciiDoc diverse __Diagrams-As-Code-DSL__s wie z.B. ``plantUML`` und ``mermaid``.

== PlantUML Beispiel _(Sequenz-Diagramm)_

.Code
[source, plantuml, opts="linenums,nowrap"]
----
[plantuml]
....
include::tutorial:image$sequence.puml[]
....
----

ifdef::building[]
[plantuml]
....
endif::[]
ifndef::building[]
[source, plantuml]
....
// da das Rendern mit Kroki in der IDE aktuell
// nicht funktioniert, wird hier stattdessen
// die PlantUML-DSL angezeigt

endif::[]
include::tutorial:image$sequence.puml[]
....

<<<

== Mermaid Beispiel _(Flowchart)_

.Code
[source, mermaid, opts="linenums,nowrap"]
----
[mermaid]
....
include::tutorial:image$flowchart.mmd[]
....
----

ifdef::building[]
[mermaid, format=png]
....
endif::[]
ifndef::building[]
[source, mermaid]
....
// da das Rendern mit Kroki in der IDE aktuell
// nicht funktioniert, wird hier stattdessen
// die Mermaid-DSL angezeigt

endif::[]
include::tutorial:image$flowchart.mmd[]
....

[NOTE]
Das _rendern_ der Diagramme wurde hier so konfiguriert (vgl. xref:antora-concepts.adoc#antora_playbook[Playbook]), dass ein _Kroki_-Server die Übersetzung der _DSL_ dynamisch in das entsprechende Bild-Artefakt übernimmt.
 +
_Kroki_ kennt viele __Diagrams-As-Code-DSL__s und ist daher hervorragend für diesen Job geeignet.

Weiter Informationen::
* https://docs.asciidoctor.org/diagram-extension/latest/diagram_types/diagrams/
* https://kroki.io/
* https://plantuml.com/de/
* https://mermaid.js.org/
"#,
        )),
    )
}
