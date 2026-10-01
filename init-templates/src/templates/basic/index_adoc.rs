use antora_fs::Resource;
use antora_project::component_version::ComponentVersion;
use init_task::InitAssistantResults;
use relative_path::RelativeFile;

pub(super) fn relative_file() -> RelativeFile {
    "index.adoc"
        .try_into()
        .expect("is a valid relative-path to a file")
}

pub fn content(results: &InitAssistantResults, component_version: &ComponentVersion) -> Resource {
    let component_name = results.component_name().as_ref();
    let component_title = results.component_title();

    let rest_of_first_sentence = if component_version.is_empty() {
        format!("unversionierte Komponente \"{component_name}\"")
    } else {
        format!("Komponente \"{component_name}\" in der Version {component_version}")
    };

    Resource::TextBased(format!(
        r#"= Willkommen zu Ihrem Antora-Projekt: {component_title}

Dies ist die Startseite Ihres neuen Antora-Dokumentationsprojekts für die {rest_of_first_sentence}.

Diese Seite dient als Ausgangspunkt für Ihre Dokumentation.

Passen Sie diese Seite an oder ersetzen Sie sie durch Ihre eigenen Inhalte.

== Nächste Schritte

Um Ihre Dokumentation für "{component_title}" weiter aufzubauen, empfehlen wir folgende Schritte:

* Legen Sie weitere benannte _Module_ an, um Ihre Inhalte logisch zu strukturieren.
* Erstellen Sie neue Seiten und weitere Inhalte in den jeweiligen Modulen, um Ihre Dokumentation zu füllen.
* Nutzen Sie die _Collector-Erweiterung_ in Ihrer `antora.yml`, um externe Inhalte (z.B. Beispiele, Konfigurationen) in die Dokumentation einzubinden.
* Verwenden Sie _Navigations_-Dateien (`nav.adoc`), um eine klare und benutzerfreundliche Menüstruktur zu schaffen.
* Experimentieren Sie mit AsciiDoc-Funktionen wie _Includes_, _Cross-Referencing_ und Diagrammen, um Ihre Dokumentation lebendig und wartbar zu gestalten.

Weitere Informationen finden Sie in der https://docs.antora.org[Antora-Dokumentation]. +
Die Navigation finden Sie links.

[plantuml, format=svg]
....
@startuml
skinparam ActivityBackgroundColor #EDF7EE
skinparam ActivityBorderColor #3F8F4F
skinparam ArrowColor #555555
skinparam NoteBackgroundColor #FFF9E6
skinparam NoteBorderColor #C9A227

title Nächste Schritte: test01 weiter aufbauen

start
repeat
  :Weitere benannte **Module** anlegen;
  note right
    Logische Struktur für Ihre Inhalte
  end note

  :Neue **Seiten und Inhalte** in den\njeweiligen Modulen erstellen;

  :Externe Inhalte mit der\n**Collector-Erweiterung** in der\nantora.yml einbinden;
  note right
    z. B. Beispiele, Konfigurationen
  end note

  :**Navigations**-Dateien (nav.adoc) pflegen;
  note right
    Klare, benutzerfreundliche Menüstruktur
  end note

  :Mit AsciiDoc-Funktionen experimentieren:\n**Includes**, Cross-Referencing, Diagramme;
  note right
    Lebendig und wartbar gestalten
  end note
repeat while (Noch mehr für test01?) is (Ja)

:Lebendige, gepflegte Dokumentation;
note right
  Weiter lernen: https://docs.antora.org
  Tipp: Die Navigation finden Sie links
end note
stop
@enduml
....

Viel Erfolg bei der Erstellung Ihrer Dokumentation für "{component_title}"!
"#
    ))
}
