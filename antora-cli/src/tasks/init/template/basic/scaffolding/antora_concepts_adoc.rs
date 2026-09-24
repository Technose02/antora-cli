use antora_fs::Resource;
use relative_path::RelativeFile;

use crate::tasks::init::assistant::InitAssistantResults;

// scaffolding/pages/antora-concepts.adoc
pub fn relative_file() -> RelativeFile {
    "antora-concepts.adoc"
        .try_into()
        .expect("antora-concepts.adoc is a valid filename")
}

pub fn content(results: &InitAssistantResults) -> Resource {
    let component_name = results.component_name().as_ref();
    let content_source_root = results.content_source_root();
    Resource::TextBased(format!(
        r#"= Grundkonzepte von Antora

Antora organisiert Dokumentation in Komponenten, Modulen und Versionen.

== Komponenten

Eine Komponente ist eine logische Einheit Ihrer Dokumentation, z.B. ein Produkt oder eine Bibliothek.

Ihre Komponente ``{component_name}`` ist unter der _Content Source Root_ ``{content_source_root}`` angelegt wurden.

Die Metadaten zu Ihrer Komponente sind im _Component Version Descriptor_ ``antora.yml`` verankert:

.Component Version Descriptor von {component_name}  (über Collector-Extension eingebunden)
[source, yaml, opts="linenums,nowrap"]
----
include::example$collected/antora.yml[]
----

Weitere Informationen::
* https://docs.antora.org/antora/latest/component-name-and-version/
* https://docs.antora.org/antora/latest/organize-content-files/#classifying-your-content-source-files
* https://docs.antora.org/antora/latest/content-source-repositories/#content-source-root

<<<

== Module

Module gruppieren Inhalte innerhalb einer Komponente, z.B. das Standardmodul _ROOT_ oder benannte Module wie z.B. "scaffolding".

Weitere Informationen::
* https://docs.antora.org/antora/latest/module-directories/
* https://docs.antora.org/antora/latest/root-module-directory/

== Versionen

Versionen erlauben parallele Dokumentation für verschiedene Releases.

Weitere Informationen::
* https://docs.antora.org/antora/latest/content-source-versioning-methods/
* https://docs.antora.org/antora/latest/component-name-and-version/

== Navigation

Navigation wird über `nav.adoc` Dateien gesteuert, die Menüs definieren.

Weitere Informationen::
* https://docs.antora.org/antora/latest/navigation/

== Cross-Referencing

Mit `xref:` können Sie auf andere Seiten verlinken, z.B.:

.Code
[source, asciidoc, opts="linenums,nowrap"]
----
xref:scaffolding:index.adoc[Startseite]
----

xref:scaffolding:index.adoc[Startseite]

Weitere Informationen::
* https://docs.antora.org/antora/latest/navigation/xrefs-and-link-text/

<<<

== Codebeispiel mit Syntax-Highlighting

Hier ein Beispiel für die Darstellung eines Playbooks mit Zeilennummern, Caption und Syntax-Highlighting:

.Code
[source, asciidoc, opts="linenums,nowrap"]
----
.Ihre antora-playbook.yml (über Collector-Extension eingebunden)
[[antora_playbook]]
[source,yaml,opts="linenums,nowrap"]
-----
\include::example$collected/antora-playbook.yml[]
-----
----

.Ihre antora-playbook.yml (über Collector-Extension eingebunden)
[[antora_playbook]]
[source,yaml,opts="linenums,nowrap"]
----
include::example$collected/antora-playbook.yml[]
----

Weitere Informationen::
* https://docs.antora.org/antora-ui-default/code-blocks/
* https://docs.antora.org/antora/latest/page/include-an-example/
* https://docs.antora.org/collector-extension/latest/use-cases/
* https://docs.antora.org/antora/latest/playbook/

<<<"#,
    ))
}
